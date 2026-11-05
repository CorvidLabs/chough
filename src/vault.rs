//! The knowledge directory as an Obsidian-style vault (REQ-cli-044, KNOW-14, KNOW-15):
//! YAML front matter, ISO 8601 dates, standard Markdown links, and the module, intent and
//! rules notes that connect each archived change to what it touched. Every note points
//! to its sources and copies no lesson or criterion text (KNOW-3, KNOW-12).

use crate::records::{local_path, read_native_text};
use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// One front matter value.
pub(super) enum Prop {
    /// A plain token written as is: a note type, status, date or `null`.
    Raw(String),
    /// Any text, written as a double-quoted JSON string, which YAML reads unchanged.
    Text(String),
    /// A list, written as a YAML block list, or `[]` when empty.
    List(Vec<String>),
}

/// YAML front matter in the given key order, fenced by `---` lines.
pub(super) fn front_matter(props: &[(&str, Prop)]) -> String {
    let mut out = String::from("---\n");
    for (key, value) in props {
        match value {
            Prop::Raw(raw) => out.push_str(&format!("{key}: {raw}\n")),
            Prop::Text(text) => out.push_str(&format!("{key}: {}\n", quote(text))),
            Prop::List(items) if items.is_empty() => out.push_str(&format!("{key}: []\n")),
            Prop::List(items) => {
                out.push_str(&format!("{key}:\n"));
                for item in items {
                    out.push_str(&format!("  - {}\n", list_item(item)));
                }
            }
        }
    }
    out.push_str("---\n\n");
    out
}

fn quote(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_else(|_| "\"\"".into())
}

/// Plain IDs and tags stay bare; anything else is quoted.
fn list_item(item: &str) -> String {
    let plain = !item.is_empty()
        && item
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '/'));
    if plain { item.to_string() } else { quote(item) }
}

/// An optional value as quoted text, or `null`.
pub(super) fn text_or_null(value: &Option<String>) -> Prop {
    match value {
        Some(text) => Prop::Text(text.clone()),
        None => Prop::Raw("null".into()),
    }
}

/// Unix seconds as ISO 8601 UTC, such as `2026-11-04T21:56:12Z`.
pub(super) fn iso(seconds: u64) -> String {
    let days = (seconds / 86_400) as i64;
    let rest = seconds % 86_400;
    // Civil date from days since 1970-01-01 (Howard Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rest / 3_600,
        rest % 3_600 / 60,
        rest % 60
    )
}

/// The front matter fields of an archived-change note.
pub(super) struct ArchiveProps<'a> {
    pub(super) id: &'a str,
    pub(super) title: &'a str,
    pub(super) hot: bool,
    pub(super) finalized: u64,
    pub(super) modules: &'a [String],
    pub(super) requirements: &'a [String],
    pub(super) criteria: &'a [String],
    /// Base, implementation and verification commits.
    pub(super) commits: [&'a Option<String>; 3],
}

/// Front matter for an archived-change note (REQ-cli-044).
pub(super) fn archive_front_matter(archive: &ArchiveProps) -> String {
    let mut tags = vec!["chough/archive".to_string()];
    tags.extend(
        archive
            .modules
            .iter()
            .map(|module| format!("module/{module}")),
    );
    let [base, implementation, verification] = archive.commits;
    front_matter(&[
        ("id", Prop::Text(archive.id.to_string())),
        ("title", Prop::Text(archive.title.to_string())),
        ("type", Prop::Raw("archived-change".into())),
        (
            "status",
            Prop::Raw(if archive.hot { "hot" } else { "cold" }.into()),
        ),
        ("finalized", Prop::Raw(iso(archive.finalized))),
        ("modules", Prop::List(archive.modules.to_vec())),
        ("requirements", Prop::List(archive.requirements.to_vec())),
        ("criteria", Prop::List(archive.criteria.to_vec())),
        ("base_commit", text_or_null(base)),
        ("implementation_commit", text_or_null(implementation)),
        ("verification_commit", text_or_null(verification)),
        ("tags", Prop::List(tags)),
    ])
}

/// Front matter for the index.
pub(super) fn index_front_matter(archived: usize) -> String {
    front_matter(&[
        ("id", Prop::Text("index".into())),
        ("title", Prop::Text("Project knowledge index".into())),
        ("type", Prop::Raw("index".into())),
        ("archived_changes", Prop::Raw(archived.to_string())),
        ("tags", Prop::List(vec!["chough/index".into()])),
    ])
}

/// An archived change's traced criteria and its module and criterion links.
pub(super) struct ArchiveLinks {
    pub(super) criteria: Vec<String>,
    pub(super) modules: String,
    pub(super) criterion_links: String,
}

pub(super) fn archive_links(
    modules: &[String],
    requirements: &[String],
    traces: &BTreeMap<String, Vec<String>>,
    families: &BTreeMap<String, String>,
    known: &[String],
) -> ArchiveLinks {
    let criteria = traced(requirements, traces);
    let join = |items: Vec<String>, none: &str| {
        if items.is_empty() {
            none.to_string()
        } else {
            items.join(", ")
        }
    };
    ArchiveLinks {
        modules: join(
            modules
                .iter()
                .map(|module| module_link(module, known, "../"))
                .collect(),
            "none recorded",
        ),
        criterion_links: join(
            criteria
                .iter()
                .map(|id| criterion_link(id, families, "../"))
                .collect(),
            "none traced",
        ),
        criteria,
    }
}

/// The module directories under `specs/`, sorted.
pub(super) fn modules(root: &Path) -> Result<Vec<String>> {
    let mut found = vec![];
    let specs = local_path(root, "specs")?;
    if specs.is_dir() {
        for entry in std::fs::read_dir(specs)? {
            let entry = entry?;
            if entry.file_type()?.is_dir()
                && let Some(name) = entry.file_name().to_str()
            {
                found.push(name.to_string());
            }
        }
    }
    found.sort();
    Ok(found)
}

fn read(root: &Path, relative: &str) -> Option<String> {
    let path = local_path(root, relative).ok()?;
    path.is_file()
        .then(|| read_native_text(&path).ok())
        .flatten()
}

fn exists(root: &Path, relative: &str) -> bool {
    local_path(root, relative).is_ok_and(|path| path.exists())
}

/// The HI criterion IDs each living requirement names in its `Trace:` list.
pub(super) fn traces(root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let mut found = BTreeMap::new();
    for module in modules(root)? {
        let Some(text) = read(root, &format!("specs/{module}/requirements.md")) else {
            continue;
        };
        let mut current: Option<String> = None;
        for line in text.lines() {
            if let Some(id) = line.strip_prefix("### ") {
                let id = id.split_whitespace().next().unwrap_or_default();
                current = id.starts_with("REQ-").then(|| id.to_string());
                continue;
            }
            let (Some(requirement), Some(index)) = (current.as_ref(), line.rfind("Trace:")) else {
                continue;
            };
            // The real list is the last `Trace:` on the line (the text may mention it inline)
            // and runs to the end of its sentence; IDs such as `UX-1.a` contain dots.
            let rest = &line[index + "Trace:".len()..];
            let list = rest.split(". ").next().unwrap_or_default();
            let ids: &mut Vec<String> = found.entry(requirement.clone()).or_default();
            for id in list.split(',').map(|id| id.trim().trim_end_matches('.')) {
                if is_criterion(id) && !ids.iter().any(|known| known == id) {
                    ids.push(id.to_string());
                }
            }
        }
    }
    Ok(found)
}

/// An HI criterion ID, including sub-criteria such as `UX-1.a` (DRIVE-1).
fn is_criterion(id: &str) -> bool {
    crate::records::hi_criterion_id(id)
}

/// The sorted HI criterion IDs an archive's requirements trace.
pub(super) fn traced(
    requirements: &[String],
    traces: &BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let set: BTreeSet<&String> = requirements
        .iter()
        .filter_map(|id| traces.get(id))
        .flatten()
        .collect();
    set.into_iter().cloned().collect()
}

/// One HI file: its note stem, its families and its criterion IDs in file order.
pub(super) struct IntentFile {
    pub(super) stem: String,
    pub(super) families: Vec<String>,
    pub(super) criteria: Vec<String>,
}

/// Every HI file under `hi/` except `AGENTS.md` and `CLAUDE.md`, sorted by name.
pub(super) fn intent_files(root: &Path) -> Result<Vec<IntentFile>> {
    let mut names = vec![];
    let hi = local_path(root, "hi")?;
    if hi.is_dir() {
        for entry in std::fs::read_dir(hi)? {
            let name = entry?.file_name().to_string_lossy().into_owned();
            if name.ends_with(".md") && name != "AGENTS.md" && name != "CLAUDE.md" {
                names.push(name);
            }
        }
    }
    names.sort();
    let mut files = vec![];
    for name in names {
        let text = read(root, &format!("hi/{name}")).unwrap_or_default();
        let families = families(&text);
        let criteria = text
            .lines()
            .filter_map(|line| line.strip_prefix("- **"))
            .filter_map(|rest| rest.split_once("**").map(|(id, _)| id.to_string()))
            .filter(|id| is_criterion(id))
            .collect();
        files.push(IntentFile {
            stem: name.trim_end_matches(".md").to_string(),
            families,
            criteria,
        });
    }
    Ok(files)
}

/// The families an HI file declares in its front matter, in any form hi accepts: a flow
/// list (`[UX]` or `["UX"]`) or a block list (`families:` then `  - UX`).
fn families(text: &str) -> Vec<String> {
    let header: Vec<&str> = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.split("\n---").next())
        .map(|block| block.lines().collect())
        .unwrap_or_default();
    let clean = |item: &str| {
        item.trim()
            .trim_matches(|c| c == '"' || c == '\'')
            .to_string()
    };
    let Some(index) = header.iter().position(|line| line.starts_with("families:")) else {
        return vec![];
    };
    let inline = header[index]["families:".len()..].trim();
    let found: Vec<String> = if inline.is_empty() {
        header[index + 1..]
            .iter()
            .take_while(|line| line.starts_with(' ') || line.starts_with('-'))
            .filter_map(|line| line.trim().strip_prefix('-'))
            .map(clean)
            .collect()
    } else {
        inline
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(clean)
            .collect()
    };
    found
        .into_iter()
        .filter(|family| !family.is_empty())
        .collect()
}

/// The intent note stem for each HI family.
pub(super) fn family_notes(files: &[IntentFile]) -> BTreeMap<String, String> {
    files
        .iter()
        .flat_map(|file| {
            file.families
                .iter()
                .map(|family| (family.clone(), file.stem.clone()))
        })
        .collect()
}

/// A criterion ID linked to its intent note, or plain when no HI file names its family.
pub(super) fn criterion_link(id: &str, families: &BTreeMap<String, String>, up: &str) -> String {
    let family = id.split('-').next().unwrap_or_default();
    match families.get(family) {
        Some(stem) => format!("[{id}]({up}intent/{stem}.md)"),
        None => id.to_string(),
    }
}

/// A module linked to its note, or plain when it is no longer under `specs/`.
pub(super) fn module_link(module: &str, known: &[String], up: &str) -> String {
    if known.iter().any(|name| name == module) {
        format!("[{module}]({up}modules/{module}.md)")
    } else {
        module.to_string()
    }
}

/// Lessons recorded in a module's `## Lessons` section: entries that start with a date.
fn lesson_count(text: &str) -> usize {
    text.lines()
        .skip_while(|line| line.trim_end() != "## Lessons")
        .skip(1)
        .take_while(|line| !line.starts_with("## "))
        .filter(|line| {
            let bytes = line.as_bytes();
            bytes.len() > 11
                && bytes[..4].iter().all(u8::is_ascii_digit)
                && bytes[4] == b'-'
                && bytes[7] == b'-'
                && bytes[10] == b','
        })
        .count()
}

/// What a module or intent note needs to know about one archived change.
pub(super) struct Touch<'a> {
    pub(super) id: &'a str,
    pub(super) title: &'a str,
    pub(super) modules: &'a [String],
    pub(super) criteria: &'a [String],
}

fn change_links(changes: &[&Touch]) -> String {
    if changes.is_empty() {
        return "none yet".into();
    }
    changes
        .iter()
        .map(|change| format!("[{id}](../archives/{id}.md)", id = change.id))
        .collect::<Vec<_>>()
        .join(", ")
}

/// `modules/<module>.md` for each module under `specs/`.
pub(super) fn module_notes(
    root: &Path,
    dir: &str,
    touches: &[Touch],
) -> Result<Vec<(String, String)>> {
    let mut notes = vec![];
    for module in modules(root)? {
        let base = format!("specs/{module}");
        let sources: String = [
            format!("{base}/{module}.spec.md"),
            format!("{base}/requirements.md"),
            format!("{base}/context.md"),
            format!("{base}/testing.md"),
            format!("{base}/tasks.md"),
        ]
        .into_iter()
        .filter(|path| exists(root, path))
        .map(|path| format!("- [{path}](../../{path})\n"))
        .collect();
        let lessons = read(root, &format!("{base}/context.md"))
            .map(|text| lesson_count(&text))
            .unwrap_or(0);
        let changes: Vec<&Touch> = touches
            .iter()
            .filter(|touch| touch.modules.contains(&module))
            .collect();
        let rows: String = changes
            .iter()
            .map(|change| {
                format!(
                    "- [{id}](../archives/{id}.md) {title}\n",
                    id = change.id,
                    title = change.title.replace(['\n', '\r'], " ")
                )
            })
            .collect();
        let content = format!(
            "{props}# Module {module}\n\n\
Generated by `chough library index`. The module's spec and companions own its contract;\n\
this note only links them.\n\n\
## Sources\n\n{sources}\n\
## Lessons\n\n{lessons} lessons recorded in the `## Lessons` section of\n\
[{base}/context.md](../../{base}/context.md); the lessons stay there.\n\n\
## Archived changes that touched {module}\n\n{count} archived changes, newest first.\n\n{rows}",
            props = front_matter(&[
                ("id", Prop::Text(format!("module/{module}"))),
                ("title", Prop::Text(format!("Module {module}"))),
                ("type", Prop::Raw("module".into())),
                ("modules", Prop::List(vec![module.clone()])),
                ("lessons", Prop::Raw(lessons.to_string())),
                ("archived_changes", Prop::Raw(changes.len().to_string())),
                (
                    "tags",
                    Prop::List(vec!["chough/module".into(), format!("module/{module}")])
                ),
            ]),
            sources = if sources.is_empty() {
                "- none\n".to_string()
            } else {
                sources
            },
            count = changes.len(),
            rows = if rows.is_empty() {
                "- none yet\n".to_string()
            } else {
                rows
            },
        );
        notes.push((format!("{dir}/modules/{module}.md"), content));
    }
    Ok(notes)
}

/// `intent/<file>.md` for each HI file: criterion IDs and the changes that trace them.
pub(super) fn intent_notes(
    dir: &str,
    files: &[IntentFile],
    touches: &[Touch],
) -> Vec<(String, String)> {
    files
        .iter()
        .map(|file| {
            let rows: String = file
                .criteria
                .iter()
                .map(|id| {
                    let changes: Vec<&Touch> = touches
                        .iter()
                        .filter(|touch| touch.criteria.iter().any(|criterion| criterion == id))
                        .collect();
                    format!("- **{id}**: {}\n", change_links(&changes))
                })
                .collect();
            let content = format!(
                "{props}# Intent: {stem}\n\n\
Generated by `chough library index`. The human's words live in\n\
[hi/{stem}.md](../../hi/{stem}.md); this note lists criterion IDs only, each with the\n\
archived changes whose requirements trace it.\n\n\
## Criteria\n\n{rows}",
                props = front_matter(&[
                    ("id", Prop::Text(format!("intent/{}", file.stem))),
                    ("title", Prop::Text(format!("Intent: {}", file.stem))),
                    ("type", Prop::Raw("intent".into())),
                    ("families", Prop::List(file.families.clone())),
                    ("criteria", Prop::List(file.criteria.clone())),
                    ("tags", Prop::List(vec!["chough/intent".into()])),
                ]),
                stem = file.stem,
                rows = if rows.is_empty() {
                    "- none\n".to_string()
                } else {
                    rows
                },
            );
            (format!("{dir}/intent/{}.md", file.stem), content)
        })
        .collect()
}

/// `rules.md`: the project's rule, skill, decision and policy files that exist.
pub(super) fn rules_note(root: &Path, dir: &str) -> Result<(String, String)> {
    let mut rules = vec![
        "AGENTS.md".to_string(),
        "CLAUDE.md".into(),
        "hi/AGENTS.md".into(),
        ".agents/skills/chough-workflow/SKILL.md".into(),
        "skills/chough-workflow/SKILL.md".into(),
        "docs/DECISIONS.md".into(),
    ];
    rules.extend(
        modules(root)?
            .into_iter()
            .map(|module| format!("specs/{module}/context.md")),
    );
    rules.push(".specsync/sdd.json".into());
    let rows: String = rules
        .iter()
        .filter(|path| exists(root, path))
        .map(|path| format!("- [{path}](../{path})\n"))
        .collect();
    let content = format!(
        "{props}# Rules, skills and decisions\n\n\
Generated by `chough library index`. These files own how work is done here: agent\n\
instructions, the workflow skill, recorded decisions, each module's decisions and\n\
lessons, and the SDD policy. A lesson that changes behaviour becomes a change to one of\n\
them through a native change.\n\n{rows}",
        props = front_matter(&[
            ("id", Prop::Text("rules".into())),
            ("title", Prop::Text("Rules, skills and decisions".into())),
            ("type", Prop::Raw("rules".into())),
            ("tags", Prop::List(vec!["chough/rules".into()])),
        ]),
        rows = if rows.is_empty() {
            "- none\n".to_string()
        } else {
            rows
        },
    );
    Ok((format!("{dir}/rules.md"), content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iso_dates_are_utc_calendar_dates() {
        assert_eq!(iso(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso(100), "1970-01-01T00:01:40Z");
        assert_eq!(iso(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(iso(1_793_762_536), "2026-11-04T03:22:16Z");
    }

    #[test]
    fn front_matter_quotes_text_and_writes_block_lists() {
        let text = front_matter(&[
            ("title", Prop::Text("a: \"b\"".into())),
            (
                "tags",
                Prop::List(vec!["chough/archive".into(), "two words".into()]),
            ),
            ("none", Prop::List(vec![])),
        ]);
        assert_eq!(
            text,
            "---\ntitle: \"a: \\\"b\\\"\"\ntags:\n  - chough/archive\n  - \"two words\"\nnone: []\n---\n\n"
        );
    }
}
