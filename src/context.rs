//! Bounded, deterministic context assembly from native sources and the checkpoint.

use crate::checkpoint::Session;
use crate::records::{NativeJson, Section, file_section, local_path, native, native_json};
use crate::{bugs, prefs, stage};
use anyhow::{Result, ensure};
use std::collections::BTreeSet;
use std::path::Path;

const CONTEXT_LIMIT: usize = 131_072;

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
    let mut ancestors: Vec<_> = root.ancestors().collect();
    ancestors.reverse();
    for ancestor in ancestors {
        if ancestor.join("AGENTS.md").exists() {
            let mut section = file_section(ancestor, "AGENTS.md")?;
            section.source = ancestor.join("AGENTS.md").display().to_string();
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
