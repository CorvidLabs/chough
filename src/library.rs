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
    /// Git merge driver for the generated `index.md` and `journal/feed.xml`
    /// (`chough library merge %O %A %B`, `merge=chough-generated`): keeps ours byte for
    /// byte and exits 0, so a merge never holds a union of two feeds; the next
    /// `chough library index` rebuilds both from the merged sources (REQ-cli-047).
    Merge {
        base: std::path::PathBuf,
        ours: std::path::PathBuf,
        theirs: std::path::PathBuf,
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
    build_pending(root, dir, &crate::vault::Pending::default())
}

/// [`build`], as the tree will be once a migration plan applies (REQ-records-009).
fn build_pending(
    root: &Path,
    dir: &str,
    pending: &crate::vault::Pending,
) -> Result<Vec<(String, String)>> {
    let posts = crate::journal::posts(root, dir)?;
    let files = vec![
        (
            format!("{dir}/index.md"),
            crate::vault::index(root, dir, &posts, pending)?,
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

/// Whether `bytes` are a README an earlier release generated, read with CRLF as LF.
fn generated_readme(bytes: &[u8]) -> bool {
    let text = String::from_utf8_lossy(bytes).replace("\r\n", "\n");
    let digest = format!("{:x}", sha2::Sha256::digest(text.as_bytes()));
    GENERATED_READMES.contains(&digest.as_str())
}

/// The folders of notes 0.5.x `library index` generated, one note per record or month.
const GENERATED_FOLDERS: [&str; 6] = [
    "archives", "modules", "intent", "bugs", "sessions", "timeline",
];

/// What `chough migrate` does with one file of a 0.5.x knowledge directory, by its path
/// inside the directory: None when 0.6.0 keeps it byte for byte (lessons, snapshots,
/// briefs, anything a person wrote), Some(why) when it is removed (REQ-cli-044), and
/// Some("") for a rolled session, which the session mapper carries.
fn disposition_0_5(inside: &str, bytes: &[u8]) -> Option<&'static str> {
    let parts: Vec<&str> = inside.split('/').collect();
    match parts.as_slice() {
        ["INDEX.md"] => Some(
            "the index 0.5.x `chough library index` generated; 0.6.0 writes index.md, the vault's reading guide, instead (REQ-cli-043)",
        ),
        ["README.md"] if generated_readme(bytes) => Some(
            "the README an earlier release generated, unedited; index.md is the vault's reading guide now (REQ-cli-044)",
        ),
        ["rules.md"] | ["timeline.md"] => Some(
            "a note 0.5.x `chough library index` generated; 0.6.0 generates no rules.md or timeline.md and links the sources from index.md (REQ-cli-044)",
        ),
        [folder, .., name] if GENERATED_FOLDERS.contains(folder) && name.ends_with(".md") => Some(
            "a note 0.5.x `chough library index` generated; 0.6.0 generates no per-change, module, intent, bug, session or timeline note and links the sources from index.md (REQ-cli-044)",
        ),
        ["attachments", _, "sessions", name] if name.ends_with(".json") => Some(""),
        ["attachments", .., "pruned.jsonl"] => Some(
            "a shard of the 0.5.x retention ledger, retired in 0.6.0 (REQ-cli-049); Git keeps its bytes",
        ),
        ["attachments", ..] => Some(
            "moved into the vault by the 0.5.x trim at done, retired in 0.6.0 (REQ-cli-046); a story or bug that references it carries it as a receipt, and Git keeps its bytes",
        ),
        ["snapshots", staging, ..] if staging.starts_with(".staging-") => Some(
            "the staging copy of an interrupted 0.5.x snapshot; `library snapshot` is retired (REQ-cli-015)",
        ),
        _ => None,
    }
}

/// Every entry under `relative` that is not a folder, as project-relative paths with
/// whether it is a symbolic link; links are listed, never followed.
fn files_0_5(root: &Path, relative: &str, found: &mut Vec<(String, bool)>) -> Result<()> {
    let Ok(entries) = fs::read_dir(local_path(root, relative)?) else {
        return Ok(());
    };
    for entry in entries {
        let entry = entry?;
        let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            files_0_5(root, &name, found)?;
        } else {
            found.push((name, kind.is_symlink()));
        }
    }
    Ok(())
}

/// A vault file's bytes; a link is not read, so nothing outside the tree is.
fn bytes_0_5(root: &Path, path: &str, link: bool) -> Result<Vec<u8>> {
    if link {
        return Ok(vec![]);
    }
    Ok(fs::read(local_path(root, path)?)?)
}

/// Mapper 5 of `chough migrate` (REQ-records-009, REQ-cli-044): the 0.5.x knowledge
/// directory's generated and copied content is removed with why; lessons, snapshots,
/// briefs, an earlier `library/` folder and anything a person wrote are kept byte for
/// byte, each one holding a leak listed with its kinds; and, with the librarian on,
/// `index.md` and `journal/feed.xml` are written as `chough library index` builds them
/// once the plan applies, given what the earlier mappers carry (`earlier`). Rolled
/// sessions under `attachments/*/sessions/` are the session mapper's. It reads only.
pub(super) fn migrate_0_5(
    root: &Path,
    earlier: &crate::migrate::Carried,
) -> Result<crate::migrate::Carried> {
    let mut carried = crate::migrate::Carried::default();
    let dir = prefs::effective(root, "library-dir")?.0;
    let local = crate::leaks::Local::current();
    let mut files = vec![];
    files_0_5(root, &dir, &mut files)?;
    let mut kept = vec![];
    for (path, link) in files {
        let bytes = bytes_0_5(root, &path, link)?;
        let inside = path.strip_prefix(&format!("{dir}/")).unwrap_or(&path);
        match disposition_0_5(inside, &bytes) {
            Some("") => {}
            Some(why) => {
                // Removed without carrying: the framework digests and removes every
                // `removed` path, so it is not also a source.
                carried.removed.insert(path, why.to_string());
            }
            None => kept.push((path, bytes)),
        }
    }
    if dir != "library" {
        let mut earlier_library = vec![];
        files_0_5(root, "library", &mut earlier_library)?;
        for (path, link) in earlier_library {
            let bytes = bytes_0_5(root, &path, link)?;
            kept.push((path, bytes));
        }
    }
    for (path, bytes) in kept {
        let mut kinds: Vec<String> = crate::leaks::find(local, &String::from_utf8_lossy(&bytes))
            .into_iter()
            .map(|found| found.kind.name().to_string())
            .collect();
        kinds.sort();
        kinds.dedup();
        if !kinds.is_empty() {
            carried.kept_with_leaks.insert(path, kinds);
        }
    }
    if librarian_dir(root)?.is_some() {
        let mut pending = crate::vault::Pending {
            migrating: true,
            ..Default::default()
        };
        pending.present = earlier
            .written
            .iter()
            .map(|file| file.path.clone())
            .collect();
        pending.gone = earlier
            .sources
            .iter()
            .chain(earlier.removed.keys())
            .chain(carried.sources.iter())
            .cloned()
            .collect();
        for file in &earlier.written {
            let Some(name) = file
                .path
                .strip_prefix(&format!("{}/", crate::bugs::BUGS_DIR))
            else {
                continue;
            };
            if name.contains('/') || !name.ends_with(".md") {
                continue;
            }
            let record = crate::markdown::parse(&file.path, &file.bytes)?;
            let field = |key: &str| record.scalar(key).unwrap_or_default().to_string();
            if !crate::bugs::is_closed_status(&field("status")) {
                pending.bugs.push((
                    file.path.clone(),
                    field("id"),
                    field("title"),
                    field("status"),
                ));
            }
        }
        for (path, bytes) in build_pending(root, &dir, &pending)? {
            carried
                .written
                .push(crate::migrate::Written { path, bytes });
        }
    }
    Ok(carried)
}

/// Remove `<dir>/README.md` when it is exactly a README an earlier release generated;
/// `index.md` is the vault's reading guide now (REQ-cli-044). An edited one is kept, and
/// so is an untouched one whose bytes are not committed at HEAD (REQ-records-012).
fn fold_generated_readme(root: &Path, dir: &str) -> Result<serde_json::Value> {
    let relative = format!("{dir}/README.md");
    let Some(text) = current_text(root, &relative) else {
        return Ok(json!({"status": "none"}));
    };
    if !generated_readme(text.as_bytes()) {
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
    // The merge driver touches only the files Git hands it, librarian on or off.
    if let LibraryCommand::Merge { ours, .. } = &command {
        ensure!(
            ours.is_file(),
            "{} is not a file; Git runs this driver as `chough library merge %O %A %B`",
            ours.display()
        );
        return Ok(());
    }
    let dir = enabled_dir(root)?;
    match command {
        LibraryCommand::Merge { .. } => Ok(()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migrate::{Carried, Written};

    fn bug(id: &str, title: &str, status: &str) -> Written {
        Written {
            path: format!(".chough/bugs/{id}.md"),
            bytes: format!(
                "---\nkind: bug\nid: {id}\ntitle: {title}\nstatus: {status}\n---\n\n# {title}\n"
            ),
        }
    }

    /// REQ-records-009: the planned index is the tree's once every earlier mapper's plan
    /// applies: the stories, `.chough/README.md` and open bugs they write count, a bug
    /// they remove or close does not, and a 0.5.x JSON bug is left to the bug mapper.
    #[test]
    fn the_planned_index_counts_what_the_earlier_mappers_write_and_remove() {
        let folder = tempfile::tempdir().unwrap();
        let root = folder.path();
        fs::create_dir_all(root.join(".chough/bugs")).unwrap();
        fs::write(root.join(".chough/bugs/BUG-0001.json"), "{}\n").unwrap();
        fs::write(
            root.join(".chough/bugs/2026-01-01-gone.md"),
            bug(
                "2026-01-01-gone",
                "A SYNTHETIC bug the plan removes",
                "open",
            )
            .bytes,
        )
        .unwrap();
        let mut earlier = Carried::default();
        earlier.written.push(Written {
            path: ".chough/README.md".into(),
            bytes: "# Records\n".into(),
        });
        earlier.written.push(Written {
            path: ".chough/archive/sessions/2026-01-02-old.md".into(),
            bytes: "---\nid: 2026-01-02-old\n---\n".into(),
        });
        earlier
            .written
            .push(bug("2026-01-03-open-one", "An open SYNTHETIC bug", "open"));
        earlier.written.push(bug(
            "2026-01-04-closed-one",
            "A closed SYNTHETIC bug",
            "verified",
        ));
        earlier
            .sources
            .insert(".chough/bugs/2026-01-01-gone.md".into());
        let carried = migrate_0_5(root, &earlier).unwrap();
        let index = &carried
            .written
            .iter()
            .find(|file| file.path == "knowledge/index.md")
            .unwrap()
            .bytes;
        for expected in [
            "Chough's records: [.chough/README.md](../.chough/README.md)",
            "Finished sessions: [.chough/archive/sessions/](../.chough/archive/sessions/)",
            "- [2026-01-03-open-one](../.chough/bugs/2026-01-03-open-one.md) An open SYNTHETIC bug (open)",
        ] {
            assert!(index.contains(expected), "missing {expected:?} in\n{index}");
        }
        assert!(
            !index.contains("closed-one") && !index.contains("2026-01-01-gone"),
            "{index}"
        );
        assert!(!index.contains("BUG-0001"), "{index}");
    }

    #[test]
    fn a_vault_file_is_removed_kept_or_left_to_the_session_mapper() {
        let readme = include_str!("../tests/fixtures/knowledge-readme-at-8fbbbe7.md");
        let crlf = readme.replace('\n', "\r\n");
        assert!(disposition_0_5("README.md", readme.as_bytes()).is_some());
        assert!(disposition_0_5("README.md", crlf.as_bytes()).is_some());
        let edited = format!("{readme}\nOur own note.\n");
        assert!(disposition_0_5("README.md", edited.as_bytes()).is_none());
        assert_eq!(
            disposition_0_5("attachments/k/sessions/k.json", b"{}"),
            Some("")
        );
        assert!(
            disposition_0_5("attachments/k/evidence/OBS-a.json", b"{}")
                .is_some_and(|why| !why.is_empty())
        );
        for kept in [
            "lessons/a.md",
            "briefs/b.md",
            "snapshots/SNAP-x/manifest.json",
            "index.md",
            "archives/notes.txt",
        ] {
            assert!(disposition_0_5(kept, b"x").is_none(), "{kept}");
        }
    }
}
