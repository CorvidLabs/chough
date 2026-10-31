//! The single canonical checkpoint: `.chough/session.json`.

use crate::records::{
    RECORD_LIMIT, ensure_text, local_path, print_json, read_record, valid_name, write_new_bytes,
    write_replace,
};
use crate::stage;
use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::Path;

pub(super) const SESSION: &str = ".chough/session.json";
pub(super) const SESSION_V1_BACKUP: &str = ".chough/session.v1.json";
const MAX_CALLERS: usize = 4;
const MAX_ITEMS: usize = 200;

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
        modules.iter().all(|module| valid_name(module)),
        "invalid session module name"
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
        serde_json::to_vec(session)?.len() as u64 <= RECORD_LIMIT,
        "session exceeds 65536 bytes"
    );
    Ok(())
}

fn ensure_ids(label: &str, values: &[String]) -> Result<()> {
    ensure!(values.len() <= MAX_ITEMS, "too many {label}");
    for value in values {
        ensure!(valid_name(value), "invalid {label} ID: {value}");
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
        ensure!(valid_name(change), "invalid native change ID: {change}");
    }
    ensure_ids("criterion", &session.references.criteria)?;
    ensure_ids("requirement", &session.references.requirements)?;
    ensure_ids("case", &session.references.cases)?;
    ensure_ids("observation", &session.observations)?;
    ensure_ids("bug", &session.bugs)?;
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
    for consent in &session.run_consents {
        ensure!(valid_name(&consent.key), "invalid consent key");
    }
    ensure!(
        serde_json::to_vec(session)?.len() as u64 <= RECORD_LIMIT,
        "session exceeds 65536 bytes"
    );
    Ok(())
}

fn parse(content: &str) -> Result<Session> {
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
    let path = local_path(root, SESSION)?;
    parse(&read_record(&path).context("run chough init first")?)
}

/// Read a schema 2 checkpoint for a command that changes workflow state.
pub(super) fn read_checkpoint(root: &Path) -> Result<Checkpoint> {
    match read_session(root)? {
        Session::V2(session) => Ok(*session),
        Session::V1(_) => bail!(
            "the session uses schema 1; run `chough migrate` to keep its bytes in {SESSION_V1_BACKUP} and continue with schema 2"
        ),
    }
}

pub(super) fn save_checkpoint(root: &Path, session: &Checkpoint) -> Result<()> {
    validate_v2(session)?;
    write_replace(
        root,
        SESSION,
        &(serde_json::to_string_pretty(session)? + "\n"),
    )
}

pub(super) fn migrate(root: &Path, initial_stage: &str) -> Result<()> {
    ensure!(
        stage::is_main(initial_stage),
        "migration stage must be a main stage"
    );
    let path = local_path(root, SESSION)?;
    let original = read_record(&path).context("run chough init first")?;
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
    #[arg(long = "blocker")]
    blockers: Vec<String>,
    #[arg(long)]
    clear_blockers: bool,
    /// A real gate still waiting for a human decision.
    #[arg(long = "awaiting-human")]
    awaiting_human: Vec<String>,
    #[arg(long)]
    clear_awaiting_human: bool,
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

pub(super) fn checkpoint_command(root: &Path, args: CheckpointArgs) -> Result<()> {
    let mut session = read_checkpoint(root)?;
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
        ensure!(valid_name(&change), "invalid native change ID");
        session.native_change = Some(change);
        changed = true;
    }
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
    if args.clear_blockers {
        session.blockers.clear();
        changed = true;
    }
    if args.clear_awaiting_human {
        session.awaiting_human.clear();
        changed = true;
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
        save_checkpoint(root, &session)?;
    }
    print_json(&json!({"schema_version": 1, "saved": changed, "checkpoint": session}))
}
