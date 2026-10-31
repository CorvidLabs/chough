mod bugs;
mod cases;
mod checkpoint;
mod context;
mod evidence;
mod library;
mod prefs;
mod records;
mod skill;
mod stage;
mod validate;

use anyhow::{Context, Result, ensure};
use checkpoint::{SESSION, Session, new_checkpoint, read_session};
use clap::{Parser, Subcommand};
use records::{local_path, native_json, print_json, valid_name, write_new};
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
    /// Report the checkpoint, observations and native SpecSync status without claiming approval.
    Status {
        #[arg(long)]
        environment: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// Show the next valid action from the checkpoint beside native next-action authority.
    Next {
        #[arg(long)]
        environment: Option<String>,
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
        Commands::Status { environment, .. } | Commands::Next { environment, .. } => {
            print_json(&status(&root, environment.as_deref())?)?;
        }
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

/// Native SpecSync status: a bounded section for display and, when available, the
/// summary parsed from the complete response.
fn native_status(root: &Path, change: Option<&str>) -> Result<(records::Section, Value)> {
    if !local_path(root, ".specsync/config.toml")?.is_file()
        && !local_path(root, ".specsync/config.json")?.is_file()
    {
        return Ok((
            records::Section {
                source: "specsync change status --json".into(),
                state: "not-configured".into(),
                content: "No native SpecSync configuration found; lifecycle state is unknown."
                    .into(),
            },
            Value::Null,
        ));
    }
    let response = match change {
        Some(change) => native_json(root, "specsync", &["change", "status", change, "--json"]),
        None => native_json(root, "specsync", &["change", "status", "--json"]),
    };
    let summary = response
        .value
        .as_ref()
        .and_then(|value| value.get("summary"))
        .map_or(Value::Null, |summary| {
            json!({"state": summary.get("state"), "next_action": summary.get("next_action")})
        });
    Ok((response.section, summary))
}

fn status(root: &Path, environment: Option<&str>) -> Result<Value> {
    let session = read_session(root)?;
    let current = evidence::snapshot(root);
    let observations = evidence::observations(root, &current, environment)?;
    let observation_references = evidence::reference_labels(root, &current, environment)?;
    let checkpoint = match &session {
        Session::V2(checkpoint) => Some(checkpoint.as_ref()),
        Session::V1(_) => None,
    };
    let (native_status, native_summary) =
        native_status(root, checkpoint.and_then(|c| c.native_change.as_deref()))?;
    let open_bugs = bugs::open_backlog(root)?;
    let blocking = bugs::open_blocking(root)?;
    let (stage_report, next_valid_action) = match checkpoint {
        None => (
            json!({"schema": 1, "migration": "Run `chough migrate` to adopt the schema 2 checkpoint; the original bytes are kept."}),
            "Run `chough migrate`, then `chough next` again.".to_string(),
        ),
        Some(checkpoint) => {
            let current_stage = checkpoint.stage.current.as_str();
            let gate = match current_stage {
                "plan" => Some(("build", stage::build_gate(root, checkpoint)?)),
                "accept" => Some(("done", stage::accept_gate(root, checkpoint)?)),
                _ => None,
            };
            let action = if checkpoint.stage.status == "paused" {
                format!(
                    "The {current_stage} stage is paused. Check native state and freshness, then `chough stage resume`."
                )
            } else if !checkpoint.awaiting_human.is_empty() {
                format!(
                    "Waiting for a human decision: {}. Do not record it on their behalf; continue only independent work within scope.",
                    checkpoint.awaiting_human.join("; ")
                )
            } else if !blocking.is_empty() && matches!(current_stage, "test" | "accept") {
                format!(
                    "Acceptance-blocking bugs are open ({}); go back to build and retest the affected cases.",
                    blocking.join(", ")
                )
            } else if let Some((_, gate)) = gate.as_ref().filter(|(_, gate)| !gate.allowed) {
                format!("Gate not open ({}): {}", gate.label, gate.detail)
            } else {
                format!("Continue {current_stage}: {}", checkpoint.next_action)
            };
            (
                json!({
                    "schema": 2,
                    "stage": checkpoint.stage,
                    "native_change": checkpoint.native_change,
                    "gate": gate.map(|(target, gate)| json!({"to": target, "allowed": gate.allowed, "label": gate.label, "detail": gate.detail})),
                    "valid_transitions": stage::valid_transitions(checkpoint),
                    "awaiting_human": checkpoint.awaiting_human,
                    "blockers": checkpoint.blockers,
                    "references": checkpoint.references,
                }),
                action,
            )
        }
    };
    Ok(json!({
        "schema_version": 1,
        "objective": session.objective(),
        "session_next_action": session.next_action(),
        "next_valid_action": next_valid_action,
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
        "open_bugs": open_bugs,
        "blocking_bugs": blocking,
        "human_approval": "unknown; Chough does not authenticate human identity or approval",
        "delivery": "unknown; a recorded observation does not establish deployment or release",
    }))
}
