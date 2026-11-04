//! Sessions per objective or native change (REQ-cli-037, REQ-history-006).
//!
//! With `.chough/sessions/` present the canonical checkpoint is the active session
//! file. Only that file is written; other sessions are read, and archived sessions
//! keep their bytes. Subagents report through create-only inbox entries.

use crate::checkpoint::{self, Checkpoint, Session};
use crate::records::{
    CHANGE_ID_LIMIT, ensure_text, local_path, now, print_json, read_bounded_record,
    valid_change_id, valid_name, write_new, write_new_bounded, write_replace_bounded,
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
#[cfg(test)]
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
        if relative != checkpoint::SESSION {
            ensure!(
                local_path(root, &relative)?.is_file(),
                "{relative} was archived or removed while this command ran; nothing was saved"
            );
        }
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
/// `archived session <key>`. Any session that cannot be read is an error.
pub(super) fn every_checkpoint(root: &Path) -> Result<Vec<(String, Checkpoint)>> {
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
        all.push((label, read_at(root, &file)?));
    }
    for key in keys_in(root, ARCHIVE)? {
        all.push((
            format!("archived session {key}"),
            read_at(root, &archive_file(&key))?,
        ));
    }
    Ok(all)
}

fn read_at(root: &Path, relative: &str) -> Result<Checkpoint> {
    let content = read_bounded_record(&local_path(root, relative)?, checkpoint::SESSION_LIMIT)
        .with_context(|| format!("reading {relative}"))?;
    match checkpoint::parse(&content).with_context(|| format!("reading {relative}"))? {
        Session::V2(session) => Ok(*session),
        Session::V1(_) => bail!("{relative} uses schema 1"),
    }
}

fn point(root: &Path, key: &str) -> Result<()> {
    write_new(root, &format!("{LOCAL}/.gitignore"), "*\n")?;
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
        if key == keep || read_at(root, &session_file(&key))?.stage.current != "done" {
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
            ensure!(
                modules.iter().all(|module| valid_name(module)),
                "module names use letters, digits, hyphens, underscores"
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
                    let session = read_at(root, &file)?;
                    sessions.push(json!({
                        "key": key,
                        "file": file,
                        "objective": session.objective,
                        "native_change": session.native_change,
                        "stage": session.stage.current,
                        "status": session.stage.status,
                        "active": !archived && current.as_deref() == Some(key.as_str()),
                        "archived": archived,
                        "inbox_unfolded": unfolded(root, &key)?,
                    }));
                }
            }
            print_json(&json!({"schema_version":1,"active":current,"sessions":sessions}))
        }
    }
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
                write_new_bounded(root, &file, &content, INBOX_LIMIT)?,
                "inbox collision at {file}; no retry or overwrite"
            );
            print_json(&json!({"schema_version":1,"added":file,"id":id}))
        }
        InboxCommand::List => {
            let key = active(root)?;
            let entries: Vec<_> = pending(root, &key)?
                .into_iter()
                .map(|(_, entry)| entry)
                .collect();
            print_json(&json!({"schema_version":1,"session":key,"entries":entries}))
        }
        InboxCommand::Fold => {
            let key = active(root)?;
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
