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
use std::fs;
use std::path::Path;

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
    let Ok(entries) = fs::read_dir(&path) else {
        return Ok(keys);
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
pub(super) fn checkpoint_path(root: &Path) -> Result<String> {
    if !enabled(root)? {
        return Ok(checkpoint::SESSION.to_string());
    }
    Ok(session_file(&active(root)?))
}

fn read_at(root: &Path, relative: &str) -> Result<Checkpoint> {
    let content = read_bounded_record(&local_path(root, relative)?, checkpoint::SESSION_LIMIT)
        .with_context(|| format!("reading {relative}"))?;
    match checkpoint::parse(&content)? {
        Session::V2(session) => Ok(*session),
        Session::V1(_) => bail!("{relative} uses schema 1"),
    }
}

fn point(root: &Path, key: &str) -> Result<()> {
    write_new(root, &format!("{LOCAL}/.gitignore"), "*\n")?;
    write_replace_bounded(root, POINTER, &format!("{key}\n"), 512)
}

/// Move every other open session at stage done to the archive, bytes unchanged.
fn archive_done(root: &Path, keep: &str) -> Result<Vec<String>> {
    let mut archived = vec![];
    for key in keys_in(root, DIR)? {
        if key == keep || read_at(root, &session_file(&key))?.stage.current != "done" {
            continue;
        }
        let target = local_path(root, &archive_file(&key))?;
        ensure!(!target.exists(), "{} already exists", archive_file(&key));
        fs::create_dir_all(local_path(root, ARCHIVE)?)?;
        fs::rename(local_path(root, &session_file(&key))?, &target)?;
        archived.push(key);
    }
    Ok(archived)
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
            let archived = archive_done(root, &key)?;
            point(root, &key)?;
            print_json(
                &json!({"schema_version":1,"created":session_file(&key),"active":key,"archived":archived}),
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
            let archived = archive_done(root, &key)?;
            point(root, &key)?;
            print_json(&json!({"schema_version":1,"active":key,"archived":archived}))
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
            ensure!(
                valid_name(&from),
                "--from uses letters, digits, hyphens, underscores"
            );
            match kind {
                Kind::Observation => ensure!(
                    valid_name(&text),
                    "an observation entry names an evidence ID"
                ),
                _ => ensure_text("text", &text)?,
            }
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
            for (_, entry) in &entries {
                let list = match entry.kind.as_str() {
                    "observation" => &mut session.observations,
                    "decision" => &mut session.decisions,
                    "completed" => &mut session.completed,
                    "question" => &mut session.open_questions,
                    other => bail!("inbox entry {} has unknown kind {other}", entry.id),
                };
                if !list.contains(&entry.text) {
                    list.push(entry.text.clone());
                }
            }
            if !entries.is_empty() {
                checkpoint::save_checkpoint(root, &session)?;
                let folded = format!("{}/folded", inbox_dir(&key));
                fs::create_dir_all(local_path(root, &folded)?)?;
                for (file, entry) in &entries {
                    let target = local_path(root, &format!("{folded}/{}.json", entry.id))?;
                    ensure!(!target.exists(), "{} already folded", entry.id);
                    fs::rename(local_path(root, file)?, target)?;
                }
            }
            let ids: Vec<_> = entries.iter().map(|(_, entry)| entry.id.clone()).collect();
            print_json(&json!({"schema_version":1,"session":key,"folded":ids}))
        }
    }
}
