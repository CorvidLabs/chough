//! Immutable transition events. The checkpoint selects the current lineage; storage
//! receipts explain removal and never supply lifecycle or timing evidence.

use crate::checkpoint::{Checkpoint, read_checkpoint, save_checkpoint};
use crate::records::{local_path, now, print_json};
use crate::{evidence, prefs, stage};
use anyhow::{Context, Result, ensure};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::Path;

pub(super) const ROOT: &str = ".chough/history";
pub(super) const EVENT_BYTES: u64 = 65_536;
pub(super) const RECENT_EVENTS: usize = 128;
pub(super) const RECENT_BYTES: u64 = 512 * 1_024;
pub(super) const CYCLE_EVENTS: usize = 4_096;
pub(super) const CYCLE_BYTES: u64 = 16 * 1_048_576;
const SOURCE_FILES: usize = 16_384;
const SOURCE_BYTES: u64 = 64 * 1_048_576;
const RECEIPT_BYTES: u64 = 1_048_576;
const DAY: u64 = 86_400;

thread_local! {
    // One ephemeral budget covers inventory and every subsequent protection
    // revalidation in the same maintenance operation, including repeated reads.
    static MAINTENANCE: std::cell::Cell<Option<(usize, u64)>> = const { std::cell::Cell::new(None) };
}

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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct Anchor {
    digest: String,
    bytes: u64,
    lines: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct Cursor {
    pub(super) cycle: String,
    pub(super) head: Option<String>,
    pub(super) legacy: Option<Anchor>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Event {
    schema_version: u32,
    cycle: String,
    id: String,
    parent: Option<String>,
    previous_cycle: Option<Cursor>,
    native_change: Option<String>,
    transition: stage::Transition,
}

#[derive(Subcommand)]
pub(super) enum HistoryCommand {
    /// Inspect cycles, selected and unselected tips, limits and partial history.
    Show,
    /// Preview adopting immutable history; --apply preserves the legacy bytes.
    Adopt {
        #[arg(long)]
        apply: bool,
    },
    /// Preview whole-cycle retention. Apply also needs human retention authorization.
    Prune {
        #[arg(long)]
        apply: bool,
    },
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn valid_id(id: &str, prefix: &str) -> bool {
    id.strip_prefix(prefix).is_some_and(|suffix| {
        suffix.len() == 24
            && suffix
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}

fn allocate(prefix: &str) -> Result<String> {
    allocate_with(prefix, |bytes| {
        getrandom::fill(bytes).map_err(|error| anyhow::anyhow!("history random source: {error}"))
    })
}

fn allocate_with(prefix: &str, fill: impl FnOnce(&mut [u8; 12]) -> Result<()>) -> Result<String> {
    let mut bytes = [0_u8; 12];
    fill(&mut bytes)?;
    Ok(format!(
        "{prefix}{}",
        bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
    ))
}

pub(super) fn new_cursor(root: &Path) -> Result<Cursor> {
    let cycle = allocate("CYC-")?;
    ensure!(
        !local_path(root, &format!("{ROOT}/cycles/{cycle}"))?.exists(),
        "history cycle collision; no retry"
    );
    Ok(Cursor {
        cycle,
        head: None,
        legacy: None,
    })
}

/// What this process published, per project root, since it last found its checkpoint in
/// place: event paths with the SHA-256 of their bytes, and cycle directories it reserved
/// (#82, REQ-history-008). `retire_published` forgets them when a save may write them;
/// `discard_published` removes them when the command's save is refused because its
/// checkpoint vanished.
static PUBLISHED: std::sync::Mutex<Published> = std::sync::Mutex::new(Published {
    events: Vec::new(),
    directories: Vec::new(),
});

#[derive(Default)]
struct Published {
    /// Project root, project-relative event path and the SHA-256 of the bytes written.
    events: Vec<(std::path::PathBuf, String, Vec<u8>)>,
    /// Project root and project-relative cycle directory reserved.
    directories: Vec<(std::path::PathBuf, String)>,
}

/// Take this process's published events and reserved cycle directories under `root` out
/// of the record, leaving other roots' entries.
fn take_published(root: &Path) -> Published {
    let mut published = PUBLISHED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let (events, others): (Vec<_>, Vec<_>) = std::mem::take(&mut published.events)
        .into_iter()
        .partition(|(owner, _, _)| owner == root);
    published.events = others;
    let (directories, other_dirs): (Vec<_>, Vec<_>) = std::mem::take(&mut published.directories)
        .into_iter()
        .partition(|(owner, _)| owner == root);
    published.directories = other_dirs;
    Published {
        events,
        directories,
    }
}

/// Forget what this process published under `root`: the command found its checkpoint in
/// place, and every save does so just before it writes, so a save may now point to those
/// events and they are never this command's to remove (second #96 review). A save that
/// then fails for another reason keeps REQ-history-002's unselected orphan.
pub(super) fn retire_published(root: &Path) {
    take_published(root);
}

/// Remove the events this process published under `root` since it last found its
/// checkpoint in place, when their bytes are unchanged, then the cycle directories it
/// reserved there when they are empty; returns how many events it removed. Called only
/// when a save is refused because the checkpoint the command resolved was archived,
/// removed or adopted meanwhile (REQ-history-008); any other failed save keeps
/// REQ-history-002's unselected orphan. No save can point to these events: the
/// checkpoint has not been in place since they were published.
pub(super) fn discard_published(root: &Path) -> usize {
    let Published {
        events,
        directories,
    } = take_published(root);
    let mut removed = 0;
    for (_, relative, digest) in events {
        let Ok(path) = local_path(root, &relative) else {
            continue;
        };
        if fs::read(&path).is_ok_and(|bytes| Sha256::digest(&bytes).to_vec() == digest)
            && fs::remove_file(&path).is_ok()
        {
            removed += 1;
        }
    }
    for (_, relative) in directories.into_iter().rev() {
        if let Ok(path) = local_path(root, &relative) {
            // Fails, and so keeps the directory, unless it is empty.
            let _ = fs::remove_dir(path);
        }
    }
    removed
}

fn reserve_cursor(root: &Path, cycle: String) -> Result<Cursor> {
    ensure!(valid_id(&cycle, "CYC-"), "invalid allocated cycle identity");
    let directory = format!("{ROOT}/cycles/{cycle}");
    let path = local_path(root, &directory)?;
    fs::create_dir_all(path.parent().context("cycle directory has no parent")?)?;
    local_path(root, &directory)?;
    fs::create_dir(path)
        .context("history cycle collision or directory creation failure; no retry")?;
    PUBLISHED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .directories
        .push((root.to_path_buf(), directory));
    Ok(Cursor {
        cycle,
        head: None,
        legacy: None,
    })
}

pub(super) fn validate_cursor(cursor: &Cursor) -> Result<()> {
    ensure!(valid_id(&cursor.cycle, "CYC-"), "invalid history cycle ID");
    ensure!(
        cursor.head.as_ref().is_none_or(|id| valid_id(id, "EVT-")),
        "invalid history event ID"
    );
    if let Some(anchor) = &cursor.legacy {
        ensure!(
            anchor.bytes <= CYCLE_BYTES
                && anchor.digest.len() == 64
                && anchor
                    .digest
                    .bytes()
                    .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()),
            "invalid legacy history anchor"
        );
    }
    Ok(())
}

fn event_path(cycle: &str, id: &str) -> Result<String> {
    ensure!(
        valid_id(cycle, "CYC-") && valid_id(id, "EVT-"),
        "invalid history path identity"
    );
    Ok(format!("{ROOT}/cycles/{cycle}/{id}.json"))
}

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

fn validate_payload(transition: &stage::Transition) -> Result<()> {
    ensure!(
        transition.schema_version == 1
            && stage::is_stage(&transition.from)
            && stage::is_stage(&transition.to),
        "unsupported or invalid transition payload"
    );
    ensure!(
        stage::STAGE_KINDS.contains(&transition.kind.as_str())
            || matches!(
                transition.kind.as_str(),
                stage::GATE_OPENED | stage::GATE_CLOSED
            ),
        "unknown transition kind"
    );
    Ok(())
}

fn anchor(root: &Path) -> Result<Option<Anchor>> {
    if !local_path(root, stage::TRANSITIONS)?.exists() {
        return Ok(None);
    }
    let bytes = bounded_bytes(root, stage::TRANSITIONS, CYCLE_BYTES)?;
    let text = std::str::from_utf8(&bytes)?;
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        ensure!(
            line.len() as u64 <= EVENT_BYTES,
            "legacy transition exceeds the event bound; adoption refused"
        );
        let transition: stage::Transition =
            serde_json::from_str(line).context("invalid legacy transition; adoption refused")?;
        validate_payload(&transition).context("invalid legacy transition; adoption refused")?;
    }
    Ok(Some(Anchor {
        digest: digest(&bytes),
        bytes: bytes.len() as u64,
        lines: text.lines().count(),
    }))
}

fn verify_anchor(root: &Path, cursor: &Cursor) -> Result<()> {
    let current = anchor(root)
        .context("legacy history anchor changed or is unreadable; restore the recorded bytes")?;
    let recorded = match (&cursor.legacy, &current) {
        // A session started after the project adopted a legacy log records no anchor of
        // its own: the log is the project's past, not this session's. Its writes are
        // guarded by the anchor an adopting session recorded (REQ-history-006).
        (None, Some(current)) => adopted_anchor(root, current)?,
        (recorded, _) => recorded.clone(),
    };
    ensure!(
        current == recorded,
        "legacy history anchor changed; restore the recorded legacy bytes before writing events"
    );
    Ok(())
}

/// `current` when an open or archived session's cursor recorded it as its legacy anchor,
/// else None. Only a checkpoint within the session size bound is read.
fn adopted_anchor(root: &Path, current: &Anchor) -> Result<Option<Anchor>> {
    // The session that adopted the legacy history may be open, archived, shelved
    // (REQ-history-006) or rolled into the vault (KNOW-29); #141.
    let mut dirs = vec![
        crate::sessions::DIR.to_string(),
        format!("{}/archive", crate::sessions::DIR),
        format!("{}/shelved", crate::sessions::DIR),
    ];
    if let Ok((library, _)) = crate::prefs::effective(root, "library-dir")
        && let Ok(base) = local_path(root, &format!("{library}/attachments"))
        && let Ok(folders) = fs::read_dir(base)
    {
        let mut rolled: Vec<String> = folders
            .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
            .map(|folder| format!("{library}/attachments/{folder}/sessions"))
            .collect();
        rolled.sort();
        dirs.extend(rolled);
    }
    for dir in dirs.iter().map(String::as_str) {
        let path = local_path(root, dir)?;
        let Ok(entries) = fs::read_dir(&path) else {
            continue;
        };
        let mut names: Vec<String> = entries
            .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
            .filter(|name| name.ends_with(".json") && !name.starts_with('.'))
            .collect();
        names.sort();
        for name in names {
            let Ok(bytes) = bounded_bytes(
                root,
                &format!("{dir}/{name}"),
                crate::checkpoint::SESSION_LIMIT,
            ) else {
                continue;
            };
            let recorded = serde_json::from_slice::<Value>(&bytes)
                .ok()
                .and_then(|session| {
                    serde_json::from_value::<Anchor>(session["history_cursor"]["legacy"].clone())
                        .ok()
                });
            if recorded.as_ref() == Some(current) {
                return Ok(recorded);
            }
        }
    }
    Ok(None)
}

fn load_event(root: &Path, cycle: &str, id: &str, budget: u64) -> Result<(Event, u64)> {
    let path = event_path(cycle, id)?;
    let bytes = bounded_bytes(root, &path, budget.min(EVENT_BYTES))?;
    Ok((parse_event(&bytes, cycle, id)?, bytes.len() as u64))
}

fn parse_event(bytes: &[u8], cycle: &str, id: &str) -> Result<Event> {
    let path = event_path(cycle, id)?;
    let event: Event =
        serde_json::from_slice(bytes).with_context(|| format!("invalid history event {path}"))?;
    ensure!(
        event.schema_version == 1 && event.cycle == cycle && event.id == id,
        "conflicting history identity at {path}"
    );
    ensure!(
        event.parent.as_ref().is_none_or(|id| valid_id(id, "EVT-")),
        "invalid parent at {path}"
    );
    ensure!(
        event.parent.is_none() || event.previous_cycle.is_none(),
        "only a cycle root may name a predecessor cycle"
    );
    if let Some(previous) = &event.previous_cycle {
        validate_cursor(previous)?;
        ensure!(
            event.transition.kind == "reset" && previous.cycle != event.cycle,
            "invalid predecessor cycle at {path}"
        );
    }
    ensure!(
        event.transition.schema_version == 1
            && stage::is_stage(&event.transition.from)
            && stage::is_stage(&event.transition.to),
        "invalid transition payload at {path}"
    );
    ensure!(
        stage::STAGE_KINDS.contains(&event.transition.kind.as_str())
            || matches!(
                event.transition.kind.as_str(),
                stage::GATE_OPENED | stage::GATE_CLOSED
            ),
        "unknown transition kind at {path}"
    );
    Ok(event)
}

#[derive(Default)]
pub(super) struct HistoryRead {
    pub(super) records: Vec<Value>,
    pub(super) partial: Vec<String>,
    pub(super) bytes: u64,
    pub(super) lineages: Vec<Value>,
    cached: BTreeMap<(String, String), Event>,
}

/// Follow only selected parents. Bounds include all event content read; directory
/// enumeration is reserved for explicit maintenance and unfiltered history reports.
pub(super) fn selected(
    root: &Path,
    cursor: &Cursor,
    limit: usize,
    byte_limit: u64,
    previous_cycles: bool,
) -> HistoryRead {
    let mut result = HistoryRead::default();
    let mut current = cursor.clone();
    let mut visited = BTreeSet::new();
    while let Some(id) = current.head.clone() {
        if result.records.len() >= limit || result.bytes >= byte_limit {
            result
                .partial
                .push("selected history exceeds the event or byte read bound".into());
            break;
        }
        if !visited.insert((current.cycle.clone(), id.clone())) {
            result
                .partial
                .push("history parent loop; order is unknown".into());
            break;
        }
        let loaded = (|| -> Result<Event> {
            let path = event_path(&current.cycle, &id)?;
            let bytes = bounded_bytes(root, &path, (byte_limit - result.bytes).min(EVENT_BYTES))?;
            result.bytes += bytes.len() as u64;
            parse_event(&bytes, &current.cycle, &id)
        })();
        let event = match loaded {
            Ok(event) => event,
            Err(error) => {
                let path = event_path(&current.cycle, &id).unwrap_or_default();
                let receipt_path = format!("{ROOT}/pruned/{}.json", current.cycle);
                let receipted = (|| -> Result<bool> {
                    let bytes = bounded_bytes(
                        root,
                        &receipt_path,
                        (byte_limit - result.bytes).min(RECEIPT_BYTES),
                    )?;
                    result.bytes += bytes.len() as u64;
                    let receipt: Receipt = serde_json::from_slice(&bytes)?;
                    validate_receipt(&receipt)?;
                    Ok(receipt.cycle == current.cycle
                        && receipt.files.iter().any(|file| file.path == path)
                        && !local_path(root, &path)?.exists())
                })()
                .unwrap_or(false);
                let state = if receipted {
                    "pruned or interrupted"
                } else {
                    "unavailable"
                };
                result
                    .partial
                    .push(format!("{state} history {id}: {error}"));
                break;
            }
        };
        result
            .cached
            .insert((event.cycle.clone(), event.id.clone()), event.clone());
        result
            .records
            .push(serde_json::to_value(&event.transition).expect("serializable transition"));
        current.head = event.parent;
        if current.head.is_none()
            && previous_cycles
            && let Some(previous) = event.previous_cycle
        {
            current = previous;
        }
    }
    // Legacy history is a separate, explicitly anchored predecessor, never a second
    // source for events. It is read only when the selected walk reached its start.
    if previous_cycles && result.partial.is_empty() && current.legacy.is_some() {
        let remaining = byte_limit.saturating_sub(result.bytes);
        let legacy = (|| -> Result<Vec<Value>> {
            let expected = current.legacy.as_ref().expect("legacy anchor");
            let data = bounded_bytes(root, stage::TRANSITIONS, remaining)?;
            ensure!(
                data.len() as u64 == expected.bytes && digest(&data) == expected.digest,
                "legacy history anchor changed"
            );
            result.bytes += data.len() as u64;
            let text = std::str::from_utf8(&data)?;
            let room = limit.saturating_sub(result.records.len());
            if text.lines().filter(|line| !line.trim().is_empty()).count() > room {
                result
                    .partial
                    .push("legacy events exceed the history read bound".into());
            }
            let mut records = Vec::new();
            for line in text
                .lines()
                .rev()
                .filter(|line| !line.trim().is_empty())
                .take(room)
            {
                let record: stage::Transition = serde_json::from_str(line)?;
                validate_payload(&record)?;
                records.push(serde_json::to_value(record)?);
            }
            Ok(records)
        })();
        match legacy {
            Ok(records) => {
                result.records.extend(records);
            }
            Err(error) => result
                .partial
                .push(format!("legacy history unknown: {error}")),
        }
    }
    result.records.reverse();
    result
}

/// Unfiltered timing can inspect other tips within the same content budget. Their
/// event files are retained observations, not selected checkpoints, so their
/// completion and durations stay unknown rather than joining the current timeline.
pub(super) fn timing(root: &Path, cursor: &Cursor, all: bool) -> HistoryRead {
    let mut read = selected(root, cursor, CYCLE_EVENTS, CYCLE_BYTES, true);
    read.lineages.push(json!({"cycle":cursor.cycle,"tip":cursor.head,"selection":"checkpoint-selected","durations":"derived only along the selected parent chain"}));
    if !all {
        return read;
    }
    let selected_keys: BTreeSet<_> = read.cached.keys().cloned().collect();
    let initial_records = read.records.len();
    let initial_cached = read.cached.len();
    let scan = (|| -> Result<()> {
        let directory = local_path(root, &format!("{ROOT}/cycles"))?;
        if !directory.exists() {
            return Ok(());
        }
        let mut cycles = Vec::new();
        for entry in fs::read_dir(directory)? {
            ensure!(
                cycles.len() < CYCLE_EVENTS,
                "timing cycle enumeration bound reached"
            );
            let entry = entry?;
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| anyhow::anyhow!("non-UTF-8 timing cycle"))?;
            ensure!(
                entry.file_type()?.is_dir() && valid_id(&name, "CYC-"),
                "unsafe timing cycle"
            );
            cycles.push(name);
        }
        cycles.sort();
        let mut names_seen = 0;
        for cycle in cycles {
            let mut names = Vec::new();
            for entry in fs::read_dir(local_path(root, &format!("{ROOT}/cycles/{cycle}"))?)? {
                ensure!(
                    names_seen < CYCLE_EVENTS,
                    "timing event enumeration bound reached"
                );
                names_seen += 1;
                let entry = entry?;
                let name = entry
                    .file_name()
                    .into_string()
                    .map_err(|_| anyhow::anyhow!("non-UTF-8 timing event"))?;
                let id = name
                    .strip_suffix(".json")
                    .context("unexpected timing event path")?;
                ensure!(
                    entry.file_type()?.is_file() && valid_id(id, "EVT-"),
                    "unsafe timing event path"
                );
                names.push(id.to_string());
            }
            names.sort();
            for id in names {
                if read.cached.contains_key(&(cycle.clone(), id.clone())) {
                    continue;
                }
                ensure!(
                    initial_records + read.cached.len() - initial_cached < CYCLE_EVENTS
                        && read.bytes < CYCLE_BYTES,
                    "timing content read bound reached; other lineages are unknown"
                );
                let (event, bytes) = load_event(root, &cycle, &id, CYCLE_BYTES - read.bytes)?;
                read.bytes += bytes;
                read.cached.insert((cycle.clone(), id), event);
            }
        }
        let parents: BTreeSet<_> = read
            .cached
            .values()
            .filter_map(|event| {
                event
                    .parent
                    .as_ref()
                    .map(|parent| (event.cycle.clone(), parent.clone()))
            })
            .collect();
        for (cycle, id) in read.cached.keys() {
            if parents.contains(&(cycle.clone(), id.clone()))
                || (cycle == &cursor.cycle && cursor.head.as_ref() == Some(id))
            {
                continue;
            }
            // A historical selected tip is known from a successor reset; it is not
            // an unrelated head and already appears in the selected derivation.
            let prior_selected = read
                .cached
                .iter()
                .filter(|(key, _)| selected_keys.contains(*key))
                .any(|(_, event)| {
                    event.previous_cycle.as_ref().is_some_and(|previous| {
                        previous.cycle == *cycle && previous.head.as_ref() == Some(id)
                    })
                });
            if prior_selected {
                continue;
            }
            read.lineages.push(json!({"cycle":cycle,"tip":id,"selection":"unselected","completion":"unknown","duration":null,"reason":"no selected checkpoint for this branch or orphan"}));
        }
        Ok(())
    })();
    if let Err(error) = scan {
        read.partial
            .push(format!("other transition lineages incomplete: {error}"));
    }
    if read
        .lineages
        .iter()
        .any(|lineage| lineage["selection"] == "unselected")
    {
        read.partial.push("unselected branch or orphan lineages are retained separately; their completion and durations are unknown".into());
    }
    read
}

fn cycle_usage(root: &Path, cycle: &str) -> Result<(usize, u64)> {
    let directory = local_path(root, &format!("{ROOT}/cycles/{cycle}"))?;
    if !directory.exists() {
        return Ok((0, 0));
    }
    let mut count = 0;
    let mut bytes = 0;
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("non-UTF-8 history path"))?;
        let id = name.strip_suffix(".json").unwrap_or("");
        ensure!(
            valid_id(id, "EVT-") && entry.file_type()?.is_file(),
            "unsafe or unexpected cycle file: {name}"
        );
        count += 1;
        bytes += entry.metadata()?.len();
        ensure!(
            count <= CYCLE_EVENTS && bytes <= CYCLE_BYTES,
            "cycle storage bound exceeded; start a reasoned new cycle without deleting proof"
        );
    }
    Ok((count, bytes))
}

/// Publish only complete content. A same-directory hard link is atomic and
/// refuses replacement on every supported platform; a failed publication leaves
/// the checkpoint unchanged and removes its unpublished temporary file.
fn publish(root: &Path, relative: &str, content: &[u8]) -> Result<bool> {
    let destination = local_path(root, relative)?;
    if destination.exists() {
        ensure!(
            destination.is_file(),
            "history destination is not a regular file"
        );
        return Ok(false);
    }
    fs::create_dir_all(
        destination
            .parent()
            .context("history destination has no parent")?,
    )?;
    local_path(root, relative)?;
    let name = destination
        .file_name()
        .context("history destination has no filename")?
        .to_string_lossy();
    let temporary = destination.with_file_name(format!(".{name}.{}.tmp", std::process::id()));
    let mut created = false;
    let outcome = (|| -> Result<bool> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        created = true;
        file.write_all(content)?;
        file.sync_all()?;
        match fs::hard_link(&temporary, &destination) {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
            Err(error) => Err(error.into()),
        }
    })();
    if created && temporary.exists() {
        fs::remove_file(&temporary)?;
    }
    outcome
}

pub(super) fn append(
    root: &Path,
    session: &mut Checkpoint,
    transition: stage::Transition,
) -> Result<()> {
    append_using(root, session, transition, allocate)
}

fn append_using(
    root: &Path,
    session: &mut Checkpoint,
    transition: stage::Transition,
    mut allocate: impl FnMut(&str) -> Result<String>,
) -> Result<()> {
    let old = session
        .history_cursor
        .as_ref()
        .context("history cursor missing")?;
    validate_cursor(old)?;
    verify_anchor(root, old)?;
    if old.head.is_some() {
        let read = selected(root, old, CYCLE_EVENTS, CYCLE_BYTES, false);
        ensure!(
            read.partial.is_empty(),
            "cannot append to partial history: {}",
            read.partial.join("; ")
        );
    }
    let reset = transition.kind == "reset";
    let mut cursor = if reset {
        Cursor {
            cycle: allocate("CYC-")?,
            head: None,
            legacy: old.legacy.clone(),
        }
    } else {
        old.clone()
    };
    let (count, bytes) = cycle_usage(root, &cursor.cycle)?;
    let id = allocate("EVT-")?;
    let event = Event {
        schema_version: 1,
        cycle: cursor.cycle.clone(),
        id: id.clone(),
        parent: cursor.head.clone(),
        previous_cycle: reset.then(|| old.clone()),
        native_change: session.native_change.clone(),
        transition,
    };
    let content = serde_json::to_vec(&event)?;
    ensure!(
        content.len() as u64 <= EVENT_BYTES,
        "history event exceeds 65536 bytes"
    );
    ensure!(
        count < CYCLE_EVENTS && bytes + content.len() as u64 <= CYCLE_BYTES,
        "cycle storage capacity reached; start a reasoned new cycle without deleting proof"
    );
    // New projects carry only an empty checkpoint. The first complete event
    // claims its directory atomically after every allocation and size check.
    if cursor.head.is_none() {
        reserve_cursor(root, cursor.cycle.clone())?;
    }
    let path = event_path(&cursor.cycle, &id)?;
    ensure!(
        publish(root, &path, &content)?,
        "history collision at {path}; no retry or overwrite"
    );
    PUBLISHED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .events
        .push((root.to_path_buf(), path, Sha256::digest(&content).to_vec()));
    cursor.head = Some(id);
    session.history_cursor = Some(cursor);
    Ok(())
}

#[derive(Default)]
struct Budget {
    files: usize,
    bytes: u64,
    directories: usize,
}

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

fn protection_sources(
    root: &Path,
    budget: &mut Budget,
    include_artifacts: bool,
) -> Result<BTreeMap<String, Vec<u8>>> {
    let mut sources = BTreeMap::new();
    for directory in [
        ".chough/session.json",
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

/// Used by evidence cleanup to notice newly added or changed reference records.
pub(super) fn protection_fingerprint(root: &Path) -> Result<String> {
    source_fingerprint(root, false)
}

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

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct FilePlan {
    path: String,
    digest: String,
    bytes: u64,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    schema_version: u32,
    cycle: String,
    files: Vec<FilePlan>,
    authority: String,
    reason: String,
    recorded_at_unix_seconds: u64,
    closed_at: u64,
    tips: Vec<String>,
}

struct Cycle {
    id: String,
    events: BTreeMap<String, Event>,
    files: Vec<FilePlan>,
    present_files: Vec<FilePlan>,
    tips: Vec<String>,
    reasons: BTreeSet<String>,
    missing_references: BTreeMap<String, BTreeSet<String>>,
    closed_at: Option<u64>,
    interrupted: bool,
    closure_rank: usize,
}

fn empty_cycle(id: String) -> Cycle {
    Cycle {
        id,
        events: BTreeMap::new(),
        files: vec![],
        present_files: vec![],
        tips: vec![],
        reasons: BTreeSet::new(),
        missing_references: BTreeMap::new(),
        closed_at: None,
        interrupted: false,
        closure_rank: 0,
    }
}

fn validate_receipt(receipt: &Receipt) -> Result<()> {
    ensure!(
        receipt.schema_version == 1 && valid_id(&receipt.cycle, "CYC-"),
        "invalid history receipt identity"
    );
    ensure!(
        !receipt.authority.trim().is_empty()
            && receipt.files.len() <= CYCLE_EVENTS
            && !receipt.files.is_empty(),
        "invalid history receipt plan"
    );
    ensure!(
        receipt.tips.len() == 1 && receipt.tips.iter().all(|id| valid_id(id, "EVT-")),
        "invalid history receipt tips"
    );
    let mut paths = BTreeSet::new();
    let mut bytes = 0;
    for file in &receipt.files {
        let id = file
            .path
            .rsplit('/')
            .next()
            .and_then(|name| name.strip_suffix(".json"))
            .context("invalid receipt event path")?;
        ensure!(
            file.path == event_path(&receipt.cycle, id)? && paths.insert(file.path.clone()),
            "unsafe or duplicate receipt path"
        );
        ensure!(
            file.digest.len() == 64
                && file
                    .digest
                    .bytes()
                    .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
                && file.bytes <= EVENT_BYTES,
            "invalid receipt file identity"
        );
        bytes += file.bytes;
    }
    ensure!(
        bytes <= CYCLE_BYTES
            && receipt
                .tips
                .iter()
                .all(|id| paths.contains(&event_path(&receipt.cycle, id).unwrap_or_default())),
        "receipt exceeds cycle bounds or omits its tip"
    );
    Ok(())
}

fn read_receipt(root: &Path, cycle: &str, budget: u64) -> Result<Receipt> {
    ensure!(valid_id(cycle, "CYC-"), "invalid receipt cycle");
    let bytes = bounded_bytes(
        root,
        &format!("{ROOT}/pruned/{cycle}.json"),
        budget.min(RECEIPT_BYTES),
    )?;
    let receipt: Receipt = serde_json::from_slice(&bytes)?;
    validate_receipt(&receipt)?;
    ensure!(receipt.cycle == cycle, "conflicting receipt cycle");
    Ok(receipt)
}

fn normalize_reference_path(text: &str) -> String {
    text.replace('\\', "/")
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect::<Vec<_>>()
        .join("/")
}

fn normalize_reference_paths(value: &mut Value) {
    match value {
        Value::String(text) if text.to_lowercase().starts_with(".chough") => {
            *text = normalize_reference_path(text);
        }
        Value::Array(values) => {
            for value in values {
                normalize_reference_paths(value);
            }
        }
        Value::Object(values) => {
            for value in values.values_mut() {
                normalize_reference_paths(value);
            }
        }
        _ => {}
    }
}

fn reference_boundary(character: char) -> bool {
    character.is_whitespace()
        || (character.is_ascii_punctuation() && !matches!(character, '.' | '/' | '\\' | '-' | '_'))
}

fn normalize_plaintext_references(text: &str) -> String {
    text.split_inclusive(reference_boundary)
        .map(|part| {
            let token = part.trim_end_matches(reference_boundary);
            if token.to_lowercase().starts_with(".chough") {
                format!(
                    "{}{}",
                    normalize_reference_path(token),
                    &part[token.len()..]
                )
            } else {
                part.to_string()
            }
        })
        .collect::<String>()
        .to_lowercase()
}

fn reference_text(path: &str, bytes: &[u8]) -> Result<String> {
    let text = String::from_utf8_lossy(bytes);
    if path.starts_with(".chough/artifacts/") {
        return Ok(normalize_plaintext_references(&text));
    }
    if path.ends_with(".json") {
        let mut value: Value = serde_json::from_str(&text)?;
        normalize_reference_paths(&mut value);
        return Ok(value.to_string().to_lowercase());
    }
    if path.ends_with(".toml") {
        let value: toml::Value = toml::from_str(&text)?;
        let mut value = serde_json::to_value(value)?;
        normalize_reference_paths(&mut value);
        return Ok(value.to_string().to_lowercase());
    }
    Ok(normalize_plaintext_references(&text))
}

fn note_missing_references(
    cycle: &mut Cycle,
    source: &str,
    missing: &[usize],
    referenced: impl Fn(usize) -> bool,
) {
    for &index in missing {
        if referenced(index) {
            cycle
                .missing_references
                .entry(cycle.files[index].path.clone())
                .or_default()
                .insert(source.to_string());
        }
    }
}

fn directory_reference(text: &str, path: &str) -> bool {
    text.split(reference_boundary)
        .any(|token| token.trim_end_matches(['/', '.']) == path)
}

fn file_reference_boundaries(text: &str, start: usize, end: usize) -> bool {
    let (Some(before), Some(after)) = (text.get(..start), text.get(end..)) else {
        return false;
    };
    before.chars().next_back().is_none_or(reference_boundary)
        && after
            .trim_start_matches('.')
            .chars()
            .next()
            .is_none_or(reference_boundary)
}

/// Every cycle mention is either inside a bounded known file path or names the
/// whole cycle. Matching byte spans with get keeps surrounding Unicode safe.
fn cycle_references(
    text: &str,
    cycle: &str,
    paths: &BTreeMap<&str, usize>,
) -> (bool, BTreeSet<usize>) {
    // Validated event paths have fixed-shape cycle and event identities.
    let file_length = paths.first_key_value().map_or(0, |(path, _)| path.len());
    let prefix_length = ROOT.len() + "/cycles/".len();
    let mut files = BTreeSet::new();
    for (position, _) in text.match_indices(cycle) {
        let exact_file = position.checked_sub(prefix_length).and_then(|start| {
            let end = start.checked_add(file_length)?;
            let file = paths.get(text.get(start..end)?)?;
            file_reference_boundaries(text, start, end).then_some(file)
        });
        if let Some(&index) = exact_file {
            files.insert(index);
        } else {
            return (true, files);
        }
    }
    (false, files)
}

fn inventory(root: &Path, session: &Checkpoint) -> Result<Vec<Cycle>> {
    let mut budget = Budget::default();
    let mut files = BTreeMap::new();
    walk(
        root,
        &format!("{ROOT}/cycles"),
        EVENT_BYTES,
        &mut budget,
        &mut files,
        0,
    )?;
    let mut receipt_files = BTreeMap::new();
    walk(
        root,
        &format!("{ROOT}/pruned"),
        RECEIPT_BYTES,
        &mut budget,
        &mut receipt_files,
        0,
    )?;
    let sources = protection_sources(root, &mut budget, true)?;
    let native = evidence::retention_protection(root)?;
    let mut cycles = BTreeMap::<String, Cycle>::new();
    let directory = local_path(root, &format!("{ROOT}/cycles"))?;
    if directory.exists() {
        for (index, entry) in fs::read_dir(directory)?.enumerate() {
            ensure!(
                index < SOURCE_FILES,
                "incomplete cycle directory enumeration"
            );
            let entry = entry?;
            let id = entry
                .file_name()
                .into_string()
                .map_err(|_| anyhow::anyhow!("invalid cycle path"))?;
            ensure!(
                entry.file_type()?.is_dir() && valid_id(&id, "CYC-"),
                "unsafe or unexpected cycle directory"
            );
            cycles.insert(id.clone(), empty_cycle(id));
        }
    }
    for (path, bytes) in files {
        let cycle_id = path.split('/').nth(3).context("invalid cycle path")?;
        let id = path
            .rsplit('/')
            .next()
            .and_then(|name| name.strip_suffix(".json"))
            .context("invalid event path")?;
        let event = parse_event(&bytes, cycle_id, id)?;
        ensure!(
            path == event_path(&event.cycle, &event.id)?,
            "conflicting event path {path}"
        );
        let cycle = cycles
            .entry(event.cycle.clone())
            .or_insert_with(|| empty_cycle(event.cycle.clone()));
        let file = FilePlan {
            path,
            digest: digest(&bytes),
            bytes: bytes.len() as u64,
        };
        cycle.present_files.push(file.clone());
        cycle.files.push(file);
        ensure!(
            cycle.events.insert(event.id.clone(), event).is_none(),
            "duplicate history event identity"
        );
    }
    let mut receipts = BTreeMap::new();
    for (path, bytes) in receipt_files {
        let receipt: Receipt = serde_json::from_slice(&bytes)?;
        validate_receipt(&receipt)?;
        ensure!(
            path == format!("{ROOT}/pruned/{}.json", receipt.cycle),
            "conflicting receipt path"
        );
        cycles
            .entry(receipt.cycle.clone())
            .or_insert_with(|| empty_cycle(receipt.cycle.clone()));
        receipts.insert(receipt.cycle.clone(), receipt);
    }
    // Only a reset on the checkpoint-selected chain establishes closure. A reset
    // published before a failed checkpoint save is an orphan, not a completed reset.
    let mut successors = BTreeMap::new();
    let mut selected = session.history_cursor.clone();
    let mut seen = BTreeSet::new();
    while let Some(cursor) = selected {
        let Some(head) = cursor.head.as_ref() else {
            break;
        };
        if !seen.insert((cursor.cycle.clone(), head.clone())) {
            break;
        }
        let Some(event) = cycles
            .get(&cursor.cycle)
            .and_then(|cycle| cycle.events.get(head))
        else {
            break;
        };
        selected = if let Some(parent) = &event.parent {
            Some(Cursor {
                cycle: cursor.cycle,
                head: Some(parent.clone()),
                legacy: cursor.legacy,
            })
        } else {
            if let Some(previous) = &event.previous_cycle
                && let Some(head) = &previous.head
            {
                successors.insert(
                    (previous.cycle.clone(), head.clone()),
                    (seen.len(), event.transition.at_unix_seconds),
                );
            }
            event.previous_cycle.clone()
        };
    }
    let at = now()?;
    let cycles_directory = format!("{ROOT}/cycles");
    let texts: Vec<(String, String, bool)> = sources
        .iter()
        .map(|(path, bytes)| {
            let text = reference_text(path, bytes)?;
            let all_cycles = [ROOT, cycles_directory.as_str()]
                .iter()
                .any(|directory| directory_reference(&text, directory));
            Ok((path.clone(), text, all_cycles))
        })
        .collect::<Result<_>>()?;
    for cycle in cycles.values_mut() {
        let parents: BTreeSet<&String> = cycle
            .events
            .values()
            .filter_map(|event| event.parent.as_ref())
            .collect();
        cycle.tips = cycle
            .events
            .keys()
            .filter(|id| !parents.contains(id))
            .cloned()
            .collect();
        let receipt = receipts.get(&cycle.id);
        if let Some(receipt) = receipt {
            if cycle.files.iter().any(|file| !receipt.files.contains(file)) {
                cycle
                    .reasons
                    .insert("candidate conflicts with existing receipt".into());
            }
            cycle.interrupted = !cycle.files.is_empty() && cycle.files.len() < receipt.files.len();
            if cycle.files.is_empty() {
                cycle.reasons.insert("already pruned".into());
            }
            cycle.files = receipt.files.clone();
            cycle.tips = receipt.tips.clone();
        } else {
            if cycle.tips.len() != 1 {
                cycle
                    .reasons
                    .insert("unresolved fork, empty orphan or parent loop".into());
            }
            let mut reached = BTreeSet::new();
            for tip in &cycle.tips {
                let mut next = Some(tip.clone());
                while let Some(id) = next {
                    if !reached.insert(id.clone()) {
                        break;
                    }
                    let Some(event) = cycle.events.get(&id) else {
                        cycle
                            .reasons
                            .insert("missing parent or interrupted cleanup".into());
                        break;
                    };
                    next = event.parent.clone();
                }
            }
            if reached.len() != cycle.events.len() {
                cycle
                    .reasons
                    .insert("unreachable orphan or parent loop".into());
            }
        }
        if session
            .history_cursor
            .as_ref()
            .is_some_and(|cursor| cursor.cycle == cycle.id)
        {
            cycle.reasons.insert("active checkpoint cycle".into());
        }
        if !cycle.tips.is_empty()
            && cycle
                .tips
                .iter()
                .all(|tip| successors.contains_key(&(cycle.id.clone(), tip.clone())))
        {
            cycle.closure_rank = cycle
                .tips
                .iter()
                .filter_map(|tip| successors.get(&(cycle.id.clone(), tip.clone())))
                .map(|(rank, _)| *rank)
                .max()
                .unwrap_or(0);
            // Retention age starts at the selected reset that closed the cycle,
            // even when it was idle or an older receipt names an earlier time.
            cycle.closed_at = cycle
                .tips
                .iter()
                .filter_map(|tip| successors.get(&(cycle.id.clone(), tip.clone())))
                .map(|(_, closed_at)| *closed_at)
                .max();
        } else {
            cycle
                .reasons
                .insert("unselected orphan or unresolved cycle closure".into());
        }
        let cycle_key = cycle.id.to_ascii_lowercase();
        let normalized_files: Vec<String> = cycle
            .files
            .iter()
            .map(|file| file.path.to_ascii_lowercase())
            .collect();
        let paths: BTreeMap<&str, usize> = normalized_files
            .iter()
            .enumerate()
            .map(|(index, path)| (path.as_str(), index))
            .collect();
        let missing: Vec<usize> = {
            let present: BTreeSet<&str> = cycle
                .present_files
                .iter()
                .map(|file| file.path.as_str())
                .collect();
            cycle
                .files
                .iter()
                .enumerate()
                .filter_map(|(index, file)| {
                    (!present.contains(file.path.as_str())).then_some(index)
                })
                .collect()
        };
        for (source, text, all_cycles) in &texts {
            let (whole_cycle, files) = if *all_cycles {
                (true, BTreeSet::new())
            } else {
                cycle_references(text, &cycle_key, &paths)
            };
            if whole_cycle || !files.is_empty() {
                cycle.reasons.insert(format!("referenced by {source}"));
                note_missing_references(cycle, source, &missing, |index| {
                    whole_cycle || files.contains(&index)
                });
            }
        }
        for path in &native {
            let key = path.replace('\\', "/").to_lowercase();
            let directory = format!("{}/", key.trim_end_matches('/'));
            if normalized_files
                .iter()
                .any(|file| file == &key || file.starts_with(&directory))
            {
                cycle.reasons.insert("native acceptance manifest".into());
                note_missing_references(cycle, "native acceptance manifest", &missing, |index| {
                    normalized_files[index] == key
                        || normalized_files[index].starts_with(&directory)
                });
            }
        }
        if cycle
            .closed_at
            .is_some_and(|closed| at.saturating_sub(closed) < 14 * DAY)
        {
            cycle.reasons.insert("younger than 14 days".into());
        }
    }
    let mut newest: Vec<_> = cycles
        .values()
        .filter_map(|cycle| cycle.closed_at)
        .collect();
    newest.sort_by(|left, right| right.cmp(left));
    if let Some(cutoff) = newest.get(newest.len().min(3).saturating_sub(1)) {
        for cycle in cycles
            .values_mut()
            .filter(|cycle| cycle.closed_at.is_some_and(|at| at >= *cutoff))
        {
            cycle
                .reasons
                .insert("newest three closed cycles, including timestamp ties".into());
        }
    }
    Ok(cycles.into_values().collect())
}

fn reference_partial(cycles: &[Cycle]) -> Vec<Value> {
    cycles
        .iter()
        .flat_map(|cycle| {
            cycle.missing_references.iter().map(|(path, sources)| {
                json!({"path":path,"sources":sources,"reason":"referenced receipted history is missing; restore exact event bytes from Git"})
            })
        })
        .collect()
}

fn ensure_references_available(cycles: &[Cycle]) -> Result<()> {
    if let Some((path, sources)) = cycles
        .iter()
        .flat_map(|cycle| &cycle.missing_references)
        .next()
    {
        anyhow::bail!(
            "referenced history is missing: {path}, referenced by {}; inspect history show and restore exact event bytes from Git before cleanup",
            sources
                .iter()
                .next()
                .map_or("unknown source", String::as_str)
        );
    }
    Ok(())
}

fn show(root: &Path) -> Result<Value> {
    let session = read_checkpoint(root)?;
    let cycles = inventory(root, &session)?;
    let partial = reference_partial(&cycles);
    Ok(
        json!({"schema_version":1,"complete":partial.is_empty(),"partial":partial,"cursor":session.history_cursor,"cycles":cycles.iter().map(|cycle| {
        let selected = session.history_cursor.as_ref().filter(|cursor| cursor.cycle == cycle.id).and_then(|cursor| cursor.head.as_ref());
        json!({"id":cycle.id,"events":cycle.events.len(),"bytes":cycle.present_files.iter().map(|file| file.bytes).sum::<u64>(),"planned_bytes":cycle.files.iter().map(|file| file.bytes).sum::<u64>(),"tips":cycle.tips,"unselected_tips":cycle.tips.iter().filter(|id| Some(*id) != selected).collect::<Vec<_>>(),"protected":cycle.reasons,"interrupted":cycle.interrupted})
    }).collect::<Vec<_>>(),"limits":{"event_bytes":EVENT_BYTES,"recent_events":RECENT_EVENTS,"recent_bytes":RECENT_BYTES,"cycle_events":CYCLE_EVENTS,"cycle_bytes":CYCLE_BYTES},"authority":"checkpoint cursor selects current lineage; unselected events are not completed transitions"}),
    )
}

fn prune(root: &Path, apply: bool) -> Result<Value> {
    let session = read_checkpoint(root)?;
    let cycles = inventory(root, &session)?;
    let partial = reference_partial(&cycles);
    if apply {
        ensure_references_available(&cycles)?;
    }
    let authority = if apply {
        Some(prefs::authorized(root, &session, "retention")?.context("history pruning needs human retention authorization; preview first and grant retention consent")?)
    } else {
        None
    };
    let mut eligible = Vec::new();
    let mut protected = Vec::new();
    let mut removed = Vec::new();
    let mut skipped = Vec::new();
    let mut reclaimed_bytes = 0_u64;
    let mut reclaimable_bytes = 0_u64;
    for cycle in &cycles {
        if cycle.reasons.is_empty() {
            let bytes = cycle
                .present_files
                .iter()
                .map(|file| file.bytes)
                .sum::<u64>();
            reclaimable_bytes += bytes;
            eligible.push(json!({"cycle":cycle.id,"files":cycle.present_files,"bytes":bytes,"plan_files":cycle.files,"planned_bytes":cycle.files.iter().map(|file| file.bytes).sum::<u64>(),"interrupted":cycle.interrupted}));
        } else {
            protected.push(json!({"cycle":cycle.id,"reasons":cycle.reasons}));
        }
    }
    if let Some(authority) = &authority {
        let mut candidates: Vec<_> = cycles
            .iter()
            .filter(|cycle| cycle.reasons.is_empty())
            .collect();
        // Oldest ancestors go first, so removing a newer historical link never
        // prevents revalidation of an older candidate in the same plan.
        candidates.sort_by_key(|cycle| std::cmp::Reverse(cycle.closure_rank));
        for cycle in candidates {
            let protection = source_fingerprint(root, true)?;
            let fresh = inventory(root, &read_checkpoint(root)?)?;
            ensure_references_available(&fresh)?;
            let Some(current) = fresh.iter().find(|current| {
                current.id == cycle.id && current.reasons.is_empty() && current.files == cycle.files
            }) else {
                skipped.push(
                    json!({"cycle":cycle.id,"reason":"protection or candidate identity changed"}),
                );
                continue;
            };
            let receipt = Receipt {
                schema_version: 1,
                cycle: current.id.clone(),
                files: current.files.clone(),
                authority: authority.clone(),
                reason: "complete closed cycle outside every protected set".into(),
                recorded_at_unix_seconds: now()?,
                closed_at: current.closed_at.context("cycle closure missing")?,
                tips: current.tips.clone(),
            };
            let bytes = serde_json::to_vec(&receipt)?;
            ensure!(
                bytes.len() as u64 <= RECEIPT_BYTES,
                "retention receipt exceeds 1 MiB; apply refused"
            );
            let path = format!("{ROOT}/pruned/{}.json", cycle.id);
            if !publish(root, &path, &bytes)? {
                let prior = read_receipt(root, &cycle.id, RECEIPT_BYTES)?;
                ensure!(
                    prior.schema_version == 1
                        && prior.cycle == receipt.cycle
                        && prior.files == receipt.files,
                    "conflicting history receipt; restore or inspect interrupted cleanup"
                );
            }
            for file in &current.files {
                ensure!(
                    source_fingerprint(root, true)? == protection,
                    "protection changed during history cleanup; remaining files retained"
                );
                if !local_path(root, &file.path)?.exists() {
                    let id = file
                        .path
                        .rsplit('/')
                        .next()
                        .and_then(|name| name.strip_suffix(".json"))
                        .context("invalid receipt path")?;
                    ensure!(
                        !current.events.contains_key(id),
                        "candidate became unavailable during cleanup; remaining files retained"
                    );
                    continue;
                }
                let data = bounded_bytes(root, &file.path, EVENT_BYTES)?;
                ensure!(
                    data.len() as u64 == file.bytes && digest(&data) == file.digest,
                    "candidate changed during history cleanup; remaining files retained"
                );
                fs::remove_file(local_path(root, &file.path)?)?;
                reclaimed_bytes += data.len() as u64;
                removed.push(file.path.clone());
            }
        }
    }
    Ok(
        json!({"schema_version":1,"applied":apply,"complete":partial.is_empty(),"partial":partial,"eligible":eligible,"protected":protected,"removed":removed,"skipped":skipped,"reclaimable_bytes":reclaimable_bytes,"reclaimed_bytes":reclaimed_bytes,"note":"protected proof can prevent reclamation; legacy logs, migration bytes and native archives are never removed"}),
    )
}

pub(super) fn run(root: &Path, command: HistoryCommand) -> Result<()> {
    let _budget = matches!(command, HistoryCommand::Show | HistoryCommand::Prune { .. })
        .then(maintenance_budget);
    let report = match command {
        HistoryCommand::Show => show(root),
        HistoryCommand::Prune { apply } => prune(root, apply),
        HistoryCommand::Adopt { apply } => (|| -> Result<Value> {
            let mut session = read_checkpoint(root)?;
            if let Some(cursor) = &session.history_cursor {
                validate_cursor(cursor)?;
                verify_anchor(root, cursor)?;
                return Ok(json!({"applied":false,"already_adopted":true,"cursor":cursor}));
            }
            let legacy = anchor(root)?;
            if apply {
                let mut cursor = new_cursor(root)?;
                cursor.legacy = legacy.clone();
                session.history_cursor = Some(cursor);
                save_checkpoint(root, &session)?;
            }
            Ok(
                json!({"applied":apply,"legacy_anchor":legacy,"cursor":session.history_cursor,"note":"legacy transition and migration bytes are unchanged; older helpers may refuse the adopted checkpoint"}),
            )
        })(),
    };
    match report {
        Ok(report) => print_json(&report),
        Err(error)
            if matches!(
                command,
                HistoryCommand::Prune { apply: false } | HistoryCommand::Show
            ) =>
        {
            print_json(
                &json!({"schema_version":1,"complete":false,"eligible":[],"cycles":[],"reason":error.to_string(),"note":"incomplete protection/history scan; apply is refused"}),
            )
        }
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transition(kind: &str) -> stage::Transition {
        stage::Transition {
            schema_version: 1,
            at_unix_seconds: 100,
            kind: kind.into(),
            from: "intent".into(),
            to: "intent".into(),
            reason: Some("test event".into()),
            gate: None,
            revision: None,
            refresh: vec![],
            backfill: None,
        }
    }

    #[test]
    fn one_random_fill_has_independent_known_answer_vectors_and_failure_is_not_retried() {
        for (byte, expected) in [
            (0, "EVT-000000000000000000000000"),
            (255, "EVT-ffffffffffffffffffffffff"),
        ] {
            let mut calls = 0;
            let id = allocate_with("EVT-", |bytes| {
                calls += 1;
                bytes.fill(byte);
                Ok(())
            })
            .unwrap();
            assert_eq!(id, expected);
            assert_eq!(calls, 1);
        }
        let mixed = allocate_with("CYC-", |bytes| {
            *bytes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 255];
            Ok(())
        })
        .unwrap();
        assert_eq!(mixed, "CYC-000102030405060708090aff");
        let mut calls = 0;
        assert!(
            allocate_with("EVT-", |_| {
                calls += 1;
                Err(anyhow::anyhow!("source unavailable"))
            })
            .is_err()
        );
        assert_eq!(calls, 1);
    }

    #[test]
    fn failed_random_draw_and_same_id_event_creation_preserve_the_checkpoint_and_original() {
        let root = tempfile::tempdir().unwrap();
        let mut session =
            crate::checkpoint::new_checkpoint("fixture".into(), "LIFE".into(), vec![]);
        session.history_cursor = Some(new_cursor(root.path()).unwrap());
        let before = serde_json::to_value(&session).unwrap();
        let mut calls = 0;
        assert!(
            append_using(root.path(), &mut session, transition("iterate"), |_| {
                calls += 1;
                Err(anyhow::anyhow!("random failure"))
            })
            .is_err()
        );
        assert_eq!(calls, 1);
        assert_eq!(serde_json::to_value(&session).unwrap(), before);
        let fixed = "EVT-000000000000000000000000";
        append_using(root.path(), &mut session, transition("iterate"), |_| {
            Ok(fixed.into())
        })
        .unwrap();
        let cursor = session.history_cursor.as_ref().unwrap().clone();
        let path = event_path(&cursor.cycle, fixed).unwrap();
        let bytes = fs::read(local_path(root.path(), &path).unwrap()).unwrap();
        let before = serde_json::to_value(&session).unwrap();
        let mut calls = 0;
        assert!(
            append_using(root.path(), &mut session, transition("pause"), |_| {
                calls += 1;
                Ok(fixed.into())
            })
            .is_err()
        );
        assert_eq!(calls, 1);
        assert_eq!(serde_json::to_value(&session).unwrap(), before);
        assert_eq!(
            fs::read(local_path(root.path(), &path).unwrap()).unwrap(),
            bytes
        );
        assert_eq!(cycle_usage(root.path(), &cursor.cycle).unwrap().0, 1);
    }

    #[test]
    fn a_cycle_collision_and_an_existing_temporary_file_are_never_replaced() {
        let root = tempfile::tempdir().unwrap();
        let cursor = new_cursor(root.path()).unwrap();
        reserve_cursor(root.path(), cursor.cycle.clone()).unwrap();
        assert!(reserve_cursor(root.path(), cursor.cycle.clone()).is_err());
        let path = "event.json";
        let temporary = root
            .path()
            .join(format!(".event.json.{}.tmp", std::process::id()));
        fs::write(&temporary, b"another operation owns these bytes").unwrap();
        assert!(publish(root.path(), path, b"new content").is_err());
        assert_eq!(
            fs::read(&temporary).unwrap(),
            b"another operation owns these bytes"
        );
        assert!(!root.path().join(path).exists());
    }
}
