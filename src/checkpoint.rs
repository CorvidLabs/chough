//! The single canonical checkpoint: `.chough/session.json`, or the active session
//! file under `.chough/sessions/` (REQ-cli-037).

use crate::records::{
    CHANGE_ID_LIMIT, ensure_text, local_path, native_json, print_json, read_bounded_record,
    valid_change_id, valid_name, write_new_bytes, write_replace_bounded,
};
use crate::records::{valid_criterion_id, valid_module_name, valid_reference_id};
use crate::stage;
use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

pub(super) const SESSION: &str = ".chough/session.json";
pub(super) const SESSION_V1_BACKUP: &str = ".chough/session.v1.json";
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
pub(super) struct SessionV1 {
    pub(super) schema_version: u32,
    pub(super) objective: String,
    pub(super) intent_scope: String,
    pub(super) modules: Vec<String>,
    pub(super) decisions: Vec<String>,
    pub(super) open_questions: Vec<String>,
    pub(super) next_action: String,
}

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) history_cursor: Option<crate::history::Cursor>,
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
}

/// A readable session in either supported schema.
pub(super) enum Session {
    V1(SessionV1),
    V2(Box<Checkpoint>),
}

impl Session {
    pub(super) fn objective(&self) -> &str {
        match self {
            Self::V1(session) => &session.objective,
            Self::V2(session) => &session.objective,
        }
    }

    pub(super) fn intent_scope(&self) -> &str {
        match self {
            Self::V1(session) => &session.intent_scope,
            Self::V2(session) => &session.intent_scope,
        }
    }

    pub(super) fn modules(&self) -> &[String] {
        match self {
            Self::V1(session) => &session.modules,
            Self::V2(session) => &session.modules,
        }
    }

    pub(super) fn next_action(&self) -> &str {
        match self {
            Self::V1(session) => &session.next_action,
            Self::V2(session) => &session.next_action,
        }
    }

    pub(super) fn to_value(&self) -> Result<Value> {
        Ok(match self {
            Self::V1(session) => serde_json::to_value(session)?,
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
        history_cursor: None,
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

pub(super) fn validate_v1(session: &SessionV1) -> Result<()> {
    ensure!(
        session.schema_version == 1,
        "unsupported session schema_version"
    );
    validate_common(&session.objective, &session.modules)?;
    ensure!(
        serde_json::to_vec(session)?.len() as u64 <= SESSION_LIMIT,
        "session exceeds {SESSION_LIMIT} bytes"
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
    if let Some(cursor) = &session.history_cursor {
        crate::history::validate_cursor(cursor)?;
    }
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

pub(super) fn parse(content: &str) -> Result<Session> {
    let value: Value = serde_json::from_str(content).context("invalid session record")?;
    match value.get("schema_version").and_then(Value::as_u64) {
        Some(1) => {
            let session: SessionV1 =
                serde_json::from_value(value).context("invalid schema 1 session record")?;
            validate_v1(&session)?;
            Ok(Session::V1(session))
        }
        Some(2) => {
            let session: Checkpoint =
                serde_json::from_value(value).context("invalid schema 2 session record")?;
            validate_v2(&session)?;
            Ok(Session::V2(Box::new(session)))
        }
        _ => bail!("unsupported session schema_version"),
    }
}

pub(super) fn read_session(root: &Path) -> Result<Session> {
    let relative = crate::sessions::checkpoint_path(root)?;
    let path = local_path(root, &relative)?;
    if !path.exists() {
        return Err(anyhow::anyhow!("{relative} does not exist")).context("run chough init first");
    }
    read_bounded_record(&path, SESSION_LIMIT)
        .and_then(|content| parse(&content))
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
    if local_path(root, SESSION)?.is_file() || crate::sessions::enabled(root)? {
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
        Session::V1(_) => bail!(
            "the session uses schema 1; run `chough migrate` to keep its bytes in {SESSION_V1_BACKUP} and continue with schema 2"
        ),
    }
}

fn serialize_checkpoint(session: &Checkpoint) -> Result<String> {
    validate_v2(session)?;
    let content = serde_json::to_string_pretty(session)? + "\n";
    ensure!(
        content.len() as u64 <= SESSION_LIMIT,
        "session exceeds {SESSION_LIMIT} bytes"
    );
    Ok(content)
}

pub(super) fn save_checkpoint(root: &Path, session: &Checkpoint) -> Result<()> {
    lock_checkpoint(root)?;
    remove_orphaned_session_temps(root)?;
    write_replace_bounded(
        root,
        &crate::sessions::checkpoint_path(root)?,
        &serialize_checkpoint(session)?,
        SESSION_LIMIT,
    )
}

pub(super) fn migrate(root: &Path, initial_stage: &str) -> Result<()> {
    ensure!(
        stage::is_main(initial_stage),
        "migration stage must be a main stage"
    );
    lock_for_update(root)?;
    let path = local_path(root, &crate::sessions::checkpoint_path(root)?)?;
    let original = read_bounded_record(&path, SESSION_LIMIT).context("run chough init first")?;
    let session = match parse(&original)? {
        Session::V2(_) => {
            return print_json(
                &json!({"schema_version": 1, "migrated": false, "reason": "session already uses schema 2"}),
            );
        }
        Session::V1(session) => session,
    };
    ensure!(
        !local_path(root, SESSION_V1_BACKUP)?.exists(),
        "{SESSION_V1_BACKUP} already exists; inspect it before migrating again"
    );
    let mut checkpoint = new_checkpoint(
        session.objective.clone(),
        session.intent_scope.clone(),
        session.modules.clone(),
    );
    checkpoint.stage.current = initial_stage.into();
    checkpoint.decisions = session.decisions.clone();
    checkpoint.open_questions = session.open_questions.clone();
    checkpoint.next_action = session.next_action.clone();
    validate_v2(&checkpoint)?;
    ensure!(
        write_new_bytes(root, SESSION_V1_BACKUP, original.as_bytes())?,
        "{SESSION_V1_BACKUP} already exists"
    );
    save_checkpoint(root, &checkpoint)?;
    print_json(&json!({
        "schema_version": 1,
        "migrated": true,
        "preserved": SESSION_V1_BACKUP,
        "stage": initial_stage,
    }))
}

fn read_merge_operand(path: &Path) -> Result<(PathBuf, Checkpoint)> {
    let metadata = fs::symlink_metadata(path)
        .with_context(|| format!("reading merge operand {}", path.display()))?;
    ensure!(
        metadata.file_type().is_file(),
        "merge operand must be an existing regular file whose final component is not a symbolic link: {}",
        path.display()
    );
    let resolved = path.canonicalize()?;
    let content = read_bounded_record(path, SESSION_LIMIT)?;
    match parse(&content)? {
        Session::V2(session) => Ok((resolved, *session)),
        Session::V1(_) => bail!(
            "checkpoint --merge requires schema 2 session inputs; schema 1 requires explicit migration"
        ),
    }
}

fn merge_strings(base: &[String], ours: &[String], theirs: &[String]) -> Vec<String> {
    let base_members: HashSet<&str> = base.iter().map(String::as_str).collect();
    let ours_members: HashSet<&str> = ours.iter().map(String::as_str).collect();
    let theirs_members: HashSet<&str> = theirs.iter().map(String::as_str).collect();
    let mut emitted = HashSet::new();
    let mut merged = Vec::new();
    for value in base {
        if ours_members.contains(value.as_str())
            && theirs_members.contains(value.as_str())
            && emitted.insert(value.as_str())
        {
            merged.push(value.clone());
        }
    }
    for value in ours.iter().chain(theirs) {
        if !base_members.contains(value.as_str())
            && (ours_members.contains(value.as_str()) || theirs_members.contains(value.as_str()))
            && emitted.insert(value.as_str())
        {
            merged.push(value.clone());
        }
    }
    merged
}

const CURRENT_CYCLE_FIELDS: [&str; 10] = [
    "objective",
    "intent_scope",
    "modules",
    "stage",
    "native_change",
    "next_action",
    "run_consents",
    "blockers",
    "awaiting_human",
    "history_cursor",
];

fn current_cycle(session: &Checkpoint) -> Value {
    json!({
        "history_cursor": session.history_cursor,
        "objective": session.objective,
        "intent_scope": session.intent_scope,
        "modules": session.modules,
        "stage": session.stage,
        "native_change": session.native_change,
        "next_action": session.next_action,
        "run_consents": session.run_consents,
        "blockers": session.blockers,
        "awaiting_human": session.awaiting_human,
    })
}

fn merge_checkpoints(operands: &[PathBuf]) -> Result<()> {
    ensure!(
        operands.len() == 3,
        "checkpoint --merge needs base, ours and theirs"
    );
    let (base_path, base) = read_merge_operand(&operands[0])?;
    let (ours_path, ours) = read_merge_operand(&operands[1])?;
    let (theirs_path, theirs) = read_merge_operand(&operands[2])?;
    let mut distinct = HashSet::new();
    ensure!(
        distinct.insert(&base_path) && distinct.insert(&ours_path) && distinct.insert(&theirs_path),
        "merge operands must resolve to three distinct paths"
    );

    let base_cycle = current_cycle(&base);
    let ours_cycle = current_cycle(&ours);
    let theirs_cycle = current_cycle(&theirs);
    let ours_changed = ours_cycle != base_cycle;
    let theirs_changed = theirs_cycle != base_cycle;
    let selected = match (ours_changed, theirs_changed) {
        (false, false) => &base,
        (true, false) => &ours,
        (false, true) => &theirs,
        (true, true) if ours_cycle == theirs_cycle => &ours,
        (true, true) => {
            let differing = CURRENT_CYCLE_FIELDS
                .iter()
                .filter(|field| ours_cycle[**field] != theirs_cycle[**field])
                .copied()
                .collect::<Vec<_>>()
                .join(", ");
            bail!("current-cycle conflict: {differing}");
        }
    };
    let mut merged = selected.clone();
    merged.references.criteria = merge_strings(
        &base.references.criteria,
        &ours.references.criteria,
        &theirs.references.criteria,
    );
    merged.references.requirements = merge_strings(
        &base.references.requirements,
        &ours.references.requirements,
        &theirs.references.requirements,
    );
    merged.references.cases = merge_strings(
        &base.references.cases,
        &ours.references.cases,
        &theirs.references.cases,
    );
    merged.references.design = merge_strings(
        &base.references.design,
        &ours.references.design,
        &theirs.references.design,
    );
    merged.references.files = merge_strings(
        &base.references.files,
        &ours.references.files,
        &theirs.references.files,
    );
    merged.decisions = merge_strings(&base.decisions, &ours.decisions, &theirs.decisions);
    merged.open_questions = merge_strings(
        &base.open_questions,
        &ours.open_questions,
        &theirs.open_questions,
    );
    merged.completed = merge_strings(&base.completed, &ours.completed, &theirs.completed);
    merged.observations =
        merge_strings(&base.observations, &ours.observations, &theirs.observations);
    merged.bugs = merge_strings(&base.bugs, &ours.bugs, &theirs.bugs);

    validate_v2(&merged)?;
    let serialized = serde_json::to_string_pretty(&merged)? + "\n";
    ensure!(
        serialized.len() as u64 <= SESSION_LIMIT,
        "merged session exceeds {SESSION_LIMIT} bytes"
    );
    match parse(&serialized)? {
        Session::V2(_) => {}
        Session::V1(_) => bail!("merged session is not schema 2"),
    }
    let report = serde_json::to_string(&json!({
        "schema_version": 1,
        "merged": true,
        "checkpoint": merged
    }))?;
    replace_merged_ours(&operands[1], &serialized)?;
    let mut output = std::io::stdout().lock();
    let _ = writeln!(output, "{report}");
    Ok(())
}

fn replace_merged_ours(ours: &Path, content: &str) -> Result<()> {
    let metadata = fs::symlink_metadata(ours)?;
    ensure!(
        metadata.file_type().is_file(),
        "merge output must remain a regular non-symlink file: {}",
        ours.display()
    );
    let parent = ours.parent().context("merge output has no parent")?;
    let name = ours
        .file_name()
        .and_then(|name| name.to_str())
        .context("merge output name must be UTF-8")?;
    let temporary = parent.join(format!(".{name}.{}.chough-merge.tmp", std::process::id()));
    let result = (|| -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
        fs::rename(&temporary, ours)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result.with_context(|| format!("writing merged checkpoint {}", ours.display()))
}

#[derive(clap::Args)]
pub(super) struct CheckpointArgs {
    /// Three-way merge schema 2 checkpoints, replacing only ours after every check passes.
    #[arg(
        long,
        num_args = 3,
        value_names = ["BASE", "OURS", "THEIRS"],
        conflicts_with_all = [
            "objective",
            "next_action",
            "native_change",
            "decisions",
            "questions",
            "resolved",
            "completed",
            "remove_completed",
            "blockers",
            "clear_blockers",
            "awaiting_human",
            "clear_awaiting_human",
            "close_gate",
            "decided_by",
            "words",
            "place",
            "recorded_by",
            "criteria",
            "requirements",
            "cases",
            "design",
            "files",
            "observations",
            "bugs",
        ]
    )]
    merge: Option<Vec<PathBuf>>,
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
    let decision = format!(
        "{} decided \"{gate}\": \"{}\" ({}); recorded by {}; a relayed claim, not authenticated",
        relay.decided_by, relay.words, relay.place, relay.recorded_by
    );
    if decision.len() > stage::RELAY_TEXT_LIMIT {
        return Err(RelayProblem::TooLong(gate.into(), decision.len()));
    }
    Ok(Some((gate.into(), relay, decision)))
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

/// Save `session` with a gate-closed event for each of `cleared` and a gate-opened
/// event for each of `opened`, sizing the cursor first so a refused save publishes no
/// event. `checkpoint` and the doctor's gate withdrawal (REQ-history-012) share it.
fn save_with_gate_events(
    root: &Path,
    session: &mut Checkpoint,
    current_stage: &str,
    cleared: &[String],
    opened: &[String],
    relay_for: impl Fn(&String) -> Option<stage::RelayedBy>,
) -> Result<()> {
    let mut candidate = session.clone();
    if (!cleared.is_empty() || !opened.is_empty())
        && let Some(cursor) = candidate.history_cursor.as_mut()
        && cursor.head.is_none()
    {
        // Every gate event keeps the cycle and leaves a fixed-width head.
        // Size the final cursor before publishing any events; this ID is
        // only a serialization placeholder and is never allocated or saved.
        cursor.head = Some("EVT-000000000000000000000000".into());
    }
    serialize_checkpoint(&candidate)?;
    if session.history_cursor.is_some() {
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
        save_checkpoint(root, session)?;
    } else {
        save_checkpoint(root, session)?;
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
    if let Some(operands) = args.merge {
        return merge_checkpoints(&operands);
    }
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
        };
        stage::validate_relay(&probe)
            .context("the summary digest and list are not a valid pair; nothing was saved")?;
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
        save_with_gate_events(
            root,
            &mut session,
            &current_stage,
            &cleared,
            &opened,
            relay_for,
        )?;
    }
    let mut output = json!({"schema_version": 1, "saved": changed, "checkpoint": session});
    if let Some((gate, relay, _)) = &relayed {
        output["relayed_decision"] = json!({
            "gate": gate,
            "relayed_by": relay,
            "provenance": RELAY_PROVENANCE,
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
        assert!(error.contains(".chough/sessions/alpha.json"), "{error}");
        assert!(!error.contains(".chough/session.json"), "{error}");
        assert!(error.contains("nothing was saved"), "{error}");
        drop(other);
    }
}
