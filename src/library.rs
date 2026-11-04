//! Project-local librarian, on by default in `knowledge/` with a recorded opt-out:
//! source-linked snapshots, briefs, one regenerated entry per archived native change, a
//! lessons index and a deterministic INDEX.md. Canonical sources stay where their owners
//! keep them; nothing here moves or edits them, and no gate reads this directory.

use crate::records::{
    digest_bytes, ensure_text, head_revision, local_path, now, print_json, read_record, write_new,
    write_new_bytes, write_replace,
};
use crate::{bugs, evidence, prefs};
use anyhow::{Context, Result, ensure};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;

const SNAPSHOT_FILE_LIMIT: u64 = 4 * 1_048_576;
const SKIP_DIRS: [&str; 5] = [".git", "target", "node_modules", ".chough", ".specsync"];

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    source: String,
    copy: String,
    sha256: String,
    bytes: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema_version: u32,
    reason: String,
    revision: Option<String>,
    created_at_unix_seconds: u64,
    entries: Vec<Entry>,
}

#[derive(Subcommand)]
pub(super) enum LibraryCommand {
    /// Create the library folder with its README and first index.
    Init,
    /// Copy originals into a dated snapshot with source path, revision and SHA-256.
    Snapshot {
        paths: Vec<String>,
        #[arg(long)]
        reason: String,
    },
    /// Regenerate the deterministic INDEX.md and the archived-change entries.
    Index,
    /// Preview (default) or apply the post-archive summary: proof only cold archived
    /// changes list, committed identically at HEAD, kept as a summary entry.
    Archive {
        /// Remove the eligible files; needs retention authority and no open change.
        #[arg(long)]
        apply: bool,
    },
}

pub(super) const HOT_ARCHIVES: usize = 3;
/// Rows each knowledge entry lists in full before pointing at the manifest.
const ENTRY_ROWS: usize = 100;
/// The managed-file bound every generated knowledge file stays within.
const ENTRY_LIMIT: usize = 65_536;
const ARCHIVES: &str = ".specsync/archive/changes";

/// One archived native change, read from its local records.
struct Archive {
    dir: String,
    id: String,
    timestamp: u64,
    title: String,
    base_commit: Option<String>,
    implementation_commit: Option<String>,
    verification_commit: Option<String>,
    requirements: Vec<String>,
    criteria: Vec<String>,
    /// (path, payload digest) of each listed `.chough/` file.
    listed: Vec<(String, String)>,
    all_paths: Vec<String>,
    lesson_bundle: bool,
    /// Why the verification manifest could not be read, if it could not.
    manifest_error: Option<String>,
}

/// A native archive record read within the native bound. Cumulative verification
/// manifests outgrow the 64 KiB structured-record bound, so they use the native one.
fn read_json(root: &Path, relative: &str) -> std::result::Result<serde_json::Value, String> {
    let path = local_path(root, relative).map_err(|error| format!("{relative}: {error}"))?;
    let bytes = crate::records::read_native_record(&path, crate::records::NATIVE_RECORD_LIMIT)
        .map_err(|error| format!("{relative}: {error}"))?
        .ok_or_else(|| {
            format!(
                "{relative} exceeds {} bytes",
                crate::records::NATIVE_RECORD_LIMIT
            )
        })?;
    serde_json::from_slice(&bytes).map_err(|error| format!("{relative}: {error}"))
}

/// The change ID and finalization timestamp an archive directory records, when its
/// `finalization.json` reads within the native bound and its directory name ends with
/// that ID, the same check `archives` uses.
pub(super) fn finalized(root: &Path, dir: &str) -> Option<(String, u64)> {
    let value = read_json(root, &format!("{ARCHIVES}/{dir}/finalization.json")).ok()?;
    let id = value["change_id"].as_str().filter(|id| dir.ends_with(id))?;
    Some((id.to_string(), value["timestamp"].as_u64()?))
}

/// When the archived native change `change` was finalized, if an archive records it.
pub(super) fn finalized_at(root: &Path, change: &str) -> Option<u64> {
    sorted_entries(root, ARCHIVES)
        .ok()?
        .iter()
        .filter(|dir| dir.ends_with(change))
        .filter_map(|dir| finalized(root, dir))
        .find(|(id, _)| id == change)
        .map(|(_, timestamp)| timestamp)
}

/// One archive directory the librarian counts: a UTF-8 name that is not hidden, a
/// directory, and a `finalization.json` that reads and names a change ID the directory
/// name ends with.
pub(super) struct ArchiveDir {
    pub(super) dir: String,
    id: String,
    timestamp: u64,
    finalization: serde_json::Value,
    pub(super) lesson_bundle: bool,
}

/// The archives the librarian counts, newest first by (finalization timestamp,
/// directory); an unreadable timestamp sorts as 0. This is the one rule `chough next`
/// and the knowledge index share for the hot archives.
pub(super) fn archive_order(root: &Path) -> Result<Vec<ArchiveDir>> {
    let mut found = vec![];
    for dir in sorted_entries(root, ARCHIVES)? {
        let base = format!("{ARCHIVES}/{dir}");
        if !local_path(root, &base)?.is_dir() {
            continue;
        }
        let Ok(finalization) = read_json(root, &format!("{base}/finalization.json")) else {
            continue;
        };
        let Some(id) = finalization["change_id"]
            .as_str()
            .filter(|id| dir.ends_with(id))
            .map(str::to_string)
        else {
            continue;
        };
        found.push(ArchiveDir {
            timestamp: finalization["timestamp"].as_u64().unwrap_or(0),
            lesson_bundle: local_path(root, &format!("{base}/lesson-bundle.md"))?.is_file(),
            dir,
            id,
            finalization,
        });
    }
    found.sort_by(|left, right| (right.timestamp, &right.dir).cmp(&(left.timestamp, &left.dir)));
    Ok(found)
}

/// Archived changes newest first by (finalization timestamp, directory).
fn archives(root: &Path) -> Result<Vec<Archive>> {
    let mut found = vec![];
    for ArchiveDir {
        dir,
        id,
        timestamp,
        finalization,
        lesson_bundle,
    } in archive_order(root)?
    {
        let base = format!("{ARCHIVES}/{dir}");
        let (verification, manifest_error) =
            match read_json(root, &format!("{base}/verification.json")) {
                Ok(value) if value["acceptance_manifest"]["entries"].is_array() => (value, None),
                Ok(_) => (
                    serde_json::Value::Null,
                    Some(format!(
                        "{base}/verification.json has no acceptance_manifest entries"
                    )),
                ),
                Err(error) => (serde_json::Value::Null, Some(error)),
            };
        let change = local_path(root, &format!("{base}/change.md"))
            .ok()
            .and_then(|path| read_record(&path).ok())
            .unwrap_or_default();
        let base_commit = change
            .lines()
            .take_while(|line| !line.starts_with("# "))
            .find_map(|line| line.strip_prefix("base_commit:"))
            .map(|value| value.trim().to_string());
        let title = change
            .lines()
            .find_map(|line| line.strip_prefix("# "))
            .unwrap_or(&id)
            .trim()
            .to_string();
        let criteria = change
            .lines()
            .skip_while(|line| line.trim_end() != "## Acceptance Criteria")
            .skip(1)
            .take_while(|line| !line.starts_with("## "))
            .filter_map(|line| line.strip_prefix("- "))
            .map(|line| line.trim().to_string())
            .collect();
        let entries = verification["acceptance_manifest"]["entries"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        let mut listed = vec![];
        let mut all_paths = vec![];
        for entry in entries {
            let Some(path) = entry["path"].as_str() else {
                continue;
            };
            all_paths.push(path.to_string());
            if path.starts_with(".chough/") {
                listed.push((
                    path.to_string(),
                    entry["payload_digest"].as_str().unwrap_or("?").to_string(),
                ));
            }
        }
        let requirements = verification["requirement_ids"]
            .as_array()
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| id.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        let text = |value: &serde_json::Value| value.as_str().map(str::to_string);
        found.push(Archive {
            timestamp,
            implementation_commit: text(&finalization["implementation_commit"]),
            verification_commit: text(&verification["commit"]),
            lesson_bundle,
            dir,
            id,
            title,
            base_commit,
            requirements,
            criteria,
            listed,
            all_paths,
            manifest_error,
        });
    }
    found.sort_by(|left, right| (right.timestamp, &right.dir).cmp(&(left.timestamp, &left.dir)));
    Ok(found)
}

fn one_line(text: &str) -> String {
    text.replace(['\n', '\r', '|'], " ")
}

/// The knowledge entry for one archived change: generated, deterministic, not a ledger.
fn archive_entry(
    root: &Path,
    archive: &Archive,
    hot: bool,
    first: &[(String, String)],
    ledger: &std::collections::BTreeMap<String, serde_json::Value>,
    bug_refs: &[(String, Vec<String>)],
) -> Result<String> {
    let full = archive_entry_rows(root, archive, hot, first, ledger, bug_refs, ENTRY_ROWS)?;
    if full.len() <= ENTRY_LIMIT {
        return Ok(full);
    }
    let counts = archive_entry_rows(root, archive, hot, first, ledger, bug_refs, 0)?;
    ensure!(
        counts.len() <= ENTRY_LIMIT,
        "knowledge entry for {} exceeds {ENTRY_LIMIT} bytes even without its file tables",
        archive.id
    );
    Ok(counts)
}

/// One knowledge entry listing at most `rows` file and observation rows; the rest are
/// counted and left to the native manifest.
fn archive_entry_rows(
    root: &Path,
    archive: &Archive,
    hot: bool,
    first: &[(String, String)],
    ledger: &std::collections::BTreeMap<String, serde_json::Value>,
    bug_refs: &[(String, Vec<String>)],
    rows_limit: usize,
) -> Result<String> {
    let base = format!("{ARCHIVES}/{}", archive.dir);
    let commit = |value: &Option<String>| value.clone().unwrap_or_else(|| "unrecorded".into());
    let mut removed_at: std::collections::BTreeMap<&str, &str> = Default::default();
    for entry in ledger.values() {
        let head = entry["head"].as_str();
        for path in entry["removed"].as_array().into_iter().flatten() {
            if let (Some(path), Some(head)) = (path.as_str(), head) {
                removed_at.insert(path, head);
            }
        }
    }
    let fallback = archive
        .verification_commit
        .clone()
        .unwrap_or_else(|| "unrecorded".into());
    let mut files = archive
        .manifest_error
        .as_ref()
        .map(|error| format!("| (manifest unreadable: {}) | | |\n", one_line(error)))
        .unwrap_or_default();
    let mut observations = vec![];
    for (path, listed_digest) in first.iter().take(rows_limit) {
        let full = local_path(root, path)?;
        let location = if full.is_file() {
            if crate::records::file_digest(root, path).is_ok_and(|digest| digest == *listed_digest)
            {
                "working tree".to_string()
            } else {
                "working tree (changed since archive)".to_string()
            }
        } else {
            format!(
                "Git {}",
                removed_at.get(path.as_str()).copied().unwrap_or(&fallback)
            )
        };
        files.push_str(&format!("| `{path}` | {listed_digest} | {location} |\n"));
        if let Some(id) = path
            .strip_prefix(".chough/evidence/")
            .and_then(|name| name.strip_suffix(".json"))
        {
            observations.push(id.to_string());
        }
    }
    let shown = first.len().min(rows_limit);
    let mut files_note = format!(
        "{listed} `.chough/` paths of {total} manifest entries; {first} first listed by this archive, \
{shown} shown below. Paths first listed by an earlier archive are summarized there; the full \
list is the native manifest [verification.json](../../{base}/verification.json).",
        listed = archive.listed.len(),
        total = archive.all_paths.len(),
        first = first.len(),
    );
    if shown < first.len() {
        files_note.push_str(&format!(
            " {} more first listed here are not shown; read them from the manifest.",
            first.len() - shown
        ));
    }
    let mut rows = String::new();
    for id in &observations {
        let (outcome, environment, cases) = match evidence::observation_summary(root, id) {
            Some(summary) => summary,
            None => match ledger.get(id) {
                Some(entry) => (
                    format!(
                        "{} (pruned)",
                        entry["outcome"].as_str().unwrap_or("unknown")
                    ),
                    entry["environment"].as_str().unwrap_or("?").to_string(),
                    entry["cases"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(|case| case.as_str().map(str::to_string))
                        .collect(),
                ),
                None => ("unavailable".into(), "?".into(), vec![]),
            },
        };
        rows.push_str(&format!(
            "| {id} | {outcome} | {environment} | {} |\n",
            cases.join(", ")
        ));
    }
    let bugs: Vec<String> = bug_refs
        .iter()
        .filter_map(|(bug, ids)| {
            let shared: Vec<&String> = ids.iter().filter(|id| observations.contains(id)).collect();
            (!shared.is_empty()).then(|| {
                format!(
                    "{bug} (references {})",
                    shared
                        .iter()
                        .map(|id| id.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
        })
        .collect();
    let list = |items: &[String]| {
        if items.is_empty() {
            "- none\n".to_string()
        } else {
            items
                .iter()
                .map(|item| format!("- {}\n", one_line(item)))
                .collect()
        }
    };
    Ok(format!(
        "# {id}: {title}\n\n\
Generated by `chough library index` from [{base}](../../{base}/). \n\
This summary is not a ledger and no gate reads it: the archive, its finalization and\n\
verification records, the retention ledger and Git stay authoritative.\n\n\
- Status: {status} (hot = the {HOT_ARCHIVES} most recently finalized archived changes)\n\
- Finalized: {timestamp}\n\
- Base commit: {base_commit}\n\
- Implementation commit: {implementation}\n\
- Verification commit: {verification}\n\
- Requirements: {requirements}\n\
- Lesson bundle: {lesson}\n\n\
## Acceptance criteria\n\n{criteria}\n\
## Listed .chough files\n\n{files_note} Location is where the bytes are now.\n\n\
| Path | Digest | Location |\n|---|---|---|\n{files}\n\
## Observations\n\n| ID | Outcome | Environment | Cases |\n|---|---|---|---|\n{rows}\n\
## Bugs\n\n{bugs}",
        id = archive.id,
        title = one_line(&archive.title),
        status = if hot { "hot" } else { "cold" },
        timestamp = archive.timestamp,
        base_commit = commit(&archive.base_commit),
        implementation = commit(&archive.implementation_commit),
        verification = commit(&archive.verification_commit),
        requirements = if archive.requirements.is_empty() {
            "none recorded".to_string()
        } else {
            archive.requirements.join(", ")
        },
        lesson = if archive.lesson_bundle {
            format!("[lesson-bundle.md](../../{base}/lesson-bundle.md)")
        } else {
            "none".into()
        },
        criteria = list(&archive.criteria),
        files_note = files_note,
        files = files,
        rows = rows,
        bugs = list(&bugs),
    ))
}

/// Regenerate every archived-change entry and the index.
fn regenerate(root: &Path, dir: &str) -> Result<()> {
    write_all(root, dir, build(root, dir)?)
}

/// Every generated knowledge file, built and bounded in memory before anything is written.
fn build(root: &Path, dir: &str) -> Result<Vec<(String, String)>> {
    let found = archives(root)?;
    let ledger = evidence::ledger_records(root)?;
    let bug_refs = bugs::observation_references(root)?;
    // Cumulative manifests repeat earlier proof; each entry details what it first listed.
    let mut seen = std::collections::BTreeSet::new();
    let mut first: Vec<Vec<(String, String)>> = vec![vec![]; found.len()];
    for (index, archive) in found.iter().enumerate().rev() {
        first[index] = archive
            .listed
            .iter()
            .filter(|(path, _)| seen.insert(path.clone()))
            .cloned()
            .collect();
    }
    let mut files = vec![];
    for (index, archive) in found.iter().enumerate() {
        let content = archive_entry(
            root,
            archive,
            index < HOT_ARCHIVES,
            &first[index],
            &ledger,
            &bug_refs,
        )?;
        files.push((format!("{dir}/archives/{}.md", archive.id), content));
    }
    let content = index(root, dir, &found)?;
    ensure!(
        content.len() <= ENTRY_LIMIT,
        "index exceeds {ENTRY_LIMIT} bytes; summarize older snapshots in a brief"
    );
    files.push((format!("{dir}/INDEX.md"), content));
    Ok(files)
}

fn write_all(root: &Path, dir: &str, files: Vec<(String, String)>) -> Result<()> {
    fs::create_dir_all(local_path(root, &format!("{dir}/archives"))?)?;
    for (relative, content) in files {
        write_replace(root, &relative, &content)?;
    }
    Ok(())
}

fn enabled_dir(root: &Path) -> Result<String> {
    let (enabled, origin) = prefs::effective(root, "librarian")?;
    ensure!(
        enabled == "on",
        "the librarian is off here ({origin}); turn it back on with `chough pref set librarian on --source <who asked>`"
    );
    Ok(prefs::effective(root, "library-dir")?.0)
}

fn sorted_entries(root: &Path, relative: &str) -> Result<Vec<String>> {
    let path = if relative.is_empty() {
        root.to_path_buf()
    } else {
        local_path(root, relative)?
    };
    if !path.is_dir() {
        return Ok(vec![]);
    }
    let mut names = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str()
            && !name.starts_with('.')
        {
            names.push(name.to_string());
        }
    }
    names.sort();
    Ok(names)
}

fn find_companions(
    root: &Path,
    relative: &str,
    depth: usize,
    found: &mut Vec<String>,
) -> Result<()> {
    if depth == 0 {
        return Ok(());
    }
    for name in sorted_entries(root, relative)? {
        if SKIP_DIRS.contains(&name.as_str()) {
            continue;
        }
        let child = if relative.is_empty() {
            name.clone()
        } else {
            format!("{relative}/{name}")
        };
        let path = root.join(&child);
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.is_dir() {
            find_companions(root, &child, depth - 1, found)?;
        } else if name == "COMPANION.md" {
            found.push(relative.to_string());
        }
    }
    Ok(())
}

fn index(root: &Path, dir: &str, found: &[Archive]) -> Result<String> {
    let list = |items: Vec<String>| {
        if items.is_empty() {
            "- none\n".to_string()
        } else {
            items.iter().map(|item| format!("- {item}\n")).collect()
        }
    };
    let hi_files = sorted_entries(root, "hi")?
        .into_iter()
        .filter(|name| name.ends_with(".md") && name != "AGENTS.md" && name != "CLAUDE.md")
        .map(|name| format!("[hi/{name}](../hi/{name})"))
        .collect();
    let modules = sorted_entries(root, "specs")?
        .into_iter()
        .filter(|name| root.join("specs").join(name).is_dir())
        .map(|name| format!("[specs/{name}/](../specs/{name}/)"))
        .collect();
    let mut prototypes = vec![];
    find_companions(root, "", 5, &mut prototypes)?;
    let prototypes = prototypes
        .into_iter()
        .filter(|path| !path.starts_with(dir))
        .map(|path| format!("[{path}/](../{path}/) with COMPANION.md"))
        .collect();
    let active = sorted_entries(root, ".specsync/changes")?
        .into_iter()
        .map(|name| format!("{name} (active; `specsync change status {name}`)"))
        .collect();
    let archived = found
        .iter()
        .enumerate()
        .map(|(index, archive)| {
            format!(
                "[{id}](archives/{id}.md) {title} ({status})",
                id = archive.id,
                title = one_line(&archive.title),
                status = if index < HOT_ARCHIVES { "hot" } else { "cold" }
            )
        })
        .collect();
    let lesson_sources = sorted_entries(root, "specs")?
        .into_iter()
        .filter(|name| root.join("specs").join(name).is_dir())
        .map(|module| {
            let source = crate::lesson_source(root, &module);
            let path = source["path"].as_str().unwrap_or("?").to_string();
            let detail = match (source["section_line"].as_u64(), source["note"].as_str()) {
                (Some(line), _) => format!(" (## Lessons at line {line})"),
                (None, Some(note)) => format!(" ({note})"),
                _ => String::new(),
            };
            format!("{module}: [{path}](../{path}){detail}")
        })
        .collect();
    let bundles = found
        .iter()
        .filter(|archive| archive.lesson_bundle)
        .map(|archive| {
            format!(
                "[{id}](../{ARCHIVES}/{dir}/lesson-bundle.md)",
                id = archive.id,
                dir = archive.dir
            )
        })
        .collect();
    let mut earlier = vec![];
    if dir != "library" {
        let mut stack = vec!["library".to_string()];
        while let Some(relative) = stack.pop() {
            for name in sorted_entries(root, &relative)?.into_iter().rev() {
                let child = format!("{relative}/{name}");
                let path = local_path(root, &child)?;
                if fs::symlink_metadata(&path)?.is_dir() {
                    stack.push(child);
                } else {
                    earlier.push(child);
                }
            }
        }
        earlier.sort();
    }
    let earlier = earlier
        .into_iter()
        .map(|path| format!("[{path}](../{path})"))
        .collect();
    let open = bugs::open_backlog(root)?
        .into_iter()
        .map(|bug| {
            format!(
                "{} {} ({}, {}, {})",
                bug["id"].as_str().unwrap_or("?"),
                bug["title"].as_str().unwrap_or("?"),
                bug["status"].as_str().unwrap_or("?"),
                bug["scope"].as_str().unwrap_or("?"),
                bug["verification"].as_str().unwrap_or("?")
            )
        })
        .collect();
    let mut observations = evidence::load_all(root)?;
    observations.sort_by(|left, right| {
        right
            .observed_at_unix_seconds
            .cmp(&left.observed_at_unix_seconds)
            .then(left.id.cmp(&right.id))
    });
    let total = observations.len();
    let recent = observations
        .iter()
        .take(10)
        .map(|record| {
            format!(
                "{} {:?} {:?} in {} at {}",
                record.id,
                record.kind,
                record.outcome,
                record.environment,
                &record.snapshot.revision[..record.snapshot.revision.len().min(12)]
            )
        })
        .collect();
    let briefs = sorted_entries(root, &format!("{dir}/briefs"))?
        .into_iter()
        .map(|name| format!("[{name}](briefs/{name})"))
        .collect();
    let mut snapshots = vec![];
    for name in sorted_entries(root, &format!("{dir}/snapshots"))? {
        let manifest = format!("{dir}/snapshots/{name}/manifest.json");
        let stored = read_record(&local_path(root, &manifest)?)
            .ok()
            .and_then(|text| serde_json::from_str::<Manifest>(&text).ok());
        snapshots.push((name, stored));
    }
    snapshots.sort_by(|(left_name, left), (right_name, right)| {
        right
            .as_ref()
            .map(|manifest| manifest.created_at_unix_seconds)
            .cmp(
                &left
                    .as_ref()
                    .map(|manifest| manifest.created_at_unix_seconds),
            )
            .then(left_name.cmp(right_name))
    });
    let snapshots = snapshots
        .into_iter()
        .map(|(name, manifest)| {
            let reason = manifest.map_or_else(
                || "manifest unreadable".to_string(),
                |manifest| manifest.reason,
            );
            format!("[{name}](snapshots/{name}/manifest.json): {reason}")
        })
        .collect::<Vec<_>>();
    Ok(format!(
        "# Project knowledge index\n\n\
Generated by `chough library index`. This index points to living sources; it does not\n\
replace them, and no gate reads it. Snapshots are historical copies with source links and\n\
digests; each archived change has one regenerated entry under `archives/`.\n\n\
## Living sources (owned elsewhere)\n\n\
Intent: [INTENT.md](../INTENT.md)\n\n{hi}\n\
Requirements: [docs/FRD.md](../docs/FRD.md) when present, and native module requirements:\n\n{modules}\n\
Canonical prototypes:\n\n{prototypes}\n\
## Native SpecSync changes\n\nActive:\n\n{active}\nArchived changes (newest first; the newest {HOT_ARCHIVES} are hot):\n\n{archived}\n\
## Lessons index\n\nPointers only; the lessons stay in their sources.\n\nModule lesson sources:\n\n{lesson_sources}\n\
Archived lesson bundles:\n\n{bundles}\n\
## Open bug backlog (live)\n\n{open}\n\
## Observations\n\n{total} recorded; most recent:\n\n{recent}\n\
## Briefs\n\n{briefs}\n\
## Snapshots\n\n{snapshots}\n\
## Earlier library folder\n\nKept as is; listed here so nothing in it is lost.\n\n{earlier}",
        hi = list(hi_files),
        modules = list(modules),
        prototypes = list(prototypes),
        active = list(active),
        archived = list(archived),
        lesson_sources = list(lesson_sources),
        bundles = list(bundles),
        earlier = list(earlier),
        open = list(open),
        total = total,
        recent = list(recent),
        briefs = list(briefs),
        snapshots = list(snapshots),
    ))
}

fn write_snapshot(
    root: &Path,
    dir: &str,
    id: &str,
    sources: Vec<(String, Vec<u8>)>,
    reason: &str,
    at: u64,
) -> Result<serde_json::Value> {
    let base = format!("{dir}/snapshots/{id}");
    let path = local_path(root, &base)?;
    // Read the HEAD first: a stop during that Git ends Chough before anything is
    // written (FLOW-14). The snapshot is then staged under a hidden name, which the
    // index skips, and renamed into place whole, so no copy ever lacks its manifest.
    let revision = head_revision(root);
    crate::records::stop_if_noted();
    fs::create_dir_all(path.parent().context("snapshot has no parent")?)?;
    ensure!(
        fs::symlink_metadata(&path).is_err(),
        "creating snapshot {id} without replacement: {base} already exists"
    );
    let staged = format!("{dir}/snapshots/.staging-{id}");
    let staged_path = local_path(root, &staged)?;
    fs::create_dir(&staged_path)
        .with_context(|| format!("creating staged snapshot {staged} without replacement"))?;
    let written = (|| -> Result<Manifest> {
        let mut entries = vec![];
        for (source, bytes) in sources {
            let copy = format!("{base}/files/{source}");
            let staged_copy = format!("{staged}/files/{source}");
            ensure!(
                write_new_bytes(root, &staged_copy, &bytes)?,
                "{copy} already exists"
            );
            entries.push(Entry {
                source,
                copy,
                sha256: digest_bytes(&bytes),
                bytes: bytes.len() as u64,
            });
        }
        let manifest = Manifest {
            schema_version: 1,
            reason: reason.into(),
            revision,
            created_at_unix_seconds: at,
            entries,
        };
        ensure!(
            write_new(
                root,
                &format!("{staged}/manifest.json"),
                &(serde_json::to_string_pretty(&manifest)? + "\n"),
            )?,
            "snapshot manifest {base}/manifest.json already exists"
        );
        ensure!(
            fs::symlink_metadata(&path).is_err(),
            "creating snapshot {id} without replacement: {base} already exists"
        );
        fs::rename(&staged_path, &path)
            .with_context(|| format!("moving staged snapshot {staged} to {base}"))?;
        Ok(manifest)
    })();
    let manifest = match written {
        Ok(manifest) => manifest,
        Err(error) => {
            let _ = fs::remove_dir_all(&staged_path);
            return Err(error);
        }
    };
    Ok(json!({"schema_version": 1, "snapshot": base, "manifest": manifest}))
}

fn allocate_snapshot(
    root: &Path,
    dir: &str,
    sources: Vec<(String, Vec<u8>)>,
    reason: &str,
    at: u64,
) -> Result<serde_json::Value> {
    let id = crate::identity::snapshot(reason)?;
    ensure!(
        crate::identity::valid_snapshot_id(&id),
        "invalid allocated snapshot ID"
    );
    write_snapshot(root, dir, &id, sources, reason, at)
}

fn snapshot(root: &Path, dir: &str, paths: &[String], reason: &str) -> Result<serde_json::Value> {
    ensure_text("reason", reason)?;
    ensure!(!paths.is_empty(), "name at least one file to snapshot");
    let mut sources = vec![];
    for path in paths {
        ensure!(
            !path.starts_with(&format!("{dir}/")) && !path.starts_with(".git/"),
            "{path} is inside the library or Git metadata"
        );
        let full = local_path(root, path)?;
        let metadata = fs::symlink_metadata(&full).with_context(|| format!("missing {path}"))?;
        ensure!(metadata.is_file(), "{path} is not a regular file");
        ensure!(
            metadata.len() <= SNAPSHOT_FILE_LIMIT,
            "{path} exceeds the 4 MiB snapshot limit"
        );
        sources.push((path.clone(), fs::read(&full)?));
    }
    allocate_snapshot(root, dir, sources, reason, now()?)
}

const README: &str = "# Project knowledge\n\n\
This folder is the project's librarian, on by default (`chough pref set librarian off\n\
--source <who asked>` opts out). It keeps historical copies with source links and digests\n\
(`snapshots/`), focused next-session briefs (`briefs/`), one regenerated entry per archived\n\
native change (`archives/`) and a generated index (`INDEX.md`) with a lessons index. It is\n\
not a ledger and no gate reads it. The living sources stay where their owners keep\n\
them: HI under `hi/`, contracts under `specs/`, native changes and archives under\n\
`.specsync/`, prototypes beside their companions, and the checkpoint and bug backlog\n\
under `.chough/`. Nothing here is a second current version of those sources.\n";

pub(super) fn run(root: &Path, command: LibraryCommand) -> Result<()> {
    let dir = enabled_dir(root)?;
    match command {
        LibraryCommand::Init => {
            // Build and bound everything first so a refusal leaves no half-built folder.
            let files = build(root, &dir)?;
            let created_readme = write_new(root, &format!("{dir}/README.md"), README)?;
            fs::create_dir_all(local_path(root, &format!("{dir}/briefs"))?)?;
            write_all(root, &dir, files)?;
            print_json(&json!({
                "schema_version": 1,
                "library": dir,
                "readme": if created_readme { "created" } else { "preserved" },
                "index": format!("{dir}/INDEX.md"),
            }))
        }
        LibraryCommand::Snapshot { paths, reason } => {
            // A stop before the snapshot is written records nothing; once writing starts
            // it finishes the snapshot and its index first (FLOW-14).
            let _watch = crate::records::stop_watch();
            // Refuse an unbuildable index before the snapshot is written.
            build(root, &dir)?;
            let value = snapshot(root, &dir, &paths, &reason)?;
            regenerate(root, &dir)?;
            print_json(&value)
        }
        LibraryCommand::Index => {
            regenerate(root, &dir)?;
            print_json(&json!({"schema_version": 1, "index": format!("{dir}/INDEX.md")}))
        }
        LibraryCommand::Archive { apply } => {
            let authority = if apply {
                Some(evidence::retention_authority(
                    root,
                    "summarizing archived proof",
                )?)
            } else {
                None
            };
            print_json(&archive_value(root, &dir, authority.as_deref())?)
        }
    }
}

/// The authority recorded when trimming at done relies on the librarian default
/// (KNOW-13): the standing preference KNOW-11 names.
const LIBRARIAN_DEFAULT: &str = "standing preference: librarian on by default (KNOW-13); opt out with `chough pref set retention manual`";

/// Trim at done (REQ-cli-041, KNOW-13, #88): with the librarian on and `retention` not
/// `manual`, apply the post-archive summary with the recorded retention authority or
/// else the librarian default. Never fails: the result is `trimmed`, `skipped` or
/// `refused` with its reason, so the move into done never depends on it.
pub(super) fn trim_at_done(root: &Path) -> serde_json::Value {
    let refused = |reason: String| json!({"status": "refused", "reason": reason});
    let librarian = match prefs::effective(root, "librarian") {
        Ok((value, _)) => value,
        Err(error) => return refused(format!("{error:#}")),
    };
    if librarian != "on" {
        return json!({"status": "skipped", "reason": "the librarian is off"});
    }
    let (retention, origin) = match prefs::effective(root, "retention") {
        Ok(value) => value,
        Err(error) => return refused(format!("{error:#}")),
    };
    if retention == "manual" {
        return json!({"status": "skipped", "reason": format!("retention is manual ({origin}); run `chough library archive --apply` when you choose")});
    }
    let authority = evidence::retention_authority(root, "trimming at done")
        .unwrap_or_else(|_| LIBRARIAN_DEFAULT.to_string());
    let summary = enabled_dir(root).and_then(|dir| archive_value(root, &dir, Some(&authority)));
    match summary {
        Ok(summary) => json!({"status": "trimmed", "authority": authority, "summary": summary}),
        Err(error) => {
            json!({"status": "refused", "authority": authority, "reason": format!("{error:#}")})
        }
    }
}

/// `chough library archive`: the post-archive summary, previewed or, with `authority`,
/// applied (REQ-cli-033).
fn archive_value(root: &Path, dir: &str, authority: Option<&str>) -> Result<serde_json::Value> {
    let found = archives(root)?;
    let unreadable: Vec<String> = found
        .iter()
        .filter_map(|archive| {
            archive
                .manifest_error
                .as_ref()
                .map(|error| format!("{}: {error}", archive.id))
        })
        .collect();
    ensure!(
        unreadable.is_empty(),
        "archived verification manifests could not be read, so nothing can be classified safely: {}",
        unreadable.join("; ")
    );
    let listed: Vec<evidence::ListedArchive> = found
        .into_iter()
        .enumerate()
        .map(|(index, archive)| evidence::ListedArchive {
            id: archive.id,
            hot: index < HOT_ARCHIVES,
            paths: archive.all_paths.into_iter().collect(),
        })
        .collect();
    let write = |root: &Path| regenerate(root, dir);
    evidence::archive_summary(root, &listed, dir, authority, &write)
}

/// The knowledge pointer for start and resume views: the librarian state and the index.
pub(super) fn pointer(root: &Path) -> serde_json::Value {
    let librarian = prefs::effective(root, "librarian")
        .map_or_else(|_| "unreadable".to_string(), |(value, _)| value);
    let dir = prefs::effective(root, "library-dir")
        .map_or_else(|_| "knowledge".to_string(), |(value, _)| value);
    let index = format!("{dir}/INDEX.md");
    let present = librarian == "on" && local_path(root, &index).is_ok_and(|path| path.is_file());
    json!({
        "librarian": librarian,
        "dir": dir,
        "index": present.then_some(index),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(bytes: &[u8]) -> Vec<(String, Vec<u8>)> {
        vec![("src/input.txt".into(), bytes.to_vec())]
    }

    #[test]
    fn snapshot_collision_preserves_the_original_without_fallback() {
        let root = tempfile::tempdir().unwrap();
        let id = "SNAP-collision-abababababababababababab";
        crate::identity::with_random_results(vec![Ok([0xab; 12])], || {
            allocate_snapshot(root.path(), "library", source(b"first"), "collision", 10).unwrap()
        });
        let base = format!("library/snapshots/{id}");
        let copy = root.path().join(format!("{base}/files/src/input.txt"));
        let manifest = root.path().join(format!("{base}/manifest.json"));
        let original_copy = fs::read(&copy).unwrap();
        let original_manifest = fs::read(&manifest).unwrap();

        let error = crate::identity::with_random_results(vec![Ok([0xab; 12])], || {
            allocate_snapshot(
                root.path(),
                "library",
                source(b"different"),
                "collision",
                11,
            )
            .unwrap_err()
        });
        assert!(error.to_string().contains(id), "{error:#}");
        assert_eq!(crate::identity::random_draws(), 1);
        assert_eq!(fs::read(&copy).unwrap(), original_copy);
        assert_eq!(fs::read(&manifest).unwrap(), original_manifest);
        assert_eq!(
            fs::read_dir(root.path().join("library/snapshots"))
                .unwrap()
                .count(),
            1
        );
    }

    #[test]
    fn snapshot_random_failure_preserves_existing_records() {
        let root = tempfile::tempdir().unwrap();
        let id = "SNAP-collision-abababababababababababab";
        crate::identity::with_random_results(vec![Ok([0xab; 12])], || {
            allocate_snapshot(root.path(), "library", source(b"first"), "collision", 10).unwrap()
        });
        let base = root.path().join(format!("library/snapshots/{id}"));
        let original_copy = fs::read(base.join("files/src/input.txt")).unwrap();
        let original_manifest = fs::read(base.join("manifest.json")).unwrap();

        let error =
            crate::identity::with_random_results(vec![Err("injected random failure")], || {
                allocate_snapshot(
                    root.path(),
                    "library",
                    source(b"different"),
                    "collision",
                    11,
                )
                .unwrap_err()
            });
        assert!(error.to_string().contains("injected random failure"));
        assert_eq!(crate::identity::random_draws(), 1);
        assert_eq!(
            fs::read(base.join("files/src/input.txt")).unwrap(),
            original_copy
        );
        assert_eq!(
            fs::read(base.join("manifest.json")).unwrap(),
            original_manifest
        );
        assert_eq!(
            fs::read_dir(root.path().join("library/snapshots"))
                .unwrap()
                .count(),
            1
        );
    }

    #[test]
    fn an_interrupted_snapshot_has_no_completion_manifest() {
        let root = tempfile::tempdir().unwrap();
        let id = "SNAP-interrupted-000000000000000000000000";
        let error = write_snapshot(
            root.path(),
            "library",
            id,
            vec![
                ("src/input.txt".into(), b"first".to_vec()),
                ("src/input.txt".into(), b"second".to_vec()),
            ],
            "interrupted",
            10,
        )
        .unwrap_err();
        assert!(error.to_string().contains("already exists"), "{error:#}");
        // Nothing of the failed snapshot is left: no copy, no manifest, no staging.
        let snapshots = root.path().join("library/snapshots");
        assert!(!snapshots.join(id).exists());
        assert_eq!(fs::read_dir(&snapshots).unwrap().count(), 0);
    }
}
