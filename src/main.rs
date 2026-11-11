mod actor;
mod attachments;
mod bugs;
mod cases;
mod change;
mod checkpoint;
mod context;
mod direct_writes;
mod doctor;
mod evidence;
mod gates;
mod history;
mod humans;
mod identity;
mod journal;
mod leaks;
mod library;
mod markdown;
mod native;
mod native_check;
mod policy;
mod prefs;
mod records;
mod session_md;
mod sessions;
mod skill;
mod stage;
mod timing;
mod validate;
mod vault;

use anyhow::{Context, Result, ensure};
use checkpoint::{Session, read_session};
use clap::{Parser, Subcommand};
use records::valid_module_name;
use records::{NativeJson, local_path, native_json, print_json, write_new};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    version,
    about = "Checkpointed stages, focused context and honest evidence; native tools own lifecycle state"
)]
struct Cli {
    /// The project directory to work in.
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,
    /// Store `<redacted:<kind>>` in place of a local path, user or host name, email or
    /// credential in a committed record, instead of refusing the write (REQ-records-006).
    #[arg(long, global = true)]
    redact: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install the workflow skill and a schema 2 checkpoint without overwriting existing files.
    Init {
        /// The first session's objective; without it no session is created.
        #[arg(long)]
        objective: Option<String>,
        /// The first session's key; the objective's slug when omitted.
        #[arg(long = "session")]
        session: Option<String>,
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
    /// Correct an earlier Timeline entry by appending a correction linked to it; history
    /// is never rewritten (REQ-history-015).
    Correct(history::CorrectArgs),
    /// The active session's Timeline (REQ-history-002).
    History {
        #[command(subcommand)]
        command: history::HistoryCommand,
    },
    /// Validate, select and run test-case definitions.
    Cases {
        #[command(subcommand)]
        command: cases::CasesCommand,
    },
    /// Record an observation; never executes a test or authenticates an actor.
    Evidence {
        #[command(subcommand)]
        command: evidence::EvidenceCommand,
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
    /// The knowledge journal: short dated posts that link the records (REQ-records-011).
    Journal {
        #[command(subcommand)]
        command: journal::JournalCommand,
    },
    /// Report how long stages, tests, bug fixes and human gates took, from existing
    /// records; read-only, and every duration is elapsed wall-clock time, not effort.
    Timing(Box<timing::TimingArgs>),
    /// Inspect or update installed copies of the workflow skill.
    Skill {
        #[command(subcommand)]
        command: skill::SkillCommand,
    },
    /// Sessions per objective or native change: new, adopt, use, list.
    Session {
        #[command(subcommand)]
        command: sessions::SessionCommand,
    },
    /// Create-only inbox entries for subagents, folded into the active session.
    Inbox {
        #[command(subcommand)]
        command: sessions::InboxCommand,
    },
    /// Run SpecSync's change new, answer, check (with --commit) and status steps and pass
    /// their output, refusals and exit code through unchanged (REQ-cli-057).
    Change {
        #[command(subcommand)]
        command: change::ChangeCommand,
    },
    /// Number every open human gate with a one-screen summary and its digest, and record
    /// one relayed reply for listed gates only (REQ-cli-061).
    Gate {
        #[command(subcommand)]
        command: gates::GateCommand,
    },
    /// Find the Chough debt that stops a fresh worktree from entering Chough; `--fix`
    /// fixes it, and decision fixes also need `--reason` (DRIVE-17).
    Doctor(Box<doctor::DoctorArgs>),
}

fn main() -> Result<()> {
    // A stop then finds orphans of what Chough started (#81, Linux; no effect elsewhere).
    records::become_subreaper();
    let cli = Cli::parse();
    leaks::set_redacting(cli.redact);
    let root = cli.root.canonicalize().context("project root must exist")?;
    ensure!(root.is_dir(), "project root must be a directory");
    match cli.command {
        Commands::Init {
            objective,
            session,
            intent_scope,
            modules,
            hosts,
        } => init(&root, objective, session, intent_scope, modules, hosts)?,
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
            let report = status(&root, environment.as_deref())?;
            let phase = std::time::Instant::now();
            print_json(&report)?;
            records::trace_phase("status.render", phase);
        }
        Commands::Next {
            environment,
            json: as_json,
        } => print!("{}", next(&root, environment.as_deref(), as_json)?),
        Commands::Migrate { stage: _ } => anyhow::bail!(
            "`chough migrate` for 0.5.x projects is being rebuilt in this 0.6.0 branch (plan step 10); use a released Chough until then"
        ),
        Commands::Checkpoint(args) => checkpoint::checkpoint_command(&root, *args)?,
        Commands::Stage { command } => stage::run(&root, command)?,
        Commands::History { command } => history::run(&root, command)?,
        Commands::Correct(args) => history::correct(&root, args)?,
        Commands::Cases { command } => {
            if let Some(code) = cases::run(&root, command)? {
                std::process::exit(code);
            }
        }
        Commands::Evidence { command } => evidence::run(&root, command)?,
        Commands::Bug { command } => bugs::run(&root, command)?,
        Commands::Pref { command } => prefs::pref(&root, command)?,
        Commands::Consent { command } => prefs::consent(&root, command)?,
        Commands::Validate { command } => validate::run(&root, command)?,
        Commands::Library { command } => library::run(&root, command)?,
        Commands::Journal { command } => journal::run(&root, command)?,
        Commands::Timing(args) => timing::run(&root, *args)?,
        Commands::Skill { command } => skill::run(&root, command)?,
        Commands::Session { command } => sessions::run(&root, command)?,
        Commands::Inbox { command } => sessions::inbox(&root, command)?,
        Commands::Change { command } => {
            let code = change::run(&root, command)?;
            if code != 0 {
                std::process::exit(code);
            }
        }
        Commands::Gate { command } => gates::run(&root, command)?,
        Commands::Doctor(args) => {
            if doctor::run(&root, *args)? {
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

fn init(
    root: &Path,
    objective: Option<String>,
    session_key: Option<String>,
    intent_scope: String,
    modules: Vec<String>,
    hosts: Vec<String>,
) -> Result<()> {
    sessions::ensure_migrated(root)?;
    ensure!(
        modules.iter().all(|module| valid_module_name(module)),
        "invalid module name: use a module ID SpecSync accepts, such as cli or chat.service"
    );
    let hosts = if hosts.is_empty() {
        vec!["openai".to_string()]
    } else {
        hosts
    };
    let files = skill::planned_files(&hosts)?;
    for (name, _) in &files {
        local_path(root, name)?;
    }
    let mut created = vec![];
    let mut preserved = vec![];
    // REQ-cli (0.6.0): `--objective` creates the first session as `chough session new`
    // would, keyed by `--session` or the objective's slug; without it, none.
    if let Some(objective) = objective {
        ensure!(!objective.trim().is_empty(), "objective must not be blank");
        ensure!(
            sessions::open_ids(root)?.is_empty(),
            "the project already has an open session; run `chough session new <key>` for new work"
        );
        let key = session_key.unwrap_or_else(|| identity::slug(&objective, "session", 64));
        let id = sessions::create(root, &key, objective, intent_scope, modules)?;
        created.push(sessions::session_file(&id));
    }
    for (name, content) in files {
        if write_new(root, &name, content)? {
            created.push(name);
        } else {
            preserved.push(name);
        }
    }
    // REQ-cli-043: with the librarian on, the knowledge directory exists from the start.
    let library = library::create_at_init(root);
    print_json(&json!({
        "schema_version": 1,
        "created": created,
        "preserved": preserved,
        "hosts": hosts,
        "library": library,
    }))
}

/// The one native status read of a report, made whenever SpecSync is configured.
fn native_read(root: &Path, change: Option<&str>) -> Result<Option<NativeJson>> {
    if !stage::native_configured(root)? {
        return Ok(None);
    }
    Ok(Some(match change {
        Some(change) => native::report_status(root, change),
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
    history: Option<Value>,
    /// The approved-delta result (REQ-cli-022); None for a schema 1 session.
    deltas: Option<String>,
    open_bugs: Vec<Value>,
    blocking: Vec<String>,
}

fn assess(root: &Path) -> Result<Assessment> {
    let session = read_session(root)?;
    let checkpoint = match &session {
        Session::V2(checkpoint) => Some(checkpoint.as_ref()),
    };
    let change = checkpoint.and_then(|checkpoint| checkpoint.native_change.as_deref());
    let native = native_read(root, change)?;
    let open_bugs = bugs::open_backlog(root)?;
    let blocking = bugs::open_blocking(root)?;
    let history_read = checkpoint.map(|checkpoint| {
        history::selected(
            checkpoint,
            history::RECENT_EVENTS,
            history::RECENT_BYTES,
            true,
        )
    });
    let history = history_read.as_ref().map(|read| json!({"partial":read.partial,"events_read":read.records.len(),"bytes_read":read.bytes,"source":"the session timeline"}));
    let (gate, mut action, deltas) = match checkpoint {
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
            let (gate, action) = stage::assess(root, checkpoint, &state, &blocking, history_read.as_ref());
            (gate, action, Some(stage::delta_summary(root, change, &state)))
        }
    };
    if let Some(read) = &history_read
        && !read.partial.is_empty()
    {
        action.reason = format!(
            "History partial or unknown: {}. {}",
            read.partial.join("; "),
            action.reason
        );
    }
    Ok(Assessment {
        history,
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
    let phase = std::time::Instant::now();
    let assessment = assess(root)?;
    records::trace_phase("status.assess", phase);
    let session = &assessment.session;
    let current = evidence::snapshot(root);
    let reads = evidence::Reads::new(root);
    let phase = std::time::Instant::now();
    let observations = reads.observations(&current, environment)?;
    records::trace_phase("status.observations", phase);
    let phase = std::time::Instant::now();
    let observation_references = reads.reference_labels(&current, environment)?;
    records::trace_phase("status.references", phase);
    let stage_report = match session {
        Session::V2(checkpoint) => json!({
            "schema": 2,
            "stage": checkpoint.stage,
            "native_change": checkpoint.native_change,
            "gate": assessment.gate.as_ref().map(|(target, gate)| json!({"to": target, "allowed": gate.allowed, "label": gate.label, "detail": gate.detail})),
            "valid_transitions": stage::valid_transitions(checkpoint),
            "awaiting_human": checkpoint.awaiting_human,
            "gates": open_gates(checkpoint, usize::MAX),
            "blockers": checkpoint.blockers,
            "open_questions": checkpoint.open_questions,
            "references": checkpoint.references,
        }),
    };
    let verification_warning = policy::verification_warning(
        root,
        assessment
            .native
            .as_ref()
            .and_then(|native| native.value.as_ref())
            .and_then(|value| value.pointer("/summary/verification_commands")),
    )?;
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
    // REQ-history-015: the corrected text of a corrected next action or decision.
    let corrections = match session {
        Session::V2(checkpoint) => history::corrections(checkpoint),
    };
    Ok(json!({
        "schema_version": 1,
        "objective": session.objective(),
        "session_next_action": corrections.apply(session.next_action()),
        "next_valid_action": corrections.apply(&assessment.action.action),
        "corrections": corrections.view(),
        "decisions": match session {
            Session::V2(checkpoint) => checkpoint.decisions.iter().map(|decision| corrections.decision(decision)).collect::<Vec<_>>(),
        },
        "checkpoint": stage_report,
        "direct_writes": direct_writes::view(root, match session {
            Session::V2(checkpoint) => checkpoint.native_change.as_deref(),
        }),
        "transition_history": assessment.history,
        "last_actor": match session {
            Session::V2(checkpoint) => history::last_actor(checkpoint),
        },
        "native_next_action_authority": "Read native_specsync output and follow its actual next action before changing lifecycle state.",
        "native_specsync": native_status,
        "native_summary": native_summary,
        "native_verification_warning": verification_warning,
        "snapshot": match &current { Ok(value) => serde_json::to_value(value)?, Err(error) => json!({"unknown": error.to_string()}) },
        "evidence_state": if observations.is_empty() { "missing" } else { "observations-present-not-acceptance" },
        "observations": observations,
        "observation_references": observation_references,
        "observation_label_note": "current: present with matching inputs; historical: present but stale; pruned: removed by retention (see .chough/pruned.jsonl); unavailable: missing and not in the ledger. Pruned and unavailable observations never count as current, passed or verifying evidence.",
        "environment": environment,
        "open_bugs": assessment.open_bugs,
        "blocking_bugs": assessment.blocking,
        "removal_holds": match evidence::stale_holds(root) { Ok(holds) => json!(holds), Err(error) => json!({"unknown": error.to_string()}) },
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
/// Entries shown of the newest decisions, of a failure's linked bugs and of archived
/// lesson bundles; the bundles shown are the librarian's hot archives (KNOW-8).
const DECISION_LIMIT: usize = 5;
const LINKED_BUG_LIMIT: usize = 3;
const BUNDLE_LIMIT: usize = library::HOT_ARCHIVES;
/// Decision prefixes read best-effort; any other entry is shown whole.
const DECISION_KINDS: [&str; 3] = ["decided", "rejected", "failed"];
const ARCHIVED_CHANGES: &str = ".specsync/archive/changes";
/// The lists the next view may shorten, each with its total kept, as JSON pointers.
const LISTS: [&str; 9] = [
    "/awaiting_human/items",
    "/blockers/items",
    "/open_questions/items",
    "/blocking_bugs/items",
    "/observations/items",
    "/decisions/items",
    "/last_failure/bugs/items",
    "/lessons/sources/items",
    "/lessons/archived_bundles/items",
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

/// The first `limit` open gates with the stage each holds and the native step each
/// drives (REQ-cli-030).
fn open_gates(checkpoint: &checkpoint::Checkpoint, limit: usize) -> Vec<Value> {
    checkpoint
        .awaiting_human
        .iter()
        .take(limit)
        .map(|gate| {
            let meta = session_md::gate(checkpoint, gate);
            json!({"gate": cut(gate), "holds": meta.holds, "drives": meta.drives})
        })
        .collect()
}

fn listed<'a>(items: impl ExactSizeIterator<Item = &'a String>) -> Value {
    let total = items.len();
    json!({"total": total, "items": items.take(LIST_LIMIT).map(|item| cut(item)).collect::<Vec<_>>()})
}

/// One checkpoint decision split by the `decided:`, `rejected:` or `failed:` convention:
/// a prefixed entry's reason is the text after its last ` because `; any other entry is
/// shown whole with no kind and no reason.
fn decision(entry: &str) -> Value {
    let prefixed = DECISION_KINDS.iter().find_map(|kind| {
        entry
            .strip_prefix(kind)
            .and_then(|rest| rest.strip_prefix(':'))
            .map(|rest| (*kind, rest.trim_start()))
    });
    match prefixed {
        Some((kind, rest)) => match rest.rsplit_once(" because ") {
            Some((text, reason)) => json!({"kind": kind, "text": cut(text), "reason": cut(reason)}),
            None => json!({"kind": kind, "text": cut(rest), "reason": null}),
        },
        None => json!({"kind": null, "text": cut(entry), "reason": null}),
    }
}

/// The most recent decisions, newest first, with their total.
fn decisions(entries: &[String]) -> Value {
    json!({
        "total": entries.len(),
        "items": entries.iter().rev().take(DECISION_LIMIT).map(|entry| decision(entry)).collect::<Vec<_>>(),
    })
}

/// The failed observation with the greatest recorded time, ties to the greater ID, and
/// every bug that links it, by bug ID. Recorded fields only: never file times or order.
fn last_failure(root: &Path) -> Result<Value> {
    let latest = evidence::load_all(root)?
        .into_iter()
        .filter(|record| record.outcome == evidence::Outcome::Failed)
        .max_by(|left, right| {
            (left.observed_at_unix_seconds, &left.id)
                .cmp(&(right.observed_at_unix_seconds, &right.id))
        });
    let Some(latest) = latest else {
        return Ok(Value::Null);
    };
    let mut linked: Vec<(String, String)> = bugs::observation_references(root)?
        .into_iter()
        .filter(|(_, ids)| ids.contains(&latest.id))
        .filter_map(|(label, _)| {
            let (id, status) = label.split_once(' ')?;
            Some((id.to_string(), status.to_string()))
        })
        .collect();
    linked.sort();
    Ok(json!({
        "id": latest.id,
        "observed_at_unix_seconds": latest.observed_at_unix_seconds,
        "summary": cut(&latest.summary),
        "bugs": {
            "total": linked.len(),
            "items": linked.iter().take(LINKED_BUG_LIMIT).map(|(id, status)| json!({"id": id, "status": status})).collect::<Vec<_>>(),
        },
    }))
}

/// The 1-based line of the first line that is exactly `## Lessons`. A module's
/// `context.md` is a native spec file, so it is searched at any size (REQ-cli-034),
/// line by line without loading the whole file. Ok(None) for a missing file, Err with a
/// note when it cannot be read.
fn lessons_heading(path: &Path) -> std::result::Result<Option<Option<usize>>, String> {
    use std::io::BufRead;
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("cannot be read: {error}")),
    };
    let mut reader = std::io::BufReader::new(file);
    let mut line = Vec::new();
    let mut number = 0;
    loop {
        line.clear();
        match reader.read_until(b'\n', &mut line) {
            Ok(0) => return Ok(Some(None)),
            Ok(_) => {}
            Err(error) => return Err(format!("cannot be read: {error}")),
        }
        number += 1;
        let mut text = line.as_slice();
        if let Some(rest) = text.strip_suffix(b"\n") {
            text = rest;
        }
        while let Some(rest) = text.strip_suffix(b"\r") {
            text = rest;
        }
        if text == b"## Lessons" {
            return Ok(Some(Some(number)));
        }
    }
}

/// Where each module's lessons live, as SpecSync briefs them: `<module>.3md` when
/// present, otherwise the Lessons section of `context.md`. No lesson text is read out.
/// A path that `local_path` refuses (a symlink, for one) gets a note instead of failing
/// the view.
fn lesson_source(root: &Path, module: &str) -> Value {
    let native = format!("specs/{module}/{module}.3md");
    let context = format!("specs/{module}/context.md");
    let unreadable = |error: anyhow::Error| json!({"module": module, "path": context, "section_line": null, "note": format!("cannot be read: {error}")});
    match local_path(root, &native) {
        Ok(path) if path.is_file() => {
            return json!({"module": module, "path": native, "section_line": null, "note": null});
        }
        Ok(_) => {}
        Err(error) => return unreadable(error),
    }
    let path = match local_path(root, &context) {
        Ok(path) => path,
        Err(error) => return unreadable(error),
    };
    let (line, note) = match lessons_heading(&path) {
        Ok(Some(Some(line))) => (Some(line), None),
        Ok(Some(None)) => (None, Some("no Lessons section".to_string())),
        Ok(None) => (None, Some("missing".to_string())),
        Err(note) => (None, Some(note)),
    };
    json!({"module": module, "path": context, "section_line": line, "note": note})
}

/// Archived SpecSync lesson bundles for the librarian's hot archives: the newest 3 by
/// recorded finalization timestamp, then directory, read with the librarian's own rule
/// (`library::archive_order`), so hidden or non-UTF-8 directories and archives without
/// a readable `finalization.json` naming their change are not counted. A hot archive
/// without a `lesson-bundle.md` is shown with `(missing)`. Nothing records which were
/// folded. When the archives cannot be read (a symlinked archive directory, for one),
/// none is listed and `note` carries the same error `library index` refuses with.
fn archived_bundles(root: &Path) -> Result<Value> {
    let found = match library::archive_order(root) {
        Ok(found) => found,
        Err(error) => {
            return Ok(json!({
                "total": 0,
                "items": [],
                "note": cut(&format!("cannot be read: {error:#}")),
            }));
        }
    };
    Ok(json!({
        "total": found.len(),
        "items": found.iter().take(BUNDLE_LIMIT).map(|archive| {
            let path = format!("{ARCHIVED_CHANGES}/{}/lesson-bundle.md", archive.dir);
            cut(&if archive.lesson_bundle { path } else { format!("{path} (missing)") })
        }).collect::<Vec<_>>(),
    }))
}

/// The lesson pointer: each module's native lesson source and the archived bundles.
fn lessons(root: &Path, modules: &[String]) -> Result<Value> {
    let sources = modules
        .iter()
        .take(LIST_LIMIT)
        .map(|module| lesson_source(root, module))
        .collect::<Vec<_>>();
    Ok(json!({
        "sources": {"total": modules.len(), "items": sources},
        "archived_bundles": archived_bundles(root)?,
    }))
}

/// The focused next view (REQ-cli-021): text by default, compact JSON with `--json`.
fn next(root: &Path, environment: Option<&str>, as_json: bool) -> Result<String> {
    let phase = std::time::Instant::now();
    let assessment = assess(root)?;
    records::trace_phase("next.assess", phase);
    let session = &assessment.session;
    let checkpoint = match session {
        Session::V2(checkpoint) => Some(checkpoint.as_ref()),
    };
    let change = checkpoint.and_then(|checkpoint| checkpoint.native_change.clone());
    // REQ-history-015: corrections, and the corrected text of what they correct.
    let corrections = checkpoint.map(history::corrections).unwrap_or_default();
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
    let reads = evidence::Reads::new(root);
    let phase = std::time::Instant::now();
    let references = reads.reference_labels(&current, environment)?;
    records::trace_phase("next.references", phase);
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
    let modules = match session {
        Session::V2(checkpoint) => &checkpoint.modules,
    };
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
        "native_check": native_check::view(root, change.as_deref(), summary)?,
        "direct_writes": direct_writes::view(root, change.as_deref()),
        "native": native,
        "native_verification_warning": policy::verification_warning(root, field("/verification_commands"))?,
        "next_valid_action": {
            "action": cut(&corrections.apply(&assessment.action.action)),
            "reason": cut(&assessment.action.reason),
            "command": assessment.action.command.as_deref().map(cut),
        },
        "gate": assessment.gate.as_ref().map(|(to, gate)| json!({"to": to, "allowed": gate.allowed, "label": cut(&gate.label), "detail": cut(&gate.detail)})),
        "awaiting_human": listed(checkpoint.map_or(&empty, |checkpoint| &checkpoint.awaiting_human).iter()),
        "gates": checkpoint.map_or_else(Vec::new, |checkpoint| open_gates(checkpoint, LIST_LIMIT)),
        "blockers": listed(checkpoint.map_or(&empty, |checkpoint| &checkpoint.blockers).iter()),
        "open_questions": listed(checkpoint.map_or(&empty, |checkpoint| &checkpoint.open_questions).iter()),
        "blocking_bugs": listed(assessment.blocking.iter()),
        "other_open_bugs": assessment.open_bugs.len().saturating_sub(assessment.blocking.len()),
        "observations": {
            "needs_attention_total": attention.len(),
            "items": items,
            "referenced_current": references.len() - attention.len(),
            "unreferenced": unreferenced,
        },
        "decisions": decisions(
            &checkpoint
                .map_or(&empty, |checkpoint| &checkpoint.decisions)
                .iter()
                .map(|decision| corrections.decision(decision))
                .collect::<Vec<_>>(),
        ),
        "corrections": corrections.view(),
        "last_actor": checkpoint.and_then(history::last_actor),
        "last_failure": last_failure(root)?,
        "lessons": lessons(root, modules)?,
        "knowledge": library::pointer(root),
        "environment": environment.map(cut),
        "full_view": "chough status",
        "truncated": false,
    });
    let phase = std::time::Instant::now();
    let output = if as_json {
        fit(view, |view| format!("{view}\n"))
    } else {
        fit(view, next_text)
    };
    records::trace_phase("next.render", phase);
    Ok(output)
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
            .filter_map(|pointer| {
                let items = view.pointer(pointer)?.as_array()?;
                (!items.is_empty())
                    .then(|| (Value::from(items.clone()).to_string().len(), *pointer))
            })
            .max();
        let Some((_, pointer)) = longest else {
            return output;
        };
        if let Some(items) = view.pointer_mut(pointer).and_then(Value::as_array_mut) {
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
    if let Some(warning) = view["native_verification_warning"].as_str() {
        lines.push(format!("Warning: {warning}"));
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
    lines.extend(native_check::lines(&view["native_check"]));
    let action = &view["next_valid_action"];
    if let Ok(actor) = serde_json::from_value::<actor::Actor>(view["last_actor"].clone()) {
        // An unauthenticated claim of which agent wrote the newest entry (REQ-records-007).
        lines.push(format!("Last actor: {}", actor.display()));
    }
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
    if let Some(line) = direct_writes::next_line(&view["direct_writes"]) {
        lines.push(line);
    }
    for (key, title) in [
        ("awaiting_human", "Human gates"),
        ("blockers", "Blockers"),
        ("open_questions", "Open questions"),
    ] {
        let total = view[key]["total"].as_u64().unwrap_or(0);
        let items = view[key]["items"].as_array().cloned().unwrap_or_default();
        if total == 0 {
            lines.push(format!("{title}: none"));
            continue;
        }
        lines.push(format!("{title}: {total} open"));
        lines.extend(items.iter().map(|item| {
            // A gate shows the stage it holds and the step it drives (REQ-cli-030).
            let held = (key == "awaiting_human")
                .then(|| {
                    view["gates"]
                        .as_array()?
                        .iter()
                        .find(|gate| gate["gate"] == *item)
                })
                .flatten()
                .map(|gate| {
                    format!(
                        " (holds {}; drives {})",
                        text(&gate["holds"]),
                        text(&gate["drives"])
                    )
                })
                .unwrap_or_default();
            format!("  - {}{held}", text(item))
        }));
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
    lines.extend(handoff_lines(view, &text));
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

/// The handoff blocks as text lines: decisions, the last failure and the lesson pointer.
fn handoff_lines(view: &Value, text: &dyn Fn(&Value) -> String) -> Vec<String> {
    let items = |value: &Value| value["items"].as_array().cloned().unwrap_or_default();
    let total = |value: &Value| value["total"].as_u64().unwrap_or(0);
    let mut lines = vec![];

    let block = &view["decisions"];
    let shown = items(block);
    if total(block) == 0 {
        lines.push("Decisions: none".into());
    } else {
        lines.push(format!(
            "Decisions ({} of {}, newest first):",
            shown.len(),
            total(block)
        ));
        for item in &shown {
            let kind = if item["kind"].is_string() {
                format!("{}: ", text(&item["kind"]))
            } else {
                String::new()
            };
            let reason = if item["reason"].is_string() {
                format!(" (because {})", text(&item["reason"]))
            } else {
                String::new()
            };
            lines.push(format!("  {kind}{}{reason}", text(&item["text"])));
        }
    }

    let failure = &view["last_failure"];
    if failure.is_null() {
        lines.push("Last failure: none".into());
    } else {
        lines.push(format!(
            "Last failure: {} at {}: {}",
            text(&failure["id"]),
            failure["observed_at_unix_seconds"]
                .as_u64()
                .map_or_else(|| text(&failure["observed_at_unix_seconds"]), timing::utc),
            text(&failure["summary"])
        ));
        let bugs = &failure["bugs"];
        let shown = items(bugs);
        lines.push(if total(bugs) == 0 {
            "  Linked bugs: none".into()
        } else {
            let listed: Vec<String> = shown
                .iter()
                .map(|bug| format!("{} {}", text(&bug["id"]), text(&bug["status"])))
                .collect();
            format!(
                "  Linked bugs: {} ({} of {})",
                listed.join(", "),
                shown.len(),
                total(bugs)
            )
        });
    }

    let sources = &view["lessons"]["sources"];
    let shown = items(sources);
    if total(sources) == 0 {
        lines.push("Lesson sources: none (no modules in the checkpoint)".into());
    } else {
        lines.push(format!(
            "Lesson sources ({} of {} modules):",
            shown.len(),
            total(sources)
        ));
        for source in &shown {
            let detail = if source["section_line"].is_u64() {
                format!(" (## Lessons at line {})", text(&source["section_line"]))
            } else if source["note"].is_string() {
                format!(" ({})", text(&source["note"]))
            } else {
                String::new()
            };
            lines.push(format!(
                "  {}: {}{detail}",
                text(&source["module"]),
                text(&source["path"])
            ));
        }
    }
    let knowledge = &view["knowledge"];
    lines.push(
        match (knowledge["librarian"].as_str(), knowledge["index"].as_str()) {
            (_, Some(index)) => format!("Knowledge index: {index}"),
            (Some("on"), None) => format!(
                "Knowledge index: not built; run `chough library init` to create {}/",
                text(&knowledge["dir"])
            ),
            _ => "Knowledge index: off (librarian opted out)".into(),
        },
    );
    let bundles = &view["lessons"]["archived_bundles"];
    let shown = items(bundles);
    if let Some(note) = bundles["note"].as_str() {
        lines.push(format!("Archived lesson bundles: {note}"));
    } else if total(bundles) == 0 {
        lines.push("Archived lesson bundles: none".into());
    } else {
        lines.push(format!(
            "Archived lesson bundles ({} of {}, newest first):",
            shown.len(),
            total(bundles)
        ));
        lines.extend(shown.iter().map(|path| format!("  {}", text(path))));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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

    fn git_in(root: &Path, args: &[&str]) {
        let output = std::process::Command::new("git")
            .current_dir(root)
            .args([
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.invalid",
            ])
            .args([
                "-c",
                "commit.gpgsign=false",
                "-c",
                "init.defaultBranch=main",
            ])
            .args(["-c", "core.autocrlf=false", "-c", "merge.ff=false"])
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    /// Issue #30: the same bound case run in two independent Git roots on separate
    /// branches at the same pinned second keeps distinct identities and valid artifacts
    /// after the branches merge.
    #[test]
    fn same_second_runs_in_two_git_roots_keep_distinct_identities_after_merge() {
        const SECOND: u64 = 1_800_000_000;
        const HI: &str =
            r#"{"hi":1,"export":1,"files":[{"criteria":[{"id":"LIFE-1"}],"retired":[]}]}"#;
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let a = first.path().canonicalize().unwrap();
        for (path, text) in [
            ("specs/cli/cli.spec.md", "# CLI\n"),
            (
                "specs/cli/requirements.md",
                "### REQ-cli-001\nPreserve files.\n",
            ),
            ("src/input.txt", "input\n"),
            (
                ".chough/cases/cli.cases.toml",
                "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Shared\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\npaths = [\"src/\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"git prints its version\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"git\", \"--version\"]\nexpect_output = [\"git version\"]\n",
            ),
        ] {
            fs::create_dir_all(a.join(path).parent().unwrap()).unwrap();
            fs::write(a.join(path), text).unwrap();
        }
        git_in(&a, &["init", "-q"]);
        init(
            &a,
            Some("Same-second runs".into()),
            None,
            String::new(),
            vec!["cli".into()],
            vec![],
        )
        .unwrap();
        git_in(&a, &["add", "-A"]);
        git_in(&a, &["commit", "-q", "-m", "base"]);
        let b = second.path().canonicalize().unwrap().join("b");
        // Git gets the plain temporary paths: it cannot read Windows `\\?\` verbatim
        // paths, which `canonicalize` returns there.
        git_in(
            second.path(),
            &[
                "clone",
                "-q",
                "--config",
                "core.autocrlf=false",
                first.path().to_str().unwrap(),
                "b",
            ],
        );
        git_in(&a, &["checkout", "-q", "-b", "a"]);
        git_in(&b, &["checkout", "-q", "-b", "b"]);

        let (run_a, run_b) = records::with_native_stdout("hi", HI, || {
            records::with_pinned_clock(SECOND, || {
                (
                    cases::run_case(&a, "TC-cli-0001", "local", None).unwrap(),
                    cases::run_case(&b, "TC-cli-0001", "local", None).unwrap(),
                )
            })
        });
        assert_ne!(run_a.observation, run_b.observation);
        let record = |root: &Path, id: &str| {
            let path = root.join(format!(".chough/evidence/{id}.json"));
            let text = fs::read(&path).unwrap();
            let evidence: evidence::Evidence = serde_json::from_slice(&text).unwrap();
            (text, evidence)
        };
        let (text_a, evidence_a) = record(&a, &run_a.observation);
        let (text_b, evidence_b) = record(&b, &run_b.observation);
        assert_eq!(evidence_a.observed_at_unix_seconds, SECOND);
        assert_eq!(evidence_b.observed_at_unix_seconds, SECOND);
        assert_eq!(evidence_a.artifacts.len(), 1);
        assert_eq!(evidence_b.artifacts.len(), 1);
        assert_ne!(evidence_a.artifacts, evidence_b.artifacts);
        let artifact_a = fs::read(a.join(&evidence_a.artifacts[0])).unwrap();
        let artifact_b = fs::read(b.join(&evidence_b.artifacts[0])).unwrap();

        git_in(&a, &["add", "-A"]);
        git_in(&a, &["commit", "-q", "-m", "run a"]);
        git_in(&b, &["add", "-A"]);
        git_in(&b, &["commit", "-q", "-m", "run b"]);
        git_in(
            &a,
            &[
                "pull",
                "-q",
                "--no-rebase",
                "--no-edit",
                second.path().join("b").to_str().unwrap(),
                "b",
            ],
        );

        // Both sides survive byte for byte under their own names, and still read current.
        assert_eq!(record(&a, &run_a.observation).0, text_a);
        assert_eq!(record(&a, &run_b.observation).0, text_b);
        assert_eq!(
            fs::read(a.join(&evidence_a.artifacts[0])).unwrap(),
            artifact_a
        );
        assert_eq!(
            fs::read(a.join(&evidence_b.artifacts[0])).unwrap(),
            artifact_b
        );
        let merged = evidence::load_all(&a).unwrap();
        assert_eq!(merged.len(), 2);
        let current = evidence::snapshot(&a);
        let observations = evidence::Reads::new(&a)
            .observations(&current, Some("local"))
            .unwrap();
        assert_eq!(observations.len(), 2);
        for observation in &observations {
            assert_eq!(
                observation["freshness"], "current-observation",
                "{}",
                observation["record"]["id"]
            );
        }
    }
}
