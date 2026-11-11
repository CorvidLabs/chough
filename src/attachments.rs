//! Retired in 0.6.0 with REQ-cli-046 (vault attachments) and REQ-cli-041 (trims): the
//! vault no longer holds attachments. Only what `src/evidence.rs` still calls is left, and
//! this file is deleted at integration once plan step 4 removes those calls.

use crate::records::local_path;
use anyhow::Result;
use sha2::Digest as _;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// Where loose artifacts live.
pub(super) const ARTIFACTS_DIR: &str = ".chough/artifacts";

const SCREENSHOTS: [&str; 6] = ["png", "jpg", "jpeg", "gif", "webp", "svg"];
const FAILURE_FIRST: [&str; 6] = [
    "failure-first",
    "failing-first",
    "fails-first",
    "before-fix",
    "pre-fix",
    "reproduction",
];

/// A screenshot or a failure-first log, by file name (KNOW-19).
pub(super) fn is_key_item(path: &str) -> bool {
    let lower = path.to_lowercase();
    let name = lower.rsplit('/').next().unwrap_or(&lower);
    name.rsplit_once('.')
        .is_some_and(|(_, extension)| SCREENSHOTS.contains(&extension))
        || FAILURE_FIRST.iter().any(|marker| name.contains(marker))
}

/// The longest attachment path the summary writes: Windows refuses a checkout path of
/// 260 characters or more, so this leaves 39 for the folder a clone sits in (#109).
pub(super) const MAX_ATTACHMENT_PATH: usize = 220;

/// The attachments folder of an archive (REQ-cli-046): the ID when it has at most 24
/// characters, otherwise its first 16 cut back to before the last hyphen among them, then
/// `-` and 8 hex digits of its SHA-256. At most 25 characters, and the same on every run.
pub(super) fn attachment_folder(archive: &str) -> String {
    if archive.chars().count() <= 24 {
        return archive.to_string();
    }
    let head: String = archive.chars().take(16).collect();
    let prefix = match head.rfind('-') {
        Some(cut) if cut > 0 => &head[..cut],
        _ => head.as_str(),
    };
    let digest = format!("{:x}", sha2::Sha256::digest(archive.as_bytes()));
    format!("{prefix}-{}", &digest[..8])
}

/// Where a file under `.chough/` moves in the vault: `<dir>/attachments/<folder>/<rest>`.
pub(super) fn attachment(dir: &str, archive: &str, file: &str) -> String {
    let rest = file.strip_prefix(".chough/").unwrap_or(file);
    format!("{dir}/attachments/{}/{rest}", attachment_folder(archive))
}

/// The oldest commit time per file in `git log --diff-filter=A --format=@%ct --name-only`
/// output (newest commit first), so each file maps to the commit that first added it.
pub(super) fn parse_first_commits(log: &str) -> BTreeMap<String, u64> {
    let mut found = BTreeMap::new();
    let mut at = None;
    for line in log.lines() {
        if let Some(time) = line.strip_prefix('@') {
            at = time.trim().parse::<u64>().ok();
        } else if let (Some(time), false) = (at, line.trim().is_empty()) {
            found.insert(line.trim().to_string(), time);
        }
    }
    found
}

/// One loose artifact entry: a top-level file or folder under `.chough/artifacts/` and
/// the files in it that no observation record references.
pub(super) struct LooseGroup {
    /// `artifact:<entry>`, never an observation ID.
    pub(super) id: String,
    pub(super) files: Vec<String>,
    pub(super) bytes: u64,
}

fn walk(path: &Path, relative: &str, files: &mut Vec<(String, u64)>) -> Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.is_file() {
        files.push((relative.to_string(), metadata.len()));
    } else if metadata.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(path)?.collect::<std::io::Result<_>>()?;
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries {
            let name = entry.file_name().to_string_lossy().into_owned();
            walk(&entry.path(), &format!("{relative}/{name}"), files)?;
        }
    }
    Ok(())
}

/// The loose groups under `.chough/artifacts/`, skipping hidden entries. `referenced`
/// tells whether a record references a file, directly or through a referenced folder.
pub(super) fn loose_groups(
    root: &Path,
    referenced: &dyn Fn(&str) -> bool,
) -> Result<Vec<LooseGroup>> {
    let dir = local_path(root, ARTIFACTS_DIR)?;
    if !dir.is_dir() {
        return Ok(vec![]);
    }
    let mut entries: Vec<_> = fs::read_dir(&dir)?.collect::<std::io::Result<_>>()?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    let mut groups = vec![];
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let mut files = vec![];
        walk(
            &entry.path(),
            &format!("{ARTIFACTS_DIR}/{name}"),
            &mut files,
        )?;
        let loose: Vec<(String, u64)> = files
            .into_iter()
            .filter(|(file, _)| !referenced(file))
            .collect();
        if loose.is_empty() {
            continue;
        }
        groups.push(LooseGroup {
            id: format!("artifact:{name}"),
            bytes: loose.iter().map(|(_, bytes)| bytes).sum(),
            files: loose.into_iter().map(|(file, _)| file).collect(),
        });
    }
    Ok(groups)
}

/// The fixture paths that open bugs name, as (bug ID, path): they hold a loose artifact;
/// a closed bug (verified, duplicate or wont-fix) no longer does (KNOW-18).
pub(super) fn open_bug_fixtures(root: &Path) -> Result<Vec<(String, String)>> {
    let open: BTreeSet<String> = crate::bugs::load_all(root)?
        .into_iter()
        .filter(crate::bugs::Bug::is_open)
        .map(|bug| bug.id)
        .collect();
    Ok(crate::bugs::fixture_references(root)?
        .into_iter()
        .filter_map(|(label, fixture)| {
            let id = label.split_whitespace().next()?.to_string();
            open.contains(&id).then_some((id, fixture))
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_items_are_screenshots_and_failure_first_logs() {
        assert!(is_key_item(".chough/artifacts/run/screen.PNG"));
        assert!(is_key_item(
            ".chough/artifacts/issue-100/failure-first-on-main.log"
        ));
        assert!(is_key_item(".chough/artifacts/bug-0004-reproduction.log"));
        assert!(!is_key_item(".chough/artifacts/run/out.log"));
        assert!(!is_key_item(".chough/evidence/OBS-a.json"));
    }

    #[test]
    fn attachments_keep_the_path_under_chough() {
        assert_eq!(
            attachment("knowledge", "a1", ".chough/artifacts/x/y.log"),
            "knowledge/attachments/a1/artifacts/x/y.log"
        );
    }

    #[test]
    fn long_archive_ids_get_a_short_stable_folder() {
        assert_eq!(attachment_folder("a1"), "a1");
        assert_eq!(
            attachment_folder("twenty-four-characters-x"),
            "twenty-four-characters-x"
        );
        assert_eq!(
            attachment_folder("claude-host-installs-the-shared-agents-skill-copy"),
            "claude-host-65ab2407"
        );
        assert_eq!(
            attachment_folder("abcdefghijklmnopqrstuvwxyz"),
            "abcdefghijklmnop-71c480df"
        );
        assert_eq!(
            attachment(
                "knowledge",
                "claude-host-installs-the-shared-agents-skill-copy",
                ".chough/evidence/OBS-a.json"
            ),
            "knowledge/attachments/claude-host-65ab2407/evidence/OBS-a.json"
        );
    }

    #[test]
    fn first_commits_keep_the_oldest_time() {
        let log = "@300\n\n.chough/artifacts/a.log\n@100\n\n.chough/artifacts/a.log\n.chough/artifacts/b.log\n";
        let found = parse_first_commits(log);
        assert_eq!(found[".chough/artifacts/a.log"], 100);
        assert_eq!(found[".chough/artifacts/b.log"], 100);
    }
}
