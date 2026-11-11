//! The session Timeline (REQ-history-002): every transition is appended to the active
//! session's `session.md` in the same save as the checkpoint change it goes with, and
//! read back from there. No event file, cycle directory, cursor or transitions log is
//! written. Evidence cleanup's protection fingerprint stays here until plan step 8
//! removes it.

use crate::checkpoint::{Checkpoint, read_checkpoint_unlocked};
use crate::records::{local_path, print_json};
use crate::{evidence, session_md, stage};
use anyhow::{Context, Result, ensure};
use clap::Subcommand;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// `chough next` reads at most this many of the newest entries (REQ-history-003).
pub(super) const RECENT_EVENTS: usize = 128;
pub(super) const RECENT_BYTES: u64 = 512 * 1_024;
/// One timeline holds at most this many entries (REQ-history-003).
pub(super) const CYCLE_EVENTS: usize = session_md::MAX_ENTRIES;
/// `chough timing` reads at most this much timeline content in one request.
pub(super) const CYCLE_BYTES: u64 = 16 * 1_048_576;
#[allow(dead_code)] // removed at integration (t382)
const EVENT_BYTES: u64 = 65_536;
#[allow(dead_code)] // removed at integration (t382)
const SOURCE_FILES: usize = 16_384;
#[allow(dead_code)] // removed at integration (t382)
const SOURCE_BYTES: u64 = 64 * 1_048_576;
#[allow(dead_code)] // removed at integration (t382)
const RECEIPT_BYTES: u64 = 1_048_576;

thread_local! {
    // One ephemeral budget covers inventory and every subsequent protection
    // revalidation in the same maintenance operation, including repeated reads.
    static MAINTENANCE: std::cell::Cell<Option<(usize, u64)>> = const { std::cell::Cell::new(None) };
}

#[allow(dead_code)] // removed at integration (t382)
pub(super) struct MaintenanceGuard {
    owner: bool,
}

impl Drop for MaintenanceGuard {
    fn drop(&mut self) {
        if self.owner {
            MAINTENANCE.with(|budget| budget.set(None));
        }
    }
}

#[allow(dead_code)] // removed at integration (t382)
pub(super) fn maintenance_budget() -> MaintenanceGuard {
    let owner = MAINTENANCE.with(|budget| {
        if budget.get().is_some() {
            false
        } else {
            budget.set(Some((0, 0)));
            true
        }
    });
    MaintenanceGuard { owner }
}

#[allow(dead_code)] // removed at integration (t382)
fn charge_maintenance(bytes: u64) -> Result<()> {
    MAINTENANCE.with(|budget| -> Result<()> {
        if let Some((files, used)) = budget.get() {
            ensure!(
                files < SOURCE_FILES && used.saturating_add(bytes) <= SOURCE_BYTES,
                "incomplete maintenance scan or revalidation: 16384-file/64-MiB metadata bound"
            );
            budget.set(Some((files + 1, used + bytes)));
        }
        Ok(())
    })
}

#[allow(dead_code)] // removed at integration (t382)
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[allow(dead_code)] // removed at integration (t382)
fn bounded_bytes(root: &Path, name: &str, limit: u64) -> Result<Vec<u8>> {
    use std::io::Read;
    let path = local_path(root, name)?;
    let mut file = fs::File::open(&path).with_context(|| format!("unavailable {name}"))?;
    ensure!(
        file.metadata()?.is_file(),
        "not a regular history source: {name}"
    );
    ensure!(
        file.metadata()?.len() <= limit,
        "{name} exceeds the {limit}-byte history bound"
    );
    charge_maintenance(file.metadata()?.len())?;
    let mut data = Vec::new();
    std::io::Read::by_ref(&mut file)
        .take(limit)
        .read_to_end(&mut data)?;
    ensure!(
        file.metadata()?.len() == data.len() as u64,
        "{name} changed while reading within the {limit}-byte history bound"
    );
    Ok(data)
}

/// Transitions read from a timeline, newest first, with what could not be read.
#[derive(Default)]
pub(super) struct HistoryRead {
    pub(super) records: Vec<Value>,
    pub(super) partial: Vec<String>,
    pub(super) bytes: u64,
    pub(super) lineages: Vec<Value>,
}

/// Queue a transition on the checkpoint; the next save appends it to the Timeline in
/// the same atomic replace as the checkpoint change (REQ-history-002).
pub(super) fn append(
    _root: &Path,
    session: &mut Checkpoint,
    transition: stage::Transition,
) -> Result<()> {
    ensure!(
        transition.schema_version == 1
            && stage::is_stage(&transition.from)
            && stage::is_stage(&transition.to),
        "invalid transition payload"
    );
    stage::validate_records(&transition)?;
    let mut transition = transition;
    if transition.actor.is_none() {
        transition.actor = crate::actor::Actor::current();
    }
    session.pending.push(transition);
    Ok(())
}

/// The acting agent of the session's newest Timeline entry that names one
/// (REQ-records-007), read leniently: an unreadable entry is passed over.
pub(super) fn last_actor(session: &Checkpoint) -> Option<crate::actor::Actor> {
    let read = selected(session, CYCLE_EVENTS, CYCLE_BYTES, true);
    read.records
        .iter()
        .rev()
        .find_map(|entry| serde_json::from_value(entry.get("actor")?.clone()).ok())
}

/// The session's transitions, oldest first, within `limit` entries and `byte_limit`
/// bytes: the current cycle (from the newest reset) unless `previous_cycles`, when the
/// whole timeline is read. Transitions queued for the next save are included.
pub(super) fn selected(
    session: &Checkpoint,
    limit: usize,
    byte_limit: u64,
    previous_cycles: bool,
) -> HistoryRead {
    let mut result = HistoryRead::default();
    let saved = match session.source.as_ref().map(session_md::timeline) {
        Some(Ok(entries)) => entries.into_iter().map(|entry| entry.transition).collect(),
        Some(Err(error)) => {
            result
                .partial
                .push(format!("the timeline cannot be read: {error:#}"));
            return result;
        }
        None => vec![],
    };
    let transitions: Vec<&stage::Transition> = saved.iter().chain(session.pending.iter()).collect();
    // Walk newest first so the bounds keep the newest entries.
    for transition in transitions.into_iter().rev() {
        let value = serde_json::to_value(transition).expect("serializable transition");
        let bytes = value.to_string().len() as u64;
        if result.records.len() >= limit || result.bytes + bytes > byte_limit {
            result
                .partial
                .push("the timeline exceeds the entry or byte read bound".into());
            break;
        }
        result.bytes += bytes;
        let reset = transition.kind == "reset";
        result.records.push(value);
        if reset && !previous_cycles {
            break;
        }
    }
    // Oldest first, as every reader expects.
    result.records.reverse();
    result.lineages.push(json!({
        "session": session.meta.id,
        "selection": "session timeline",
        "durations": "derived from this session's timeline"
    }));
    result
}

/// The timeline a timing report reads: the whole of the session's timeline, read
/// leniently (REQ-reporting-001): an unreadable time is unknown and an unreadable entry
/// is passed on as its line for timing to count, within the timing bounds.
pub(super) fn timing(session: &Checkpoint) -> HistoryRead {
    let mut result = HistoryRead::default();
    let mut values: Vec<Value> = session
        .source
        .as_ref()
        .map(session_md::timeline_for_timing)
        .unwrap_or_default();
    values.extend(
        session
            .pending
            .iter()
            .map(|transition| serde_json::to_value(transition).expect("serializable transition")),
    );
    let skip = values.len().saturating_sub(CYCLE_EVENTS);
    if skip > 0 {
        result.partial.push(format!(
            "earliest {skip} entries skipped beyond the {CYCLE_EVENTS}-entry timing bound"
        ));
    }
    for value in values.into_iter().skip(skip) {
        let bytes = value.to_string().len() as u64;
        if result.bytes + bytes > CYCLE_BYTES {
            result
                .partial
                .push("the timeline exceeds the 16 MiB timing bound".into());
            break;
        }
        result.bytes += bytes;
        result.records.push(value);
    }
    result.lineages.push(json!({
        "session": session.meta.id,
        "selection": "session timeline",
        "durations": "derived from this session's timeline"
    }));
    result
}

#[derive(Subcommand)]
pub(super) enum HistoryCommand {
    /// Show the active session's timeline, oldest first, with what could not be read.
    /// With no open session, it shows the newest archived story's.
    Show,
}

pub(super) fn run(root: &Path, command: HistoryCommand) -> Result<()> {
    match command {
        HistoryCommand::Show => {
            // With no open session, the newest archived story is the one to show, as
            // `chough timing` reads it after done.
            let (session, archived) = match read_checkpoint_unlocked(root) {
                Ok(session) => (session, false),
                Err(error) if crate::sessions::open_ids(root)?.is_empty() => {
                    match crate::sessions::newest_archived(root)? {
                        Some(crate::checkpoint::Session::V2(session)) => (*session, true),
                        None => return Err(error),
                    }
                }
                Err(error) => return Err(error),
            };
            let read = selected(&session, CYCLE_EVENTS, CYCLE_BYTES, true);
            // An entry that is not a valid transition is refused, never shown as history.
            if let Some(invalid) = read
                .partial
                .iter()
                .find(|why| why.contains("invalid transition"))
            {
                anyhow::bail!("{invalid}");
            }
            print_json(&json!({
                "schema_version": 1,
                "session": session.meta.id,
                "archived": archived,
                "complete": read.partial.is_empty(),
                "partial": read.partial,
                "entries": read.records,
                "limits": {"entry_bytes": session_md::ENTRY_BYTES, "recent_events": RECENT_EVENTS, "timeline_entries": CYCLE_EVENTS, "timing_bytes": CYCLE_BYTES},
            }))
        }
    }
}

#[allow(dead_code)] // removed at integration (t382)
#[derive(Default)]
struct Budget {
    files: usize,
    bytes: u64,
    directories: usize,
}

#[allow(dead_code)] // removed at integration (t382)
fn walk(
    root: &Path,
    relative: &str,
    limit: u64,
    budget: &mut Budget,
    output: &mut BTreeMap<String, Vec<u8>>,
    depth: usize,
) -> Result<()> {
    ensure!(depth <= 16, "history maintenance directory depth exceeded");
    let path = local_path(root, relative)?;
    if !path.exists() {
        return Ok(());
    }
    let metadata = fs::symlink_metadata(&path)?;
    ensure!(
        !metadata.file_type().is_symlink(),
        "unsafe symlink at {relative}"
    );
    if metadata.is_dir() {
        budget.directories += 1;
        ensure!(
            budget.directories <= SOURCE_FILES,
            "incomplete maintenance scan: directory bound"
        );
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            ensure!(
                entries.len() < SOURCE_FILES,
                "incomplete maintenance scan: directory entry bound"
            );
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| anyhow::anyhow!("non-UTF-8 maintenance path"))?;
            entries.push(name);
        }
        entries.sort();
        for name in entries {
            walk(
                root,
                &format!("{relative}/{name}"),
                limit,
                budget,
                output,
                depth + 1,
            )?;
        }
    } else {
        ensure!(metadata.is_file(), "unsafe maintenance source {relative}");
        ensure!(
            budget.files < SOURCE_FILES && budget.bytes + metadata.len() <= SOURCE_BYTES,
            "incomplete maintenance scan: 16384-file/64-MiB metadata bound"
        );
        let bytes = bounded_bytes(root, relative, limit.min(SOURCE_BYTES - budget.bytes))?;
        budget.files += 1;
        budget.bytes += bytes.len() as u64;
        output.insert(relative.to_string(), bytes);
    }
    Ok(())
}

#[allow(dead_code)] // removed at integration (t382)
fn protection_sources(
    root: &Path,
    budget: &mut Budget,
    include_artifacts: bool,
) -> Result<BTreeMap<String, Vec<u8>>> {
    let mut sources = BTreeMap::new();
    for directory in [
        ".chough/sessions",
        ".chough/bugs",
        ".chough/cases",
        ".chough/evidence",
        ".chough/validation",
        ".specsync/changes",
        ".specsync/archive",
    ] {
        walk(root, directory, RECEIPT_BYTES, budget, &mut sources, 0)?;
    }
    if include_artifacts {
        walk(
            root,
            ".chough/artifacts",
            RECEIPT_BYTES,
            budget,
            &mut sources,
            0,
        )?;
    }
    for (path, bytes) in &sources {
        if path.ends_with(".json") {
            let value: Value = serde_json::from_slice(bytes)
                .with_context(|| format!("unreadable protection source {path}"))?;
            if path.starts_with(".chough/evidence/") {
                ensure!(
                    bytes.len() as u64 <= EVENT_BYTES,
                    "oversized evidence protection source {path}"
                );
                let record: evidence::Evidence = serde_json::from_value(value.clone())
                    .with_context(|| format!("invalid evidence protection source {path}"))?;
                evidence::validate_record(
                    &record,
                    path.rsplit('/')
                        .next()
                        .context("evidence source has no name")?,
                )?;
            }
            if path.starts_with(".chough/bugs/") {
                let record: crate::bugs::Bug = serde_json::from_value(value.clone())
                    .with_context(|| format!("invalid bug protection source {path}"))?;
                ensure!(
                    value["schema_version"] == 1 && path.ends_with(&format!("/{}.json", record.id)),
                    "invalid bug protection identity"
                );
            }
        }
        if !path.starts_with(".chough/artifacts/") {
            std::str::from_utf8(bytes)
                .with_context(|| format!("unreadable protection source {path}"))?;
        }
    }
    Ok(sources)
}

#[allow(dead_code)] // removed at integration (t382)
/// Used by evidence cleanup to notice newly added or changed reference records.
pub(super) fn protection_fingerprint(root: &Path) -> Result<String> {
    source_fingerprint(root, false)
}

#[allow(dead_code)] // removed at integration (t382)
fn source_fingerprint(root: &Path, include_artifacts: bool) -> Result<String> {
    let sources = protection_sources(root, &mut Budget::default(), include_artifacts)?;
    let mut hash = Sha256::new();
    for (path, bytes) in sources {
        hash.update(path.as_bytes());
        hash.update([0]);
        hash.update(digest(&bytes));
    }
    Ok(format!("{:x}", hash.finalize()))
}
