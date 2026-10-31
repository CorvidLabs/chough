//! Optional project-local librarian: source-linked snapshots, an index and briefs.
//! Canonical sources stay where their owners keep them; nothing here moves or edits them.

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
    /// Regenerate the deterministic INDEX.md.
    Index,
}

fn enabled_dir(root: &Path) -> Result<String> {
    let (enabled, _) = prefs::effective(root, "librarian")?;
    ensure!(
        enabled == "on",
        "the librarian is optional and off; enable it with `chough pref set librarian on --source <who asked>`"
    );
    Ok(prefs::effective(root, "library-dir")?.0)
}

/// Civil date (UTC) for a Unix time, without a date library.
fn date(seconds: u64) -> String {
    let days = (seconds / 86_400) as i64 + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
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

fn index(root: &Path, dir: &str) -> Result<String> {
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
    let archived = sorted_entries(root, ".specsync/archive/changes")?
        .into_iter()
        .map(|name| format!("[{name}](../.specsync/archive/changes/{name}/)"))
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
        let reason = read_record(&local_path(root, &manifest)?)
            .ok()
            .and_then(|text| serde_json::from_str::<Manifest>(&text).ok())
            .map_or_else(
                || "manifest unreadable".to_string(),
                |manifest| manifest.reason,
            );
        snapshots.push(format!(
            "[{name}](snapshots/{name}/manifest.json): {reason}"
        ));
    }
    Ok(format!(
        "# Project library index\n\n\
Generated by `chough library index`. This index points to living sources; it does not\n\
replace them. Snapshots are historical copies with source links and digests.\n\n\
## Living sources (owned elsewhere)\n\n\
Intent: [INTENT.md](../INTENT.md)\n\n{hi}\n\
Requirements: [docs/FRD.md](../docs/FRD.md) when present, and native module requirements:\n\n{modules}\n\
Canonical prototypes:\n\n{prototypes}\n\
## Native SpecSync changes\n\nActive:\n\n{active}\nArchived:\n\n{archived}\n\
## Open bug backlog (live)\n\n{open}\n\
## Observations\n\n{total} recorded; most recent:\n\n{recent}\n\
## Briefs\n\n{briefs}\n\
## Snapshots\n\n{snapshots}",
        hi = list(hi_files),
        modules = list(modules),
        prototypes = list(prototypes),
        active = list(active),
        archived = list(archived),
        open = list(open),
        total = total,
        recent = list(recent),
        briefs = list(briefs),
        snapshots = list(snapshots),
    ))
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
    let at = now()?;
    let day = date(at);
    let mut number = 1;
    while local_path(root, &format!("{dir}/snapshots/{day}-{number}"))?.exists() {
        number += 1;
    }
    let base = format!("{dir}/snapshots/{day}-{number}");
    let mut entries = vec![];
    for (source, bytes) in sources {
        let copy = format!("{base}/files/{source}");
        ensure!(
            write_new_bytes(root, &copy, &bytes)?,
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
        revision: head_revision(root),
        created_at_unix_seconds: at,
        entries,
    };
    write_new(
        root,
        &format!("{base}/manifest.json"),
        &(serde_json::to_string_pretty(&manifest)? + "\n"),
    )?;
    Ok(json!({"schema_version": 1, "snapshot": base, "manifest": manifest}))
}

const README: &str = "# Project library\n\n\
This optional folder is the project's librarian. It keeps historical copies with\n\
source links and digests (`snapshots/`), focused next-session briefs (`briefs/`)\n\
and a generated index (`INDEX.md`). The living sources stay where their owners keep\n\
them: HI under `hi/`, contracts under `specs/`, native changes and archives under\n\
`.specsync/`, prototypes beside their companions, and the checkpoint and bug backlog\n\
under `.chough/`. Nothing here is a second current version of those sources.\n";

pub(super) fn run(root: &Path, command: LibraryCommand) -> Result<()> {
    let dir = enabled_dir(root)?;
    match command {
        LibraryCommand::Init => {
            let created_readme = write_new(root, &format!("{dir}/README.md"), README)?;
            write_replace(root, &format!("{dir}/INDEX.md"), &index(root, &dir)?)?;
            fs::create_dir_all(local_path(root, &format!("{dir}/briefs"))?)?;
            print_json(&json!({
                "schema_version": 1,
                "library": dir,
                "readme": if created_readme { "created" } else { "preserved" },
                "index": format!("{dir}/INDEX.md"),
            }))
        }
        LibraryCommand::Snapshot { paths, reason } => {
            let value = snapshot(root, &dir, &paths, &reason)?;
            write_replace(root, &format!("{dir}/INDEX.md"), &index(root, &dir)?)?;
            print_json(&value)
        }
        LibraryCommand::Index => {
            let content = index(root, &dir)?;
            ensure!(
                content.len() <= 65_536,
                "index exceeds 65536 bytes; summarize older snapshots in a brief"
            );
            write_replace(root, &format!("{dir}/INDEX.md"), &content)?;
            print_json(&json!({"schema_version": 1, "index": format!("{dir}/INDEX.md")}))
        }
    }
}
