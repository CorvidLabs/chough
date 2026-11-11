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
        .find_map(|entry| serde_json::from_value(entry.get("by")?.clone()).ok())
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
    let saved: Vec<(Option<String>, stage::Transition)> =
        match session.source.as_ref().map(session_md::timeline) {
            Some(Ok(entries)) => entries
                .into_iter()
                .map(|entry| (Some(entry.id), entry.transition))
                .collect(),
            Some(Err(error)) => {
                result
                    .partial
                    .push(format!("the timeline cannot be read: {error:#}"));
                return result;
            }
            None => vec![],
        };
    let transitions: Vec<(Option<&str>, &stage::Transition)> = saved
        .iter()
        .map(|(id, transition)| (id.as_deref(), transition))
        .chain(session.pending.iter().map(|transition| (None, transition)))
        .collect();
    // Walk newest first so the bounds keep the newest entries.
    for (id, transition) in transitions.into_iter().rev() {
        let mut value = serde_json::to_value(transition).expect("serializable transition");
        // The entry ID a correction names it by (REQ-history-015); a queued entry has
        // none until it is saved.
        if let Some(id) = id {
            value["id"] = json!(id);
        }
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

/// How many corrections `chough next` lists (REQ-history-015).
const CORRECTIONS_LISTED: usize = 10;
/// The cut of each listed correction's text.
const CORRECTION_CUT: usize = 512;

/// A session's corrections (REQ-history-015): the total, the newest listed, and for
/// each corrected entry its text, latest statement and count.
#[derive(Default)]
pub(super) struct Corrections {
    total: usize,
    items: Vec<Value>,
    corrected: Vec<(String, String, usize)>,
}

impl Corrections {
    /// The total and the newest ten, newest first, each cut to 512 bytes.
    pub(super) fn view(&self) -> Value {
        json!({"total": self.total, "items": self.items})
    }

    /// `text` with each corrected entry's text replaced by its latest statement, marked
    /// as corrected with the count, as a next action shows it.
    pub(super) fn apply(&self, text: &str) -> String {
        let mut shown = text.to_string();
        for (original, statement, count) in &self.corrected {
            if shown.contains(original.as_str()) {
                shown = shown.replace(original.as_str(), &marked(statement, *count));
            }
        }
        shown
    }

    /// A decision holding a corrected entry's text, shown as its latest statement,
    /// marked as corrected with the count.
    pub(super) fn decision(&self, text: &str) -> String {
        self.corrected
            .iter()
            .find(|(original, _, _)| text.contains(original.as_str()))
            .map_or_else(
                || text.to_string(),
                |(_, statement, count)| marked(statement, *count),
            )
    }
}

fn marked(statement: &str, count: usize) -> String {
    format!(
        "{statement} (corrected, {count} correction{})",
        if count == 1 { "" } else { "s" }
    )
}

/// The corrections on the session's Timeline; an unreadable Timeline has none to show
/// (`history show` reports it).
pub(super) fn corrections(session: &Checkpoint) -> Corrections {
    let Some(Ok(entries)) = session.source.as_ref().map(session_md::timeline) else {
        return Corrections::default();
    };
    let mut found: Vec<&session_md::Entry> = entries
        .iter()
        .filter(|entry| entry.transition.kind == stage::CORRECTION)
        .collect();
    found.sort_by(|left, right| {
        (right.transition.at_unix_seconds, &right.id)
            .cmp(&(left.transition.at_unix_seconds, &left.id))
    });
    let mut result = Corrections {
        total: found.len(),
        ..Corrections::default()
    };
    for entry in found.iter().take(CORRECTIONS_LISTED) {
        let text = entry.transition.reason.clone().unwrap_or_default();
        let mut end = text.len().min(CORRECTION_CUT);
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        result.items.push(json!({
            "entry": format!("^{}", entry.id),
            "corrects": entry.transition.corrects,
            "text": &text[..end],
            "at": session_md::iso_time(entry.transition.at_unix_seconds),
        }));
    }
    // Newest first, so the first statement seen for a target is its latest.
    for entry in &found {
        let Some(target) = entry
            .transition
            .corrects
            .as_deref()
            .and_then(|id| id.strip_prefix('^'))
        else {
            continue;
        };
        let Some(original) = entries
            .iter()
            .find(|other| other.id == target)
            .and_then(|other| {
                other
                    .transition
                    .reason
                    .clone()
                    .or_else(|| other.transition.gate.clone())
            })
        else {
            continue;
        };
        // A very short text would match by accident; such an entry is only listed.
        if original.trim().len() < 4 {
            continue;
        }
        match result
            .corrected
            .iter_mut()
            .find(|(known, _, _)| *known == original)
        {
            Some((_, _, count)) => *count += 1,
            None => result.corrected.push((
                original,
                entry.transition.reason.clone().unwrap_or_default(),
                1,
            )),
        }
    }
    result
}

/// The bound of a correction's statement (REQ-history-015).
const CORRECTION_BYTES: usize = 4_096;

#[derive(clap::Args)]
pub(super) struct CorrectArgs {
    /// The entry: `^<entry id>` in the active session, or `<session>#^<entry id>` for
    /// any open session or archived story, by ID or alias.
    entry: String,
    /// What is true instead, at most 4,096 bytes.
    #[arg(long, allow_hyphen_values = true)]
    reason: String,
}

/// `chough correct` (REQ-history-015): append a `correction` entry linked to the entry
/// it corrects, leaving every existing byte of the file unchanged.
pub(super) fn correct(root: &Path, args: CorrectArgs) -> Result<()> {
    crate::checkpoint::lock_for_update(root)?;
    let (file, target) = match args.entry.split_once("#^") {
        Some((session, id)) => (crate::sessions::resolve_any(root, session)?, id.to_string()),
        None => match args.entry.strip_prefix('^') {
            Some(id) => (crate::sessions::checkpoint_path(root)?, id.to_string()),
            None => anyhow::bail!(
                "refused: name the entry as ^<entry id> in the active session, or <session>#^<entry id>; got {:?}, so nothing was written",
                args.entry
            ),
        },
    };
    ensure!(
        session_md::valid_entry_id(&target),
        "refused: ^{target} is not a Timeline entry ID (t- and 6 lowercase hexadecimal digits); nothing was written"
    );
    let mut statement = args.reason.trim().to_string();
    ensure!(
        !statement.is_empty(),
        "refused: a correction needs a statement that is not blank; nothing was written"
    );
    ensure!(
        statement.len() <= CORRECTION_BYTES,
        "refused: the statement is {} bytes, over the {CORRECTION_BYTES}-byte bound; nothing was written",
        statement.len()
    );
    crate::leaks::guard(crate::leaks::Local::current(), "--reason", &mut statement)?;
    let path = local_path(root, &file)?;
    let text = crate::records::read_bounded_record(&path, crate::checkpoint::SESSION_LIMIT)?;
    let correction = stage::Transition {
        schema_version: 1,
        at_unix_seconds: crate::records::now()?,
        kind: stage::CORRECTION.into(),
        from: String::new(),
        to: String::new(),
        reason: Some(statement.clone()),
        gate: None,
        revision: None,
        refresh: vec![],
        backfill: None,
        relayed_by: None,
        direct_write: None,
        actor: crate::actor::Actor::current(),
        holds: None,
        drives: None,
        corrects: None,
    };
    let (updated, id) = session_md::append_correction(&file, &text, &target, correction)?;
    crate::records::write_replace_bounded(root, &file, &updated, crate::checkpoint::SESSION_LIMIT)?;
    print_json(&json!({
        "schema_version": 1,
        "file": file,
        "entry": format!("^{id}"),
        "corrects": format!("^{target}"),
        "text": statement,
    }))
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
