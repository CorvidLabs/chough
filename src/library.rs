//! Project-local librarian (REQ-cli-015), on by default in `knowledge/` with a recorded
//! opt-out. It keeps the vault of REQ-cli-044: the reading guide `index.md` and the
//! journal feed, regenerated deterministically from their sources (REQ-cli-043,
//! REQ-records-011), beside the journal posts and the lessons that span projects. It never
//! moves, copies, rewrites or replaces a canonical source or a Chough record (KNOW-30),
//! and no gate reads the vault.

use crate::prefs;
use crate::records::{local_path, print_json, write_new, write_replace};
use anyhow::{Result, ensure};
use clap::Subcommand;
use serde_json::json;
use sha2::Digest as _;
use std::fs;
use std::path::Path;

#[derive(Subcommand)]
pub(super) enum LibraryCommand {
    /// Build the knowledge directory: index.md, journal/ with its feed, and lessons/.
    Init,
    /// Regenerate the vault's index.md and the journal feed from their sources.
    Index {
        /// Write nothing; fail naming each generated file that is missing or differs from
        /// a rebuild (line endings ignored), so a merged vault is rebuilt before CI passes.
        #[arg(long)]
        check: bool,
    },
}

/// The archives whose lesson bundles the index and `chough next` show, newest first.
pub(super) const HOT_ARCHIVES: usize = 3;
/// The bound every generated knowledge file stays within (REQ-cli-044).
const ENTRY_LIMIT: usize = 65_536;
pub(super) const ARCHIVES: &str = ".specsync/archive/changes";

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
    pub(super) id: String,
    timestamp: u64,
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
        });
    }
    found.sort_by(|left, right| (right.timestamp, &right.dir).cmp(&(left.timestamp, &left.dir)));
    Ok(found)
}

/// The folder of archived native change `change`, if an archive records it.
pub(super) fn archive_dir_of(root: &Path, change: &str) -> Option<String> {
    archive_order(root)
        .ok()?
        .into_iter()
        .find(|archive| archive.id == change)
        .map(|archive| archive.dir)
}

/// SHA-256 digests (over LF line endings) of the knowledge README earlier releases
/// generated. `index.md` replaces it, so `chough library index` removes one whose bytes
/// are exactly those, and keeps any README a person edited.
const GENERATED_READMES: &[&str] = &[
    // Main 8fbbbe7 (#106), before the vault front matter.
    "5365c2f960fb72b71fe9e2ccfbbc9e29f638a8d324714c808e9502dcdc135b2f",
    // 0.5.x, with the vault front matter (REQ-cli-044 before 0.6.0).
    "0518cde607c068100782610deab3cac3f705ad040062b066d344bfef2f6f0e85",
];

/// Every generated knowledge file, built and bounded in memory before anything is
/// written: `index.md` and `journal/feed.xml`.
fn build(root: &Path, dir: &str) -> Result<Vec<(String, String)>> {
    let posts = crate::journal::posts(root, dir)?;
    let files = vec![
        (
            format!("{dir}/index.md"),
            crate::vault::index(root, dir, &posts)?,
        ),
        (
            format!("{}/{}", crate::journal::dir_of(dir), crate::journal::FEED),
            crate::journal::feed(&posts),
        ),
    ];
    for (relative, content) in &files {
        ensure!(
            content.len() <= ENTRY_LIMIT,
            "{relative} would be {} bytes, over the {ENTRY_LIMIT}-byte bound",
            content.len()
        );
    }
    Ok(files)
}

/// A file's text with CRLF read as LF, or None when it is missing or unreadable.
fn current_text(root: &Path, relative: &str) -> Option<String> {
    let bytes = fs::read(local_path(root, relative).ok()?).ok()?;
    Some(String::from_utf8_lossy(&bytes).replace("\r\n", "\n"))
}

/// Each generated file that is missing or differs from a rebuild, with CRLF read as LF so
/// a Windows checkout of a current file is current (REQ-cli-047, DRIVE-12).
fn stale_files(root: &Path, dir: &str) -> Result<Vec<String>> {
    Ok(build(root, dir)?
        .into_iter()
        .filter(|(relative, content)| current_text(root, relative).as_deref() != Some(content))
        .map(|(relative, _)| relative)
        .collect())
}

/// Regenerate `index.md` and `feed.xml` in the knowledge directory `dir`, writing only a
/// file that differs from its rebuild, and make sure `journal/` and `lessons/` exist.
/// Nothing else under `dir` is touched. Returns the index path.
pub(super) fn regenerate(root: &Path, dir: &str) -> Result<String> {
    let files = build(root, dir)?;
    for folder in [crate::journal::dir_of(dir), format!("{dir}/lessons")] {
        fs::create_dir_all(local_path(root, &folder)?)?;
    }
    for (relative, content) in files {
        if current_text(root, &relative).as_deref() != Some(content.as_str()) {
            write_replace(root, &relative, &content)?;
        }
    }
    Ok(format!("{dir}/index.md"))
}

/// Remove `<dir>/README.md` when it is exactly a README an earlier release generated;
/// `index.md` is the vault's reading guide now (REQ-cli-044). An edited one is kept, and
/// so is an untouched one whose bytes are not committed at HEAD (REQ-records-012).
fn fold_generated_readme(root: &Path, dir: &str) -> Result<serde_json::Value> {
    let relative = format!("{dir}/README.md");
    let Some(text) = current_text(root, &relative) else {
        return Ok(json!({"status": "none"}));
    };
    let digest = format!("{:x}", sha2::Sha256::digest(text.as_bytes()));
    if !GENERATED_READMES.contains(&digest.as_str()) {
        return Ok(json!({"status": "kept", "path": relative, "reason": "a person edited it"}));
    }
    if !crate::sessions::committed_at_head(root, &relative) {
        return Ok(json!({
            "status": "kept",
            "path": relative,
            "reason": "its bytes are not committed at HEAD; commit it and run `chough library index` again to remove it",
        }));
    }
    fs::remove_file(local_path(root, &relative)?)?;
    Ok(json!({"status": "folded", "path": relative}))
}

/// `chough init` with the librarian on (REQ-cli-043, KNOW-1, KNOW-5): create the knowledge
/// directory create-only. An existing index is `preserved` untouched; a directory that
/// cannot be built or bounded is `refused` with nothing written, and init still succeeds.
pub(super) fn create_at_init(root: &Path) -> serde_json::Value {
    let refused =
        |error: anyhow::Error| json!({"status": "refused", "reason": format!("{error:#}")});
    let dir = match librarian_dir(root) {
        Ok(Some(dir)) => dir,
        Ok(None) => return json!({"status": "skipped", "reason": "the librarian is off"}),
        Err(error) => return refused(error),
    };
    let index = format!("{dir}/index.md");
    match local_path(root, &index) {
        Ok(path) if path.exists() => {
            return json!({"status": "preserved", "dir": dir, "index": index});
        }
        Err(error) => return refused(error),
        Ok(_) => {}
    }
    let created = (|| -> Result<()> {
        let files = build(root, &dir)?;
        for folder in [crate::journal::dir_of(&dir), format!("{dir}/lessons")] {
            fs::create_dir_all(local_path(root, &folder)?)?;
        }
        for (relative, content) in files {
            write_new(root, &relative, &content)?;
        }
        Ok(())
    })();
    match created {
        Ok(()) => json!({"status": "created", "dir": dir, "index": index}),
        Err(error) => refused(error),
    }
}

/// The move into done (REQ-cli-043), after the session's archive step and its journal
/// post: regenerate `index.md` and `feed.xml`, so `library index --check` passes right
/// after. Never fails: `refreshed`, `skipped` with the librarian off, or `refused`.
pub(super) fn refresh_at_done(root: &Path) -> serde_json::Value {
    match librarian_dir(root) {
        Ok(None) => json!({"status": "skipped", "reason": "the librarian is off"}),
        Ok(Some(dir)) => match regenerate(root, &dir) {
            Ok(index) => json!({"status": "refreshed", "index": index}),
            Err(error) => json!({"status": "refused", "reason": format!("{error:#}")}),
        },
        Err(error) => json!({"status": "refused", "reason": format!("{error:#}")}),
    }
}

/// The knowledge directory when the librarian is on, None when it is off.
pub(super) fn librarian_dir(root: &Path) -> Result<Option<String>> {
    let (enabled, _) = prefs::effective(root, "librarian")?;
    if enabled != "on" {
        return Ok(None);
    }
    Ok(Some(prefs::effective(root, "library-dir")?.0))
}

/// The knowledge directory, refusing with the opt-in command when the librarian is off.
pub(super) fn enabled_dir(root: &Path) -> Result<String> {
    let (enabled, origin) = prefs::effective(root, "librarian")?;
    ensure!(
        enabled == "on",
        "the librarian is off here ({origin}); turn it back on with `chough pref set librarian on --source <who asked>`"
    );
    Ok(prefs::effective(root, "library-dir")?.0)
}

pub(super) fn sorted_entries(root: &Path, relative: &str) -> Result<Vec<String>> {
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

pub(super) fn run(root: &Path, command: LibraryCommand) -> Result<()> {
    let dir = enabled_dir(root)?;
    match command {
        LibraryCommand::Init => {
            let index = regenerate(root, &dir)?;
            print_json(&json!({
                "schema_version": 1,
                "library": dir,
                "index": index,
                "feed": format!("{}/{}", crate::journal::dir_of(&dir), crate::journal::FEED),
            }))
        }
        LibraryCommand::Index { check: false } => {
            let index = regenerate(root, &dir)?;
            let folded = fold_generated_readme(root, &dir)?;
            print_json(&json!({
                "schema_version": 1,
                "index": index,
                "feed": format!("{}/{}", crate::journal::dir_of(&dir), crate::journal::FEED),
                "readme": folded,
            }))
        }
        LibraryCommand::Index { check: true } => {
            let stale = stale_files(root, &dir)?;
            ensure!(
                stale.is_empty(),
                "the knowledge directory is not what its sources generate; rebuild it with `chough library index` and commit it: {}",
                stale.join(", ")
            );
            print_json(
                &json!({"schema_version": 1, "index": format!("{dir}/index.md"), "current": true}),
            )
        }
    }
}

/// The knowledge pointer for start and resume views: the librarian state and the index.
pub(super) fn pointer(root: &Path) -> serde_json::Value {
    let librarian = prefs::effective(root, "librarian")
        .map_or_else(|_| "unreadable".to_string(), |(value, _)| value);
    let dir = prefs::effective(root, "library-dir")
        .map_or_else(|_| "knowledge".to_string(), |(value, _)| value);
    let index = format!("{dir}/index.md");
    let present = librarian == "on" && local_path(root, &index).is_ok_and(|path| path.is_file());
    json!({
        "librarian": librarian,
        "dir": dir,
        "index": present.then_some(index),
    })
}
