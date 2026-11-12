//! The knowledge vault's reading guide, `index.md` (REQ-cli-043, REQ-cli-044, KNOW-31). The
//! vault holds only what has no other home: this index, the journal and its feed
//! (REQ-records-011), lessons that span projects, and existing snapshots and briefs as
//! they are (KNOW-6). The index links everything else with standard relative Markdown
//! links, copies no text from what it links, and starts with YAML front matter, so the
//! folder reads as an Obsidian vault and on GitHub alike (KNOW-14). No gate reads it.

use crate::journal::Post;
use crate::records::{local_path, read_native_text};
use anyhow::Result;
use std::collections::BTreeSet;
use std::path::Path;

/// One front matter value.
pub(super) enum Prop {
    /// A plain token written as is: a note type, status or date.
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

/// The newest journal posts the index lists (REQ-cli-043).
const RECENT_POSTS: usize = 10;

/// The module folders under `specs/`, sorted.
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

/// A module's lesson source and how many lessons it records: the planes after the first
/// of `<module>.3md` when that file exists, otherwise the dated entries of the
/// `## Lessons` section of `context.md`. No lesson text is read out.
fn lesson_source(root: &Path, module: &str) -> (String, usize) {
    let native = format!("specs/{module}/{module}.3md");
    if let Some(text) = read(root, &native) {
        let planes = text
            .lines()
            .filter(|line| line.starts_with("@plane "))
            .count();
        return (native, planes.saturating_sub(1));
    }
    let context = format!("specs/{module}/context.md");
    let count = read(root, &context).map_or(0, |text| dated_lessons(&text));
    (context, count)
}

/// Lessons recorded in a `## Lessons` section: entries that start with a `YYYY-MM-DD`
/// date followed by a space, comma or colon, as a list item or a paragraph, and
/// `### YYYY-MM-DD` headings, whose date may stand alone.
fn dated_lessons(text: &str) -> usize {
    text.lines()
        .skip_while(|line| line.trim_end() != "## Lessons")
        .skip(1)
        .take_while(|line| !line.starts_with("## "))
        .filter(|line| match line.strip_prefix("### ") {
            Some(heading) => starts_with_date(heading.trim_end(), true),
            None => {
                let entry = line
                    .strip_prefix("- ")
                    .or_else(|| line.strip_prefix("* "))
                    .unwrap_or(line);
                starts_with_date(entry, false)
            }
        })
        .count()
}

/// Whether `text` starts with a `YYYY-MM-DD` date followed by a space, comma or colon;
/// a heading's date may also end the line.
fn starts_with_date(text: &str, heading: bool) -> bool {
    let bytes = text.as_bytes();
    let dated = bytes.len() >= 10
        && bytes[..10].iter().enumerate().all(|(at, byte)| match at {
            4 | 7 => *byte == b'-',
            _ => byte.is_ascii_digit(),
        });
    dated
        && match bytes.get(10) {
            None => heading,
            Some(b' ' | b',' | b':') => true,
            Some(_) => false,
        }
}

/// The title of the archived change in `dir`: the first `# ` heading of its `change.md`
/// after the front matter, read at full size within SpecSync's artifact bound, never
/// Chough's record bound (REQ-cli-042). None when the file is missing or unreadable.
fn archive_title(root: &Path, dir: &str) -> Option<String> {
    let text = read(
        root,
        &format!("{}/{dir}/change.md", crate::library::ARCHIVES),
    )?;
    let mut lines = text.lines().peekable();
    if lines.peek().is_some_and(|line| line.trim_end() == "---") {
        lines.next();
        for line in lines.by_ref() {
            if line.trim_end() == "---" {
                break;
            }
        }
    }
    lines
        .find_map(|line| line.strip_prefix("# "))
        .map(one_line)
        .filter(|title| !title.is_empty())
}

/// Every file under `relative`, sorted, as paths from the root.
fn files_under(root: &Path, relative: &str) -> Result<Vec<String>> {
    let mut found = vec![];
    let mut stack = vec![relative.to_string()];
    while let Some(folder) = stack.pop() {
        for name in crate::library::sorted_entries(root, &folder)? {
            let child = format!("{folder}/{name}");
            if std::fs::symlink_metadata(local_path(root, &child)?)?.is_dir() {
                stack.push(child);
            } else {
                found.push(child);
            }
        }
    }
    found.sort();
    Ok(found)
}

/// The project's rule, skill, decision and policy files that exist (REQ-cli-043).
fn rule_files(root: &Path) -> Result<Vec<String>> {
    let mut rules = vec![
        "AGENTS.md".to_string(),
        "CLAUDE.md".into(),
        "hi/AGENTS.md".into(),
    ];
    let mut installed: Vec<&str> = crate::skill::HOSTS.iter().map(|(_, dir)| *dir).collect();
    installed.push(".claude/skills");
    installed.sort_unstable();
    installed.dedup();
    for dir in installed {
        rules.push(format!("{dir}/chough-workflow/SKILL.md"));
    }
    rules.push("skills/chough-workflow/SKILL.md".into());
    rules.push("docs/DECISIONS.md".into());
    for module in modules(root)? {
        rules.push(format!("specs/{module}/context.md"));
        rules.push(format!("specs/{module}/{module}.3md"));
    }
    rules.push(".specsync/sdd.json".into());
    Ok(rules
        .into_iter()
        .filter(|path| exists(root, path))
        .collect())
}

/// The file of open bug `id`: its Markdown record, or its JSON record before plan step 5.
fn bug_file(root: &Path, id: &str) -> Option<String> {
    [
        format!("{}/{id}.md", crate::bugs::BUGS_DIR),
        format!("{}/{id}.json", crate::bugs::BUGS_DIR),
    ]
    .into_iter()
    .find(|path| exists(root, path))
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Text from a record or a file name, inert in Markdown (REQ-cli-044): one line, with each
/// backslash, bracket, angle bracket and backtick escaped. So it can neither end a link
/// early nor start another, open a code span past its own text, or become HTML, and a
/// reader sees it unchanged.
fn inert(text: &str) -> String {
    let mut out = String::new();
    for c in one_line(text).chars() {
        if matches!(c, '\\' | '[' | ']' | '<' | '>' | '`') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

fn list(items: Vec<String>, empty: &str) -> String {
    if items.is_empty() {
        format!("- {empty}\n")
    } else {
        items.iter().map(|item| format!("- {item}\n")).collect()
    }
}

/// What a migration plan changes before the index it writes is read (REQ-records-009):
/// files the earlier mappers write count as present, files the plan removes as gone, and
/// the bug records it writes join the open bugs, so the planned `index.md` is the one
/// `chough library index` builds after the apply. Outside a migration it is empty.
/// One open bug as the index lists it: its record file (None without one), ID, title
/// and status.
type OpenBug = (Option<String>, String, String, String);

#[derive(Default)]
pub(super) struct Pending {
    /// Whether a migration plan is being built; a 0.5.x bug record is then left to the
    /// bug mapper instead of refused.
    pub(crate) migrating: bool,
    /// Paths the plan writes.
    pub(crate) present: BTreeSet<String>,
    /// Paths the plan removes.
    pub(crate) gone: BTreeSet<String>,
    /// Open bugs the plan writes: (file, id, title, status).
    pub(crate) bugs: Vec<(String, String, String, String)>,
}

impl Pending {
    /// Whether `relative`, a file or a folder ending in `/`, exists once the plan applies.
    fn exists(&self, root: &Path, relative: &str) -> bool {
        let folder = format!("{}/", relative.trim_end_matches('/'));
        if self
            .present
            .iter()
            .any(|path| path == relative || path.starts_with(&folder))
        {
            return true;
        }
        !self.gone.contains(relative) && exists(root, relative)
    }

    /// The open bugs once the plan applies, by file name as `chough bug` reads them:
    /// (file, id, title, status), the file None when the bug has no record file.
    fn open_bugs(&self, root: &Path) -> Result<Vec<OpenBug>> {
        if !self.migrating {
            return Ok(crate::bugs::open_backlog(root)?
                .into_iter()
                .map(|bug| {
                    let id = bug["id"].as_str().unwrap_or("?").to_string();
                    let title = bug["title"].as_str().unwrap_or("").to_string();
                    let status = bug["status"].as_str().unwrap_or("?").to_string();
                    (bug_file(root, &id), id, title, status)
                })
                .collect());
        }
        let bound = crate::markdown::Kind::Bug.bound();
        let mut found: Vec<OpenBug> = crate::bugs::records_in(root, crate::bugs::BUGS_DIR, bound)
            .into_iter()
            .map(|(name, record)| {
                let field = |key: &str| record.scalar(key).unwrap_or_default().to_string();
                let file = format!("{}/{name}", crate::bugs::BUGS_DIR);
                (Some(file), field("id"), field("title"), field("status"))
            })
            .filter(|(file, ..)| file.as_ref().is_some_and(|file| !self.gone.contains(file)))
            .filter(|(.., status)| !crate::bugs::is_closed_status(status))
            .collect();
        found.extend(self.bugs.iter().map(|(file, id, title, status)| {
            (
                Some(file.clone()),
                id.clone(),
                title.clone(),
                status.clone(),
            )
        }));
        found.sort();
        found.dedup();
        Ok(found)
    }
}

/// `index.md` for the knowledge directory `dir`, generated from its sources with
/// identical bytes for the same sources (REQ-cli-043, KNOW-4). It links and copies no
/// text: posts by title, bugs and archived changes by ID and title, lesson sources by
/// path and count.
pub(super) fn index(root: &Path, dir: &str, posts: &[Post], pending: &Pending) -> Result<String> {
    let up = "../".repeat(dir.split('/').filter(|part| !part.is_empty()).count());
    let link = |path: &str| format!("[{}]({up}{path})", inert(path));
    let newest: Vec<String> = posts
        .iter()
        .take(RECENT_POSTS)
        .map(|post| {
            let kind = if post.kind.is_empty() {
                String::new()
            } else {
                format!(", {}", post.kind)
            };
            format!(
                "[{title}](journal/{name}.md) ({date}{kind})",
                title = inert(&post.title),
                name = post.name,
                date = post.date,
            )
        })
        .collect();
    let places: Vec<String> = [
        ("Intent and acceptance criteria", "hi/"),
        ("Contracts and requirements", "specs/"),
        ("Archived native changes", ".specsync/archive/"),
        ("Chough's records", ".chough/README.md"),
        ("Finished sessions", ".chough/archive/sessions/"),
        ("Tasks", "fledge.toml"),
    ]
    .into_iter()
    .filter(|(_, path)| pending.exists(root, path))
    .map(|(what, path)| format!("{what}: {}", link(path)))
    .collect();
    let bugs: Vec<String> = pending
        .open_bugs(root)?
        .into_iter()
        .map(|(file, id, title, status)| {
            let named = file.map_or(inert(&id), |path| format!("[{}]({up}{path})", inert(&id)));
            format!("{named} {} ({})", inert(&title), inert(&status))
        })
        .collect();
    let rules: Vec<String> = rule_files(root)?.iter().map(|path| link(path)).collect();
    let sources: Vec<String> = modules(root)?
        .into_iter()
        .map(|module| {
            let (path, count) = lesson_source(root, &module);
            let shown = if exists(root, &path) {
                link(&path)
            } else {
                format!("`{path}` (not present)")
            };
            let noun = if count == 1 { "lesson" } else { "lessons" };
            format!("{module}: {shown}, {count} {noun}")
        })
        .collect();
    let lessons: Vec<String> = files_under(root, &format!("{dir}/lessons"))?
        .into_iter()
        .map(|path| {
            let inside = path.strip_prefix(&format!("{dir}/")).unwrap_or(&path);
            format!("[{}]({inside})", inert(inside))
        })
        .collect();
    // An archive that cannot be read refuses the index, with the error `chough next`
    // shows for its lesson bundles.
    // The newest 3 archives by finalization, as `chough next` shows them: a bundle an
    // archive lacks is named as missing, not linked.
    let bundles: Vec<String> = crate::library::archive_order(root)?
        .into_iter()
        .take(crate::library::HOT_ARCHIVES)
        .map(|archive| {
            let path = format!(
                "{archives}/{dir}/lesson-bundle.md",
                archives = crate::library::ARCHIVES,
                dir = archive.dir,
            );
            let named = if archive.lesson_bundle {
                format!("[{id}]({up}{path})", id = inert(&archive.id))
            } else {
                format!("{id}: `{path}` (missing)", id = inert(&archive.id))
            };
            match archive_title(root, &archive.dir) {
                Some(title) => format!("{named}, {}", inert(&title)),
                None => named,
            }
        })
        .collect();
    let mut kept = files_under(root, &format!("{dir}/briefs"))?;
    for name in crate::library::sorted_entries(root, &format!("{dir}/snapshots"))? {
        let manifest = format!("{dir}/snapshots/{name}/manifest.json");
        if exists(root, &manifest) {
            kept.push(manifest);
        }
    }
    let prefix = format!("{dir}/");
    let mut kept: Vec<String> = kept
        .into_iter()
        .map(|path| {
            let inside = path.strip_prefix(&prefix).unwrap_or(&path).to_string();
            format!("[{}]({inside})", inert(&inside))
        })
        .collect();
    if dir != "library" {
        kept.extend(files_under(root, "library")?.iter().map(|path| link(path)));
    }
    let archives_link = if exists(root, crate::library::ARCHIVES) {
        format!(
            "Changes archived before the journal began are in {}.",
            link(&format!("{}/", crate::library::ARCHIVES))
        )
    } else {
        "No native change was archived before the journal began.".to_string()
    };
    let props = front_matter(&[
        ("id", Prop::Text("index".into())),
        ("title", Prop::Text("Project knowledge".into())),
        ("type", Prop::Raw("guide".into())),
        ("tags", Prop::List(vec!["chough/index".into()])),
    ]);
    Ok(format!(
        "{props}# Project knowledge\n\n\
Generated by `chough library index` from the files it links. It links them and copies\n\
nothing from them, and no gate reads it.\n\n\
## Where to start\n\n\
Read the journal below for what changed lately, newest first. Then read the project's\n\
rules and the lesson source of the module you will work on. Intent, contracts, native\n\
changes and Chough's records each stay where they live, linked below.\n\n\
## Journal: the timeline of archived changes\n\n\
The [journal](journal/) and its [feed](journal/feed.xml) are the timeline of archived\n\
changes. {archives_link}\n\n\
Newest posts:\n\n{newest}\n\
## Where things live\n\n{places}\n\
## Open bugs\n\n{bugs}\n\
## Rules, skills and decisions\n\n{rules}\n\
## Lessons\n\n\
Each module's lessons stay in its lesson source:\n\n{sources}\n\
Lessons that span projects:\n\n{lessons}\n\
Lesson bundles of the newest 3 archived changes:\n\n{bundles}\n\
## Snapshots and briefs\n\n\
Kept as they are.\n\n{kept}",
        newest = list(newest, "none yet"),
        places = list(places, "none"),
        bugs = list(bugs, "none"),
        rules = list(rules, "none"),
        sources = list(sources, "none"),
        lessons = list(lessons, "none"),
        bundles = list(bundles, "none"),
        kept = list(kept, "none"),
    ))
}
