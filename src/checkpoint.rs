//! The single canonical checkpoint: `.chough/session.json`.

use crate::records::{
    CHANGE_ID_LIMIT, RECORD_LIMIT, ensure_text, local_path, print_json, read_record,
    valid_change_id, valid_name, write_new_bytes, write_replace,
};
use crate::stage;
use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

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
        ensure!(
            valid_change_id(change),
            "invalid native change ID: {change}"
        );
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
    ensure!(
        session.run_consents.len() <= MAX_ITEMS,
        "too many run consents"
    );
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

fn read_merge_operand(path: &Path) -> Result<(PathBuf, Checkpoint)> {
    let metadata = fs::symlink_metadata(path)
        .with_context(|| format!("reading merge operand {}", path.display()))?;
    ensure!(
        metadata.file_type().is_file(),
        "merge operand must be an existing regular file whose final component is not a symbolic link: {}",
        path.display()
    );
    let resolved = path.canonicalize()?;
    let content = read_record(path)?;
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

const CURRENT_CYCLE_FIELDS: [&str; 9] = [
    "objective",
    "intent_scope",
    "modules",
    "stage",
    "native_change",
    "next_action",
    "run_consents",
    "blockers",
    "awaiting_human",
];

fn current_cycle(session: &Checkpoint) -> Value {
    json!({
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
        serialized.len() as u64 <= RECORD_LIMIT,
        "merged session exceeds 65536 bytes"
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
            "blockers",
            "clear_blockers",
            "awaiting_human",
            "clear_awaiting_human",
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
    if let Some(operands) = args.merge {
        return merge_checkpoints(&operands);
    }
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
        ensure!(
            valid_change_id(&change),
            "invalid native change ID: up to {CHANGE_ID_LIMIT} bytes of letters, digits, - and _"
        );
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
    } else {
        vec![]
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
        save_checkpoint(root, &session)?;
        // Gate records follow a successful save, so a failed save writes none: closed
        // first (clears run before additions), then each gate that was not open.
        let stage = session.stage.current.as_str();
        for gate in &cleared {
            stage::log_gate(root, stage::GATE_CLOSED, stage, gate)?;
        }
        for gate in &session.awaiting_human {
            if !open_before_additions.contains(gate) {
                stage::log_gate(root, stage::GATE_OPENED, stage, gate)?;
            }
        }
    }
    print_json(&json!({"schema_version": 1, "saved": changed, "checkpoint": session}))
}
