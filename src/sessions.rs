//! Sessions in their own folders (REQ-history-006, REQ-records-004).
//!
//! Each open session is `.chough/sessions/<id>/session.md`, with `evidence/` and
//! `agent/` beside it; the active one is named by `.chough/local/active-session`. A
//! session only ever closes itself: at its own move into done, with `chough session
//! close`, or by `chough doctor --fix`. Starting or switching work never archives,
//! closes or changes another session (Q3, #207). Archiving writes one story,
//! `.chough/archive/sessions/<id>.md`, and then removes the session folder file by file,
//! each only when its exact bytes are committed at HEAD (REQ-records-012).

use crate::checkpoint::{self, Checkpoint, Session};
use crate::markdown::{Kind as RecordKind, Value};
use crate::records::{
    CHANGE_ID_LIMIT, ensure_text, local_path, now, print_json, read_bounded_record,
    valid_change_id, valid_module_name, valid_name, write_new, write_new_bounded,
    write_replace_bounded,
};
use anyhow::{Context, Result, bail, ensure};
use clap::{Subcommand, ValueEnum};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};

/// The session each project root resolved to in this process. A command resolves its
/// session once, so a `session use` in another process partway through cannot
/// redirect its save into a different session's file.
static PINNED: Mutex<Option<HashMap<PathBuf, String>>> = Mutex::new(None);

fn pinned(root: &Path) -> Option<String> {
    PINNED
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .as_ref()
        .and_then(|map| map.get(root).cloned())
}

/// Forgets this process's resolved session for `root`, as a fresh process would start.
pub(super) fn unpin(root: &Path) {
    if let Some(map) = PINNED
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .as_mut()
    {
        map.remove(root);
    }
}

pub(super) fn pin(root: &Path, relative: &str) {
    PINNED
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .get_or_insert_with(HashMap::new)
        .insert(root.to_path_buf(), relative.to_string());
}

pub(super) const DIR: &str = ".chough/sessions";
/// Archived session stories, kept for good (KNOW-34).
const ARCHIVE: &str = ".chough/archive/sessions";
/// Session folders set aside by `chough session shelve`, moved byte for byte.
const SHELVED: &str = ".chough/archive/shelved";
const LOCAL: &str = ".chough/local";
pub(super) const POINTER: &str = ".chough/local/active-session";
const INBOX_LIMIT: u64 = 16_384;
const MAX_INBOX: usize = 200;

#[derive(Subcommand)]
pub(super) enum SessionCommand {
    /// Create a session for an objective or native change and make it active. No other
    /// session is touched.
    New {
        key: String,
        #[arg(long, default_value = DEFAULT_OBJECTIVE)]
        objective: String,
        #[arg(long, default_value = "")]
        intent_scope: String,
        #[arg(long = "module")]
        modules: Vec<String>,
    },
    /// Make an open session active, by ID, alias or a unique bare slug. No other session
    /// is touched.
    Use { key: String },
    /// List open and archived sessions, with each finished open session's fix.
    List,
    /// Close an open session for abandoned work: record the reason and archive it,
    /// without a native read (DRIVE-16).
    Close {
        key: String,
        /// Why the work is abandoned; recorded in the session as a decision.
        #[arg(long)]
        reason: String,
    },
    /// Move an oversized or unreadable session folder, unparsed and byte for byte, into
    /// `.chough/archive/shelved/`.
    Shelve {
        /// The session to shelve.
        key: String,
    },
}

#[derive(Clone, Copy, ValueEnum)]
pub(super) enum Kind {
    Observation,
    Decision,
    Completed,
    Question,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Self::Observation => "observation",
            Self::Decision => "decision",
            Self::Completed => "completed",
            Self::Question => "question",
        }
    }
}

#[derive(Subcommand)]
pub(super) enum InboxCommand {
    /// Append one entry to a session's inbox without editing the session.
    Add {
        #[arg(long)]
        session: String,
        #[arg(long)]
        from: String,
        #[arg(long, value_enum)]
        kind: Kind,
        /// Text; for an observation, its evidence ID.
        #[arg(long)]
        text: String,
    },
    /// List the active session's unfolded entries.
    List,
    /// Add the active session's unfolded entries to it and move them to `folded/`.
    Fold,
}

/// An inbox entry: one Markdown record in the session's `agent/inbox/`
/// (REQ-history-009). It has no approval, review, consent, gate or stage field.
#[derive(Serialize)]
struct Entry {
    id: String,
    session: String,
    from: String,
    kind: String,
    text: String,
    added: String,
}

/// Front matter keys an inbox entry may not carry (REQ-history-009).
const FORBIDDEN_ENTRY_KEYS: [&str; 5] = ["approval", "review", "consent", "gate", "stage"];

/// The session file of open session `id`.
pub(super) fn session_file(id: &str) -> String {
    format!("{DIR}/{id}/session.md")
}

/// The story of archived session `id`.
pub(super) fn archive_file(id: &str) -> String {
    format!("{ARCHIVE}/{id}.md")
}

/// The objective `chough session new` records when none is given.
const DEFAULT_OBJECTIVE: &str = "Describe the current objective before beginning work.";

/// The section of an archived story that names its session's bugs (REQ-history-006).
const STORY_BUGS: &str = "Bugs";

/// The stage status an archived story's front matter gives (REQ-history-006).
pub(super) const ARCHIVED_STATUS: &str = "archived";

/// Whether `relative` is an archived story, `.chough/archive/sessions/<id>.md`, whose
/// stage status may be [`ARCHIVED_STATUS`].
pub(super) fn is_story(relative: &str) -> bool {
    relative
        .strip_prefix(&format!("{ARCHIVE}/"))
        .and_then(|rest| rest.strip_suffix(".md"))
        .is_some_and(|id| !id.is_empty() && !id.contains('/'))
}

fn inbox_dir(id: &str) -> String {
    format!("{DIR}/{id}/agent/inbox")
}

/// The session ID of a session file path.
pub(super) fn id_of(relative: &str) -> Option<String> {
    relative
        .strip_prefix(&format!("{DIR}/"))
        .and_then(|rest| rest.strip_suffix("/session.md"))
        .filter(|id| valid_change_id(id))
        .map(str::to_string)
}

pub(super) fn ensure_key(key: &str) -> Result<()> {
    ensure!(
        valid_change_id(key),
        "invalid session key: up to {CHANGE_ID_LIMIT} bytes of letters, digits, - and _"
    );
    Ok(())
}

pub(super) fn enabled(root: &Path) -> Result<bool> {
    Ok(local_path(root, DIR)?.is_dir())
}

/// The names (without `.json`) of the JSON files directly in `dir`.
fn json_names_in(root: &Path, dir: &str) -> Result<Vec<String>> {
    let path = local_path(root, dir)?;
    let mut names = vec![];
    let entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(names),
        Err(error) => return Err(error).with_context(|| format!("reading {dir}")),
    };
    for entry in entries {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        if let Some(name) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.strip_suffix(".json"))
            && valid_change_id(name)
        {
            names.push(name.to_string());
        }
    }
    names.sort();
    Ok(names)
}

/// The IDs of open sessions: folders in `.chough/sessions/` holding a `session.md`
/// whose story is not written yet. A folder whose story exists counts as archived
/// even while files it could not remove are still there.
pub(super) fn open_ids(root: &Path) -> Result<Vec<String>> {
    let mut ids = vec![];
    let entries = match fs::read_dir(local_path(root, DIR)?) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(ids),
        Err(error) => return Err(error).context("reading .chough/sessions"),
    };
    for entry in entries {
        let entry = entry?;
        let Some(id) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };
        // A `session.md` that is not a regular file, such as a FIFO, still makes the
        // folder a session: one that cannot be read, which `session list` reports with
        // the reason and the recovery instead of dropping it (REQ-history-006, #262).
        if entry.file_type()?.is_dir()
            && valid_change_id(&id)
            && local_path(root, &session_file(&id))?.exists()
            && !local_path(root, &archive_file(&id))?.exists()
        {
            ids.push(id);
        }
    }
    ids.sort();
    Ok(ids)
}

/// The IDs of archived sessions, from their stories.
fn archived_ids(root: &Path) -> Result<Vec<String>> {
    let mut ids = vec![];
    let entries = match fs::read_dir(local_path(root, ARCHIVE)?) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(ids),
        Err(error) => return Err(error).context("reading .chough/archive/sessions"),
    };
    for entry in entries {
        let entry = entry?;
        if let Some(id) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.strip_suffix(".md"))
            .filter(|id| valid_change_id(id))
            && entry.file_type()?.is_file()
        {
            ids.push(id.to_string());
        }
    }
    ids.sort();
    Ok(ids)
}

/// `keys_in(root, DIR)` lists the open session IDs; any other folder lists its JSON
/// file names.
pub(super) fn keys_in(root: &Path, dir: &str) -> Result<Vec<String>> {
    if dir == DIR {
        open_ids(root)
    } else {
        json_names_in(root, dir)
    }
}

/// The open session `name` names: its ID, one of its aliases, or the bare slug of
/// exactly one open session. An alias two sessions hold is refused (REQ-records-003).
/// The file of the open session or archived story `name` names, by ID, bare key or
/// alias (REQ-records-003, REQ-history-015).
pub(super) fn resolve_any(root: &Path, name: &str) -> Result<String> {
    match resolve_open(root, name) {
        Ok(id) => return Ok(session_file(&id)),
        Err(error) => {
            let message = error.to_string();
            if !message.starts_with("no session ") && !message.ends_with("is archived") {
                return Err(error);
            }
        }
    }
    let mut found = vec![];
    for id in archived_ids(root)? {
        let aliases = read_at(root, &archive_file(&id))
            .map(|session| session.meta.aliases)
            .unwrap_or_default();
        if id == name || id.get(11..) == Some(name) || aliases.iter().any(|alias| alias == name) {
            found.push(id);
        }
    }
    match found.as_slice() {
        [id] => Ok(archive_file(id)),
        [] => bail!(
            "refused: no open session or archived story {}; nothing was written",
            crate::leaks::shown("session name", name)
        ),
        several => bail!(
            "refused: {name} names several archived stories ({}); name one by its ID; nothing was written",
            several.join(", ")
        ),
    }
}

pub(super) fn resolve_open(root: &Path, name: &str) -> Result<String> {
    let found = open_matches(root, name)?;
    match found.as_slice() {
        [id] => Ok(id.clone()),
        [] => {
            for id in archived_ids(root)? {
                let aliases = read_at(root, &archive_file(&id))
                    .map(|session| session.meta.aliases)
                    .unwrap_or_default();
                if id == name
                    || id.get(11..) == Some(name)
                    || aliases.iter().any(|alias| alias == name)
                {
                    // REQ-records-006 (#367): a name is shown only when clean.
                    bail!(
                        "session {} is archived",
                        crate::leaks::shown("session name", name)
                    );
                }
            }
            bail!("no session {}", crate::leaks::shown("session name", name))
        }
        _ => bail!(
            "{name} names more than one open session ({}); use the full ID",
            found.join(", ")
        ),
    }
}

/// The open sessions `name` names: itself when it is an open session's ID, otherwise
/// each open session that has it as an alias or bare slug.
pub(super) fn open_matches(root: &Path, name: &str) -> Result<Vec<String>> {
    let ids = open_ids(root)?;
    if ids.iter().any(|id| id == name) {
        return Ok(vec![name.to_string()]);
    }
    let mut found: Vec<String> = vec![];
    for id in &ids {
        let aliases = read_at(root, &session_file(id))
            .map(|session| session.meta.aliases)
            .unwrap_or_default();
        let bare = id.get(11..).unwrap_or_default();
        if aliases.iter().any(|alias| alias == name) || bare == name {
            found.push(id.clone());
        }
    }
    Ok(found)
}

pub(super) fn pointer(root: &Path) -> Result<Option<String>> {
    let path = local_path(root, POINTER)?;
    if !path.exists() {
        return Ok(None);
    }
    let id = read_bounded_record(&path, 512)?.trim().to_string();
    ensure_key(&id)?;
    Ok(Some(id))
}

/// The pointer's session: its own ID when a session folder or story has that ID,
/// otherwise the session that holds it as an alias or bare key, which a 0.5.x pointer
/// that an in-place migration keeps, or `session use <alias>`, may leave
/// (REQ-records-009, REQ-records-003). A pointer that names nothing stays as it is.
pub(super) fn pointed(root: &Path) -> Result<Option<String>> {
    let Some(id) = pointer(root)? else {
        return Ok(None);
    };
    if local_path(root, &session_file(&id))?.is_file()
        || local_path(root, &archive_file(&id))?.is_file()
    {
        return Ok(Some(id));
    }
    Ok(Some(resolve_open(root, &id).unwrap_or(id)))
}

/// Whether the project has Chough's records but no open session and no pointer: the
/// normal state after the move into done archives one, until a session is opened
/// (REQ-history-006). `chough next` then says so and exits 0, while `chough status`
/// still refuses, as `active` does.
pub(super) fn none_open(root: &Path) -> Result<bool> {
    Ok(local_path(root, ".chough")?.is_dir()
        && pinned(root).is_none()
        && pointed(root)?.is_none()
        && open_ids(root)?.is_empty())
}

/// The active session's ID, or a refusal naming the next command.
fn active(root: &Path) -> Result<String> {
    if let Some(id) = pointed(root)? {
        if local_path(root, &archive_file(&id))?.is_file() {
            bail!(
                "the active session {id} is archived; run `chough session use <id>` or `chough session new <key>`"
            );
        }
        // A record that is there but is not a regular file is a session that cannot be
        // read, not a missing one: its reader refuses it with the recovery (#262).
        if local_path(root, &session_file(&id))?.exists() {
            return Ok(id);
        }
        bail!(
            "the active session {id} does not exist; run `chough session use <id>` or `chough session new <key>`"
        );
    }
    let open = open_ids(root)?;
    if let [id] = open.as_slice() {
        return Ok(id.clone());
    }
    // A finished session does not count toward several open ones (DRIVE-16); an
    // unreadable one does, so it is never skipped silently.
    let unfinished: Vec<&String> = open
        .iter()
        .filter(|id| {
            !read_at(root, &session_file(id)).is_ok_and(|session| finished(root, &session))
        })
        .collect();
    match unfinished.as_slice() {
        [id] => Ok((*id).clone()),
        [] if open.is_empty() => bail!("no open session; run `chough session new <key>`"),
        [] => bail!(
            "no unfinished open session (finished: {}); run `chough session new <key>`",
            open.join(", ")
        ),
        _ => bail!(
            "several open sessions ({}) and no active pointer; run `chough session use <id>`",
            unfinished
                .iter()
                .map(|id| id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// Whether a session is finished: at stage done, or with its native change archived on
/// HEAD, as an archive's `finalization.json` records, at any stage. No native read
/// (REQ-history-006, DRIVE-16).
pub(super) fn finished(root: &Path, session: &Checkpoint) -> bool {
    session.stage.current == "done"
        || session
            .native_change
            .as_deref()
            .is_some_and(|change| crate::library::finalized_at(root, change).is_some())
}

/// The project-relative path of the canonical checkpoint (REQ-cli-037).
/// Resolved once per command (see [`PINNED`]); a session this command did not choose
/// that is archived or removed meanwhile is refused rather than written again.
pub(super) fn checkpoint_path(root: &Path) -> Result<String> {
    if let Some(relative) = pinned(root) {
        let archived = id_of(&relative)
            .map(|id| local_path(root, &archive_file(&id)).map(|path| path.exists()))
            .transpose()?
            .unwrap_or(false);
        if archived || !local_path(root, &relative)?.is_file() {
            bail!("{relative} was archived or removed while this command ran; nothing was saved");
        }
        return Ok(relative);
    }
    let relative = session_file(&active(root)?);
    pin(root, &relative);
    Ok(relative)
}

/// Take the checkpoint writer lock for a session command when the project's `.chough`
/// directory exists; before it exists there is nothing a concurrent writer could lose.
pub(super) fn lock_project(root: &Path) -> Result<()> {
    if local_path(root, ".chough")?.is_dir() {
        checkpoint::lock_checkpoint(root)?;
    }
    Ok(())
}

/// `.chough/README.md`, the records tree's reading guide (REQ-records-002), written
/// create-only by `chough init`; an existing one is kept. A fresh project has no folder
/// kept from Chough 0.5.x, so this guide names none (t38n's round-4 F11); `chough
/// migrate --apply` writes [`migrated_readme`].
pub(super) const README: &str = "# Chough records

This folder holds Chough's records for this project. Each record is one Markdown file:
people read its body, agents parse its front matter.

- `sessions/<id>/session.md`: an open piece of work. Its front matter says where the
  work stands; its Timeline lists every move, appended and never rewritten.
  `evidence/` holds what was checked and `agent/` an agent's notes and inbox. The
  folder leaves once the session's story is written and committed.
- `archive/sessions/<id>.md`: the story of a finished session, kept for good, with its
  key screenshots beside it in `archive/sessions/<id>/`.
- `archive/shelved/<id>/`: a session set aside unread.
- `archive/migrations/<id>.md`: the receipt of a migration from Chough 0.5.x.
- `bugs/<id>.md`: one bug each, kept for good; its Log only grows.
- `cases/*.cases.toml`: test-case definitions, which a person edits.
- `preferences.json`: the project's standing preferences, which a person edits.
- `local/`: this machine's state (the active session, the writer lock, gate lists,
  logs). It is never committed.

Intent lives in [hi/](../hi/), contracts in [specs/](../specs/) and change history in
[.specsync/archive/](../.specsync/archive/). Chough links them and copies nothing.
";

/// The reading guide `chough migrate --apply` writes create-only (REQ-records-002):
/// [`README`] with the `artifacts/` line, since a migrated project may keep files there
/// for an open bug (REQ-records-009).
pub(super) fn migrated_readme() -> String {
    README.replacen(
        "- `local/`:",
        "- `artifacts/`: kept from Chough 0.5.x only for files an open bug names as its fixture.\n- `local/`:",
        1,
    )
}

/// Whether `name` is a write temp: `.<name>.<pid>.tmp`.
fn is_write_temp(name: &str) -> bool {
    name.strip_prefix('.')
        .and_then(|rest| rest.strip_suffix(".tmp"))
        .and_then(|rest| rest.rsplit_once('.'))
        .is_some_and(|(stem, pid)| {
            (stem.ends_with(".json") || stem.ends_with(".md"))
                && !pid.is_empty()
                && pid.bytes().all(|byte| byte.is_ascii_digit())
        })
}

/// Remove write temps that killed writers left in a session folder or its inbox, and
/// make sure Git ignores that pattern. Called by `save_checkpoint` while it holds the
/// checkpoint lock, under which every session writer renames and every inbox add links,
/// so a temp seen here was left by a killed writer.
pub(super) fn remove_orphaned_temps(root: &Path) -> Result<()> {
    if !enabled(root)? {
        return Ok(());
    }
    // Temporaries live in `.chough/local/` (records::temporary_for); older ones may
    // still sit in a session folder or its inbox.
    let mut dirs = vec![(
        LOCAL.to_string(),
        "a record write that was stopped before it saved; no record was changed by it",
    )];
    for id in open_ids(root)? {
        dirs.push((
            format!("{DIR}/{id}"),
            "a session write that was stopped before it saved; the session was not changed by it",
        ));
        dirs.push((
            inbox_dir(&id),
            "an inbox add that was stopped before it published its entry; no entry was added by it",
        ));
    }
    for (dir, left_by) in dirs {
        let Ok(entries) = fs::read_dir(local_path(root, &dir)?) else {
            continue;
        };
        for entry in entries {
            let entry = entry?;
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            if !is_write_temp(name) || !entry.file_type()?.is_file() {
                continue;
            }
            let bytes = entry.metadata().map_or(0, |metadata| metadata.len());
            fs::remove_file(entry.path()).with_context(|| format!("removing {dir}/{name}"))?;
            eprintln!("chough: removed {dir}/{name} ({bytes} bytes), left by {left_by}");
        }
    }
    Ok(())
}

/// Publish an inbox entry whole (#99 review, #435): write it through the shared
/// whole-or-not-at-all writer `records::write_new_bytes`, which writes the bytes to a
/// temporary in `.chough/local/` and moves it to the entry's name without replacing
/// anything, or creates the file at its name and removes it again on error; no hard link
/// is taken, so the entry is written whole or not at all and the command works on volumes
/// without hard links, such as exFAT. An add stopped partway leaves at most a temp, never
/// a partial entry, and returns false when an entry of that name already exists.
fn publish_entry(root: &Path, file: &str, content: &str) -> Result<bool> {
    ensure!(
        content.len() as u64 <= INBOX_LIMIT,
        "managed file exceeds {INBOX_LIMIT} bytes: {file}"
    );
    crate::records::write_new_bytes(root, file, content.as_bytes())
        .with_context(|| format!("writing {file}"))
}

pub(super) fn read_at(root: &Path, relative: &str) -> Result<Checkpoint> {
    let content = read_bounded_record(&local_path(root, relative)?, checkpoint::SESSION_LIMIT)
        .with_context(|| format!("reading {relative}"))
        .with_context(|| shelve_hint(relative))?;
    match checkpoint::parse(relative, &content)
        .with_context(|| format!("reading {relative}"))
        .with_context(|| shelve_hint(relative))?
    {
        Session::V2(session) => Ok(*session),
    }
}

/// The recovery for a session that cannot be read (REQ-cli-032, DRIVE-14).
pub(super) fn shelve_hint(relative: &str) -> String {
    let id = id_of(relative).unwrap_or_else(|| "<id>".into());
    format!(
        "{relative} cannot be read: fix the line named below by hand, or set the session aside with `chough session shelve {id}` and run `chough session new <key>` (a session record is at most {} bytes)",
        checkpoint::SESSION_LIMIT
    )
}

fn point(root: &Path, id: &str) -> Result<()> {
    write_new(root, &format!("{LOCAL}/.gitignore"), "*\n")?;
    write_replace_bounded(root, POINTER, &format!("{id}\n"), 512)?;
    pin(root, &session_file(id));
    Ok(())
}

/// The number of unfolded entries in a session's inbox.
pub(super) fn unfolded(root: &Path, id: &str) -> Result<usize> {
    Ok(entry_names(root, &inbox_dir(id))?.len())
}

/// The names (without `.md`) of the entry files directly in an inbox folder.
fn entry_names(root: &Path, dir: &str) -> Result<Vec<String>> {
    let mut names = vec![];
    let entries = match fs::read_dir(local_path(root, dir)?) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(names),
        Err(error) => return Err(error).with_context(|| format!("reading {dir}")),
    };
    for entry in entries {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        if let Some(name) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.strip_suffix(".md"))
            .filter(|name| !name.starts_with('.') && valid_change_id(name))
        {
            names.push(name.to_string());
        }
    }
    names.sort();
    Ok(names)
}

/// Create a new session for `key` and make it active, touching no other session
/// (REQ-history-006, Q3). Returns its ID.
pub(super) fn create(
    root: &Path,
    key: &str,
    objective: String,
    intent_scope: String,
    modules: Vec<String>,
) -> Result<String> {
    ensure_key(key)?;
    // The key names the session folder and stays its alias (REQ-records-006).
    crate::leaks::check_name(crate::leaks::Local::current(), "key", key)?;
    ensure!(!objective.trim().is_empty(), "objective must not be blank");
    ensure!(
        modules.iter().all(|module| valid_module_name(module)),
        "invalid module name: use a module ID SpecSync accepts, such as cli or chat.service"
    );
    lock_project(root)?;
    for id in open_ids(root)?.iter().chain(archived_ids(root)?.iter()) {
        let file = if local_path(root, &archive_file(id))?.is_file() {
            archive_file(id)
        } else {
            session_file(id)
        };
        if let Ok(session) = read_at(root, &file) {
            ensure!(
                id != key && !session.meta.aliases.iter().any(|alias| alias == key),
                "session key {key} is already used by session {id}"
            );
        }
    }
    let date = crate::identity::utc_date(now()?);
    let name = crate::identity::dated_name(&date, key, "session");
    // REQ-history-002 (F11): the session's first entry starts its first stage segment,
    // with its objective, or its key when none was given.
    let start = if objective == DEFAULT_OBJECTIVE {
        key.to_string()
    } else {
        objective.clone()
    };
    let mut session = checkpoint::new_checkpoint(objective, intent_scope, modules);
    session.meta.aliases = vec![key.to_string()];
    crate::stage::log_start(root, &mut session, &start)?;
    // How many leaks `--redact` replaced, said once the session is written.
    let redacted = std::cell::Cell::new(0);
    let render = |id: &str| -> Result<String> {
        let mut draft = session.clone();
        draft.meta.id = id.to_string();
        let (record, count) = checkpoint::render_counted(&draft)?;
        redacted.set(count);
        Ok(record.render())
    };
    let id = if local_path(root, &archive_file(&name))?.exists() {
        // An archived session's ID is never reused: one suffix is drawn, and a second
        // clash is refused with nothing written (REQ-records-003).
        let suffixed = crate::identity::suffixed(&name)?;
        ensure!(
            !local_path(root, &archive_file(&suffixed))?.exists()
                && write_new_bounded(
                    root,
                    &session_file(&suffixed),
                    &render(&suffixed)?,
                    checkpoint::SESSION_LIMIT
                )?,
            "refused: {} also exists; nothing was written and no other name was drawn",
            session_file(&suffixed)
        );
        suffixed
    } else {
        crate::identity::create_named(
            root,
            DIR,
            &name,
            "/session.md",
            checkpoint::SESSION_LIMIT,
            render,
        )?
    };
    if crate::leaks::redacting() && redacted.get() > 0 {
        eprintln!(
            "chough: redacted {} value(s) in the session record (--redact)",
            redacted.get()
        );
    }
    point(root, &id)?;
    Ok(id)
}

pub(super) fn run(root: &Path, command: SessionCommand) -> Result<()> {
    match command {
        SessionCommand::New {
            key,
            objective,
            intent_scope,
            modules,
        } => {
            let id = create(root, &key, objective, intent_scope, modules)?;
            print_json(&json!({"schema_version":1,"created":session_file(&id),"id":id,"active":id}))
        }
        SessionCommand::Use { key } => {
            lock_project(root)?;
            ensure!(
                enabled(root)?,
                "no sessions; run `chough session new <key>`"
            );
            let id = resolve_open(root, &key)?;
            read_at(root, &session_file(&id))?;
            point(root, &id)?;
            print_json(&json!({"schema_version":1,"active":id}))
        }
        SessionCommand::List => {
            // Archived stories are sessions too: a tree whose sessions are all finished,
            // such as one migrated from 0.5.x, has no `.chough/sessions/` (REQ-records-009).
            ensure!(
                enabled(root)? || !archived_ids(root)?.is_empty(),
                "no sessions; run `chough session new <key>`"
            );
            let current = active(root).ok();
            let mut sessions = vec![];
            for (ids, archived) in [(open_ids(root)?, false), (archived_ids(root)?, true)] {
                for id in ids {
                    let file = if archived {
                        archive_file(&id)
                    } else {
                        session_file(&id)
                    };
                    let session = match read_at(root, &file) {
                        Ok(session) => session,
                        Err(error) => {
                            sessions.push(json!({
                                "id": id,
                                "file": file,
                                "readable": false,
                                "reason": format!("{error:#}"),
                                "recovery": format!("chough session shelve {id}"),
                                "archived": archived,
                            }));
                            continue;
                        }
                    };
                    let finished_open = !archived && finished(root, &session);
                    sessions.push(json!({
                        "id": id,
                        "aliases": session.meta.aliases,
                        "file": file,
                        "objective": session.objective,
                        "native_change": session.native_change,
                        "stage": session.stage.current,
                        "status": if archived { "archived" } else { session.stage.status.as_str() },
                        "active": !archived && current.as_deref() == Some(id.as_str()),
                        "archived": archived,
                        // An archived story is finished; only a finished open session
                        // still has a fix to run (F12c).
                        "finished": archived || finished_open,
                        "fix": finished_open.then_some("chough doctor --fix"),
                        "inbox_unfolded": if archived { 0 } else { unfolded(root, &id)? },
                    }));
                }
            }
            print_json(&json!({"schema_version":1,"active":current,"sessions":sessions}))
        }
        SessionCommand::Close { key, reason } => print_json(&close_session(root, &key, &reason)?),
        SessionCommand::Shelve { key } => {
            lock_project(root)?;
            // REQ-cli-037 (#372): a session folder's name is that folder; any other key
            // resolves as `session use` resolves it, by alias or bare slug.
            let folder =
                if valid_change_id(&key) && local_path(root, &format!("{DIR}/{key}"))?.is_dir() {
                    key
                } else {
                    // A key that names several sessions is refused as `session use`
                    // refuses it; one that names none falls through to the folder check.
                    match open_matches(root, &key)?.as_slice() {
                        [id] => id.clone(),
                        [] => key,
                        // Several: refused with `session use`'s own message. Should the two
                        // reads ever disagree, the session it resolves is shelved, never a
                        // panic.
                        _ => resolve_open(root, &key)?,
                    }
                };
            shelve(root, &folder)
        }
    }
}

/// REQ-history-006 (DRIVE-13, #155): at a successful move into done the active session
/// archives itself and the active pointer is cleared. A session with unfolded inbox
/// entries stays open, and so does one for which release is authorized; it archives
/// itself at the move from release into done. Never undoes the move.
pub(super) fn archive_at_done(root: &Path, session: &Checkpoint, from: &str) -> serde_json::Value {
    archive_self(root, session, from)
        .unwrap_or_else(|error| json!({"status": "refused", "reason": format!("{error:#}")}))
}

fn archive_self(root: &Path, session: &Checkpoint, from: &str) -> Result<serde_json::Value> {
    if from != "release" && crate::prefs::authorized(root, session, "release")?.is_some() {
        return Ok(
            json!({"status": "kept_open", "reason": "release is authorized; the session archives itself at the move from release into done"}),
        );
    }
    let relative = checkpoint_path(root)?;
    let Some(id) = id_of(&relative) else {
        return Ok(
            json!({"status": "skipped", "reason": format!("{relative} is not an open session")}),
        );
    };
    let count = unfolded(root, &id)?;
    if count > 0 {
        return Ok(json!({"status": "kept_open", "id": id, "inbox_unfolded": count}));
    }
    let report = archive(root, &id, session, "done")?;
    // The rest of this command reads its checkpoint where it now is.
    pin(root, &report.story);
    Ok(json!({"status": "archived", "id": id, "story": report.story, "kept": report.kept}))
}

/// What archiving a session wrote and left.
struct Archived {
    story: String,
    kept: Vec<String>,
}

/// Archive open session `id`, whose checkpoint is `session`, with `outcome` (`done` or
/// `closed-without-done`): write its story by create-new and read it back, then remove
/// the session folder file by file, each only when its exact bytes are committed at
/// HEAD (REQ-records-012); anything else stays and is reported, and `chough doctor
/// --fix` removes it after the commit. Clears the pointer when it names `id`.
fn archive(root: &Path, id: &str, session: &Checkpoint, outcome: &str) -> Result<Archived> {
    let story = archive_file(id);
    ensure!(
        !local_path(root, &story)?.exists(),
        "{story} already exists; archiving never replaces a file"
    );
    // Render with the session's evidence links, as every save does, so each entry's
    // line links its file and becomes its receipt line below, never a bare ID beside it.
    let mut linked = session.clone();
    linked.evidence_links = checkpoint::evidence_links(root, &linked);
    let (mut record, redacted) = checkpoint::render_counted(&linked)?;
    // REQ-history-006 (#344): when the objective changed after the Goal was filled, the
    // story says so in one line after the Goal paragraph; nothing is rewritten.
    let objective = session
        .objective
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let goal = crate::journal::first_paragraph(&record, crate::session_md::GOAL);
    if !objective.is_empty() && goal != objective {
        record.append_to_section(
            crate::session_md::GOAL,
            &format!("\nObjective at closeout: {objective}\n"),
        )?;
    }
    // REQ-history-006 (Kimi 6): the story is archived, and the session's next action no
    // longer applies.
    record.set_scalar("stage_status", ARCHIVED_STATUS)?;
    record.remove("next");
    let date = crate::identity::utc_date(now()?);
    record.set_scalar("closed", &date)?;
    record.set_scalar("outcome", outcome)?;
    let receipts = crate::evidence::receipts_of(root, id)?;
    story_verified_lines(root, &mut record, id, &receipts)?;
    // REQ-history-006 (F10b): the bugs the session's checkpoint holds, after What was
    // verified, each with its title and status, linked to its record.
    let bugs = crate::bugs::story_lines(root, &crate::bugs::of_session(root, session));
    let bugs = if bugs.is_empty() {
        "None.\n".to_string()
    } else {
        bugs.iter().map(|line| format!("- {line}\n")).collect()
    };
    record.insert_section_before(
        STORY_BUGS,
        &[crate::session_md::THREADS, crate::session_md::TIMELINE],
        &bugs,
    )?;
    // REQ-history-006 (F10a): no receipts in the front matter, so the story comes first;
    // it ends with the Timeline and its closing line, then `## Receipts`. The front
    // matter also leaves out each empty value, which a reader takes as empty.
    record.remove("receipts");
    let empty: Vec<String> = record
        .keys()
        .into_iter()
        .filter(|key| match record.get(key) {
            Some(Value::Scalar(text)) => text.is_empty(),
            Some(Value::List(items)) => items.is_empty(),
            Some(Value::Maps(maps)) => maps.is_empty(),
            None => false,
        })
        .map(str::to_string)
        .collect();
    for key in empty {
        record.remove(&key);
    }
    // REQ-records-016 (#366): the story holds the session's text as it is printed, so a
    // control character a session holds by hand is escaped, never carried raw.
    record.make_printable();
    // REQ-cli-007 (#353): the story writes the pull request as the session does,
    // double-quoted; an empty one went with the other empty values above.
    if let Some(pr) = record
        .scalar("pr")
        .map(str::to_string)
        .filter(|pr| !pr.is_empty())
    {
        record.set_quoted("pr", &pr)?;
    }
    let mut text = record.render_within(RecordKind::Story)?;
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str(&format!("\nClosed {date}: {outcome}.\n"));
    text.push_str(&crate::markdown::receipts_section(receipts)?);
    ensure!(
        write_new_bounded(root, &story, &text, RecordKind::Story.bound())?,
        "{story} already exists; archiving never replaces a file"
    );
    let written = read_bounded_record(&local_path(root, &story)?, RecordKind::Story.bound())?;
    ensure!(
        written == text,
        "{story} did not read back as written; nothing was removed"
    );
    // REQ-records-006: under `--redact`, the count of what the story's write replaced,
    // once the story is written, as `session close` over a stored leak needs.
    if crate::leaks::redacting() && redacted > 0 {
        eprintln!("chough: redacted {redacted} value(s) in the archived story {story} (--redact)");
    }
    let mut kept = crate::evidence::move_screenshots_at_finish(root, id)?;
    // A screenshot that is not committed yet is kept by both steps; list it once.
    for file in remove_committed(root, &format!("{DIR}/{id}"))? {
        if !kept.contains(&file) {
            kept.push(file);
        }
    }
    if pointed(root).ok().flatten().as_deref() == Some(id) {
        let _ = fs::remove_file(local_path(root, POINTER)?);
    }
    unpin(root);
    Ok(Archived { story, kept })
}

/// The story's What was verified lines (REQ-history-006, REQ-records-013): one per
/// receipt of the latest run of each case and per receipt bound to no case, its claim,
/// its command unless the claim already holds it, and the first 12 digits of its
/// fingerprint, a link to its key screenshot beside the story when it has one, ending
/// `^<evidence id>`; then, first in the section, one line counting the cases' earlier
/// runs, which stay in `receipts` (F8). A line Chough wrote while the session was open
/// links `evidence/<id>.md`, a folder that leaves with the session, so it becomes that
/// receipt line, or goes for an earlier run; a line a person wrote is kept, and an entry
/// it already names gets no second line.
fn story_verified_lines(
    root: &Path,
    record: &mut crate::markdown::Record,
    id: &str,
    receipts: &[Vec<(String, String)>],
) -> Result<()> {
    use crate::session_md::VERIFIED;
    // The key screenshots, by their entry's ID, as they will sit beside the story.
    let mut screenshots = std::collections::BTreeMap::new();
    if let Ok(entries) = fs::read_dir(local_path(root, &format!("{DIR}/{id}/evidence"))?) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some((stem, extension)) = name.rsplit_once('.')
                && matches!(extension, "png" | "jpg" | "jpeg")
            {
                screenshots.insert(stem.to_string(), name.clone());
            }
        }
    }
    let get = |receipt: &[(String, String)], key: &str| {
        receipt
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.clone())
            .filter(|value| !value.is_empty())
    };
    // Each case's latest run: the last of its receipts, which are in observation-time
    // order, oldest first.
    let mut latest = std::collections::BTreeMap::new();
    for receipt in receipts {
        if let (Some(case), Some(entry)) = (get(receipt, "case"), get(receipt, "id")) {
            latest.insert(case, entry);
        }
    }
    let mut earlier = 0_usize;
    for receipt in receipts {
        let Some(entry) = get(receipt, "id") else {
            continue;
        };
        let present: Vec<String> = record
            .section(VERIFIED)
            .unwrap_or_default()
            .lines()
            .filter_map(|line| line.strip_prefix("- "))
            .map(str::to_string)
            .collect();
        // The open session's line for this entry, a link to `evidence/<id>.md` or, when
        // the checkpoint was rendered without its links, the bare ID.
        let linked = format!("(evidence/{entry}.md)");
        let open = present
            .iter()
            .find(|line| line.ends_with(&linked) || **line == entry);
        if get(receipt, "case").is_some_and(|case| latest.get(&case) != Some(&entry)) {
            // An earlier run of a case: counted, and its open line goes with the file
            // it links.
            earlier += 1;
            if let Some(open) = open {
                record.remove_item(VERIFIED, open)?;
            }
            continue;
        }
        let screenshot = screenshots.get(&entry).map(|name| format!("{id}/{name}"));
        let Some(line) = receipt_line(receipt, screenshot.as_deref()) else {
            continue;
        };
        // The open line becomes the receipt line, so each entry is listed once.
        if let Some(open) = open {
            record.replace_item(VERIFIED, open, &line)?;
        } else if !present
            .iter()
            .any(|line| line.ends_with(&format!("^{entry}")))
        {
            record.append_to_section(VERIFIED, &format!("- {line}\n"))?;
        }
    }
    if earlier > 0 {
        record.insert_first_item(VERIFIED, &earlier_runs(earlier))?;
    }
    Ok(())
}

/// A receipt's What was verified line (REQ-history-006): its claim, or its ID without
/// one; after one ` - `, its command unless the claim already holds it and the first 12
/// digits of its fingerprint, when it has them; `; limits: ` and its limitations, when
/// it has any; a link to `screenshot`, its key
/// screenshot beside the story, when given; ending `^<evidence id>`. A story at
/// archiving and the migration write the same line (REQ-records-009). None for a
/// receipt with no ID.
pub(super) fn receipt_line(
    receipt: &[(String, String)],
    screenshot: Option<&str>,
) -> Option<String> {
    let get = |key: &str| {
        receipt
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.clone())
            .filter(|value| !value.is_empty())
    };
    let entry = get("id")?;
    let claim = get("claim").unwrap_or_else(|| entry.clone());
    let mut parts = vec![];
    if let Some(command) = get("command").filter(|command| !claim.contains(command)) {
        parts.push(format!("`{command}`"));
    }
    if let Some(fingerprint) = get("fingerprint") {
        parts.push(
            fingerprint
                .trim_start_matches("sha256:")
                .chars()
                .take(12)
                .collect(),
        );
    }
    let mut line = claim;
    if !parts.is_empty() {
        line.push_str(&format!(" - {}", parts.join(" ")));
    }
    // REQ-history-006 (#349): the entry's stated limits, beside its claim.
    if let Some(limitations) = get("limitations") {
        line.push_str(&format!("; limits: {limitations}"));
    }
    if let Some(link) = screenshot {
        line.push_str(&format!(" [screenshot]({link})"));
    }
    line.push_str(&format!(" ^{entry}"));
    // One line, whatever line breaks a claim or a limitation holds: a second line
    // would read as another item, or as a heading, of the story (#349).
    let line = line
        .split(['\n', '\r'])
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    Some(line)
}

/// The story line counting the earlier runs of its cases, which only its `receipts`
/// keep (REQ-history-006).
fn earlier_runs(count: usize) -> String {
    if count == 1 {
        "1 earlier run of the cases below is kept in this story's receipts, not listed here."
            .to_string()
    } else {
        format!(
            "{count} earlier runs of the cases below are kept in this story's receipts, not listed here."
        )
    }
}

/// The newest archived story's checkpoint, by its `closed` date and then its ID.
pub(super) fn newest_archived(root: &Path) -> Result<Option<Session>> {
    let mut newest: Option<(String, String, Checkpoint)> = None;
    for id in archived_ids(root)? {
        let Ok(session) = read_at(root, &archive_file(&id)) else {
            continue;
        };
        let closed = session
            .source
            .as_ref()
            .and_then(|record| record.scalar("closed").map(str::to_string))
            .unwrap_or_default();
        if newest
            .as_ref()
            .is_none_or(|(date, known, _)| (&closed, &id) > (date, known))
        {
            newest = Some((closed, id, session));
        }
    }
    Ok(newest.map(|(_, _, session)| Session::V2(Box::new(session))))
}

/// Session folders whose story exists but which still hold files (REQ-history-012).
pub(super) fn unfinished_archives(root: &Path) -> Result<Vec<String>> {
    let mut ids = vec![];
    for id in archived_ids(root)? {
        let folder = local_path(root, &format!("{DIR}/{id}"))?;
        if folder.is_dir() {
            ids.push(id);
        }
    }
    Ok(ids)
}

/// The files of archived session `id`'s folder that are not committed at HEAD as they
/// are, leaving out macOS metadata, which the archive finish removes as it is (#377), and
/// the entries the story does not hold, which it keeps (#381).
pub(super) fn uncommitted_files(root: &Path, id: &str, unheld: &[String]) -> Result<Vec<String>> {
    let mut files = vec![];
    collect_files(root, &format!("{DIR}/{id}"), &mut files)?;
    Ok(files
        .into_iter()
        .filter(|file| !os_metadata(file))
        .filter(|file| !committed_at_head(root, file))
        // An entry the story does not hold is kept, but one Git does not track yet is
        // committed first, so its printed way out (`git mv`, `git rm`) runs as printed
        // (#381, rule 3).
        .filter(|file| !unheld.contains(file) || !tracked_by_git(root, file))
        .collect())
}

/// The files of archived session `id`'s folder that its story does not hold, which the
/// archive finish keeps so no work is lost (REQ-history-012, #381): each evidence entry,
/// with the files beside it under its name, unless one of the story's receipts has its
/// ID and equals the receipt it gives now, and each entry of `agent/inbox/`, unfolded
/// while it is there. Without a story nothing is held.
pub(super) fn unheld_files(root: &Path, id: &str) -> Result<Vec<String>> {
    let folder = format!("{DIR}/{id}");
    if !local_path(root, &folder)?.is_dir() {
        return Ok(vec![]);
    }
    let story = archive_file(id);
    let held: Vec<Vec<(String, String)>> = match local_path(root, &story) {
        Ok(path) if path.is_file() => {
            // Named by its path in the project, never the absolute path the reader gives.
            let text = read_bounded_record(&path, RecordKind::Story.bound()).map_err(|_| {
                anyhow::anyhow!(
                    "the story {story} cannot be read within its bound of {} bytes",
                    RecordKind::Story.bound()
                )
            })?;
            crate::markdown::parse(&story, &text)?.story_receipts(&story)?
        }
        _ => vec![],
    };
    let mut unheld_ids = vec![];
    for (name, receipt) in crate::evidence::entry_receipts(root, id)? {
        // An entry that cannot be read is not held: the finish keeps it and names it.
        let Some(receipt) = receipt else {
            unheld_ids.push(name);
            continue;
        };
        let entry = receipt
            .iter()
            .find(|(key, _)| key == "id")
            .map(|(_, value)| value.clone())
            .unwrap_or_default();
        // Held when a story receipt has the entry's ID and agrees with the receipt the
        // entry gives now in every field it holds, so a story an earlier release wrote,
        // whose receipts hold fewer fields, still holds its entries (REQ-history-012).
        // A story writes its receipts as printed (REQ-records-016), so a field agrees as
        // written or in that form.
        let holds = |story: &Vec<(String, String)>| {
            story
                .iter()
                .any(|(key, value)| key == "id" && *value == entry)
                && story.iter().all(|(key, value)| {
                    receipt.iter().any(|(name, given)| {
                        name == key
                            && (given == value || crate::leaks::printable(given, false) == *value)
                    })
                })
        };
        // Kept by the file that was read, which may be named other than its ID (renamed,
        // or its `id` edited), and by the ID, which names the files beside it (#381).
        if !held.iter().any(holds) {
            unheld_ids.push(name);
            unheld_ids.push(entry);
        }
    }
    let mut files = vec![];
    collect_files(root, &format!("{folder}/evidence"), &mut files)?;
    let mut unheld: Vec<String> = files
        .into_iter()
        .filter(|file| {
            // A file belongs to an entry when its name is the entry's name or ID and then
            // a dot, so a name holding a dot of its own is kept too; over-keeping a sibling
            // is the safe side (corvid-agent's review of #414).
            let name = file.rsplit('/').next().unwrap_or_default();
            !os_metadata(file)
                && unheld_ids.iter().any(|entry| {
                    name.strip_prefix(entry.as_str())
                        .is_some_and(|rest| rest.starts_with('.'))
                })
        })
        .collect();
    // Every file in the inbox, whatever its name: an unfolded entry renamed with a dot or
    // a capital letter is someone's work too (#381). What `inbox fold` moved into
    // `folded/` is held by the session, and goes with the folder.
    let mut inbox = vec![];
    collect_files(root, &inbox_dir(id), &mut inbox)?;
    let folded = format!("{}/folded/", inbox_dir(id));
    unheld.extend(
        inbox
            .into_iter()
            .filter(|file| !os_metadata(file) && !file.starts_with(&folded)),
    );
    unheld.sort();
    Ok(unheld)
}

/// What a person commits to finish archived session `id`'s archive (REQ-cli-021, #371):
/// each file of its folder, its story and its journal post that Git does not hold as it is,
/// read with one `git status`. A post is the session's when its `record` is the story.
/// None when Git cannot be read, as in a project that is not a repository, where no
/// commit can finish it.
pub(super) fn archive_to_commit(root: &Path, id: &str) -> Option<Vec<String>> {
    let folder = format!("{DIR}/{id}");
    let story = archive_file(id);
    let journal = crate::prefs::effective(root, "library-dir")
        .map(|(dir, _)| format!("{}/journal", dir.trim_end_matches('/')))
        .unwrap_or_else(|_| "knowledge/journal".into());
    let Ok(output) = crate::records::run_owned(
        std::process::Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["status", "--porcelain", "-z", "--untracked-files=all", "--"])
            .args([&folder, &story, &journal]),
        None,
    ) else {
        return None;
    };
    if !output.status.success() {
        return None;
    }
    // With `-z` each path is given as it is, never quoted or escaped, and a rename or a
    // copy gives its source in the field after it.
    let listed = String::from_utf8_lossy(&output.stdout).into_owned();
    let mut fields = listed.split('\0').filter(|field| !field.is_empty());
    let mut changed = vec![];
    while let Some(field) = fields.next() {
        let (Some(status), Some(path)) = (field.get(..2), field.get(3..)) else {
            continue;
        };
        changed.push(path.to_string());
        if (status.contains('R') || status.contains('C'))
            && let Some(source) = fields.next()
        {
            changed.push(source.to_string());
        }
    }
    let mut paths = vec![];
    for path in &changed {
        let path = path.as_str();
        // macOS metadata needs no commit: the finish removes it as it is (#377).
        if os_metadata(path) {
            continue;
        }
        // Each file by name, never the folder: a pathspec for the folder would also take
        // its macOS metadata, which needs no commit (#377).
        let post = path.starts_with(&format!("{journal}/"))
            && local_path(root, path)
                .ok()
                .and_then(|full| fs::read_to_string(full).ok())
                .and_then(|text| crate::markdown::parse(path, &text).ok())
                .is_some_and(|post| post.scalar("record") == Some(story.as_str()));
        if !(path.starts_with(&format!("{folder}/")) || path == story || post) {
            continue;
        }
        let wanted = path.to_string();
        if !paths.contains(&wanted) {
            paths.push(wanted);
        }
    }
    Some(paths)
}

/// The story archived session `id`'s finish waits on (#389, Leif's option B), with
/// whether Git ignores it, while HEAD does not hold it as it is; None once it does, or
/// when there is no story. The journal post never holds the finish, since the finish
/// never removes it.
pub(super) fn held_story(root: &Path, id: &str) -> Option<(String, bool)> {
    let story = archive_file(id);
    if !local_path(root, &story).is_ok_and(|path| path.is_file()) || committed_at_head(root, &story)
    {
        return None;
    }
    let ignored = ignored_by_git(root, &story);
    Some((story, ignored))
}

/// `text` as one single-quoted POSIX shell word.
pub(super) fn shell_quoted(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}

/// The command that commits `plain` and, with `-f` since Git ignores them, `forced`, as
/// `doctor` and `next` both print it, each path quoted (REQ-cli-021, #371, #389).
pub(super) fn commit_command(plain: &[String], forced: &[String], message: &str) -> String {
    let quoted = |files: &[String]| {
        files
            .iter()
            .map(|file| shell_quoted(file))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let mut adds = vec![];
    if !plain.is_empty() {
        adds.push(format!("git add -- {}", quoted(plain)));
    }
    if !forced.is_empty() {
        adds.push(format!("git add -f -- {}", quoted(forced)));
    }
    format!(
        "{} && git commit -q -m {}",
        adds.join(" && "),
        shell_quoted(message)
    )
}

/// Whether Git can read the project's HEAD commit.
pub(super) fn head_readable(root: &Path) -> bool {
    crate::records::run_owned(
        std::process::Command::new("git").arg("-C").arg(root).args([
            "rev-parse",
            "--verify",
            "--quiet",
            "HEAD",
        ]),
        None,
    )
    .is_ok_and(|output| output.status.success())
}

/// Whether Git tracks the file at `relative`.
pub(super) fn tracked_by_git(root: &Path, relative: &str) -> bool {
    crate::records::run_owned(
        std::process::Command::new("git").arg("-C").arg(root).args([
            "ls-files",
            "--error-unmatch",
            "--",
            relative,
        ]),
        None,
    )
    .is_ok_and(|output| output.status.success())
}

/// Whether Git ignores the file at `relative`.
pub(super) fn ignored_by_git(root: &Path, relative: &str) -> bool {
    crate::records::run_owned(
        std::process::Command::new("git").arg("-C").arg(root).args([
            "check-ignore",
            "-q",
            "--",
            relative,
        ]),
        None,
    )
    .is_ok_and(|output| output.status.success())
}

/// Whether the file at `relative` is macOS metadata (REQ-cli-068).
fn os_metadata(relative: &str) -> bool {
    crate::evidence::is_os_metadata(relative.rsplit('/').next().unwrap_or_default())
}

/// Remove an archived session's folder, each file only when committed at HEAD, keeping
/// the entries its story does not hold (#381) and removing macOS metadata as it is
/// (#377); returns the files kept (REQ-records-012).
pub(super) fn remove_archived_folder(
    root: &Path,
    id: &str,
    unheld: &[String],
) -> Result<Vec<String>> {
    ensure!(
        local_path(root, &archive_file(id))?.is_file(),
        "session {id} has no story; it is not archived"
    );
    remove_committed_except(root, &format!("{DIR}/{id}"), unheld)
}

/// Archive open session `id` as finished, for `chough doctor --fix`.
pub(super) fn archive_open(root: &Path, id: &str) -> Result<()> {
    let session = read_at(root, &session_file(id))?;
    archive(root, id, &session, "done").map(|_| ())
}

/// Remove each file under `folder` whose exact bytes are committed at HEAD, then the
/// folders left empty; return the files kept (REQ-records-012).
fn remove_committed(root: &Path, folder: &str) -> Result<Vec<String>> {
    remove_committed_except(root, folder, &[])
}

/// As [`remove_committed`], keeping each file `keep` names, and removing macOS metadata
/// whether committed or not, since it holds no work and would keep the folder forever
/// (REQ-history-012, #377).
fn remove_committed_except(root: &Path, folder: &str, keep: &[String]) -> Result<Vec<String>> {
    let mut files = vec![];
    collect_files(root, folder, &mut files)?;
    let mut kept = vec![];
    for file in files {
        if keep.contains(&file) {
            kept.push(file);
        } else if os_metadata(&file) || committed_at_head(root, &file) {
            fs::remove_file(local_path(root, &file)?)
                .with_context(|| format!("removing {file}"))?;
        } else {
            kept.push(file);
        }
    }
    remove_empty_dirs(&local_path(root, folder)?);
    Ok(kept)
}

fn collect_files(root: &Path, relative: &str, files: &mut Vec<String>) -> Result<()> {
    let Ok(entries) = fs::read_dir(local_path(root, relative)?) else {
        return Ok(());
    };
    let mut names: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();
    names.sort();
    for name in names {
        let child = format!("{relative}/{name}");
        let path = local_path(root, &child)?;
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.is_dir() {
            collect_files(root, &child, files)?;
        } else {
            files.push(child);
        }
    }
    Ok(())
}

fn remove_empty_dirs(path: &Path) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|entry| entry.ok()) {
            if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                remove_empty_dirs(&entry.path());
            }
        }
    }
    let _ = fs::remove_dir(path);
}

/// Whether the file's bytes, as Git would store them, are the blob committed at HEAD.
pub(super) fn committed_at_head(root: &Path, relative: &str) -> bool {
    let git = |arguments: &[&str]| -> Option<String> {
        let output = crate::records::run_owned(
            std::process::Command::new("git")
                .arg("-C")
                .arg(root)
                .args(arguments),
            None,
        )
        .ok()?;
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
    };
    let Some(committed) = git(&[
        "rev-parse",
        "--verify",
        "--quiet",
        &format!("HEAD:{relative}"),
    ]) else {
        return false;
    };
    git(&["hash-object", "--", relative]).is_some_and(|current| current == committed)
}

/// Close `key` as `chough session close` does and return its report.
pub(super) fn close_session(root: &Path, key: &str, reason: &str) -> Result<serde_json::Value> {
    ensure!(
        !reason.trim().is_empty(),
        "the reason must not be blank; say why the work is abandoned with --reason <text>"
    );
    lock_project(root)?;
    let id = resolve_open(root, key)?;
    let count = unfolded(root, &id)?;
    ensure!(
        count == 0,
        "session {id} has {count} unfolded inbox entries; run `chough session use {id}` and `chough inbox fold` first"
    );
    let mut session = read_at(root, &session_file(&id))?;
    let decision = format!("Closed without done: {}", reason.trim());
    ensure_text("reason", &decision)?;
    // REQ-history-006 (t38m F2): an open human gate is withdrawn as
    // `chough checkpoint --clear-awaiting-human` does, one gate-closed entry each with no
    // relayed decision, and named in the report, never dropped silently.
    let withdrawn = std::mem::take(&mut session.awaiting_human);
    let current_stage = session.stage.current.clone();
    for gate in &withdrawn {
        crate::stage::log_gate(
            root,
            &mut session,
            crate::stage::GATE_CLOSED,
            &current_stage,
            gate,
            None,
        )?;
    }
    session.decisions.push(decision);
    checkpoint::validate_v2(&session)?;
    let report = archive(root, &id, &session, "closed-without-done")?;
    // REQ-records-011: the closed session's journal post; the librarian refreshes the
    // vault at release, not here.
    let post = crate::journal::post_for_close(root, &id);
    let index = crate::library::refresh_at_done(root);
    Ok(json!({
        "schema_version": 1,
        "closed": id,
        "archived": report.story,
        "kept": report.kept,
        "reason": reason.trim(),
        "withdrawn_gates": withdrawn,
        "journal_post": post,
        "librarian_index": index,
    }))
}

/// The name of the note shelving writes inside the shelved folder (REQ-history-006).
const SHELVED_NOTE: &str = "shelved.md";

/// One entry of a shelved folder: a file's size and SHA-256, or what else it is: a
/// symbolic link, which is listed and not followed, or another non-regular file, such
/// as a FIFO, which is listed and not read (#262).
type ShelvedFile = (String, Result<(u64, String), &'static str>);

/// Every file under `dir`, folder-relative and sorted (REQ-history-006).
fn shelved_files(dir: &Path, relative: &str, out: &mut Vec<ShelvedFile>) -> Result<()> {
    use sha2::{Digest, Sha256};
    use std::io::Read as _;
    let mut entries: Vec<fs::DirEntry> = fs::read_dir(dir)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = if relative.is_empty() {
            name
        } else {
            format!("{relative}/{name}")
        };
        let kind = entry.file_type()?;
        if kind.is_dir() {
            shelved_files(&entry.path(), &path, out)?;
        } else if kind.is_file() {
            // Through the non-blocking open: the entry may have changed since it was
            // listed (#262, review of #305).
            let mut bytes = vec![];
            crate::records::open_regular(&entry.path())
                .and_then(|mut file| Ok(file.read_to_end(&mut bytes)?))
                .with_context(|| format!("reading {path}"))?;
            let sha256 = format!("{:x}", Sha256::digest(&bytes));
            out.push((path, Ok((bytes.len() as u64, sha256))));
        } else if kind.is_symlink() {
            out.push((path, Err("a symbolic link, not followed")));
        } else {
            out.push((path, Err("not a regular file, not read")));
        }
    }
    Ok(())
}

/// Move session folder `key` unparsed and byte for byte to
/// `.chough/archive/shelved/<id>/`, never replacing a file, with a `shelved.md` inside
/// giving its size, each file's SHA-256, the time and the reason, and clear the active
/// pointer when it named this session (REQ-history-006, DRIVE-14).
fn shelve(root: &Path, key: &str) -> Result<()> {
    ensure_key(key)?;
    let source = format!("{DIR}/{key}");
    let from = local_path(root, &source)?;
    ensure!(
        from.is_dir(),
        "no session folder {} to shelve",
        crate::leaks::shown("session folder", &source)
    );
    let target = format!("{SHELVED}/{key}");
    let note = format!("{target}/{SHELVED_NOTE}");
    ensure!(
        !local_path(root, &target)?.exists(),
        "{target} already exists; shelving never replaces a file"
    );
    ensure!(
        !from.join(SHELVED_NOTE).exists(),
        "{source}/{SHELVED_NOTE} would become {note}, which shelving writes; it never replaces a file, so nothing was moved"
    );
    // Never block on a `session.md` that is not a regular file, such as a FIFO (#262).
    use std::io::Read as _;
    let session_path = from.join("session.md");
    let mut session_bytes = vec![];
    if let Ok(mut file) = crate::records::open_regular(&session_path)
        && file.read_to_end(&mut session_bytes).is_err()
    {
        session_bytes.clear();
    }
    let readable = session_bytes.len() as u64 <= checkpoint::SESSION_LIMIT
        && String::from_utf8(session_bytes.clone())
            .ok()
            .and_then(|text| checkpoint::parse(&session_file(key), &text).ok())
            .is_some();
    let reason = if session_bytes.len() as u64 > checkpoint::SESSION_LIMIT {
        format!(
            "over the session bound of {} bytes",
            checkpoint::SESSION_LIMIT
        )
    } else if readable {
        "set aside on request".to_string()
    } else {
        "not a readable session record".to_string()
    };
    let mut files = vec![];
    shelved_files(&from, "", &mut files)?;
    let total: u64 = files
        .iter()
        .filter_map(|(_, file)| Some(file.as_ref().ok()?.0))
        .sum();
    let mut body = format!(
        "# Shelved session {key}\n\nMoved unparsed and byte for byte from `{source}`: {reason}. No reader scans this folder.\n\n## Files\n\n"
    );
    for (path, file) in &files {
        match file {
            Ok((size, sha256)) => body.push_str(&format!("- `{path}` {size} bytes {sha256}\n")),
            Err(kind) => body.push_str(&format!("- `{path}` {kind}\n")),
        }
    }
    let mut record = crate::markdown::Record::new(&body);
    record.set_scalar("kind", "shelved")?;
    record.set_scalar("id", key)?;
    record.set_scalar("from", &source)?;
    record.set_scalar("shelved", &crate::session_md::iso_time(now()?))?;
    record.set_scalar("reason", &reason)?;
    record.set_scalar("bytes", &total.to_string())?;
    // REQ-records-006: a file name can hold what a committed record must not.
    let local = crate::leaks::Local::current();
    if crate::leaks::redacting() {
        record.redact_leaks(local);
    } else {
        record.check_leaks(local)?;
    }
    let text = record.render();
    ensure!(
        text.len() as u64 <= crate::markdown::RECEIPT_BOUND,
        "refused: the shelving note would be {} bytes, over its bound of {} bytes; nothing was moved",
        text.len(),
        crate::markdown::RECEIPT_BOUND
    );
    fs::create_dir_all(local_path(root, SHELVED)?)?;
    fs::rename(&from, local_path(root, &target)?).with_context(|| format!("shelving {source}"))?;
    ensure!(
        write_new_bounded(root, &note, &text, crate::markdown::RECEIPT_BOUND)?,
        "{note} appeared while shelving; it was not replaced"
    );
    if pointer(root).ok().flatten().as_deref() == Some(key) {
        let _ = fs::remove_file(local_path(root, POINTER)?);
    }
    unpin(root);
    print_json(&json!({
        "schema_version": 1,
        "shelved": target,
        "record": note,
        "from": source,
        "bytes": total,
        "bound": checkpoint::SESSION_LIMIT,
        "reason": reason,
        "next": "chough session new <key>",
    }))
}

fn pending(root: &Path, key: &str) -> Result<Vec<(String, Entry)>> {
    let dir = inbox_dir(key);
    let mut entries = vec![];
    for name in entry_names(root, &dir)? {
        let file = format!("{dir}/{name}.md");
        let content = read_bounded_record(&local_path(root, &file)?, INBOX_LIMIT)?;
        let record = crate::markdown::parse(&file, &content)
            .with_context(|| format!("invalid inbox entry {file}"))?;
        if let Some(key) = record
            .keys()
            .into_iter()
            .find(|key| FORBIDDEN_ENTRY_KEYS.contains(key))
        {
            bail!("invalid inbox entry {file}: an inbox entry carries no `{key}` field");
        }
        let field = |name: &str| record.scalar(name).unwrap_or_default().to_string();
        ensure!(
            field("id") == name && field("session") == key,
            "invalid inbox entry {file}"
        );
        entries.push((
            file,
            Entry {
                id: name,
                session: key.to_string(),
                from: field("from"),
                kind: field("kind"),
                text: field("value"),
                added: field("added"),
            },
        ));
    }
    Ok(entries)
}

/// The `from`, kind and text of inbox file `file` when `chough inbox add` takes them again
/// as they are, read and checked as `inbox fold` reads an entry; None for any other file a
/// person left in an inbox, whose only way out is to drop it (#381, rule 3).
pub(super) fn entry_to_add_again(
    root: &Path,
    file: &str,
) -> Option<(String, &'static str, String)> {
    let content = read_bounded_record(&local_path(root, file).ok()?, INBOX_LIMIT).ok()?;
    let record = crate::markdown::parse(file, &content).ok()?;
    let field = |name: &str| record.scalar(name).unwrap_or_default().to_string();
    let kind = match field("kind").as_str() {
        "observation" => Kind::Observation,
        "decision" => Kind::Decision,
        "completed" => Kind::Completed,
        "question" => Kind::Question,
        _ => return None,
    };
    let (from, text) = (field("from"), field("value"));
    validate_entry(&from, kind, &text).ok()?;
    Some((from, kind.name(), text))
}

/// The Markdown record of a new inbox entry named `name`.
pub(super) fn render_entry(
    name: &str,
    session: &str,
    from: &str,
    kind: Kind,
    text: &str,
    recorded: Option<bool>,
    added: &str,
) -> Result<String> {
    let mut record = crate::markdown::Record::new(&format!("{text}\n"));
    record.set_scalar("kind", kind.name())?;
    record.set_scalar("id", name)?;
    record.set_scalar("session", session)?;
    record.set_scalar("from", from)?;
    record.set_scalar("value", text)?;
    if let Some(recorded) = recorded {
        record.set_bool("recorded", recorded)?;
    }
    record.set_scalar("added", added)?;
    record.render_within(RecordKind::Inbox)
}

/// The checks `inbox add` makes on `from` and the text, also made again by `fold`.
fn validate_entry(from: &str, kind: Kind, text: &str) -> Result<()> {
    ensure!(
        valid_name(from),
        "--from uses letters, digits, hyphens, underscores"
    );
    match kind {
        Kind::Observation => ensure!(
            valid_name(text),
            "an observation entry names an evidence ID"
        ),
        _ => ensure_text("text", text)?,
    }
    Ok(())
}

pub(super) fn inbox(root: &Path, command: InboxCommand) -> Result<()> {
    ensure!(
        enabled(root)?,
        "the inbox needs sessions; run `chough session new <key>`"
    );
    match command {
        InboxCommand::Add {
            session,
            mut from,
            kind,
            mut text,
        } => {
            // A session archives itself under this lock only when it has no unfolded
            // entries, so taking it here means the entry is either seen by that archive
            // (the session stays open) or refused below; never stranded (#98).
            lock_project(root)?;
            let session = resolve_open(root, &session).map_err(|_| {
                anyhow::anyhow!(
                    "no open session {}",
                    crate::leaks::shown("session name", &session)
                )
            })?;
            validate_entry(&from, kind, &text)?;
            // REQ-records-006: an inbox entry is committed with the session; its values
            // pass the leak guard, refused or redacted under --redact.
            let local = crate::leaks::Local::current();
            let redacted = crate::leaks::guard(local, "--from", &mut from)?
                + crate::leaks::guard(local, "--text", &mut text)?;
            ensure!(
                entry_names(root, &inbox_dir(&session))?.len() < MAX_INBOX,
                "the inbox of {session} holds {MAX_INBOX} unfolded entries; fold it first"
            );
            let observation = matches!(kind, Kind::Observation);
            let recorded = observation
                .then(|| crate::evidence::is_present(root, &text))
                .transpose()?;
            let at = now()?;
            let added = crate::session_md::iso_time(at);
            let base =
                crate::identity::dated_name(&crate::identity::utc_date(at), &text, kind.name());
            let dir = inbox_dir(&session);
            // Named by date and slug; a clash draws one suffix and a second is refused
            // (REQ-records-003). Each candidate is published whole by a link.
            let mut id = base.clone();
            let mut file = format!("{dir}/{id}.md");
            if !publish_entry(
                root,
                &file,
                &render_entry(&id, &session, &from, kind, &text, recorded, &added)?,
            )? {
                id = crate::identity::suffixed(&base)?;
                file = format!("{dir}/{id}.md");
                ensure!(
                    publish_entry(
                        root,
                        &file,
                        &render_entry(&id, &session, &from, kind, &text, recorded, &added)?
                    )?,
                    "refused: {file} also exists; nothing was written and no other name was drawn"
                );
            }
            if recorded == Some(false) {
                // Leif's choice for #84: warn, do not refuse; a subagent may report
                // before this tree has the record.
                eprintln!(
                    "chough: warning: observation {text} has no record yet, as an evidence entry or a receipt; the entry was added and `chough inbox list` marks it recorded: false"
                );
            }
            let mut added = json!({"schema_version":1,"added":file,"id":id});
            if redacted > 0 {
                added["redacted"] = json!(redacted);
            }
            print_json(&added)
        }
        InboxCommand::List => {
            // REQ-history-009 (#393): the pinned session, as `inbox fold` resolves it.
            let relative = checkpoint_path(root)?;
            let key = match id_of(&relative) {
                Some(key) => key,
                None => active(root)?,
            };
            let mut entries = vec![];
            for (_, entry) in pending(root, &key)? {
                let recorded = (entry.kind == Kind::Observation.name())
                    .then(|| crate::evidence::is_present(root, &entry.text))
                    .transpose()?;
                let mut value = serde_json::to_value(&entry)?;
                if let Some(recorded) = recorded {
                    value["recorded"] = json!(recorded);
                }
                entries.push(value);
            }
            print_json(&json!({"schema_version":1,"session":key,"entries":entries}))
        }
        InboxCommand::Fold => {
            // Lock before the read, as every read-modify-write of the checkpoint does.
            checkpoint::lock_for_update(root)?;
            // Pin the session before anything is read, so the save goes to the session
            // that was read even if another process runs `chough session use` meanwhile.
            let relative = checkpoint_path(root)?;
            let key = match id_of(&relative) {
                Some(key) => key,
                None => active(root)?,
            };
            let entries = pending(root, &key)?;
            let mut session = read_at(root, &session_file(&key))?;
            let folded = format!("{}/folded", inbox_dir(&key));
            // Every entry and every target is checked before the session is saved, so a
            // refusal changes nothing.
            let mut targets = vec![];
            for (file, entry) in &entries {
                let kind = match entry.kind.as_str() {
                    "observation" => Kind::Observation,
                    "decision" => Kind::Decision,
                    "completed" => Kind::Completed,
                    "question" => Kind::Question,
                    other => bail!("inbox entry {} has unknown kind {other}", entry.id),
                };
                validate_entry(&entry.from, kind, &entry.text)
                    .with_context(|| format!("invalid inbox entry {file}"))?;
                let target = local_path(root, &format!("{folded}/{}.md", entry.id))?;
                ensure!(
                    !target.exists(),
                    "{} already folded at {folded}/{}.md; nothing was folded",
                    entry.id,
                    entry.id
                );
                let list = match kind {
                    Kind::Observation => &mut session.observations,
                    Kind::Decision => &mut session.decisions,
                    Kind::Completed => &mut session.completed,
                    Kind::Question => &mut session.open_questions,
                };
                // REQ-records-016 (#366): the entry is text Chough already stores, so it
                // folds as it is printed, a control character an earlier release wrote
                // escaped, and is never refused as a new value would be.
                let text = crate::leaks::printable(&entry.text, false).into_owned();
                if !list.contains(&text) {
                    list.push(text);
                }
                targets.push((file, target));
            }
            if !entries.is_empty() {
                checkpoint::save_checkpoint(root, &mut session)?;
                fs::create_dir_all(local_path(root, &folded)?)?;
                for (file, target) in targets {
                    fs::rename(local_path(root, file)?, target)?;
                }
            }
            let ids: Vec<_> = entries.iter().map(|(_, entry)| entry.id.clone()).collect();
            print_json(&json!({"schema_version":1,"session":key,"folded":ids}))
        }
    }
}

#[cfg(test)]
mod tests {
    /// t38n's round-4 F11 (REQ-records-002): a fresh project's guide names no folder kept
    /// from Chough 0.5.x, and a migrated project's guide is the same with the
    /// `artifacts/` line before `local/`.
    #[test]
    fn only_a_migrated_guide_names_artifacts() {
        assert!(!super::README.contains("artifacts/"));
        let migrated = super::migrated_readme();
        let line = "- `artifacts/`: kept from Chough 0.5.x only for files an open bug names as its fixture.\n";
        assert_eq!(migrated.replacen(line, "", 1), super::README);
        assert!(
            migrated.contains(&format!("{line}- `local/`:")),
            "{migrated}"
        );
    }
}
