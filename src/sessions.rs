//! Sessions per objective or native change (REQ-cli-037, REQ-history-006).
//!
//! With `.chough/sessions/` present the canonical checkpoint is the active session
//! file. Only that file is written; other sessions are read, and archived sessions
//! keep their bytes. Subagents report through create-only inbox entries.

use crate::checkpoint::{self, Checkpoint, Session};
use crate::records::{
    CHANGE_ID_LIMIT, ensure_text, local_path, now, print_json, read_bounded_record,
    valid_change_id, valid_module_name, valid_name, write_new, write_new_bounded,
    write_replace_bounded,
};
use anyhow::{Context, Result, bail, ensure};
use clap::{Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
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

#[cfg(test)]
thread_local! {
    static BEFORE_FOLD_SAVE: std::cell::RefCell<Option<Box<dyn FnMut()>>> =
        const { std::cell::RefCell::new(None) };
}

/// Runs `hook` just before `inbox fold` saves, standing in for another process's
/// `chough session use` partway through the fold. Test builds only.
#[cfg(test)]
fn with_before_fold_save<T>(hook: impl FnMut() + 'static, run: impl FnOnce() -> T) -> T {
    BEFORE_FOLD_SAVE.with(|slot| *slot.borrow_mut() = Some(Box::new(hook)));
    let result = run();
    BEFORE_FOLD_SAVE.with(|slot| *slot.borrow_mut() = None);
    result
}

fn before_fold_save() {
    #[cfg(test)]
    BEFORE_FOLD_SAVE.with(|slot| {
        if let Some(hook) = slot.borrow_mut().as_mut() {
            hook();
        }
    });
}

/// Forgets this process's resolved session for `root`, as a fresh process would start.
fn unpin(root: &Path) {
    if let Some(map) = PINNED
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .as_mut()
    {
        map.remove(root);
    }
}

fn pin(root: &Path, relative: &str) {
    PINNED
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .get_or_insert_with(HashMap::new)
        .insert(root.to_path_buf(), relative.to_string());
}

pub(super) const DIR: &str = ".chough/sessions";
const ARCHIVE: &str = ".chough/sessions/archive";
/// Checkpoints set aside by `chough session shelve`, which only retention reads.
const SHELVED: &str = ".chough/sessions/shelved";
const LOCAL: &str = ".chough/local";
const POINTER: &str = ".chough/local/active-session";
const INBOX_LIMIT: u64 = 16_384;
const MAX_INBOX: usize = 200;

#[derive(Subcommand)]
pub(super) enum SessionCommand {
    /// Create a session for an objective or native change and make it active.
    New {
        key: String,
        #[arg(
            long,
            default_value = "Describe the current objective before beginning work."
        )]
        objective: String,
        #[arg(long, default_value = "")]
        intent_scope: String,
        #[arg(long = "module")]
        modules: Vec<String>,
    },
    /// Move `.chough/session.json` (with a history cursor) to a session, byte for byte.
    Adopt { key: String },
    /// Make an open session active; done sessions are archived.
    Use { key: String },
    /// List open and archived sessions.
    List,
    /// Move an oversized or unreadable checkpoint, unparsed, into `.chough/sessions/shelved/`.
    Shelve {
        /// The session to shelve.
        #[arg(required_unless_present = "legacy", conflicts_with = "legacy")]
        key: Option<String>,
        /// Shelve the legacy `.chough/session.json` as `legacy`.
        #[arg(long)]
        legacy: bool,
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

/// An inbox entry. It has no approval, review, consent, gate or stage field.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    schema_version: u32,
    id: String,
    session: String,
    from: String,
    kind: String,
    text: String,
    recorded_at_unix_seconds: u64,
}

fn session_file(key: &str) -> String {
    format!("{DIR}/{key}.json")
}

fn archive_file(key: &str) -> String {
    format!("{ARCHIVE}/{key}.json")
}

fn inbox_dir(key: &str) -> String {
    format!("{DIR}/{key}.inbox")
}

fn ensure_key(key: &str) -> Result<()> {
    ensure!(
        valid_change_id(key),
        "invalid session key: up to {CHANGE_ID_LIMIT} bytes of letters, digits, - and _"
    );
    Ok(())
}

pub(super) fn enabled(root: &Path) -> Result<bool> {
    Ok(local_path(root, DIR)?.is_dir())
}

fn keys_in(root: &Path, dir: &str) -> Result<Vec<String>> {
    let path = local_path(root, dir)?;
    let mut keys = vec![];
    let entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(keys),
        Err(error) => return Err(error).with_context(|| format!("reading {dir}")),
    };
    for entry in entries {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        if let Some(key) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.strip_suffix(".json"))
            && valid_change_id(key)
        {
            keys.push(key.to_string());
        }
    }
    keys.sort();
    Ok(keys)
}

fn pointer(root: &Path) -> Result<Option<String>> {
    let path = local_path(root, POINTER)?;
    if !path.exists() {
        return Ok(None);
    }
    let key = read_bounded_record(&path, 512)?.trim().to_string();
    ensure_key(&key)?;
    Ok(Some(key))
}

/// The active session's key, or a refusal naming the next command.
fn active(root: &Path) -> Result<String> {
    if let Some(key) = pointer(root)? {
        if local_path(root, &session_file(&key))?.is_file() {
            return Ok(key);
        }
        if local_path(root, &archive_file(&key))?.is_file() {
            bail!(
                "the active session {key} is archived; run `chough session use <key>` or `chough session new <key>`"
            );
        }
        bail!(
            "the active session {key} does not exist; run `chough session use <key>` or `chough session new <key>`"
        );
    }
    let open = keys_in(root, DIR)?;
    match open.as_slice() {
        [key] => Ok(key.clone()),
        [] => bail!("no open session; run `chough session new <key>`"),
        _ => bail!(
            "several open sessions ({}) and no active pointer; run `chough session use <key>`",
            open.join(", ")
        ),
    }
}

/// The project-relative path of the canonical checkpoint (REQ-cli-037).
/// Resolved once per command (see [`PINNED`]); a session this command did not choose
/// that is archived or removed meanwhile is refused rather than written again.
pub(super) fn checkpoint_path(root: &Path) -> Result<String> {
    if let Some(relative) = pinned(root) {
        // The checkpoint this command resolved vanished while it ran: archived, removed,
        // or (for a legacy writer) adopted into sessions mode, so `.chough/session.json`
        // would otherwise be written again. Nothing is saved, and the history events this
        // command published are removed so no orphan is left (#82, REQ-history-008).
        let vanished = if relative == checkpoint::SESSION {
            enabled(root)?.then(|| {
                format!(
                    "{} was adopted into {DIR} while this command ran",
                    checkpoint::SESSION
                )
            })
        } else {
            (!local_path(root, &relative)?.is_file())
                .then(|| format!("{relative} was archived or removed while this command ran"))
        };
        if let Some(why) = vanished {
            // Only events published since the checkpoint was last found in place go.
            let events = match crate::history::discard_published(root) {
                0 => String::new(),
                1 => "; the history event it had written was removed".into(),
                count => format!("; the {count} history events it had written were removed"),
            };
            bail!("{why}; nothing was saved{events}");
        }
        // Found in place: a save resolves here just before it writes, so from now on the
        // events published so far may be saved and are never removed (second #96 review).
        crate::history::retire_published(root);
        return Ok(relative);
    }
    let relative = if enabled(root)? {
        session_file(&active(root)?)
    } else {
        checkpoint::SESSION.to_string()
    };
    pin(root, &relative);
    Ok(relative)
}

/// Every session checkpoint with how retention names it: the active checkpoint as
/// `checkpoint`, other open sessions as `session <key>` and archived ones as
/// `archived session <key>`, then, for each session with unfolded observation entries,
/// those observations as `inbox of session <key>` (#98). Any session or entry that
/// cannot be read is an error.
pub(super) fn every_checkpoint(root: &Path) -> Result<Vec<(String, Checkpoint)>> {
    checkpoints(root, false)
}

/// Every checkpoint that can be read, for display only: a session that cannot be read
/// is left out instead of failing the view (REQ-history-006, DRIVE-14); `session list`
/// reports it. Retention keeps using the strict [`every_checkpoint`].
pub(super) fn readable_checkpoints(root: &Path) -> Result<Vec<(String, Checkpoint)>> {
    checkpoints(root, true)
}

fn checkpoints(root: &Path, lenient: bool) -> Result<Vec<(String, Checkpoint)>> {
    let read = |file: &str| -> Result<Option<Checkpoint>> {
        match read_at(root, file) {
            Ok(session) => Ok(Some(session)),
            Err(_) if lenient => Ok(None),
            Err(error) => Err(error),
        }
    };
    if !enabled(root)? {
        return Ok(match checkpoint::read_session(root)? {
            Session::V2(session) => vec![("checkpoint".into(), *session)],
            Session::V1(_) => vec![],
        });
    }
    let current = checkpoint_path(root).ok();
    let mut all = vec![];
    for key in keys_in(root, DIR)? {
        let file = session_file(&key);
        let label = if current.as_deref() == Some(file.as_str()) {
            "checkpoint".to_string()
        } else {
            format!("session {key}")
        };
        if let Some(session) = read(&file)? {
            all.push((label, session));
        }
    }
    for key in keys_in(root, ARCHIVE)? {
        if let Some(session) = read(&archive_file(&key))? {
            all.push((format!("archived session {key}"), session));
        }
    }
    // A shelved checkpoint no longer drives work, but what it references stays
    // protected: retention reads it within the 16 MiB ledger bound (DRIVE-14).
    for key in keys_in(root, SHELVED)? {
        if key.ends_with(".shelved") {
            continue;
        }
        let file = format!("{SHELVED}/{key}.json");
        let shelved = (|| -> Result<Checkpoint> {
            let bytes = read_bounded_raw(&local_path(root, &file)?)?;
            let text = String::from_utf8(bytes).with_context(|| format!("reading {file}"))?;
            match checkpoint::parse(&text).with_context(|| format!("reading {file}"))? {
                Session::V2(session) => Ok(*session),
                Session::V1(_) => bail!("{file} uses schema 1"),
            }
        })();
        match shelved {
            Ok(session) => all.push((format!("shelved session {key}"), session)),
            Err(_) if lenient => {}
            Err(error) => return Err(error),
        }
    }
    // Observations an unfolded inbox entry reports are what the session will hold once
    // it folds, so retention protects them now (#98, REQ-history-009).
    for key in keys_in(root, DIR)?
        .into_iter()
        .chain(keys_in(root, ARCHIVE)?)
    {
        let reported: Vec<String> = pending(root, &key)?
            .into_iter()
            .filter(|(_, entry)| entry.kind == Kind::Observation.name())
            .map(|(_, entry)| entry.text)
            .collect();
        if !reported.is_empty() {
            let mut inbox = checkpoint::new_checkpoint(
                format!("Unfolded inbox of session {key}"),
                String::new(),
                vec![],
            );
            inbox.observations = reported;
            all.push((format!("inbox of session {key}"), inbox));
        }
    }
    Ok(all)
}

/// Take the checkpoint writer lock for a session command when the project's `.chough`
/// directory exists; before it exists there is nothing a concurrent writer could lose.
fn lock_project(root: &Path) -> Result<()> {
    if local_path(root, ".chough")?.is_dir() {
        checkpoint::lock_checkpoint(root)?;
    }
    Ok(())
}

/// Git ignore rules for `.chough/sessions/` and the inbox directories in it: the temps
/// a session-file write uses before its rename and an inbox add before its link.
const IGNORE: &str = ".*.tmp\n";

/// Whether `name` is `.<stem>.json.<pid>.tmp` for a valid session key or entry ID: the
/// temp a session-file write or an inbox add uses before it publishes.
fn is_write_temp(name: &str) -> bool {
    name.strip_prefix('.')
        .and_then(|rest| rest.strip_suffix(".tmp"))
        .and_then(|rest| rest.rsplit_once(".json."))
        .is_some_and(|(stem, pid)| {
            valid_change_id(stem)
                && !pid.is_empty()
                && pid.bytes().all(|byte| byte.is_ascii_digit())
        })
}

/// Remove `.<key>.json.<pid>.tmp` files left directly in `.chough/sessions/` by session
/// writers killed before their rename (#77 review, gap ii), and `.<id>.json.<pid>.tmp`
/// files left in an inbox directory by an `inbox add` killed before its link (#99
/// review), and make sure Git ignores that pattern. Called by `save_checkpoint` while it
/// holds the checkpoint lock, under which every session-file writer renames and every
/// inbox add links, so a temp seen here was left by a killed writer. Only regular files
/// with exactly that name go.
pub(super) fn remove_orphaned_temps(root: &Path) -> Result<()> {
    if !enabled(root)? {
        return Ok(());
    }
    write_new(root, &format!("{DIR}/.gitignore"), IGNORE)?;
    let mut dirs = vec![(
        DIR.to_string(),
        "a session write that was stopped before it saved; the session was not changed by it",
    )];
    for entry in fs::read_dir(local_path(root, DIR)?)? {
        let entry = entry?;
        if entry.file_type()?.is_dir()
            && let Some(key) = entry
                .file_name()
                .to_str()
                .and_then(|name| name.strip_suffix(".inbox"))
            && valid_change_id(key)
        {
            dirs.push((
                inbox_dir(key),
                "an inbox add that was stopped before it published its entry; no entry was added by it",
            ));
        }
    }
    for (dir, left_by) in dirs {
        for entry in fs::read_dir(local_path(root, &dir)?)? {
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

/// Publish an inbox entry whole (#99 review): write it to `.<id>.json.<pid>.tmp` in the
/// inbox directory, which Git ignores and no reader lists, then hard-link it to its
/// name, which never replaces an existing file. An add stopped partway leaves at most
/// that temp, never a partial `INB-*.json`, and the next checkpoint save removes it.
/// Returns false when an entry of that name already exists.
fn publish_entry(root: &Path, file: &str, content: &str) -> Result<bool> {
    use std::io::Write as _;
    ensure!(
        content.len() as u64 <= INBOX_LIMIT,
        "managed file exceeds {INBOX_LIMIT} bytes: {file}"
    );
    let path = local_path(root, file)?;
    let parent = path.parent().context("inbox entry has no parent")?;
    fs::create_dir_all(parent)?;
    local_path(root, file)?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("inbox entry name must be UTF-8")?;
    let temporary = parent.join(format!(".{name}.{}.tmp", std::process::id()));
    let published = (|| -> std::io::Result<bool> {
        let mut out = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        out.write_all(content.as_bytes())?;
        out.sync_all()?;
        match fs::hard_link(&temporary, &path) {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
            Err(error) => Err(error),
        }
    })();
    let _ = fs::remove_file(&temporary);
    published.with_context(|| format!("writing {file}"))
}

fn read_at(root: &Path, relative: &str) -> Result<Checkpoint> {
    let content = read_bounded_record(&local_path(root, relative)?, checkpoint::SESSION_LIMIT)
        .with_context(|| format!("reading {relative}"))
        .with_context(|| shelve_hint(relative))?;
    match checkpoint::parse(&content)
        .with_context(|| format!("reading {relative}"))
        .with_context(|| shelve_hint(relative))?
    {
        Session::V2(session) => Ok(*session),
        Session::V1(_) => bail!("{relative} uses schema 1"),
    }
}

/// The recovery for a checkpoint that cannot be read (REQ-cli-032, DRIVE-14).
pub(super) fn shelve_hint(relative: &str) -> String {
    let command = if relative == checkpoint::SESSION {
        "chough session shelve --legacy".to_string()
    } else {
        let key = relative
            .rsplit('/')
            .next()
            .and_then(|name| name.strip_suffix(".json"))
            .unwrap_or("<key>");
        format!("chough session shelve {key}")
    };
    format!(
        "{relative} cannot be read (the checkpoint cap is {} bytes); set it aside with `{command}`, then `chough session new <key>`",
        checkpoint::SESSION_LIMIT
    )
}

fn point(root: &Path, key: &str) -> Result<()> {
    write_new(root, &format!("{LOCAL}/.gitignore"), "*\n")?;
    write_new(root, &format!("{DIR}/.gitignore"), IGNORE)?;
    write_replace_bounded(root, POINTER, &format!("{key}\n"), 512)?;
    pin(root, &session_file(key));
    Ok(())
}

/// The number of unfolded entries in a session's inbox.
fn unfolded(root: &Path, key: &str) -> Result<usize> {
    Ok(keys_in(root, &inbox_dir(key))?.len())
}

/// Move every other open session at stage done to the archive, bytes unchanged. A done
/// session with unfolded inbox entries stays open, so its entries are not stranded;
/// it is returned in the second list with its unfolded count.
fn archive_done(root: &Path, keep: &str) -> Result<(Vec<String>, Vec<serde_json::Value>)> {
    let mut archived = vec![];
    let mut kept_open = vec![];
    for key in keys_in(root, DIR)? {
        // An unreadable session is left where it is; `session list` reports it.
        if key == keep
            || !read_at(root, &session_file(&key))
                .is_ok_and(|session| session.stage.current == "done")
        {
            continue;
        }
        let count = unfolded(root, &key)?;
        if count > 0 {
            kept_open.push(json!({"key": key, "inbox_unfolded": count}));
            continue;
        }
        let target = local_path(root, &archive_file(&key))?;
        ensure!(!target.exists(), "{} already exists", archive_file(&key));
        fs::create_dir_all(local_path(root, ARCHIVE)?)?;
        fs::rename(local_path(root, &session_file(&key))?, &target)?;
        archived.push(key);
    }
    Ok((archived, kept_open))
}

fn ensure_unused(root: &Path, key: &str) -> Result<()> {
    ensure!(
        !local_path(root, &session_file(key))?.exists()
            && !local_path(root, &archive_file(key))?.exists(),
        "session {key} already exists"
    );
    Ok(())
}

fn ensure_no_legacy(root: &Path) -> Result<()> {
    ensure!(
        !local_path(root, checkpoint::SESSION)?.exists(),
        "{} exists; run `chough session adopt <key>` first",
        checkpoint::SESSION
    );
    Ok(())
}

pub(super) fn run(root: &Path, command: SessionCommand) -> Result<()> {
    match command {
        SessionCommand::New {
            key,
            objective,
            intent_scope,
            modules,
        } => {
            ensure_key(&key)?;
            ensure!(!objective.trim().is_empty(), "objective must not be blank");
            lock_project(root)?;
            ensure!(
                modules.iter().all(|module| valid_module_name(module)),
                "invalid module name: use a module ID SpecSync accepts, such as cli or chat.service"
            );
            ensure_no_legacy(root)?;
            ensure_unused(root, &key)?;
            let mut session = checkpoint::new_checkpoint(objective, intent_scope, modules);
            session.history_cursor = Some(crate::history::new_cursor(root)?);
            checkpoint::validate_v2(&session)?;
            let content = serde_json::to_string_pretty(&session)? + "\n";
            ensure!(
                write_new_bounded(
                    root,
                    &session_file(&key),
                    &content,
                    checkpoint::SESSION_LIMIT
                )?,
                "session {key} already exists"
            );
            let (archived, kept_open) = archive_done(root, &key)?;
            point(root, &key)?;
            print_json(
                &json!({"schema_version":1,"created":session_file(&key),"active":key,"archived":archived,"kept_open":kept_open}),
            )
        }
        SessionCommand::Adopt { key } => {
            ensure_key(&key)?;
            lock_project(root)?;
            ensure_unused(root, &key)?;
            let legacy = local_path(root, checkpoint::SESSION)?;
            let bytes = read_bounded_record(&legacy, checkpoint::SESSION_LIMIT)
                .context("no checkpoint to adopt; run `chough session new <key>`")?;
            match checkpoint::parse(&bytes)? {
                Session::V2(session) if session.history_cursor.is_some() => {}
                Session::V2(_) => bail!(
                    "the checkpoint has no history cursor; run `chough history adopt --apply` first"
                ),
                Session::V1(_) => bail!("the checkpoint uses schema 1; run `chough migrate` first"),
            }
            ensure!(
                write_new_bounded(root, &session_file(&key), &bytes, checkpoint::SESSION_LIMIT)?,
                "session {key} already exists"
            );
            fs::remove_file(&legacy)?;
            point(root, &key)?;
            print_json(
                &json!({"schema_version":1,"adopted":checkpoint::SESSION,"created":session_file(&key),"active":key}),
            )
        }
        SessionCommand::Use { key } => {
            ensure_key(&key)?;
            lock_project(root)?;
            ensure!(
                enabled(root)?,
                "no sessions; run `chough session new <key>`"
            );
            if !local_path(root, &session_file(&key))?.is_file() {
                if local_path(root, &archive_file(&key))?.is_file() {
                    bail!("session {key} is archived");
                }
                bail!("no session {key}");
            }
            read_at(root, &session_file(&key))?;
            let (archived, kept_open) = archive_done(root, &key)?;
            point(root, &key)?;
            print_json(
                &json!({"schema_version":1,"active":key,"archived":archived,"kept_open":kept_open}),
            )
        }
        SessionCommand::List => {
            ensure!(
                enabled(root)?,
                "no sessions; run `chough session new <key>`"
            );
            let current = active(root).ok();
            let mut sessions = vec![];
            for (dir, archived) in [(DIR, false), (ARCHIVE, true)] {
                for key in keys_in(root, dir)? {
                    let file = format!("{dir}/{key}.json");
                    let session = match read_at(root, &file) {
                        Ok(session) => session,
                        Err(error) => {
                            sessions.push(json!({
                                "key": key,
                                "file": file,
                                "readable": false,
                                "reason": format!("{error:#}"),
                                "recovery": format!("chough session shelve {key}"),
                                "archived": archived,
                            }));
                            continue;
                        }
                    };
                    sessions.push(json!({
                        "key": key,
                        "file": file,
                        "objective": session.objective,
                        "native_change": session.native_change,
                        "stage": session.stage.current,
                        "status": if archived { "archived" } else { session.stage.status.as_str() },
                        "active": !archived && current.as_deref() == Some(key.as_str()),
                        "archived": archived,
                        "inbox_unfolded": unfolded(root, &key)?,
                    }));
                }
            }
            print_json(&json!({"schema_version":1,"active":current,"sessions":sessions}))
        }
        SessionCommand::Shelve { key, legacy } => {
            lock_project(root)?;
            let (source, key) = if legacy {
                (checkpoint::SESSION.to_string(), "legacy".to_string())
            } else {
                let key = key.context("name the session to shelve, or pass --legacy")?;
                ensure_key(&key)?;
                (session_file(&key), key)
            };
            shelve(root, &source, &key)
        }
    }
}

/// Archived sessions kept in `.chough/sessions/archive/` (KNOW-29).
const KEEP_ARCHIVED: usize = 5;

/// REQ-history-011 (KNOW-29): at done, keep the five most recent archived sessions, by
/// their native change's finalization and then key, and move older ones byte for byte
/// into `<library-dir>/attachments/<folder>/sessions/<key>.json`. Never fails: the result
/// is `rolled`, `unchanged`, `skipped` or `refused`.
pub(super) fn roll_up_at_done(root: &Path) -> serde_json::Value {
    match crate::prefs::effective(root, "librarian") {
        Ok((value, _)) if value != "on" => {
            return json!({"status": "skipped", "reason": "the librarian is off"});
        }
        Err(error) => return json!({"status": "refused", "reason": format!("{error:#}")}),
        Ok(_) => {}
    }
    roll_up(root)
        .unwrap_or_else(|error| json!({"status": "refused", "reason": format!("{error:#}")}))
}

fn roll_up(root: &Path) -> Result<serde_json::Value> {
    let (dir, _) = crate::prefs::effective(root, "library-dir")?;
    let mut archived = vec![];
    for key in keys_in(root, ARCHIVE)? {
        // An archived session that cannot be read stays; `session list` reports it.
        let session = read_at(root, &archive_file(&key)).ok();
        // The session that adopted the legacy history never rolls and does not count
        // toward the five, so the anchor never depends on the vault (KNOW-29, #143).
        if session
            .as_ref()
            .and_then(|session| session.history_cursor.as_ref())
            .is_some_and(|cursor| cursor.legacy.is_some())
        {
            continue;
        }
        let finalized = session
            .and_then(|session| session.native_change)
            .and_then(|change| crate::library::finalized_at(root, &change));
        archived.push((finalized, key));
    }
    // Newest first: a later finalization, then a later key; none counts as oldest.
    archived.sort_by(|left, right| right.cmp(left));
    let mut moved = vec![];
    let mut kept_back = vec![];
    for (_, key) in archived.into_iter().skip(KEEP_ARCHIVED) {
        let source = local_path(root, &archive_file(&key))?;
        let target_relative = format!(
            "{dir}/attachments/{}/sessions/{key}.json",
            crate::attachments::attachment_folder(&key)
        );
        let target = local_path(root, &target_relative)?;
        if target.exists() {
            if fs::read(&target)? == fs::read(&source)? {
                fs::remove_file(&source)?;
                moved.push(key);
            } else {
                kept_back.push(json!({"key": key, "reason": format!("{target_relative} already holds different bytes; a move never overwrites")}));
            }
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::hard_link(&source, &target).with_context(|| format!("moving {key} into the vault"))?;
        fs::remove_file(&source).with_context(|| format!("moving {key} into the vault"))?;
        moved.push(key);
    }
    moved.sort();
    Ok(if moved.is_empty() && kept_back.is_empty() {
        json!({"status": "unchanged"})
    } else {
        json!({"status": "rolled", "moved": moved, "kept": kept_back})
    })
}

/// Move `source` unparsed and byte for byte to `.chough/sessions/shelved/<key>.json`,
/// never replacing a file, record why beside it, and clear the active pointer when it
/// named this session (REQ-history-006, DRIVE-14).
fn shelve(root: &Path, source: &str, key: &str) -> Result<()> {
    use sha2::{Digest, Sha256};
    let from = local_path(root, source)?;
    ensure!(from.is_file(), "no {source} to shelve");
    let bytes = read_bounded_raw(&from)?;
    let target = format!("{SHELVED}/{key}.json");
    let record = format!("{SHELVED}/{key}.shelved.json");
    for file in [&target, &record] {
        ensure!(
            !local_path(root, file)?.exists(),
            "{file} already exists; shelving never replaces a file"
        );
    }
    fs::create_dir_all(local_path(root, SHELVED)?)?;
    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    let readable = String::from_utf8(bytes.clone())
        .ok()
        .and_then(|text| checkpoint::parse(&text).ok())
        .is_some()
        && bytes.len() as u64 <= checkpoint::SESSION_LIMIT;
    let reason = if bytes.len() as u64 > checkpoint::SESSION_LIMIT {
        format!(
            "over the checkpoint cap of {} bytes",
            checkpoint::SESSION_LIMIT
        )
    } else if readable {
        "set aside on request".to_string()
    } else {
        "not a readable checkpoint".to_string()
    };
    let note = json!({
        "schema_version": 1,
        "key": key,
        "from": source,
        "bytes": bytes.len(),
        "sha256": sha256,
        "cap": checkpoint::SESSION_LIMIT,
        "reason": reason,
        "shelved_at_unix_seconds": now()?,
    });
    ensure!(
        write_new(
            root,
            &record,
            &(serde_json::to_string_pretty(&note)? + "\n")
        )?,
        "{record} already exists; shelving never replaces a file"
    );
    let to = local_path(root, &target)?;
    // A hard link refuses an existing target; then the source name goes.
    fs::hard_link(&from, &to).with_context(|| format!("shelving {source}"))?;
    fs::remove_file(&from).with_context(|| format!("shelving {source}"))?;
    if pointer(root).ok().flatten().as_deref() == Some(key) {
        let _ = fs::remove_file(local_path(root, POINTER)?);
    }
    unpin(root);
    print_json(&json!({
        "schema_version": 1,
        "shelved": target,
        "record": record,
        "from": source,
        "bytes": bytes.len(),
        "cap": checkpoint::SESSION_LIMIT,
        "reason": reason,
        "next": "chough session new <key>",
    }))
}

/// A checkpoint's raw bytes, within the 16 MiB ledger bound.
fn read_bounded_raw(path: &Path) -> Result<Vec<u8>> {
    let limit = 16 * 1_048_576;
    ensure!(
        fs::metadata(path)?.len() <= limit,
        "{} exceeds {limit} bytes",
        path.display()
    );
    Ok(fs::read(path)?)
}

fn random_id() -> Result<String> {
    let mut bytes = [0u8; 12];
    getrandom::fill(&mut bytes).map_err(|error| anyhow::anyhow!("inbox random source: {error}"))?;
    Ok(format!(
        "INB-{}",
        bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
    ))
}

fn pending(root: &Path, key: &str) -> Result<Vec<(String, Entry)>> {
    let dir = inbox_dir(key);
    let mut entries = vec![];
    for id in keys_in(root, &dir)? {
        let file = format!("{dir}/{id}.json");
        let content = read_bounded_record(&local_path(root, &file)?, INBOX_LIMIT)?;
        let entry: Entry = serde_json::from_str(&content)
            .with_context(|| format!("invalid inbox entry {file}"))?;
        ensure!(
            entry.schema_version == 1 && entry.id == id && entry.session == key,
            "invalid inbox entry {file}"
        );
        entries.push((file, entry));
    }
    Ok(entries)
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
        "the inbox needs sessions; run `chough session new <key>` or `chough session adopt <key>`"
    );
    match command {
        InboxCommand::Add {
            session,
            from,
            kind,
            text,
        } => {
            ensure_key(&session)?;
            // A switch archives a done session under this lock only when it has no
            // unfolded entries, so taking it here means the entry is either seen by that
            // switch (the session stays open) or refused below; never stranded (#98).
            lock_project(root)?;
            ensure!(
                local_path(root, &session_file(&session))?.is_file(),
                "no open session {session}"
            );
            validate_entry(&from, kind, &text)?;
            ensure!(
                keys_in(root, &inbox_dir(&session))?.len() < MAX_INBOX,
                "the inbox of {session} holds {MAX_INBOX} unfolded entries; fold it first"
            );
            let id = random_id()?;
            let entry = Entry {
                schema_version: 1,
                id: id.clone(),
                session: session.clone(),
                from,
                kind: kind.name().into(),
                text,
                recorded_at_unix_seconds: now()?,
            };
            let file = format!("{}/{id}.json", inbox_dir(&session));
            let content = serde_json::to_string_pretty(&entry)? + "\n";
            ensure!(
                publish_entry(root, &file, &content)?,
                "inbox collision at {file}; no retry or overwrite"
            );
            if matches!(kind, Kind::Observation) && !crate::evidence::is_present(root, &entry.text)?
            {
                // Leif's choice for #84: warn, do not refuse; a subagent may report
                // before this tree has the record.
                eprintln!(
                    "chough: warning: observation {} has no record in .chough/evidence yet; the entry was added and `chough inbox list` marks it recorded: false",
                    entry.text
                );
            }
            print_json(&json!({"schema_version":1,"added":file,"id":id}))
        }
        InboxCommand::List => {
            let key = active(root)?;
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
            let key = match relative
                .strip_prefix(&format!("{DIR}/"))
                .and_then(|file| file.strip_suffix(".json"))
            {
                Some(key) => key.to_string(),
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
                let target = local_path(root, &format!("{folded}/{}.json", entry.id))?;
                ensure!(
                    !target.exists(),
                    "{} already folded at {folded}/{}.json; nothing was folded",
                    entry.id,
                    entry.id
                );
                let list = match kind {
                    Kind::Observation => &mut session.observations,
                    Kind::Decision => &mut session.decisions,
                    Kind::Completed => &mut session.completed,
                    Kind::Question => &mut session.open_questions,
                };
                if !list.contains(&entry.text) {
                    list.push(entry.text.clone());
                }
                targets.push((file, target));
            }
            if !entries.is_empty() {
                before_fold_save();
                checkpoint::save_checkpoint(root, &session)?;
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
    use super::*;

    /// #96 review (REQ-history-008): a session file that moves away after a successful
    /// save keeps the event that save points to; the cleanup removes only events no save
    /// may have written.
    #[test]
    fn a_vanish_after_a_successful_save_keeps_the_saved_event() {
        saved_event_survives_a_move_to(&archive_file("alpha"));
    }

    /// Second #96 review (REQ-history-008): the same holds when the saved file is moved
    /// somewhere Chough never looks for a checkpoint, so keeping the event cannot depend
    /// on finding the saved checkpoint again.
    #[test]
    fn a_move_outside_the_session_stores_after_a_save_keeps_the_saved_event() {
        saved_event_survives_a_move_to(".chough/saved-alpha.json");
    }

    /// Save one event on session alpha, move alpha's file to `destination`, then resolve
    /// the checkpoint again: the refusal must keep the saved event.
    fn saved_event_survives_a_move_to(destination: &str) {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        run(
            &root,
            SessionCommand::New {
                key: "alpha".into(),
                objective: "Ship alpha".into(),
                intent_scope: String::new(),
                modules: vec![],
            },
        )
        .unwrap();
        unpin(&root);
        let mut session = checkpoint::read_checkpoint(&root).unwrap();
        let transition: crate::stage::Transition = serde_json::from_value(json!({
            "schema_version": 1, "at_unix_seconds": 1, "kind": "iterate",
            "from": "intent", "to": "intent"
        }))
        .unwrap();
        crate::history::append(&root, &mut session, transition).unwrap();
        checkpoint::save_checkpoint(&root, &session).unwrap();
        let cursor = session.history_cursor.clone().unwrap();
        let cycle = root.join(format!(".chough/history/cycles/{}", cursor.cycle));
        // The cleanup may remove the cycle directory too; count it as empty then.
        let count = || fs::read_dir(&cycle).map_or(0, |entries| entries.count());
        let saved = count();
        assert!(saved >= 1);
        let moved = root.join(destination);
        fs::create_dir_all(moved.parent().unwrap()).unwrap();
        fs::rename(root.join(session_file("alpha")), &moved).unwrap();
        let error = checkpoint_path(&root).unwrap_err().to_string();
        assert!(error.contains("nothing was saved"), "{error}");
        assert_eq!(
            count(),
            saved,
            "the saved event was removed after a move to {destination}"
        );
        unpin(&root);
    }

    /// Leif's review thread on #77: a `session use` in another process partway through
    /// `inbox fold` must not save the folded session or its inbox decisions into the
    /// newly active session's file (REQ-cli-037).
    #[test]
    fn a_switch_partway_through_inbox_fold_does_not_redirect_its_save() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        for (key, objective) in [("alpha", "Ship alpha"), ("beta", "Ship beta")] {
            run(
                &root,
                SessionCommand::New {
                    key: key.into(),
                    objective: objective.into(),
                    intent_scope: String::new(),
                    modules: vec![],
                },
            )
            .unwrap();
        }
        run(
            &root,
            SessionCommand::Use {
                key: "alpha".into(),
            },
        )
        .unwrap();
        inbox(
            &root,
            InboxCommand::Add {
                session: "alpha".into(),
                from: "worker".into(),
                kind: Kind::Decision,
                text: "Keep the alpha decision in alpha".into(),
            },
        )
        .unwrap();
        let beta_before = fs::read(root.join(".chough/sessions/beta.json")).unwrap();
        unpin(&root);
        let pointer = root.join(POINTER);
        with_before_fold_save(
            move || fs::write(&pointer, "beta\n").unwrap(),
            || inbox(&root, InboxCommand::Fold),
        )
        .unwrap();
        assert_eq!(
            fs::read(root.join(".chough/sessions/beta.json")).unwrap(),
            beta_before,
            "beta.json must keep its bytes"
        );
        let alpha: serde_json::Value =
            serde_json::from_slice(&fs::read(root.join(".chough/sessions/alpha.json")).unwrap())
                .unwrap();
        assert_eq!(alpha["objective"], "Ship alpha");
        assert!(
            alpha["decisions"]
                .as_array()
                .unwrap()
                .iter()
                .any(|decision| decision == "Keep the alpha decision in alpha"),
            "{alpha}"
        );
        unpin(&root);
    }
}
