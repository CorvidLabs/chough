//! Bounded, deterministic context assembly from native sources and the checkpoint.

use crate::checkpoint::Session;
use crate::records::{NativeJson, Section, file_section, local_path, native, native_json};
use crate::{bugs, prefs, stage};
use anyhow::{Result, ensure};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const CONTEXT_LIMIT: usize = 131_072;

/// The session as Markdown, for `chough context` without `--json` (REQ-cli-002, F10d):
/// one line per field, then the gates waiting on a human with the stage each holds and
/// the step it drives, and the decisions, where it stands, open threads and blockers,
/// each under the heading its `session.md` section has. An empty list says `none`.
pub(super) fn session_markdown(session: &Session) -> String {
    use crate::session_md::{DECISIONS, STANDS, THREADS};
    let Session::V2(checkpoint) = session;
    let one_line = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
    let listed = |items: &[String]| {
        if items.is_empty() {
            "none".to_string()
        } else {
            items
                .iter()
                .map(|item| one_line(item))
                .collect::<Vec<_>>()
                .join(", ")
        }
    };
    let stage = &checkpoint.stage;
    let callers: Vec<String> = stage
        .callers
        .iter()
        .map(|caller| format!("{} ({})", caller.stage, one_line(&caller.reason)))
        .collect();
    let consents: Vec<String> = checkpoint
        .run_consents
        .iter()
        .map(|consent| format!("{}: {}", consent.key, one_line(&consent.source)))
        .collect();
    let references = &checkpoint.references;
    let fields = [
        ("Objective", one_line(&checkpoint.objective)),
        (
            "Stage",
            format!(
                "{} ({}, iteration {})",
                stage.current, stage.status, stage.iteration
            ),
        ),
        ("Side-stage callers", listed(&callers)),
        (
            "Native change",
            checkpoint
                .native_change
                .clone()
                .unwrap_or_else(|| "none recorded".into()),
        ),
        (
            "Intent scope",
            if checkpoint.intent_scope.trim().is_empty() {
                "none".into()
            } else {
                one_line(&checkpoint.intent_scope)
            },
        ),
        ("Modules", listed(&checkpoint.modules)),
        ("Next action", one_line(&checkpoint.next_action)),
        ("Criteria", listed(&references.criteria)),
        ("Requirements", listed(&references.requirements)),
        ("Cases", listed(&references.cases)),
        ("Design", listed(&references.design)),
        ("Files", listed(&references.files)),
        ("Observations", listed(&checkpoint.observations)),
        ("Bugs", listed(&checkpoint.bugs)),
        ("Run consents", listed(&consents)),
    ];
    let mut out: String = fields
        .iter()
        .map(|(label, value)| format!("- {label}: {value}\n"))
        .collect();
    let gates: Vec<String> = checkpoint
        .awaiting_human
        .iter()
        .map(|gate| {
            let meta = crate::session_md::gate(checkpoint, gate);
            format!(
                "{} (holds {}; drives {})",
                one_line(gate),
                meta.holds,
                meta.drives().name()
            )
        })
        .collect();
    for (heading, items) in [
        ("Waiting on a human", &gates),
        (DECISIONS, &checkpoint.decisions),
        (STANDS, &checkpoint.completed),
        (THREADS, &checkpoint.open_questions),
        ("Blockers", &checkpoint.blockers),
    ] {
        out.push_str(&format!("\n### {heading}\n\n"));
        if items.is_empty() {
            out.push_str("none\n");
        }
        for item in items {
            out.push_str(&format!("- {}\n", one_line(item)));
        }
    }
    out
}

/// The folders whose AGENTS.md the packet reads, outermost first, each with the name the
/// packet gives it: from the project's Git root down to the project root, named relative
/// to the project (`../AGENTS.md`, then `AGENTS.md`). Nothing above the repository is
/// read, so a home folder's AGENTS.md never enters the packet, and no absolute path is
/// printed. Outside a Git work tree, only the project root.
fn instruction_folders(root: &Path) -> Vec<(PathBuf, String)> {
    let project = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let top = crate::records::run_owned(
        std::process::Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["rev-parse", "--show-toplevel"]),
        None,
    )
    .ok()
    .filter(|output| output.status.success())
    .and_then(|output| String::from_utf8(output.stdout).ok())
    .and_then(|top| {
        PathBuf::from(top.trim_end_matches(['\r', '\n']))
            .canonicalize()
            .ok()
    })
    .filter(|top| project.starts_with(top));
    let mut folders = vec![];
    for (depth, folder) in project.ancestors().enumerate() {
        folders.push((
            folder.to_path_buf(),
            format!("{}AGENTS.md", "../".repeat(depth)),
        ));
        if top.as_deref().is_none_or(|top| folder == top) {
            break;
        }
    }
    folders.reverse();
    folders
}

/// The HI export shown in `chough context`: a display section within 16 KiB.
pub(super) fn hi_export(root: &Path, scope: &str) -> Section {
    if scope.is_empty() {
        native(root, "hi", &["export"])
    } else {
        native(root, "hi", &["export", "--", scope])
    }
}

/// The HI export a decision reads: parsed from the complete output (up to SpecSync's
/// 4 MiB parse bound), never from the 16 KiB display section. HI owns the intent and its
/// size; a Chough bound must never make anyone shorten it (LIFE-15).
pub(super) fn hi_export_json(root: &Path, scope: &str) -> NativeJson {
    if scope.is_empty() {
        native_json(root, "hi", &["export"])
    } else {
        native_json(root, "hi", &["export", "--", scope])
    }
}

pub(super) fn context(root: &Path, session: &Session) -> Result<Vec<Section>> {
    let mut sections = vec![];
    for (folder, name) in instruction_folders(root) {
        if folder.join("AGENTS.md").exists() {
            let mut section = file_section(&folder, "AGENTS.md")?;
            section.source = name;
            sections.push(section);
        }
    }
    sections.push(file_section(root, "INTENT.md")?);
    for name in ["hi/AGENTS.md", "specs/AGENTS.md"] {
        if local_path(root, name)?.is_file() {
            sections.push(file_section(root, name)?);
        }
    }
    sections.push(hi_export(root, session.intent_scope()));
    let modules: BTreeSet<_> = session.modules().iter().collect();
    ensure!(
        modules.len() <= 16,
        "select at most 16 modules for focused context"
    );
    for module in modules {
        let base = format!("specs/{module}");
        let instructions = format!("{base}/AGENTS.md");
        if local_path(root, &instructions)?.is_file() {
            sections.push(file_section(root, &instructions)?);
        }
        for name in [
            format!("{module}.spec.md"),
            "requirements.md".into(),
            "tasks.md".into(),
            "testing.md".into(),
        ] {
            sections.push(file_section(root, &format!("{base}/{name}"))?);
        }
        let lesson = format!("{base}/{module}.3md");
        if local_path(root, &lesson)?.is_file() {
            let mut section = file_section(root, &lesson)?;
            section.content = format!(
                "Native lesson source, verbatim (planes are not parsed by Chough).\n\n{}",
                section.content
            );
            sections.push(section);
        } else {
            sections.push(file_section(root, &format!("{base}/context.md"))?);
        }
    }
    {
        let Session::V2(checkpoint) = session;
        let (transitions, partial) = stage::recent_transitions(root, 20)?;
        sections.push(Section {
            source: format!(
                "{} (Timeline)",
                crate::sessions::checkpoint_path(root).unwrap_or_default()
            ),
            state: if transitions.is_empty() {
                "empty".into()
            } else if partial {
                "recent".into()
            } else {
                "present".into()
            },
            content: serde_json::to_string_pretty(&transitions)?,
        });
        sections.push(Section {
            source: "chough stage show".into(),
            state: "derived".into(),
            content: stage::valid_transitions(checkpoint).join("\n"),
        });
    }
    if local_path(root, prefs::PREFERENCES)?.is_file() {
        sections.push(file_section(root, prefs::PREFERENCES)?);
    }
    sections.push(bugs::open_backlog_section(root)?);
    let mut remaining = CONTEXT_LIMIT;
    for section in &mut sections {
        if remaining == 0 {
            section.state = "omitted-budget".into();
            section.content =
                "Source omitted from this display: the aggregate display budget is 131072 bytes. Read the source itself; never shorten it to fit a Chough display bound.".into();
        } else if section.content.len() > remaining {
            let mut boundary = remaining;
            while !section.content.is_char_boundary(boundary) {
                boundary -= 1;
            }
            section.content.truncate(boundary);
            section
                .content
                .push_str("\n[truncated for display only: the aggregate display budget is 131072 bytes; read the complete source, and never shorten it to fit a Chough display bound]");
            section.state = "truncated-budget".into();
            remaining = 0;
        } else {
            remaining -= section.content.len();
        }
    }
    Ok(sections)
}
