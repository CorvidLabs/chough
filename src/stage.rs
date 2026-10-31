//! Stage routing. Native SpecSync status decides gate eligibility; this module only
//! records transitions and refuses those that native state does not allow.

use crate::checkpoint::{Caller, Checkpoint, read_checkpoint, save_checkpoint};
use crate::records::{
    append_line, ensure_text, head_revision, local_path, native_json, now, print_json,
    read_log_tail, read_record,
};
use crate::{bugs, prefs};
use anyhow::{Result, bail, ensure};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::Path;

pub(super) const TRANSITIONS: &str = ".chough/transitions.jsonl";

/// Every stage with its kind. `references/stages.md` documents the same names.
pub(super) const STAGES: &[(&str, &str)] = &[
    ("intent", "main"),
    ("design", "conditional"),
    ("plan", "main"),
    ("build", "main"),
    ("test", "main"),
    ("accept", "main"),
    ("done", "terminal"),
    ("release", "optional"),
    ("bugs", "side"),
    ("validate", "side"),
    ("librarian", "side"),
];
const ORDER: [&str; 7] = [
    "intent", "design", "plan", "build", "test", "accept", "done",
];
const BUILD_STATES: [&str; 3] = ["approved", "implementing", "verifying"];

pub(super) fn is_stage(name: &str) -> bool {
    STAGES.iter().any(|(stage, _)| *stage == name)
}

pub(super) fn is_main(name: &str) -> bool {
    ORDER.contains(&name)
}

pub(super) fn is_side(name: &str) -> bool {
    STAGES
        .iter()
        .any(|(stage, kind)| *stage == name && *kind == "side")
}

fn position(name: &str) -> Option<usize> {
    ORDER.iter().position(|stage| *stage == name)
}

#[derive(Subcommand)]
pub(super) enum StageCommand {
    /// Show the current stage, recent transitions and the valid transitions.
    Show,
    /// Move to the next main stage, subject to native gates.
    Advance {
        #[arg(long)]
        note: Option<String>,
    },
    /// Record another iteration within the current stage.
    Iterate {
        #[arg(long)]
        note: String,
    },
    /// Skip the current stage with a recorded reason and move to the next one.
    Skip {
        #[arg(long)]
        reason: String,
    },
    /// Return to an earlier main stage; history is kept and refresh targets are named.
    Back {
        #[arg(long)]
        to: String,
        #[arg(long)]
        reason: String,
    },
    /// Start again from a main stage (intent by default); history is kept.
    Reset {
        #[arg(long, default_value = "intent")]
        to: String,
        #[arg(long)]
        reason: String,
    },
    /// Pause work in the current stage.
    Pause {
        #[arg(long)]
        reason: String,
    },
    /// Resume the paused stage with the same scope, preferences and gates.
    Resume,
    /// Enter a side stage (bugs, validate, librarian) or, when authorized, release.
    Enter {
        stage: String,
        #[arg(long)]
        reason: String,
    },
    /// Leave a side stage and return to its caller.
    Return {
        #[arg(long)]
        note: Option<String>,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Transition {
    schema_version: u32,
    at_unix_seconds: u64,
    kind: String,
    from: String,
    to: String,
    #[serde(default)]
    reason: Option<String>,
    #[serde(default)]
    gate: Option<String>,
    #[serde(default)]
    revision: Option<String>,
    #[serde(default)]
    refresh: Vec<String>,
}

/// What native lifecycle state allows, without inventing any of it.
pub(super) struct Gate {
    pub(super) allowed: bool,
    pub(super) label: String,
    pub(super) detail: String,
}

/// Native lifecycle state for the checkpoint's change.
pub(super) enum NativeState {
    NotConfigured,
    Disabled,
    NoChange,
    State(String, Value),
    Unknown(String),
}

fn sdd_enabled(root: &Path) -> Result<Option<bool>> {
    let path = local_path(root, ".specsync/sdd.json")?;
    if !path.is_file() {
        return Ok(None);
    }
    let value: Value = serde_json::from_str(&read_record(&path)?).unwrap_or(Value::Null);
    Ok(value.get("enabled").and_then(Value::as_bool))
}

pub(super) fn native_state(root: &Path, change: Option<&str>) -> Result<NativeState> {
    if !local_path(root, ".specsync/config.toml")?.is_file()
        && !local_path(root, ".specsync/config.json")?.is_file()
    {
        return Ok(NativeState::NotConfigured);
    }
    if sdd_enabled(root)? != Some(true) {
        return Ok(NativeState::Disabled);
    }
    let Some(change) = change else {
        return Ok(NativeState::NoChange);
    };
    let response = native_json(root, "specsync", &["change", "status", change, "--json"]);
    let Some(value) = response.value else {
        let section = response.section;
        return Ok(NativeState::Unknown(format!(
            "{} ({}): {}",
            section.source, section.state, section.content
        )));
    };
    match value.pointer("/summary/state").and_then(Value::as_str) {
        Some(state) => Ok(NativeState::State(state.to_string(), value.clone())),
        None => Ok(NativeState::Unknown(format!(
            "{}: no summary.state in the response",
            response.section.source
        ))),
    }
}

fn status_command(change: Option<&str>) -> String {
    match change {
        Some(change) => format!("specsync change status {change}"),
        None => "specsync change status".into(),
    }
}

fn gate(root: &Path, session: &Checkpoint, accepted: &[&str], purpose: &str) -> Result<Gate> {
    let change = session.native_change.as_deref();
    Ok(match native_state(root, change)? {
        NativeState::NotConfigured => Gate {
            allowed: true,
            label: "ungated:not-configured".into(),
            detail: "No native SpecSync configuration; this transition is not gated by a native lifecycle.".into(),
        },
        NativeState::Disabled => Gate {
            allowed: true,
            label: "ungated:sdd-disabled".into(),
            detail: "Native SDD is disabled; this transition is not gated by a native lifecycle.".into(),
        },
        NativeState::NoChange => Gate {
            allowed: false,
            label: "refused:no-native-change".into(),
            detail: format!(
                "{purpose} needs the native change: record it with `chough checkpoint --native-change <id>`."
            ),
        },
        NativeState::State(state, _) if accepted.contains(&state.as_str()) => Gate {
            allowed: true,
            label: format!("native:{state}"),
            detail: format!("Native state is {state}."),
        },
        NativeState::State(state, value) => Gate {
            allowed: false,
            label: format!("refused:native-{state}"),
            detail: format!(
                "{purpose} needs native state {}; it is {state}. Native next action: {}. Run `{}`.",
                accepted.join(" or "),
                value
                    .pointer("/summary/next_action")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
                status_command(change)
            ),
        },
        NativeState::Unknown(reason) => Gate {
            allowed: false,
            label: "refused:native-unknown".into(),
            detail: format!(
                "{purpose} cannot be checked, so it is refused. {reason}. Run `{}`.",
                status_command(change)
            ),
        },
    })
}

pub(super) fn build_gate(root: &Path, session: &Checkpoint) -> Result<Gate> {
    gate(root, session, &BUILD_STATES, "Entering build")
}

pub(super) fn accept_gate(root: &Path, session: &Checkpoint) -> Result<Gate> {
    gate(root, session, &["archived"], "Leaving accept")
}

fn refresh_note(stage: &str) -> &'static str {
    match stage {
        "intent" => "intent: HI criteria and FRD; append new criteria, never rewrite approved ones",
        "design" => {
            "design: canonical prototype and companion; their human approval covers the old version"
        }
        "plan" => "plan: native definition; scope or contract changes need native reapproval",
        "build" => "build: implementation against the current definition",
        "test" => "test: rerun affected cases; earlier observations may be stale",
        "accept" => "accept: native verification and the human implementation review",
        _ => "done: closeout",
    }
}

fn log_transition(
    root: &Path,
    kind: &str,
    from: &str,
    to: &str,
    reason: Option<&str>,
    gate: Option<&str>,
    refresh: Vec<String>,
) -> Result<()> {
    let transition = Transition {
        schema_version: 1,
        at_unix_seconds: now()?,
        kind: kind.into(),
        from: from.into(),
        to: to.into(),
        reason: reason.map(str::to_string),
        gate: gate.map(str::to_string),
        revision: head_revision(root),
        refresh,
    };
    append_line(root, TRANSITIONS, &serde_json::to_string(&transition)?)
}

pub(super) fn recent_transitions(root: &Path, count: usize) -> Result<(Vec<Value>, bool)> {
    let (lines, partial) = read_log_tail(&local_path(root, TRANSITIONS)?, count)?;
    let mut values = vec![];
    for line in lines {
        let transition: Transition = serde_json::from_str(&line)
            .map_err(|error| anyhow::anyhow!("invalid transition record: {error}"))?;
        values.push(serde_json::to_value(transition)?);
    }
    Ok((values, partial))
}

/// The transitions a fresh agent may use now, as commands.
pub(super) fn valid_transitions(session: &Checkpoint) -> Vec<String> {
    let current = session.stage.current.as_str();
    if session.stage.status == "paused" {
        return vec!["chough stage resume".into()];
    }
    let mut commands = vec![];
    if is_side(current) {
        commands.push(format!(
            "chough stage return  (back to {})",
            session
                .stage
                .callers
                .last()
                .map_or("unknown", |caller| caller.stage.as_str())
        ));
    } else if current == "release" {
        commands.push("chough stage advance  (release -> done)".into());
    } else if let Some(index) = position(current) {
        if let Some(next) = ORDER.get(index + 1) {
            let gate = match (current, *next) {
                (_, "build") => "  [native gate: definition approved]",
                ("accept", _) => "  [native gate: change archived]",
                _ => "",
            };
            commands.push(format!("chough stage advance  ({current} -> {next}){gate}"));
            commands.push("chough stage skip --reason <why not applicable>".into());
        } else {
            commands.push(
                "chough stage enter release --reason <request>  [needs release consent or standing preference]"
                    .into(),
            );
        }
        commands.push("chough stage iterate --note <what changed>".into());
        if index > 0 {
            commands.push(format!(
                "chough stage back --to <{}> --reason <why>",
                ORDER[..index].join("|")
            ));
        }
    }
    commands.push("chough stage reset [--to <stage>] --reason <why>".into());
    commands.push("chough stage pause --reason <why>".into());
    if session.stage.callers.len() < 4 {
        commands.push(
            "chough stage enter <bugs|validate|librarian> --reason <why>  [validate needs consent or preference]"
                .into(),
        );
    }
    commands
}

fn report(root: &Path, session: &Checkpoint, applied: Option<Value>) -> Result<()> {
    let (transitions, partial) = recent_transitions(root, 10)?;
    print_json(&json!({
        "schema_version": 1,
        "applied": applied,
        "stage": session.stage,
        "native_change": session.native_change,
        "awaiting_human": session.awaiting_human,
        "valid_transitions": valid_transitions(session),
        "stages": STAGES.iter().map(|(name, kind)| json!({"name": name, "kind": kind})).collect::<Vec<_>>(),
        "recent_transitions": transitions,
        "transitions_partial": partial,
        "authority": "Native SpecSync status decides lifecycle gates; Chough never records human approval or review.",
    }))
}

fn ensure_active(session: &Checkpoint) -> Result<()> {
    ensure!(
        session.stage.status == "active",
        "the stage is paused; run `chough stage resume` first"
    );
    Ok(())
}

fn ensure_main(session: &Checkpoint, verb: &str) -> Result<()> {
    ensure!(
        !is_side(&session.stage.current),
        "cannot {verb} from side stage {}; run `chough stage return` first",
        session.stage.current
    );
    Ok(())
}

/// Apply a move to the next stage, checking the gates that apply to it.
fn move_forward(
    root: &Path,
    session: &mut Checkpoint,
    kind: &str,
    reason: Option<&str>,
) -> Result<Value> {
    ensure_active(session)?;
    ensure_main(session, kind)?;
    let from = session.stage.current.clone();
    let to = if from == "release" {
        "done".to_string()
    } else {
        let index = position(&from).ok_or_else(|| anyhow::anyhow!("unknown stage {from}"))?;
        match ORDER.get(index + 1) {
            Some(next) => (*next).to_string(),
            None => bail!(
                "done is terminal; release is entered only with `chough stage enter release` when authorized"
            ),
        }
    };
    let mut gate_label = None;
    if from == "test" && to == "accept" {
        let blocking = bugs::open_blocking(root)?;
        ensure!(
            blocking.is_empty(),
            "{kind} refused (refused:blocking-bugs): open acceptance-blocking bugs {}; go back to build, fix and retest",
            blocking.join(", ")
        );
    }
    for (applies, gate) in [
        (to == "build", "build"),
        (from == "accept" && to == "done", "accept"),
    ] {
        if !applies {
            continue;
        }
        let gate = if gate == "build" {
            build_gate(root, session)?
        } else {
            accept_gate(root, session)?
        };
        ensure!(
            gate.allowed,
            "{} refused ({}): {}",
            kind,
            gate.label,
            gate.detail
        );
        gate_label = Some(gate.label);
    }
    log_transition(
        root,
        kind,
        &from,
        &to,
        reason,
        gate_label.as_deref(),
        vec![],
    )?;
    session.stage.current = to.clone();
    session.stage.iteration = 1;
    Ok(json!({"kind": kind, "from": from, "to": to, "gate": gate_label}))
}

fn move_back(
    root: &Path,
    session: &mut Checkpoint,
    kind: &str,
    target: &str,
    reason: &str,
) -> Result<Value> {
    ensure_text("reason", reason)?;
    ensure!(is_main(target), "{target} is not a main stage");
    let from = session.stage.current.clone();
    let origin = session
        .stage
        .callers
        .first()
        .map_or(from.as_str(), |caller| caller.stage.as_str())
        .to_string();
    let origin_index = position(&origin).unwrap_or(ORDER.len() - 1);
    let target_index = position(target).unwrap_or(0);
    if kind == "back" {
        ensure_active(session)?;
        ensure_main(session, "go back")?;
        ensure!(
            target_index < origin_index,
            "back goes to an earlier stage than {from}"
        );
    }
    if target == "build" {
        let gate = build_gate(root, session)?;
        ensure!(
            gate.allowed,
            "{kind} refused ({}): {}",
            gate.label,
            gate.detail
        );
    }
    let mut refresh: Vec<String> = ORDER[target_index..=origin_index.max(target_index)]
        .iter()
        .filter(|stage| **stage != "done")
        .map(|stage| refresh_note(stage).to_string())
        .collect();
    if !session.references.cases.is_empty() || !session.observations.is_empty() {
        refresh.push(format!(
            "checkpoint references: {} case(s) and {} observation(s) to re-check with `chough status`",
            session.references.cases.len(),
            session.observations.len()
        ));
    }
    if target_index <= 2
        && let NativeState::State(state, _) = native_state(root, session.native_change.as_deref())?
        && state != "draft"
    {
        refresh.push(format!(
            "native: the definition is {state}; changing intent, contract, acceptance criteria or scope needs native reapproval. Follow `{}`; Chough does not change native state.",
            status_command(session.native_change.as_deref())
        ));
    }
    log_transition(
        root,
        kind,
        &from,
        target,
        Some(reason),
        None,
        refresh.clone(),
    )?;
    session.stage.current = target.into();
    session.stage.callers.clear();
    session.stage.status = "active".into();
    session.stage.iteration = 1;
    Ok(json!({"kind": kind, "from": from, "to": target, "refresh": refresh}))
}

pub(super) fn run(root: &Path, command: StageCommand) -> Result<()> {
    let mut session = read_checkpoint(root)?;
    let applied = match command {
        StageCommand::Show => None,
        StageCommand::Advance { note } => {
            if let Some(note) = &note {
                ensure_text("note", note)?;
            }
            Some(move_forward(
                root,
                &mut session,
                "advance",
                note.as_deref(),
            )?)
        }
        StageCommand::Skip { reason } => {
            ensure_text("reason", &reason)?;
            Some(move_forward(root, &mut session, "skip", Some(&reason))?)
        }
        StageCommand::Iterate { note } => {
            ensure_text("note", &note)?;
            ensure_active(&session)?;
            let stage = session.stage.current.clone();
            log_transition(root, "iterate", &stage, &stage, Some(&note), None, vec![])?;
            session.stage.iteration += 1;
            Some(json!({"kind": "iterate", "stage": stage, "iteration": session.stage.iteration}))
        }
        StageCommand::Back { to, reason } => {
            Some(move_back(root, &mut session, "back", &to, &reason)?)
        }
        StageCommand::Reset { to, reason } => {
            Some(move_back(root, &mut session, "reset", &to, &reason)?)
        }
        StageCommand::Pause { reason } => {
            ensure_text("reason", &reason)?;
            ensure_active(&session)?;
            let stage = session.stage.current.clone();
            log_transition(root, "pause", &stage, &stage, Some(&reason), None, vec![])?;
            session.stage.status = "paused".into();
            Some(json!({"kind": "pause", "stage": stage}))
        }
        StageCommand::Resume => {
            ensure!(session.stage.status == "paused", "the stage is not paused");
            let stage = session.stage.current.clone();
            log_transition(root, "resume", &stage, &stage, None, None, vec![])?;
            session.stage.status = "active".into();
            Some(json!({
                "kind": "resume",
                "stage": stage,
                "check": "Run `chough next --environment <env>` to re-check native state and evidence freshness before continuing."
            }))
        }
        StageCommand::Enter { stage, reason } => {
            ensure_text("reason", &reason)?;
            ensure_active(&session)?;
            let from = session.stage.current.clone();
            if stage == "release" {
                ensure!(
                    from == "done",
                    "release is entered from done, after native archive and merge"
                );
                let authority = prefs::authorized(root, &session, "release")?.ok_or_else(|| {
                    anyhow::anyhow!(
                        "release is not authorized: it needs a request from the human recorded with `chough consent grant release --source <their words>` or a standing `chough pref set release standing --source <...>`"
                    )
                })?;
                log_transition(
                    root,
                    "enter",
                    &from,
                    "release",
                    Some(&reason),
                    Some(&authority),
                    vec![],
                )?;
                session.stage.current = "release".into();
                session.stage.iteration = 1;
                Some(
                    json!({"kind": "enter", "from": from, "to": "release", "authority": authority}),
                )
            } else {
                ensure!(is_side(&stage), "{stage} is not a side stage");
                ensure!(from != stage, "already in {stage}");
                ensure!(
                    session.stage.callers.len() < 4,
                    "side stages nest at most 4 deep"
                );
                let mut authority = None;
                if stage == "validate" {
                    authority = prefs::authorized(root, &session, "repeat-validation")?
                        .or(prefs::authorized(root, &session, "monitoring")?);
                    ensure!(
                        authority.is_some(),
                        "validate is optional: it needs a run consent or a standing preference for repeat-validation or monitoring"
                    );
                }
                log_transition(
                    root,
                    "enter",
                    &from,
                    &stage,
                    Some(&reason),
                    authority.as_deref(),
                    vec![],
                )?;
                session.stage.callers.push(Caller {
                    stage: from.clone(),
                    reason,
                });
                session.stage.current = stage.clone();
                session.stage.iteration = 1;
                Some(json!({"kind": "enter", "from": from, "to": stage}))
            }
        }
        StageCommand::Return { note } => {
            if let Some(note) = &note {
                ensure_text("note", note)?;
            }
            ensure_active(&session)?;
            let caller = session.stage.callers.pop().ok_or_else(|| {
                anyhow::anyhow!(
                    "{} is not a side stage with a caller",
                    session.stage.current
                )
            })?;
            let from = session.stage.current.clone();
            log_transition(
                root,
                "return",
                &from,
                &caller.stage,
                note.as_deref(),
                None,
                vec![],
            )?;
            session.stage.current = caller.stage.clone();
            session.stage.iteration = 1;
            Some(json!({"kind": "return", "from": from, "to": caller.stage}))
        }
    };
    if applied.is_some() {
        save_checkpoint(root, &session)?;
    }
    report(root, &session, applied)
}
