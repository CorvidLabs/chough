//! The canonical checkpoint: the front matter of the active session's `session.md`
//! (REQ-cli-037, REQ-records-004). A 0.5.x `.chough/session.json` is not read; it
//! names `chough migrate`.

use crate::records::{
    CHANGE_ID_LIMIT, ensure_text, local_path, native_json, print_json, read_bounded_record,
    valid_change_id, valid_name, write_replace_bounded,
};
use crate::records::{valid_criterion_id, valid_module_name, valid_reference_id};
use crate::stage;
use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

pub(super) const SESSION: &str = ".chough/session.json";
/// How long a writer waits for another Chough command to release the checkpoint.
#[cfg(not(test))]
const LOCK_WAIT: Duration = Duration::from_secs(30);
#[cfg(test)]
const LOCK_WAIT: Duration = Duration::from_millis(300);
const LOCK_POLL: Duration = Duration::from_millis(25);
const MAX_CALLERS: usize = 4;
const MAX_ITEMS: usize = 200;
/// Byte bound of every checkpoint (1 MiB, DRIVE-14: chosen from measured use on
/// 2026-11-05); other records keep the 64 KiB bound.
pub(super) const SESSION_LIMIT: u64 = 1_048_576;
/// Bound of the checkpoint `observations` list; other checkpoint lists keep `MAX_ITEMS`.
const MAX_OBSERVATIONS: usize = 400;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Caller {
    pub(super) stage: String,
    pub(super) reason: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StageState {
    pub(super) current: String,
    pub(super) status: String,
    #[serde(default)]
    pub(super) callers: Vec<Caller>,
    #[serde(default = "first_iteration")]
    pub(super) iteration: u32,
}

fn first_iteration() -> u32 {
    1
}

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct References {
    #[serde(default)]
    pub(super) criteria: Vec<String>,
    #[serde(default)]
    pub(super) requirements: Vec<String>,
    #[serde(default)]
    pub(super) cases: Vec<String>,
    #[serde(default)]
    pub(super) design: Vec<String>,
    #[serde(default)]
    pub(super) files: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Consent {
    pub(super) key: String,
    pub(super) source: String,
    pub(super) objective: String,
    pub(super) granted_at_unix_seconds: u64,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Checkpoint {
    pub(super) schema_version: u32,
    pub(super) objective: String,
    #[serde(default)]
    pub(super) intent_scope: String,
    #[serde(default)]
    pub(super) modules: Vec<String>,
    pub(super) stage: StageState,
    #[serde(default)]
    pub(super) native_change: Option<String>,
    #[serde(default)]
    pub(super) references: References,
    #[serde(default)]
    pub(super) decisions: Vec<String>,
    #[serde(default)]
    pub(super) open_questions: Vec<String>,
    #[serde(default)]
    pub(super) completed: Vec<String>,
    #[serde(default)]
    pub(super) observations: Vec<String>,
    #[serde(default)]
    pub(super) bugs: Vec<String>,
    #[serde(default)]
    pub(super) blockers: Vec<String>,
    #[serde(default)]
    pub(super) awaiting_human: Vec<String>,
    #[serde(default)]
    pub(super) run_consents: Vec<Consent>,
    pub(super) next_action: String,
    /// The session record's ID, aliases, pull request and gate details (REQ-records-004).
    #[serde(skip)]
    pub(super) meta: crate::session_md::Meta,
    /// The record this checkpoint was read from, so a save keeps every byte a person or
    /// agent wrote and every key Chough does not know (REQ-records-001).
    #[serde(skip)]
    pub(super) source: Option<crate::markdown::Record>,
    /// Transitions to append to the Timeline in the next save (REQ-history-002).
    #[serde(skip)]
    pub(super) pending: Vec<crate::stage::Transition>,
}

/// A readable session.
pub(super) enum Session {
    V2(Box<Checkpoint>),
}

impl Session {
    pub(super) fn objective(&self) -> &str {
        match self {
            Self::V2(session) => &session.objective,
        }
    }

    pub(super) fn intent_scope(&self) -> &str {
        match self {
            Self::V2(session) => &session.intent_scope,
        }
    }

    pub(super) fn modules(&self) -> &[String] {
        match self {
            Self::V2(session) => &session.modules,
        }
    }

    pub(super) fn next_action(&self) -> &str {
        match self {
            Self::V2(session) => &session.next_action,
        }
    }

    pub(super) fn to_value(&self) -> Result<Value> {
        Ok(match self {
            Self::V2(session) => serde_json::to_value(session.as_ref())?,
        })
    }
}

pub(super) fn new_checkpoint(
    objective: String,
    intent_scope: String,
    modules: Vec<String>,
) -> Checkpoint {
    Checkpoint {
        schema_version: 2,
        objective,
        intent_scope,
        modules,
        stage: StageState {
            current: "intent".into(),
            status: "active".into(),
            callers: vec![],
            iteration: 1,
        },
        native_change: None,
        references: References::default(),
        decisions: vec![],
        open_questions: vec![],
        completed: vec![],
        observations: vec![],
        bugs: vec![],
        blockers: vec![],
        awaiting_human: vec![],
        run_consents: vec![],
        next_action: "Read scoped intent and native status, then choose the next action within the authorized scope.".into(),
        meta: crate::session_md::Meta::default(),
        source: None,
        pending: vec![],
    }
}

fn validate_common(objective: &str, modules: &[String]) -> Result<()> {
    ensure!(
        !objective.trim().is_empty(),
        "session objective must not be blank"
    );
    ensure!(
        modules.iter().all(|module| valid_module_name(module)),
        "invalid session module name: use a module ID SpecSync accepts, such as chat.service"
    );
    ensure!(
        modules.len() <= 16,
        "select at most 16 modules for focused context"
    );
    Ok(())
}

fn ensure_ids(label: &str, values: &[String], limit: usize) -> Result<()> {
    ensure!(values.len() <= limit, "too many {label} (limit {limit})");
    // HI owns criterion IDs and SpecSync owns module IDs, which requirement and case IDs
    // embed, so each accepts exactly what its owner accepts (DRIVE-1).
    let valid: fn(&str) -> bool = match label {
        "criterion" => valid_criterion_id,
        "requirement" | "case" => valid_reference_id,
        _ => valid_name,
    };
    for value in values {
        ensure!(valid(value), "invalid {label} ID: {value}");
    }
    Ok(())
}

fn ensure_paths(label: &str, values: &[String]) -> Result<()> {
    ensure!(values.len() <= MAX_ITEMS, "too many {label}");
    for value in values {
        let path = Path::new(value);
        ensure!(
            !value.is_empty()
                && path
                    .components()
                    .all(|part| matches!(part, std::path::Component::Normal(_))),
            "{label} paths must be project-relative without traversal: {value}"
        );
    }
    Ok(())
}

pub(super) fn validate_v2(session: &Checkpoint) -> Result<()> {
    ensure!(
        session.schema_version == 2,
        "unsupported session schema_version"
    );
    validate_common(&session.objective, &session.modules)?;
    ensure!(
        stage::is_stage(&session.stage.current),
        "unknown stage: {}",
        session.stage.current
    );
    ensure!(
        matches!(session.stage.status.as_str(), "active" | "paused"),
        "stage status must be active or paused"
    );
    ensure!(session.stage.iteration >= 1, "stage iteration starts at 1");
    ensure!(
        session.stage.callers.len() <= MAX_CALLERS,
        "side stages nest at most {MAX_CALLERS} deep"
    );
    for caller in &session.stage.callers {
        ensure!(
            stage::is_stage(&caller.stage),
            "unknown caller stage: {}",
            caller.stage
        );
    }
    ensure!(
        stage::is_side(&session.stage.current) || session.stage.callers.is_empty(),
        "only a side stage can have callers"
    );
    if let Some(change) = &session.native_change {
        ensure!(
            valid_change_id(change),
            "invalid native change ID: {change}"
        );
    }
    ensure_ids("criterion", &session.references.criteria, MAX_ITEMS)?;
    ensure_ids("requirement", &session.references.requirements, MAX_ITEMS)?;
    ensure_ids("case", &session.references.cases, MAX_ITEMS)?;
    ensure_ids("observation", &session.observations, MAX_OBSERVATIONS)?;
    ensure_ids("bug", &session.bugs, MAX_ITEMS)?;
    ensure_paths("design", &session.references.design)?;
    ensure_paths("file", &session.references.files)?;
    for list in [
        &session.decisions,
        &session.open_questions,
        &session.completed,
        &session.blockers,
        &session.awaiting_human,
    ] {
        ensure!(list.len() <= MAX_ITEMS, "too many checkpoint entries");
    }
    ensure!(
        session.run_consents.len() <= MAX_ITEMS,
        "too many run consents"
    );
    for consent in &session.run_consents {
        ensure!(valid_name(&consent.key), "invalid consent key");
    }
    ensure!(
        serde_json::to_vec(session)?.len() as u64 <= SESSION_LIMIT,
        "session exceeds {SESSION_LIMIT} bytes"
    );
    Ok(())
}

/// Read the session record at `relative` from `content`.
pub(super) fn parse(relative: &str, content: &str) -> Result<Session> {
    let session = crate::session_md::read(relative, content)?;
    validate_v2(&session)?;
    Ok(Session::V2(Box::new(session)))
}

pub(super) fn read_session(root: &Path) -> Result<Session> {
    let relative = crate::sessions::checkpoint_path(root)?;
    let path = local_path(root, &relative)?;
    if !path.exists() {
        return Err(anyhow::anyhow!("{relative} does not exist"))
            .context("run `chough session new <key>` first");
    }
    read_bounded_record(&path, SESSION_LIMIT)
        .and_then(|content| parse(&relative, &content))
        .with_context(|| crate::sessions::shelve_hint(&relative))
}

/// The project's checkpoint writer lock (issues 62 and 83, DRIVE-11): the empty,
/// never-replaced file `.chough/local/checkpoint.lock`. `.chough/local/` holds a
/// `.gitignore` of `*` (written here when missing, create-only), so the lock is never a
/// managed record or a source change, and every Chough process working on the project
/// uses this one file whatever its temporary directory.
fn checkpoint_lock_path(root: &Path) -> Result<PathBuf> {
    let ignore = local_path(root, ".chough/local/.gitignore")?;
    fs::create_dir_all(
        ignore
            .parent()
            .context("the lock directory has no parent")?,
    )?;
    if !ignore.exists() {
        // Write it whole to a temp, then link it into place: a crash never leaves an empty
        // `.gitignore`, an existing one is never replaced, and a writer that loses the
        // race to create it carries on (#94 review).
        let temporary = ignore.with_file_name(format!(".gitignore.{}.tmp", std::process::id()));
        fs::write(&temporary, b"*\n").context("writing .chough/local/.gitignore")?;
        let linked = fs::hard_link(&temporary, &ignore);
        let _ = fs::remove_file(&temporary);
        match linked {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error).context("creating .chough/local/.gitignore"),
        }
    }
    local_path(root, ".chough/local/checkpoint.lock")
}

/// Locks this process holds, by lock path; each is released when the process exits or
/// is killed, so a killed writer never leaves a stale lock.
static HELD_LOCKS: Mutex<Vec<(PathBuf, fs::File)>> = Mutex::new(Vec::new());

/// Take the checkpoint writer lock for the rest of this process (issue 62), so every
/// read-modify-write of `.chough/session.json` applies its change to the latest
/// checkpoint. A repeated call is a no-op. After waiting `LOCK_WAIT` it refuses with
/// nothing saved. Taking the lock removes nothing: `save_checkpoint` removes session
/// temps left by killed writers once it holds the lock (issue 64), so a read-only
/// command that takes the lock leaves them in place. `init` takes no lock: it writes a new session that must not
/// exist yet, so the first writer after it creates the lock file.
pub(super) fn lock_checkpoint(root: &Path) -> Result<()> {
    let path = checkpoint_lock_path(root)?;
    let mut held = HELD_LOCKS.lock().unwrap_or_else(PoisonError::into_inner);
    if held.iter().any(|(locked, _)| *locked == path) {
        return Ok(());
    }
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&path)
    {
        // Another account may own the lock file; a shared read handle still locks.
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            OpenOptions::new().read(true).open(&path)
        }
        opened => opened,
    }
    .with_context(|| format!("opening the checkpoint lock {}", path.display()))?;
    let started = Instant::now();
    loop {
        match <fs::File as fs4::FileExt>::try_lock(&file) {
            Ok(()) => break,
            Err(fs4::TryLockError::WouldBlock) if started.elapsed() < LOCK_WAIT => {
                std::thread::sleep(LOCK_POLL);
            }
            Err(fs4::TryLockError::WouldBlock) => {
                // Name the file this command would write (#83). It refuses right after,
                // so resolving, and pinning, the checkpoint here changes nothing.
                let target = crate::sessions::checkpoint_path(root)
                    .unwrap_or_else(|_| "the checkpoint".into());
                bail!(
                    "refused:checkpoint-busy: another Chough command is changing {target} and did not finish within {} seconds; nothing was saved. Rerun when it finishes.",
                    LOCK_WAIT.as_secs()
                )
            }
            Err(fs4::TryLockError::Error(error)) => {
                return Err(error)
                    .with_context(|| format!("locking the checkpoint lock {}", path.display()));
            }
        }
    }
    held.push((path, file));
    Ok(())
}

/// Remove `.chough/.session.json.<pid>.tmp` files: the temps `save_checkpoint` writes
/// before its rename. Every writer holds the checkpoint lock through that rename, so a
/// temp seen while holding the lock was left by a writer that was killed before it
/// saved (issue 64). Its update was never saved or reported, so the session is not
/// changed. Only regular files with exactly that name directly in `.chough/` go.
fn remove_orphaned_session_temps(root: &Path) -> Result<()> {
    let session = local_path(root, SESSION)?;
    let directory = session.parent().context("session has no parent")?;
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        let orphan = name
            .strip_prefix(".session.json.")
            .and_then(|rest| rest.strip_suffix(".tmp"))
            .is_some_and(|pid| !pid.is_empty() && pid.bytes().all(|byte| byte.is_ascii_digit()));
        if !orphan || !entry.file_type()?.is_file() {
            continue;
        }
        let bytes = entry.metadata().map_or(0, |metadata| metadata.len());
        fs::remove_file(entry.path()).with_context(|| format!("removing .chough/{name}"))?;
        eprintln!(
            "chough: removed .chough/{name} ({bytes} bytes), left by a checkpoint write that was stopped before it saved; {SESSION} was not changed by it"
        );
    }
    crate::sessions::remove_orphaned_temps(root)
}

/// Take the checkpoint writer lock before a read-modify-write, in either mode: when
/// `.chough/session.json` exists or, in sessions mode (REQ-cli-037), when
/// `.chough/sessions/` exists, so a save never lands on a stale read and `session
/// adopt` cannot run between a legacy writer's read and its save. It is taken before
/// the checkpoint path is resolved, so the mode is read under the lock. Without
/// either there is no checkpoint to update.
pub(super) fn lock_for_update(root: &Path) -> Result<()> {
    if crate::sessions::enabled(root)? {
        lock_checkpoint(root)?;
    }
    Ok(())
}

/// Read a schema 2 checkpoint for a command that changes workflow state, holding the
/// checkpoint writer lock from before the read until the process exits.
pub(super) fn read_checkpoint(root: &Path) -> Result<Checkpoint> {
    lock_for_update(root)?;
    read_checkpoint_unlocked(root)
}

/// Read a schema 2 checkpoint without taking the writer lock, for a command that only
/// reads workflow state and may run for a long time (a repeated validation run checks
/// for a pause between attempts and must not block writers while it waits).
pub(super) fn read_checkpoint_unlocked(root: &Path) -> Result<Checkpoint> {
    match read_session(root)? {
        Session::V2(session) => Ok(*session),
    }
}

/// Every value this checkpoint writes, checked for leaks or redacted under `--redact`
/// (REQ-records-006); returns how many matches were redacted. Prose a person wrote in
/// the body is theirs and is not rewritten.
fn guard_leaks(session: &mut Checkpoint) -> Result<usize> {
    use crate::leaks::{Local, guard};
    let local = Local::current();
    let mut count = 0;
    count += guard(local, "objective", &mut session.objective)?;
    count += guard(local, "intent_scope", &mut session.intent_scope)?;
    count += guard(local, "next", &mut session.next_action)?;
    for (name, list) in [
        ("decision", &mut session.decisions),
        ("completed", &mut session.completed),
        ("question", &mut session.open_questions),
        ("blocker", &mut session.blockers),
        ("gate", &mut session.awaiting_human),
    ] {
        for value in list.iter_mut() {
            count += guard(local, name, value)?;
        }
    }
    for consent in &mut session.run_consents {
        count += guard(local, "consent source", &mut consent.source)?;
        count += guard(local, "consent objective", &mut consent.objective)?;
    }
    for alias in &mut session.meta.aliases {
        count += guard(local, "alias", alias)?;
    }
    count += guard(local, "pr", &mut session.meta.pr)?;
    for transition in &mut session.pending {
        if let Some(reason) = transition.reason.as_mut() {
            count += guard(local, "reason", reason)?;
        }
        if let Some(gate) = transition.gate.as_mut() {
            count += guard(local, "gate", gate)?;
        }
        if let Some(relay) = transition.relayed_by.as_mut() {
            count += guard(local, "decided_by", &mut relay.decided_by)?;
            count += guard(local, "words", &mut relay.words)?;
            count += guard(local, "where", &mut relay.place)?;
            count += guard(local, "recorded_by", &mut relay.recorded_by)?;
        }
        if let Some(backfill) = transition.backfill.as_mut() {
            count += guard(local, "quote", &mut backfill.quote)?;
        }
    }
    Ok(count)
}

/// The session record `session` saves as, within its bound (REQ-records-001), with
/// every value Chough writes checked for leaks first (REQ-records-006).
pub(super) fn render_checkpoint(session: &Checkpoint) -> Result<crate::markdown::Record> {
    validate_v2(session)?;
    let mut guarded = session.clone();
    let redacted = guard_leaks(&mut guarded)?;
    if redacted > 0 {
        eprintln!("chough: redacted {redacted} value(s) in the session record (--redact)");
    }
    let session = &guarded;
    let record = crate::session_md::render(session, crate::records::now()?)?;
    let content = record.render();
    ensure!(
        content.len() as u64 <= SESSION_LIMIT,
        "refused: the session record would be {} bytes, over its bound of {SESSION_LIMIT} bytes; run `chough session close {} --reason <text>` and then `chough session new <key>`; the session file is unchanged",
        content.len(),
        session.meta.id
    );
    Ok(record)
}

/// Save the checkpoint and its queued Timeline entries in one atomic replace; the
/// checkpoint then holds the record written and no queued entry.
pub(super) fn save_checkpoint(root: &Path, session: &mut Checkpoint) -> Result<()> {
    lock_checkpoint(root)?;
    remove_orphaned_session_temps(root)?;
    let record = render_checkpoint(session)?;
    write_replace_bounded(
        root,
        &crate::sessions::checkpoint_path(root)?,
        &record.render(),
        SESSION_LIMIT,
    )?;
    session.source = Some(record);
    session.pending.clear();
    Ok(())
}

#[derive(clap::Args)]
pub(super) struct CheckpointArgs {
    /// Replace the objective; this clears run consents tied to the old objective.
    #[arg(long)]
    objective: Option<String>,
    #[arg(long)]
    next_action: Option<String>,
    #[arg(long)]
    native_change: Option<String>,
    #[arg(long = "decision")]
    decisions: Vec<String>,
    #[arg(long = "question")]
    questions: Vec<String>,
    /// Remove an open question by its exact text.
    #[arg(long = "resolve-question")]
    resolved: Vec<String>,
    #[arg(long = "completed")]
    completed: Vec<String>,
    /// Remove a completed entry by its exact text, before `--completed` adds any, so the
    /// two together correct a mistaken entry in one call.
    #[arg(long = "remove-completed")]
    remove_completed: Vec<String>,
    #[arg(long = "blocker")]
    blockers: Vec<String>,
    #[arg(long)]
    clear_blockers: bool,
    /// A real gate still waiting for a human decision.
    #[arg(long = "awaiting-human")]
    awaiting_human: Vec<String>,
    /// Withdraw every open gate; records no human decision.
    #[arg(long)]
    clear_awaiting_human: bool,
    /// Close one open gate by its exact text with the human's decision, recorded on
    /// their behalf; needs --decided-by, --words, --where and --recorded-by
    /// (REQ-cli-050). A relayed claim, not authenticated.
    #[arg(
        long,
        allow_hyphen_values = true,
        conflicts_with_all = ["clear_awaiting_human", "awaiting_human"]
    )]
    close_gate: Option<String>,
    /// Who is said to have decided the gate named by --close-gate.
    #[arg(long, allow_hyphen_values = true)]
    decided_by: Option<String>,
    /// Their words, verbatim.
    #[arg(long, allow_hyphen_values = true)]
    words: Option<String>,
    /// Where and when they said it, including who relayed it.
    #[arg(long = "where", allow_hyphen_values = true)]
    place: Option<String>,
    /// The agent recording the decision.
    #[arg(long, allow_hyphen_values = true)]
    recorded_by: Option<String>,
    /// The review verdict, `pass` or `block`, for a gate that drives review; required
    /// there and never read from the words (REQ-records-008).
    #[arg(long, value_parser = ["pass", "block"], requires = "close_gate", conflicts_with = "no_native")]
    verdict: Option<String>,
    /// Close a gate that drives a native step without running it: the human's decision
    /// is neither an approval nor a review, such as a request for changes.
    #[arg(long, requires = "close_gate")]
    no_native: bool,
    /// The native step `chough change ship` already ran for this gate, which the
    /// decision names; nothing more runs (REQ-cli-059).
    #[arg(long, hide = true, requires = "close_gate", conflicts_with_all = ["verdict", "no_native"])]
    native_ran: Option<String>,
    /// The stage the gates added by --awaiting-human hold (REQ-cli-030); by default the
    /// current stage.
    #[arg(long, requires = "awaiting_human")]
    holds: Option<String>,
    /// The native step the gates added by --awaiting-human drive when a human closes
    /// them: approve, review or none; by default approve for a gate held at plan,
    /// review for one held at accept and none otherwise.
    #[arg(long, value_parser = ["approve", "review", "none"], requires = "awaiting_human")]
    drives: Option<String>,
    /// The listed summary's digest the decision was given on (REQ-history-014), for
    /// `chough gate record` and `chough change ship --list`.
    #[arg(long, hide = true, requires_all = ["close_gate", "list"])]
    summary_digest: Option<String>,
    /// The gate list the summary came from (REQ-history-014).
    #[arg(long, hide = true, requires_all = ["close_gate", "summary_digest"])]
    list: Option<String>,
    #[arg(long = "criterion")]
    criteria: Vec<String>,
    #[arg(long = "requirement")]
    requirements: Vec<String>,
    #[arg(long = "case")]
    cases: Vec<String>,
    #[arg(long = "design")]
    design: Vec<String>,
    #[arg(long = "file")]
    files: Vec<String>,
    #[arg(long = "observation")]
    observations: Vec<String>,
    #[arg(long = "bug")]
    bugs: Vec<String>,
}

fn push_unique(list: &mut Vec<String>, values: Vec<String>) {
    for value in values {
        if !list.contains(&value) {
            list.push(value);
        }
    }
}

/// Refuse to record a native change SpecSync does not report (issue 63). With SpecSync
/// configured, `specsync change status <id> --json` must succeed with a JSON response;
/// a missing change (SpecSync exits non-zero), a failed or timed-out read, or output
/// that is not JSON refuses before anything is written, and so does a reply that names
/// no change ID or whose `summary.id` or `change.id` names a different change. What the state means stays
/// with the stage gates. Without a configuration there is nothing to ask.
fn ensure_native_change_known(root: &Path, change: &str) -> Result<()> {
    if !stage::native_configured(root)? {
        return Ok(());
    }
    let response = native_json(root, "specsync", &["change", "status", change, "--json"]);
    if response.value.is_none() {
        let section = &response.section;
        bail!(
            "refused:native-change-unknown: SpecSync did not report native change {change}, so it was not recorded and the checkpoint is unchanged.\n{} ({}): {}\nRun `specsync change status {change}` or create the change with `specsync change new`.",
            section.source,
            section.state,
            section.content.trim_end()
        );
    }
    // The reply must be about the requested change: it must name a change ID
    // (`summary.id` or `change.id`; SpecSync 6.1 names both) and every ID it names
    // must match. A reply that names neither does not identify the change.
    let value = response.value.as_ref().expect("checked above");
    let named: Vec<&str> = [value.pointer("/summary/id"), value.pointer("/change/id")]
        .into_iter()
        .flatten()
        .map(|id| id.as_str().unwrap_or("a non-text change id"))
        .collect();
    if named.is_empty() {
        bail!(
            "refused:native-change-unknown: SpecSync's reply for native change {change} names no change ID, so it does not identify the change; it was not recorded and the checkpoint is unchanged.\nRun `specsync change status {change} --json` and check that it reports the change.",
        );
    }
    if named.iter().any(|id| *id != change) {
        let reported = named.join(" and ");
        bail!(
            "refused:native-change-mismatch: SpecSync was asked for native change {change} but its reply names {reported}, so it was not recorded and the checkpoint is unchanged.\nRun `specsync change status {change} --json` and record the ID it reports.",
        );
    }
    Ok(())
}

/// What every relayed decision says of itself (REQ-cli-050).
pub(super) const RELAY_PROVENANCE: &str = "a relayed claim, not authenticated: it records who is said to have decided, not proof that they did";

/// Why a relayed decision cannot be recorded. Each caller words the refusal with its
/// own command to run again (REQ-cli-050, REQ-cli-059).
pub(super) enum RelayProblem {
    /// Relay values were given without a gate: the flags given.
    WithoutGate(Vec<&'static str>),
    /// The gate, and the flags that are missing or blank.
    Incomplete(String, Vec<&'static str>),
    /// No open gate has this exact text.
    NotOpen(String),
    /// The gate, and the length in bytes the decision would have.
    TooLong(String, usize),
}

/// A relayed decision: the gate it closes, the relay and the decision text.
pub(super) type Relayed = (String, stage::RelayedBy, String);

/// The gate, relay and decision a `--close-gate` relay records, checked against the
/// `open` gates before anything is read from SpecSync or written (REQ-cli-050).
/// `values` are the four flags with their values, in the order of
/// [`stage::RelayedBy::fields`].
pub(super) fn check_relay(
    open: &[String],
    gate: Option<&str>,
    values: [(&'static str, Option<&str>); 4],
) -> std::result::Result<Option<Relayed>, RelayProblem> {
    let Some(gate) = gate else {
        let given: Vec<&str> = values
            .iter()
            .filter(|(_, value)| value.is_some())
            .map(|(flag, _)| *flag)
            .collect();
        if !given.is_empty() {
            return Err(RelayProblem::WithoutGate(given));
        }
        return Ok(None);
    };
    let missing: Vec<&str> = values
        .iter()
        .filter(|(_, value)| value.is_none_or(|value| value.trim().is_empty()))
        .map(|(flag, _)| *flag)
        .collect();
    if !missing.is_empty() {
        return Err(RelayProblem::Incomplete(gate.into(), missing));
    }
    if !open.iter().any(|open| open == gate) {
        return Err(RelayProblem::NotOpen(gate.into()));
    }
    let text = |index: usize| values[index].1.unwrap_or_default().to_string();
    let relay = stage::RelayedBy {
        decided_by: text(0),
        words: text(1),
        place: text(2),
        recorded_by: text(3),
        summary_digest: None,
        list: None,
    };
    let decision = decision_text(gate, &relay, None);
    if decision.len() > stage::RELAY_TEXT_LIMIT {
        return Err(RelayProblem::TooLong(gate.into(), decision.len()));
    }
    Ok(Some((gate.into(), relay, decision)))
}

/// The decision a relayed gate close adds: who decided, their words, where, the
/// recording agent and, when one ran, the native step (REQ-cli-050, REQ-records-008).
pub(super) fn decision_text(gate: &str, relay: &stage::RelayedBy, step: Option<&str>) -> String {
    let mut text = format!(
        "{} decided \"{gate}\": \"{}\" ({}); recorded by {}; a relayed claim, not authenticated",
        relay.decided_by, relay.words, relay.place, relay.recorded_by
    );
    if let Some(step) = step {
        text.push_str("; ");
        text.push_str(step);
    }
    text
}

/// Check the four relay values for leaks, or redact them under `--redact`, before
/// anything runs (REQ-cli-050, REQ-records-006); returns how many were redacted.
pub(super) fn guard_relay(relay: &mut stage::RelayedBy) -> Result<usize> {
    use crate::leaks::{Local, guard};
    let local = Local::current();
    Ok(guard(local, "--decided-by", &mut relay.decided_by)?
        + guard(local, "--words", &mut relay.words)?
        + guard(local, "--where", &mut relay.place)?
        + guard(local, "--recorded-by", &mut relay.recorded_by)?)
}

/// The native step a relayed gate close runs as the decided human (REQ-records-008).
pub(super) struct NativeStep {
    pub(super) change: String,
    pub(super) human: crate::humans::Human,
    pub(super) step: crate::humans::Step,
}

/// What closing `gate` with `relay` runs, checked before anything runs: the step the
/// gate drives as the configured human, or nothing for a gate that drives nothing or a
/// close with `--no-native`. Returns the step and the text the decision names it by.
pub(super) fn native_step_for(
    root: &Path,
    session: &Checkpoint,
    gate: &str,
    relay: &stage::RelayedBy,
    verdict: Option<&str>,
    no_native: bool,
) -> Result<(Option<NativeStep>, Option<String>)> {
    use crate::humans::{Drives, Step};
    let drives = crate::session_md::gate(session, gate).drives();
    match (drives, verdict) {
        (Drives::Review, None) if !no_native => bail!(
            "refused:verdict-required: gate \"{gate}\" drives the native review, which needs the human's verdict as --verdict pass or --verdict block (never read from their words), so nothing ran or was saved.\nRun the same command with --verdict, or with --no-native for a decision that is neither an approval nor a review."
        ),
        (Drives::Approve | Drives::None, Some(_)) => bail!(
            "refused:verdict-not-applicable: gate \"{gate}\" drives {}, not a review, so --verdict does not apply; nothing ran or was saved.\nRun the same command without --verdict.",
            if drives == Drives::None {
                "no native step"
            } else {
                "the native approval"
            }
        ),
        _ => {}
    }
    if drives == Drives::None {
        return Ok((None, None));
    }
    if no_native {
        return Ok((
            None,
            Some(format!(
                "no native {} ran (--no-native)",
                if drives == Drives::Approve {
                    "approval"
                } else {
                    "review"
                }
            )),
        ));
    }
    let Some(change) = session.native_change.clone() else {
        bail!(
            "refused:no-native-change: gate \"{gate}\" drives the native {}, and the session records no native change, so nothing ran or was saved.\nRun `chough checkpoint --native-change <id>` first, or close the gate with --no-native for a decision that is neither an approval nor a review.",
            drives.name()
        );
    };
    let human = crate::humans::resolve(root, &relay.decided_by)?;
    let step = match drives {
        Drives::Approve => Step::Approve {
            note: crate::humans::approval_note(
                &human,
                gate,
                relay,
                &crate::session_md::iso_time(crate::records::now()?),
            ),
        },
        _ => Step::Review {
            verdict: verdict.unwrap_or("pass").to_string(),
        },
    };
    let summary = step.summary(&change, &human);
    Ok((
        Some(NativeStep {
            change,
            human,
            step,
        }),
        Some(summary),
    ))
}

/// Run a checked native step; on a SpecSync refusal pass its output and exit code
/// through and end the process, so nothing is recorded (REQ-records-008).
pub(super) fn run_native_step(root: &Path, native: &NativeStep) -> Result<serde_json::Value> {
    use std::io::Write;
    let ran = crate::humans::run(root, &native.change, &native.human, &native.step)?;
    if ran.code != 0 {
        let _ = std::io::stdout().write_all(&ran.stdout);
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().write_all(&ran.stderr);
        let _ = std::io::stderr().flush();
        std::process::exit(ran.code);
    }
    // SpecSync's own report goes to standard error, so standard output stays one JSON
    // document.
    let _ = std::io::stderr().write_all(&ran.stdout);
    let _ = std::io::stderr().write_all(&ran.stderr);
    Ok(json!({
        "ran": native.step.summary(&native.change, &native.human),
        "human": native.human.name,
        "specsync_actor": native.human.specsync_actor,
        "exit_code": ran.code,
    }))
}

/// The gate, relay and decision `--close-gate` records, checked before anything is
/// read from SpecSync or written (REQ-cli-050). Every refusal saves nothing.
fn relayed_decision(session: &Checkpoint, args: &CheckpointArgs) -> Result<Option<Relayed>> {
    let values = [
        ("--decided-by", args.decided_by.as_deref()),
        ("--words", args.words.as_deref()),
        ("--where", args.place.as_deref()),
        ("--recorded-by", args.recorded_by.as_deref()),
    ];
    check_relay(&session.awaiting_human, args.close_gate.as_deref(), values).map_err(|problem| {
        match problem {
            RelayProblem::WithoutGate(given) => anyhow::anyhow!(
                "refused:relay-without-gate: {} describe a human's decision at a gate, so they need --close-gate with that gate's exact text; nothing was saved.\nRun `chough next` to list the open gates, then `{}` with one of them.",
                given.join(", "),
                stage::CLOSE_GATE_COMMAND
            ),
            RelayProblem::Incomplete(gate, missing) => anyhow::anyhow!(
                "refused:relay-incomplete: closing gate \"{gate}\" on a human's behalf needs who decided, their words, where they said them and the recording agent; {} {} missing or blank, so nothing was saved.\nRun `chough checkpoint --close-gate \"{gate}\" --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"`.",
                missing.join(", "),
                if missing.len() == 1 { "is" } else { "are" }
            ),
            RelayProblem::NotOpen(gate) => anyhow::anyhow!(
                "refused:gate-not-open: no open gate has the exact text \"{gate}\", so nothing was saved.\nRun `chough next` to list the open gates, then close one by its exact text."
            ),
            RelayProblem::TooLong(gate, length) => anyhow::anyhow!(
                "refused:relay-too-long: the decision for gate \"{gate}\" would be {length} bytes, over the {}-byte checkpoint text bound, so nothing was saved.\nShorten --words or --where (keep the full text in a committed file and name it in --where), then run `chough checkpoint --close-gate \"{gate}\"` with the four values again.",
                stage::RELAY_TEXT_LIMIT
            ),
        }
    })
}

/// Save `session` with a gate-closed entry for each of `cleared` and a gate-opened entry
/// for each of `opened`, all in the same atomic replace as the checkpoint change, so a
/// refused save writes neither. `checkpoint` and the doctor's gate withdrawal
/// (REQ-history-012) share it.
fn save_with_gate_events(
    root: &Path,
    session: &mut Checkpoint,
    current_stage: &str,
    cleared: &[String],
    opened: &[String],
    relay_for: impl Fn(&String) -> Option<stage::RelayedBy>,
) -> Result<()> {
    queue_gate_events(root, session, current_stage, cleared, opened, relay_for)?;
    save_checkpoint(root, session)
}

/// Queue a gate-closed entry for each of `cleared` and a gate-opened entry for each of
/// `opened` for the next save.
fn queue_gate_events(
    root: &Path,
    session: &mut Checkpoint,
    current_stage: &str,
    cleared: &[String],
    opened: &[String],
    relay_for: impl Fn(&String) -> Option<stage::RelayedBy>,
) -> Result<()> {
    for gate in cleared {
        stage::log_gate(
            root,
            session,
            stage::GATE_CLOSED,
            current_stage,
            gate,
            relay_for(gate),
        )?;
    }
    for gate in opened {
        stage::log_gate(root, session, stage::GATE_OPENED, current_stage, gate, None)?;
    }
    Ok(())
}

/// Withdraw every open awaiting-human gate of the canonical checkpoint, as
/// `chough checkpoint --clear-awaiting-human` does: gate-closed events with no relayed
/// decision, and no decision recorded. Returns the withdrawn gates.
pub(super) fn withdraw_gates(root: &Path) -> Result<Vec<String>> {
    let mut session = read_checkpoint(root)?;
    let cleared = std::mem::take(&mut session.awaiting_human);
    if cleared.is_empty() {
        return Ok(cleared);
    }
    let current_stage = session.stage.current.clone();
    save_with_gate_events(root, &mut session, &current_stage, &cleared, &[], |_| None)?;
    Ok(cleared)
}

pub(super) fn checkpoint_command(root: &Path, args: CheckpointArgs) -> Result<()> {
    let mut session = read_checkpoint(root)?;
    let mut relayed = relayed_decision(&session, &args)?;
    if let Some((_, relay, _)) = relayed.as_mut() {
        relay.summary_digest = args.summary_digest.clone();
        relay.list = args.list.clone();
        let probe = stage::Transition {
            schema_version: 1,
            at_unix_seconds: 0,
            kind: stage::GATE_CLOSED.into(),
            from: "intent".into(),
            to: "intent".into(),
            reason: None,
            gate: None,
            revision: None,
            refresh: vec![],
            backfill: None,
            relayed_by: Some(relay.clone()),
            direct_write: None,
            actor: None,
        };
        stage::validate_relay(&probe)
            .context("the summary digest and list are not a valid pair; nothing was saved")?;
    }
    // REQ-records-008: the relay passes the leak guard, and the native step the gate
    // drives is resolved, before anything runs.
    let mut native = None;
    let mut native_text = None;
    let mut native_report: Option<serde_json::Value> = None;
    if let Some((gate, relay, decision)) = relayed.as_mut() {
        guard_relay(relay)?;
        (native, native_text) = match &args.native_ran {
            Some(ran) => (None, Some(ran.clone())),
            None => native_step_for(
                root,
                &session,
                gate,
                relay,
                args.verdict.as_deref(),
                args.no_native,
            )?,
        };
        *decision = decision_text(gate, relay, native_text.as_deref());
        if decision.len() > stage::RELAY_TEXT_LIMIT {
            bail!(
                "refused:relay-too-long: the decision for gate \"{gate}\" would be {} bytes, over the {}-byte checkpoint text bound, so nothing ran or was saved.\nShorten --words or --where (keep the full text in a committed file and name it in --where), then run `chough checkpoint --close-gate \"{gate}\"` with the four values again.",
                decision.len(),
                stage::RELAY_TEXT_LIMIT
            );
        }
    }
    if let Some(holds) = &args.holds {
        ensure!(
            stage::is_stage(holds),
            "--holds must name a stage (a main stage, release or a side stage); got {holds:?}, so nothing was saved"
        );
    }
    let mut changed = false;
    if let Some(objective) = args.objective {
        ensure_text("objective", &objective)?;
        if objective != session.objective {
            session.objective = objective;
            session.run_consents.clear();
        }
        changed = true;
    }
    if let Some(next_action) = args.next_action {
        ensure_text("next action", &next_action)?;
        session.next_action = next_action;
        changed = true;
    }
    if let Some(change) = args.native_change {
        ensure!(
            valid_change_id(&change),
            "invalid native change ID: up to {CHANGE_ID_LIMIT} bytes of letters, digits, - and _"
        );
        ensure_native_change_known(root, &change)?;
        session.native_change = Some(change);
        changed = true;
    }
    // Clears run before additions, so entries given in the same call are kept.
    if args.clear_blockers {
        session.blockers.clear();
        changed = true;
    }
    let cleared = if args.clear_awaiting_human {
        changed = true;
        std::mem::take(&mut session.awaiting_human)
    } else if let Some((gate, _, decision)) = &relayed {
        changed = true;
        session.awaiting_human.retain(|open| open != gate);
        push_unique(&mut session.decisions, vec![decision.clone()]);
        vec![gate.clone()]
    } else {
        vec![]
    };
    let relay_for = |gate: &String| {
        relayed
            .as_ref()
            .filter(|(closed, _, _)| closed == gate)
            .map(|(_, relay, _)| relay.clone())
    };
    let open_before_additions = session.awaiting_human.clone();
    for resolved in &args.resolved {
        let before = session.open_questions.len();
        session
            .open_questions
            .retain(|question| question != resolved);
        ensure!(
            session.open_questions.len() < before,
            "no open question has that exact text: {resolved}"
        );
        changed = true;
    }
    for removed in &args.remove_completed {
        let position = session.completed.iter().position(|entry| entry == removed);
        let Some(position) = position else {
            bail!("no completed entry has that exact text: {removed}");
        };
        session.completed.remove(position);
        changed = true;
    }
    // REQ-cli-030: the gates added here hold --holds (by default the current stage) and
    // drive --drives (by default that stage's step).
    for gate in &args.awaiting_human {
        if session.awaiting_human.contains(gate) {
            continue;
        }
        let holds = args
            .holds
            .clone()
            .unwrap_or_else(|| session.stage.current.clone());
        let drives = args.drives.clone().unwrap_or_else(|| {
            crate::humans::Drives::default_for(&holds)
                .name()
                .to_string()
        });
        session.meta.gates.retain(|meta| &meta.gate != gate);
        session.meta.gates.push(crate::session_md::GateMeta {
            gate: gate.clone(),
            holds,
            drives,
            opened: String::new(),
        });
    }
    for (list, values, label) in [
        (&mut session.decisions, args.decisions, "decision"),
        (&mut session.open_questions, args.questions, "question"),
        (&mut session.completed, args.completed, "completed work"),
        (&mut session.blockers, args.blockers, "blocker"),
        (
            &mut session.awaiting_human,
            args.awaiting_human,
            "awaiting-human gate",
        ),
    ] {
        for value in &values {
            ensure_text(label, value)?;
        }
        changed |= !values.is_empty();
        push_unique(list, values);
    }
    for (list, values) in [
        (&mut session.references.criteria, args.criteria),
        (&mut session.references.requirements, args.requirements),
        (&mut session.references.cases, args.cases),
        (&mut session.references.design, args.design),
        (&mut session.references.files, args.files),
        (&mut session.observations, args.observations),
        (&mut session.bugs, args.bugs),
    ] {
        changed |= !values.is_empty();
        push_unique(list, values);
    }
    if changed {
        let current_stage = session.stage.current.clone();
        let opened: Vec<String> = session
            .awaiting_human
            .iter()
            .filter(|gate| !open_before_additions.contains(gate))
            .cloned()
            .collect();
        queue_gate_events(
            root,
            &mut session,
            &current_stage,
            &cleared,
            &opened,
            relay_for,
        )?;
        if let Some(native) = &native {
            // Every check the save makes, before the native step runs, so a refused
            // save never follows a native write.
            render_checkpoint(&session)?;
            native_report = Some(run_native_step(root, native)?);
        }
        save_checkpoint(root, &mut session)?;
    }
    let mut output = json!({"schema_version": 1, "saved": changed, "checkpoint": session});
    if let Some((gate, relay, _)) = &relayed {
        output["relayed_decision"] = json!({
            "gate": gate,
            "relayed_by": relay,
            "provenance": RELAY_PROVENANCE,
            "native_step": native_report.clone().unwrap_or_else(|| json!({"ran": null, "why": native_text})),
        });
    }
    print_json(&output)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// DRIVE-14 (REQ-cli-032): validation, which `init` runs before it writes anything,
    /// refuses a checkpoint whose serialization is over the 1 MiB cap and names the cap.
    #[test]
    fn validation_refuses_a_checkpoint_over_the_cap() {
        // JSON escapes each control character to six bytes: 180,000 of them serialize
        // to about 1.08 MB.
        let session = new_checkpoint("\u{0001}".repeat(180_000), String::new(), vec![]);
        let refused = validate_v2(&session).unwrap_err();
        assert!(format!("{refused:#}").contains("1048576"), "{refused:#}");
        let within = new_checkpoint("\u{0001}".repeat(170_000), String::new(), vec![]);
        validate_v2(&within).unwrap();
    }

    #[test]
    fn a_held_checkpoint_lock_refuses_as_busy_after_the_wait() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join(".chough")).unwrap();
        let other = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(checkpoint_lock_path(root.path()).unwrap())
            .unwrap();
        <fs::File as fs4::FileExt>::lock(&other).unwrap();
        let started = Instant::now();
        let error = lock_checkpoint(root.path()).unwrap_err().to_string();
        assert!(error.starts_with("refused:checkpoint-busy"), "{error}");
        assert!(started.elapsed() >= LOCK_WAIT);
        drop(other);
        lock_checkpoint(root.path()).unwrap();
        lock_checkpoint(root.path()).unwrap();
        // The lock lives in the project (#83, DRIVE-11): only `.chough/local/`, which
        // Git ignores, is added.
        let records: Vec<_> = fs::read_dir(root.path().join(".chough"))
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        assert_eq!(
            records,
            vec!["local"],
            "the lock added {records:?} to .chough"
        );
        assert_eq!(
            checkpoint_lock_path(root.path()).unwrap(),
            root.path().join(".chough/local/checkpoint.lock")
        );
        assert_eq!(
            fs::read_to_string(root.path().join(".chough/local/.gitignore")).unwrap(),
            "*\n"
        );
    }

    /// Issue #83 item b: in sessions mode the busy refusal names the session file the
    /// command would write, not `.chough/session.json`.
    #[test]
    fn a_busy_refusal_names_the_session_file_being_written() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        crate::sessions::run(
            &root,
            crate::sessions::SessionCommand::New {
                key: "alpha".into(),
                objective: "Ship alpha".into(),
                intent_scope: String::new(),
                modules: vec![],
            },
        )
        .unwrap();
        // `session new` took the lock in this process; hand it to another holder.
        let path = checkpoint_lock_path(&root).unwrap();
        HELD_LOCKS
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .retain(|(locked, _)| *locked != path);
        let other = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .unwrap();
        <fs::File as fs4::FileExt>::lock(&other).unwrap();
        let error = lock_for_update(&root).unwrap_err().to_string();
        assert!(error.starts_with("refused:checkpoint-busy"), "{error}");
        assert!(error.contains("-alpha/session.md"), "{error}");
        assert!(!error.contains(".chough/session.json"), "{error}");
        assert!(error.contains("nothing was saved"), "{error}");
        drop(other);
    }
}
