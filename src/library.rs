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
    fs::create_dir_all(path.parent().context("snapshot has no parent")?)?;
    local_path(root, &base)?;
    fs::create_dir(&path).with_context(|| format!("creating snapshot {id} without replacement"))?;
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
    ensure!(
        write_new(
            root,
            &format!("{base}/manifest.json"),
            &(serde_json::to_string_pretty(&manifest)? + "\n"),
        )?,
        "snapshot manifest {base}/manifest.json already exists"
    );
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
        let base = root.path().join(format!("library/snapshots/{id}"));
        assert!(!base.join("manifest.json").exists());
        assert_eq!(
            fs::read(base.join("files/src/input.txt")).unwrap(),
            b"first"
        );
    }
}
