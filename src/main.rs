mod bugs;
mod cases;
mod checkpoint;
mod context;
mod evidence;
mod identity;
mod library;
mod prefs;
mod records;
mod skill;
mod stage;
mod timing;
mod validate;

use anyhow::{Context, Result, ensure};
use checkpoint::{SESSION, Session, new_checkpoint, read_session};
use clap::{Parser, Subcommand};
use records::{NativeJson, local_path, native_json, print_json, valid_name, write_new};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    version,
    about = "Checkpointed stages, focused context and honest evidence; native tools own lifecycle state"
)]
struct Cli {
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install the workflow skill and a schema 2 checkpoint without overwriting existing files.
    Init {
        #[arg(
            long,
            default_value = "Describe the current objective before beginning work."
        )]
        objective: String,
        #[arg(long, default_value = "")]
        intent_scope: String,
        #[arg(long = "module")]
        modules: Vec<String>,
        /// Agent host whose project skill directory receives the bundle (repeatable).
        #[arg(long = "host", value_parser = skill::host_names())]
        hosts: Vec<String>,
    },
    /// Assemble bounded, deterministic session context from native sources.
    Context {
        #[arg(long)]
        json: bool,
    },
    /// The full report, always printed as JSON: checkpoint, observations and native
    /// SpecSync status, without claiming approval.
    Status {
        #[arg(long)]
        environment: Option<String>,
        /// Accepted for compatibility; status always prints JSON.
        #[arg(long)]
        json: bool,
    },
    /// The focused next action within 16384 bytes, from one native status read; text by
    /// default. `chough status` is the full view.
    Next {
        #[arg(long)]
        environment: Option<String>,
        /// Print the same fields as compact JSON (schema_version 1, view next).
        #[arg(long)]
        json: bool,
    },
    /// Explicitly migrate a schema 1 session, keeping its original bytes.
    Migrate {
        #[arg(long, default_value = "intent")]
        stage: String,
    },
    /// Show or update the canonical checkpoint.
    Checkpoint(Box<checkpoint::CheckpointArgs>),
    /// Route between stages within native lifecycle gates.
    Stage {
        #[command(subcommand)]
        command: stage::StageCommand,
    },
    /// Validate, select and run test-case definitions.
    Cases {
        #[command(subcommand)]
        command: cases::CasesCommand,
    },
    /// Record an observation; never executes a test or authenticates an actor.
    Evidence {
        #[command(subcommand)]
        command: EvidenceCommand,
    },
    /// Keep the local bug backlog.
    Bug {
        #[command(subcommand)]
        command: bugs::BugCommand,
    },
    /// Standing project preferences, each with its source.
    Pref {
        #[command(subcommand)]
        command: prefs::PrefCommand,
    },
    /// Run-only consent for the current objective.
    Consent {
        #[command(subcommand)]
        command: prefs::ConsentCommand,
    },
    /// Optional repeated validation of one case.
    Validate {
        #[command(subcommand)]
        command: validate::ValidateCommand,
    },
    /// Optional project-local librarian.
    Library {
        #[command(subcommand)]
        command: library::LibraryCommand,
    },
    /// Report how long stages, tests, bug fixes and human gates took, from existing
    /// records; read-only, and every duration is elapsed wall-clock time, not effort.
    Timing(Box<timing::TimingArgs>),
    /// Inspect or update installed copies of the workflow skill.
    Skill {
        #[command(subcommand)]
        command: skill::SkillCommand,
    },
}

#[derive(Subcommand)]
enum EvidenceCommand {
    Add(Box<evidence::EvidenceArgs>),
    /// Preview which observation sets retention could remove and what protects the rest;
    /// `--apply` removes eligible sets under a run consent or `retention auto`.
    Prune {
        /// Remove eligible sets and append each to .chough/pruned.jsonl.
        #[arg(long)]
        apply: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = cli.root.canonicalize().context("project root must exist")?;
    ensure!(root.is_dir(), "project root must be a directory");
    match cli.command {
        Commands::Init {
            objective,
            intent_scope,
            modules,
            hosts,
        } => init(&root, objective, intent_scope, modules, hosts)?,
        Commands::Context { json: as_json } => {
            let session = read_session(&root)?;
            let sections = context::context(&root, &session)?;
            let session_value = session.to_value()?;
            if as_json {
                print_json(
                    &json!({"schema_version": 1, "session": session_value, "sections": sections}),
                )?;
            } else {
                println!(
                    "# Chough context\n\n## Session\n\n{}",
                    serde_json::to_string_pretty(&session_value)?
                );
                for section in sections {
                    println!(
                        "\n## {} [{}]\n\n{}",
                        section.source, section.state, section.content
                    );
                }
            }
        }
        Commands::Status { environment, .. } => {
            print_json(&status(&root, environment.as_deref())?)?;
        }
        Commands::Next {
            environment,
            json: as_json,
        } => print!("{}", next(&root, environment.as_deref(), as_json)?),
        Commands::Migrate { stage } => checkpoint::migrate(&root, &stage)?,
        Commands::Checkpoint(args) => checkpoint::checkpoint_command(&root, *args)?,
        Commands::Stage { command } => stage::run(&root, command)?,
        Commands::Cases { command } => {
            if let Some(code) = cases::run(&root, command)? {
                std::process::exit(code);
            }
        }
        Commands::Evidence { command } => match command {
            EvidenceCommand::Add(args) => evidence::add(&root, *args)?,
            EvidenceCommand::Prune { apply } => evidence::prune(&root, apply)?,
        },
        Commands::Bug { command } => bugs::run(&root, command)?,
        Commands::Pref { command } => prefs::pref(&root, command)?,
        Commands::Consent { command } => prefs::consent(&root, command)?,
        Commands::Validate { command } => validate::run(&root, command)?,
        Commands::Library { command } => library::run(&root, command)?,
        Commands::Timing(args) => timing::run(&root, *args)?,
        Commands::Skill { command } => skill::run(&root, command)?,
    }
    Ok(())
}

fn init(
    root: &Path,
    objective: String,
    intent_scope: String,
    modules: Vec<String>,
    hosts: Vec<String>,
) -> Result<()> {
    ensure!(!objective.trim().is_empty(), "objective must not be blank");
    ensure!(
        modules.iter().all(|module| valid_name(module)),
        "module names use letters, digits, hyphens, underscores"
    );
    let hosts = if hosts.is_empty() {
        vec!["openai".to_string()]
    } else {
        hosts
    };
    let session = new_checkpoint(objective, intent_scope, modules);
    checkpoint::validate_v2(&session)?;
    let mut files = vec![(
        SESSION.to_string(),
        serde_json::to_string_pretty(&session)? + "\n",
    )];
    for (name, content) in skill::planned_files(&hosts)? {
        files.push((name, content.to_string()));
    }
    for (name, _) in &files {
        local_path(root, name)?;
    }
    let mut created = vec![];
    let mut preserved = vec![];
    for (name, content) in files {
        if write_new(root, &name, &content)? {
            created.push(name);
        } else {
            preserved.push(name);
        }
    }
    print_json(
        &json!({"schema_version": 1, "created": created, "preserved": preserved, "hosts": hosts}),
    )
}

/// The one native status read of a report, made whenever SpecSync is configured.
fn native_read(root: &Path, change: Option<&str>) -> Result<Option<NativeJson>> {
    if !stage::native_configured(root)? {
        return Ok(None);
    }
    Ok(Some(match change {
        Some(change) => native_json(root, "specsync", &["change", "status", change, "--json"]),
        None => native_json(root, "specsync", &["change", "status", "--json"]),
    }))
}

/// What `status` and `next` share: the session, the one native read, the gate for the
/// next main stage on that response and the next valid action (REQ-cli-021).
struct Assessment {
    session: Session,
    native: Option<NativeJson>,
    gate: Option<(String, stage::Gate)>,
    action: stage::Action,
    /// The approved-delta result (REQ-cli-022); None for a schema 1 session.
    deltas: Option<String>,
    open_bugs: Vec<Value>,
    blocking: Vec<String>,
}

fn assess(root: &Path) -> Result<Assessment> {
    let session = read_session(root)?;
    let checkpoint = match &session {
        Session::V2(checkpoint) => Some(checkpoint.as_ref()),
        Session::V1(_) => None,
    };
    let change = checkpoint.and_then(|checkpoint| checkpoint.native_change.as_deref());
    let native = native_read(root, change)?;
    let open_bugs = bugs::open_backlog(root)?;
    let blocking = bugs::open_blocking(root)?;
    let (gate, action, deltas) = match checkpoint {
        None => (
            None,
            stage::Action {
                action: "Run `chough migrate`, then `chough next` again.".into(),
                reason: "A schema 1 session works for context and status only; migration keeps its original bytes.".into(),
                command: Some("chough migrate".into()),
            },
            None,
        ),
        Some(checkpoint) => {
            let state = stage::native_state_from(root, change, native.as_ref())?;
            let (gate, action) = stage::assess(root, checkpoint, &state, &blocking);
            (gate, action, Some(stage::delta_summary(root, change, &state)))
        }
    };
    Ok(Assessment {
        session,
        native,
        gate,
        action,
        deltas,
        open_bugs,
        blocking,
    })
}

fn not_configured() -> records::Section {
    records::Section {
        source: "specsync change status --json".into(),
        state: "not-configured".into(),
        content: "No native SpecSync configuration found; lifecycle state is unknown.".into(),
    }
}

fn status(root: &Path, environment: Option<&str>) -> Result<Value> {
    let assessment = assess(root)?;
    let session = &assessment.session;
    let current = evidence::snapshot(root);
    let observations = evidence::observations(root, &current, environment)?;
    let observation_references = evidence::reference_labels(root, &current, environment)?;
    let stage_report = match session {
        Session::V1(_) => {
            json!({"schema": 1, "migration": "Run `chough migrate` to adopt the schema 2 checkpoint; the original bytes are kept."})
        }
        Session::V2(checkpoint) => json!({
            "schema": 2,
            "stage": checkpoint.stage,
            "native_change": checkpoint.native_change,
            "gate": assessment.gate.as_ref().map(|(target, gate)| json!({"to": target, "allowed": gate.allowed, "label": gate.label, "detail": gate.detail})),
            "valid_transitions": stage::valid_transitions(checkpoint),
            "awaiting_human": checkpoint.awaiting_human,
            "blockers": checkpoint.blockers,
            "references": checkpoint.references,
        }),
    };
    let (native_status, native_summary) = match &assessment.native {
        None => (serde_json::to_value(not_configured())?, Value::Null),
        Some(native) => (
            serde_json::to_value(&native.section)?,
            native
                .value
                .as_ref()
                .and_then(|value| value.get("summary"))
                .map_or(Value::Null, |summary| {
                    json!({"state": summary.get("state"), "next_action": summary.get("next_action")})
                }),
        ),
    };
    Ok(json!({
        "schema_version": 1,
        "objective": session.objective(),
        "session_next_action": session.next_action(),
        "next_valid_action": assessment.action.action,
        "checkpoint": stage_report,
        "native_next_action_authority": "Read native_specsync output and follow its actual next action before changing lifecycle state.",
        "native_specsync": native_status,
        "native_summary": native_summary,
        "snapshot": match &current { Ok(value) => serde_json::to_value(value)?, Err(error) => json!({"unknown": error.to_string()}) },
        "evidence_state": if observations.is_empty() { "missing" } else { "observations-present-not-acceptance" },
        "observations": observations,
        "observation_references": observation_references,
        "observation_label_note": "current: present with matching inputs; historical: present but stale; pruned: removed by retention (see .chough/pruned.jsonl); unavailable: missing and not in the ledger. Pruned and unavailable observations never count as current, passed or verifying evidence.",
        "environment": environment,
        "open_bugs": assessment.open_bugs,
        "blocking_bugs": assessment.blocking,
        "human_approval": "unknown; Chough does not authenticate human identity or approval",
        "delivery": "unknown; a recorded observation does not establish deployment or release",
    }))
}

/// Each form of the next view is at most this many bytes, the context bound.
const NEXT_LIMIT: usize = records::LIMIT;
/// Entries shown per list, and bytes kept of each string taken from a record.
const LIST_LIMIT: usize = 10;
const STRING_LIMIT: usize = 512;
const CUT: &str = " [cut]";
/// The lists the next view may shorten, each with its total kept.
const LISTS: [&str; 4] = [
    "awaiting_human",
    "blockers",
    "blocking_bugs",
    "observations",
];

/// The bytes `character` takes inside a JSON string, as serde_json escapes it.
fn escaped_width(character: char) -> usize {
    match character {
        '"' | '\\' | '\u{8}' | '\u{c}' | '\n' | '\r' | '\t' => 2,
        character if character < ' ' => 6,
        character => character.len_utf8(),
    }
}

/// Cut text at a character boundary so that it takes at most STRING_LIMIT bytes both
/// raw and escaped in JSON, marking the cut. A control character escapes to six bytes,
/// so the escaped width, not the raw length, bounds the JSON form.
fn cut(text: &str) -> String {
    if text.chars().map(escaped_width).sum::<usize>() <= STRING_LIMIT {
        return text.to_string();
    }
    let mut width = 0;
    let mut end = 0;
    for (index, character) in text.char_indices() {
        width += escaped_width(character);
        if width > STRING_LIMIT - CUT.len() {
            break;
        }
        end = index + character.len_utf8();
    }
    format!("{}{CUT}", &text[..end])
}

fn listed<'a>(items: impl ExactSizeIterator<Item = &'a String>) -> Value {
    let total = items.len();
    json!({"total": total, "items": items.take(LIST_LIMIT).map(|item| cut(item)).collect::<Vec<_>>()})
}

/// The focused next view (REQ-cli-021): text by default, compact JSON with `--json`.
fn next(root: &Path, environment: Option<&str>, as_json: bool) -> Result<String> {
    let assessment = assess(root)?;
    let session = &assessment.session;
    let checkpoint = match session {
        Session::V2(checkpoint) => Some(checkpoint.as_ref()),
        Session::V1(_) => None,
    };
    let change = checkpoint.and_then(|checkpoint| checkpoint.native_change.clone());
    let summary = assessment
        .native
        .as_ref()
        .and_then(|native| native.value.as_ref())
        .and_then(|value| value.get("summary"));
    let field = |pointer: &str| summary.and_then(|summary| summary.pointer(pointer));
    let native = json!({
        "change": change,
        "read": assessment.native.as_ref().map_or("not-configured", |native| native.section.state.as_str()),
        "state": field("/state").and_then(Value::as_str).map(cut),
        "approval_valid": field("/approval_valid").and_then(Value::as_bool),
        "correction_valid": field("/correction_valid").and_then(Value::as_bool),
        "terminal_evidence": field("/terminal_evidence/validity").and_then(Value::as_str).map(cut),
        "next_action": field("/next_action").and_then(Value::as_str).map(cut),
        "approved_deltas": assessment.deltas.as_deref().map(cut),
    });

    let current = evidence::snapshot(root);
    let references = evidence::reference_labels(root, &current, environment)?;
    let referenced: std::collections::BTreeSet<&str> = references
        .iter()
        .filter_map(|entry| entry["id"].as_str())
        .collect();
    let unreferenced = records::record_names(root, ".chough/evidence")?
        .iter()
        .filter(|name| !referenced.contains(name.trim_end_matches(".json")))
        .count();
    let attention: Vec<&Value> = references
        .iter()
        .filter(|entry| entry["label"] != "current")
        .collect();
    let items: Vec<Value> = attention
        .iter()
        .take(LIST_LIMIT)
        .map(|entry| {
            let by = entry["referenced_by"].as_array().cloned().unwrap_or_default();
            json!({
                "id": entry["id"],
                "label": entry["label"],
                "referenced_by": by.iter().take(3).map(|source| cut(source.as_str().unwrap_or_default())).collect::<Vec<_>>(),
                "referenced_by_total": by.len(),
            })
        })
        .collect();

    let empty = vec![];
    let view = json!({
        "schema_version": 1,
        "view": "next",
        "objective": cut(session.objective()),
        "stage": checkpoint.map(|checkpoint| json!({
            "current": checkpoint.stage.current,
            "status": checkpoint.stage.status,
            "iteration": checkpoint.stage.iteration,
            "callers": checkpoint.stage.callers.iter().map(|caller| caller.stage.clone()).collect::<Vec<_>>(),
        })),
        "native": native,
        "next_valid_action": {
            "action": cut(&assessment.action.action),
            "reason": cut(&assessment.action.reason),
            "command": assessment.action.command.as_deref().map(cut),
        },
        "gate": assessment.gate.as_ref().map(|(to, gate)| json!({"to": to, "allowed": gate.allowed, "label": cut(&gate.label), "detail": cut(&gate.detail)})),
        "awaiting_human": listed(checkpoint.map_or(&empty, |checkpoint| &checkpoint.awaiting_human).iter()),
        "blockers": listed(checkpoint.map_or(&empty, |checkpoint| &checkpoint.blockers).iter()),
        "blocking_bugs": listed(assessment.blocking.iter()),
        "other_open_bugs": assessment.open_bugs.len().saturating_sub(assessment.blocking.len()),
        "observations": {
            "needs_attention_total": attention.len(),
            "items": items,
            "referenced_current": references.len() - attention.len(),
            "unreferenced": unreferenced,
        },
        "environment": environment.map(cut),
        "full_view": "chough status",
        "truncated": false,
    });
    Ok(if as_json {
        fit(view, |view| format!("{view}\n"))
    } else {
        fit(view, next_text)
    })
}

/// Render the view, dropping the last entry of the longest list until it fits.
fn fit(mut view: Value, render: impl Fn(&Value) -> String) -> String {
    loop {
        let output = render(&view);
        if output.len() <= NEXT_LIMIT {
            return output;
        }
        let longest = LISTS
            .iter()
            .filter_map(|key| {
                let items = view[*key]["items"].as_array()?;
                (!items.is_empty()).then(|| (Value::from(items.clone()).to_string().len(), *key))
            })
            .max();
        let Some((_, key)) = longest else {
            return output;
        };
        if let Some(items) = view[key]["items"].as_array_mut() {
            items.pop();
        }
        view["truncated"] = Value::Bool(true);
    }
}

/// The text form: the same fields as short lines.
fn next_text(view: &Value) -> String {
    let text = |value: &Value| match value {
        Value::String(text) => text.clone(),
        Value::Null => "unknown".into(),
        other => other.to_string(),
    };
    let mut lines = vec![format!("Objective: {}", text(&view["objective"]))];
    let stage = &view["stage"];
    if stage.is_null() {
        lines.push("Stage: none (schema 1 session)".into());
    } else {
        let callers: Vec<String> = stage["callers"]
            .as_array()
            .into_iter()
            .flatten()
            .map(text)
            .collect();
        let called = if callers.is_empty() {
            String::new()
        } else {
            format!(", called from {}", callers.join(" > "))
        };
        lines.push(format!(
            "Stage: {} ({}, iteration {}{called})",
            text(&stage["current"]),
            text(&stage["status"]),
            text(&stage["iteration"])
        ));
    }
    let native = &view["native"];
    if native["change"].is_null() {
        lines.push(format!(
            "Native change: none recorded (read {})",
            text(&native["read"])
        ));
    } else {
        lines.push(format!(
            "Native change: {}: {} (read {}; approval_valid {}, correction_valid {}, terminal evidence {})",
            text(&native["change"]),
            text(&native["state"]),
            text(&native["read"]),
            text(&native["approval_valid"]),
            text(&native["correction_valid"]),
            text(&native["terminal_evidence"])
        ));
    }
    if native["next_action"].is_string() {
        lines.push(format!(
            "  Native next action: {}",
            text(&native["next_action"])
        ));
    }
    if native["approved_deltas"].is_string() {
        lines.push(format!(
            "  Approved deltas: {}",
            text(&native["approved_deltas"])
        ));
    }
    let action = &view["next_valid_action"];
    lines.push(format!("Next: {}", text(&action["action"])));
    lines.push(format!("  Why: {}", text(&action["reason"])));
    if action["command"].is_string() {
        lines.push(format!("  Run: {}", text(&action["command"])));
    }
    let gate = &view["gate"];
    if gate.is_null() {
        lines.push("Gate: none for the next stage".into());
    } else {
        lines.push(format!(
            "Gate to {}: {}",
            text(&gate["to"]),
            text(&gate["label"])
        ));
        lines.push(format!("  {}", text(&gate["detail"])));
    }
    for (key, title) in [("awaiting_human", "Human gates"), ("blockers", "Blockers")] {
        let total = view[key]["total"].as_u64().unwrap_or(0);
        let items = view[key]["items"].as_array().cloned().unwrap_or_default();
        if total == 0 {
            lines.push(format!("{title}: none"));
            continue;
        }
        lines.push(format!("{title}: {total} open"));
        lines.extend(items.iter().map(|item| format!("  - {}", text(item))));
        if total > items.len() as u64 {
            lines.push(format!(
                "  {} more; chough status",
                total - items.len() as u64
            ));
        }
    }
    let blocking = &view["blocking_bugs"];
    let ids: Vec<String> = blocking["items"]
        .as_array()
        .into_iter()
        .flatten()
        .map(text)
        .collect();
    let total = blocking["total"].as_u64().unwrap_or(0);
    let more = if total > ids.len() as u64 {
        ", ..."
    } else {
        ""
    };
    let other = text(&view["other_open_bugs"]);
    lines.push(if total == 0 {
        format!("Blocking bugs: none ({other} other open; chough bug list)")
    } else {
        format!(
            "Blocking bugs: {total} ({}{more}; {other} other open; chough bug list)",
            ids.join(", ")
        )
    });
    let observations = &view["observations"];
    let total = observations["needs_attention_total"].as_u64().unwrap_or(0);
    let items = observations["items"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    lines.push(if total == 0 {
        "Observations needing attention: none".into()
    } else {
        format!(
            "Observations needing attention: {total} referenced, not current ({} shown)",
            items.len()
        )
    });
    for item in &items {
        let by: Vec<String> = item["referenced_by"]
            .as_array()
            .into_iter()
            .flatten()
            .map(text)
            .collect();
        lines.push(format!(
            "  {} {} ({})",
            text(&item["id"]),
            text(&item["label"]),
            by.join(", ")
        ));
    }
    let more = total.saturating_sub(items.len() as u64);
    lines.push(format!(
        "  {}{} referenced current, {} unreferenced.",
        if more > 0 {
            format!("{more} more; ")
        } else {
            String::new()
        },
        text(&observations["referenced_current"]),
        text(&observations["unreferenced"])
    ));
    if view["environment"].is_string() {
        lines.push(format!("Environment: {}", text(&view["environment"])));
    }
    lines.push("Full view: chough status".into());
    if view["truncated"] == true {
        lines.push(format!(
            "Output truncated at {NEXT_LIMIT} bytes; run chough status."
        ));
    }
    lines.join("\n") + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every cut string fits STRING_LIMIT both raw and escaped in JSON, so the next view
    /// stays within its cap whatever characters a record holds.
    #[test]
    fn cut_bounds_the_escaped_width() {
        for text in [
            "\u{1}".repeat(600),
            "\"".repeat(600),
            "\\".repeat(600),
            "\n".repeat(600),
            "é".repeat(600),
            "x".repeat(600),
            format!("{}{}", "x".repeat(500), "\u{1}".repeat(20)),
        ] {
            let kept = cut(&text);
            let escaped = Value::from(kept.clone()).to_string().len() - 2;
            assert!(escaped <= STRING_LIMIT, "{escaped} escaped bytes");
            assert!(kept.ends_with(CUT), "{kept:?}");
        }
        assert_eq!(cut("short \u{1}"), "short \u{1}");
    }
}
