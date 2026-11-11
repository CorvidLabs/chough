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
            let (Some(requirement), Some(rest)) = (current.as_ref(), trace_list(line)) else {
                continue;
            };
            // The list runs to the end of its sentence; IDs such as `UX-1.a` contain dots.
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

/// The text after the last `Trace:` on a line that is not inside backticks: prose may
/// mention `` `Trace:` `` before or after the real list.
fn trace_list(line: &str) -> Option<&str> {
    // Only paired backticks open a code span; an unmatched one is literal (CommonMark).
    let ticks: Vec<usize> = line.match_indices('`').map(|(index, _)| index).collect();
    let paired = ticks.len() - ticks.len() % 2;
    let in_code = |index: usize| {
        ticks[..paired]
            .chunks(2)
            .any(|pair| pair[0] < index && index < pair[1])
    };
    line.match_indices("Trace:")
        .map(|(index, _)| index)
        .filter(|index| !in_code(*index))
        .last()
        .map(|index| &line[index + "Trace:".len()..])
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
    // `lines` also ends a line at `\r\n`, so CRLF files read the same.
    let mut lines = text.lines();
    if lines.next().map(str::trim_end) != Some("---") {
        return vec![];
    }
    let header: Vec<&str> = lines.take_while(|line| line.trim_end() != "---").collect();
    // A YAML comment starts at ` #`; quoted family names never contain one.
    let uncomment = |value: &str| {
        value
            .split(" #")
            .next()
            .unwrap_or_default()
            .trim()
            .to_string()
    };
    let clean = |item: &str| {
        item.trim()
            .trim_matches(|c| c == '"' || c == '\'')
            .to_string()
    };
    let Some(index) = header.iter().position(|line| line.starts_with("families:")) else {
        return vec![];
    };
    let inline = uncomment(&header[index]["families:".len()..]);
    let found: Vec<String> = if inline.is_empty() {
        header[index + 1..]
            .iter()
            .take_while(|line| line.starts_with(' ') || line.starts_with('-'))
            .filter_map(|line| uncomment(line).strip_prefix('-').map(clean))
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

/// How many archived changes a note lists in full; the rest are counted and every one is
/// on the timeline, so no note outgrows the entry bound as archives accumulate (#112).
pub(super) const RECENT: usize = 10;

fn change_links(changes: &[&Touch]) -> String {
    if changes.is_empty() {
        return "none yet".into();
    }
    let mut links = changes
        .iter()
        .take(RECENT)
        .map(|change| format!("[{id}](../archives/{id}.md)", id = change.id))
        .collect::<Vec<_>>()
        .join(", ");
    if changes.len() > RECENT {
        links.push_str(&format!(
            ", and {} more on the [timeline](../timeline.md)",
            changes.len() - RECENT
        ));
    }
    links
}

/// One archived change on the timeline.
pub(super) struct TimelineEntry<'a> {
    pub(super) id: &'a str,
    pub(super) title: &'a str,
    pub(super) finalized: u64,
    pub(super) hot: bool,
    /// The lesson bundle's path from the repository root, when the archive has one.
    pub(super) bundle: Option<String>,
}

/// `timeline.md`, linking one page per month, and `timeline/<YYYY-MM>.md`, listing that
/// month's archived changes newest first with their date and lesson bundle (KNOW-28).
/// `entries` are newest first.
pub(super) fn timeline_notes(dir: &str, entries: &[TimelineEntry]) -> Vec<(String, String)> {
    let mut months: Vec<(String, Vec<&TimelineEntry>)> = vec![];
    for entry in entries {
        let month = iso(entry.finalized)[..7].to_string();
        match months.last_mut() {
            Some((last, list)) if *last == month => list.push(entry),
            _ => months.push((month, vec![entry])),
        }
    }
    // A month's rows are split into pages that each stay well within the entry bound.
    const PAGE_BYTES: usize = 56_000;
    let mut pages: Vec<(String, String, String, usize)> = vec![]; // (file stem, label, rows, count)
    for (month, list) in &months {
        let mut rows = String::new();
        let mut count = 0;
        for entry in list {
            let bundle = entry
                .bundle
                .as_ref()
                .map(|path| format!(" · [lesson bundle](../../{path})"))
                .unwrap_or_default();
            let row = format!(
                "- {date} [{id}](../archives/{id}.md) {title} ({status}){bundle}\n",
                date = &iso(entry.finalized)[..10],
                id = entry.id,
                title = entry.title.replace(['\n', '\r'], " "),
                status = if entry.hot { "hot" } else { "cold" },
            );
            if count > 0 && rows.len() + row.len() > PAGE_BYTES {
                let part = pages
                    .iter()
                    .filter(|page| page.0.starts_with(month.as_str()))
                    .count();
                let stem = if part == 0 {
                    month.clone()
                } else {
                    format!("{month}-p{}", part + 1)
                };
                let label = if part == 0 {
                    month.clone()
                } else {
                    format!("{month}, page {}", part + 1)
                };
                pages.push((stem, label, std::mem::take(&mut rows), count));
                count = 0;
            }
            rows.push_str(&row);
            count += 1;
        }
        let part = pages
            .iter()
            .filter(|page| page.0.starts_with(month.as_str()))
            .count();
        let stem = if part == 0 {
            month.clone()
        } else {
            format!("{month}-p{}", part + 1)
        };
        let label = if part == 0 {
            month.clone()
        } else {
            format!("{month}, page {}", part + 1)
        };
        pages.push((stem, label, rows, count));
    }
    let mut notes = vec![];
    let index_rows: String = pages
        .iter()
        .map(|(stem, label, _, count)| {
            format!("- [{label}](timeline/{stem}.md): {count} archived changes\n")
        })
        .collect();
    notes.push((
        format!("{dir}/timeline.md"),
        format!(
            "{props}# Timeline\n\nGenerated by `chough library index`. Every archived change, by the month it was\nfinalized, newest first. The index lists only the newest {RECENT}.\n\n{rows}",
            props = front_matter(&[
                ("id", Prop::Text("timeline".into())),
                ("title", Prop::Text("Timeline".into())),
                ("type", Prop::Raw("timeline".into())),
                ("archived_changes", Prop::Raw(entries.len().to_string())),
                ("tags", Prop::List(vec!["chough/timeline".into()])),
            ]),
            rows = if index_rows.is_empty() { "- none\n".to_string() } else { index_rows },
        ),
    ));
    for (stem, label, rows, count) in pages {
        notes.push((
            format!("{dir}/timeline/{stem}.md"),
            format!(
                "{props}# Timeline: {label}\n\nGenerated by `chough library index`. [All months](../timeline.md).\n\n{rows}",
                props = front_matter(&[
                    ("id", Prop::Text(format!("timeline/{stem}"))),
                    ("title", Prop::Text(format!("Timeline: {label}"))),
                    ("type", Prop::Raw("timeline-month".into())),
                    ("archived_changes", Prop::Raw(count.to_string())),
                    ("tags", Prop::List(vec!["chough/timeline".into()])),
                ]),
            ),
        ));
    }
    notes
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
        let mut rows: String = changes
            .iter()
            .take(RECENT)
            .map(|change| {
                format!(
                    "- [{id}](../archives/{id}.md) {title}\n",
                    id = change.id,
                    title = change.title.replace(['\n', '\r'], " ")
                )
            })
            .collect();
        if changes.len() > RECENT {
            rows.push_str(&format!(
                "- {} more on the [timeline](../timeline.md)\n",
                changes.len() - RECENT
            ));
        }
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

/// `bugs/<id>.md` for each closed bug with key items in the vault (KNOW-22): a minimal
/// note linking the bug record and each file under the bug's attachments folder. A bug
/// with nothing in the vault has no note.
pub(super) fn bug_notes(root: &Path, dir: &str) -> Result<Vec<(String, String)>> {
    let mut notes = vec![];
    for bug in crate::bugs::closed_links(root)? {
        let attachments = crate::attachments::attachments_of(root, dir, &bug.id);
        if attachments.is_empty() {
            continue;
        }
        let record = format!(".chough/bugs/{}.json", bug.id);
        let links: String = attachments
            .iter()
            .map(|file| {
                let shown = file
                    .strip_prefix(&format!("{dir}/attachments/"))
                    .and_then(|rest| rest.split_once('/'))
                    .map_or(file.as_str(), |(_, rest)| rest);
                let target = file.strip_prefix(&format!("{dir}/")).unwrap_or(file);
                format!("- [{shown}](../{target})\n")
            })
            .collect();
        let props = front_matter(&[
            ("id", Prop::Text(bug.id.clone())),
            ("title", Prop::Text(bug.title.clone())),
            ("type", Prop::Raw("bug".into())),
            ("status", Prop::Raw(bug.status.clone())),
            ("tags", Prop::List(vec!["chough/bug".into()])),
        ]);
        notes.push((
            format!("{dir}/bugs/{}.md", bug.id),
            format!(
                "{props}# {}: {}\n\nGenerated by `chough library index`. Status: {}. Record: [{record}](../../{record}).\n\n## Key items in the vault\n\n{links}",
                bug.id, bug.title, bug.status
            ),
        ));
    }
    Ok(notes)
}

/// `sessions/<key>.md` for each session rolled into the vault (REQ-cli-051, KNOW-29),
/// found at `attachments/<folder>/sessions/<key>.json`: a summary that links the session
/// and, when `archived` holds its native change, that change's archive note. `archived`
/// is the set of archive notes the same build writes, so the link never depends on notes
/// already in the directory (KNOW-4, #191). A session that cannot be read is left out.
pub(super) fn session_notes(
    root: &Path,
    dir: &str,
    archived: &std::collections::BTreeSet<&str>,
) -> Result<Vec<(String, String)>> {
    let mut found = vec![];
    let base = local_path(root, &format!("{dir}/attachments"))?;
    if base.is_dir() {
        for folder in std::fs::read_dir(&base)? {
            let folder = folder?;
            let sessions = folder.path().join("sessions");
            if !sessions.is_dir() {
                continue;
            }
            for entry in std::fs::read_dir(&sessions)? {
                let entry = entry?;
                let name = entry.file_name().to_string_lossy().to_string();
                if let Some(key) = name.strip_suffix(".json") {
                    let folder_name = folder.file_name().to_string_lossy().to_string();
                    found.push((
                        key.to_string(),
                        format!("attachments/{folder_name}/sessions/{name}"),
                    ));
                }
            }
        }
    }
    found.sort();
    let mut notes = vec![];
    for (key, attachment) in found {
        let Some(text) = read(root, &format!("{dir}/{attachment}")) else {
            continue;
        };
        let Ok(crate::checkpoint::Session::V2(session)) =
            crate::checkpoint::parse(&format!("{dir}/{attachment}"), &text)
        else {
            continue;
        };
        let change = session.native_change.clone();
        let change_link = change
            .as_deref()
            .filter(|change| archived.contains(change))
            .map(|change| format!("- Native change: [{change}](../archives/{change}.md)\n"))
            .unwrap_or_default();
        let props = front_matter(&[
            ("id", Prop::Text(format!("session/{key}"))),
            ("title", Prop::Text(session.objective.clone())),
            ("type", Prop::Raw("session".into())),
            ("native_change", text_or_null(&change)),
            ("stage", Prop::Raw(session.stage.current.clone())),
            ("tags", Prop::List(vec!["chough/session".into()])),
        ]);
        notes.push((
            format!("{dir}/sessions/{key}.md"),
            format!(
                "{props}# Session {key}: {}\n\nGenerated by `chough library index`. A done session rolled out of `.chough/sessions/archive/` (KNOW-29).\n\n- Session record: [{attachment}](../{attachment})\n{change_link}- Stage: {}; decisions: {}; completed: {}\n",
                session.objective,
                session.stage.current,
                session.decisions.len(),
                session.completed.len()
            ),
        ));
    }
    Ok(notes)
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
