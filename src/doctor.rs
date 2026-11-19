//! `chough doctor`: find and fix the Chough debt that stops a fresh worktree from
//! entering Chough (REQ-cli-055, REQ-history-012, DRIVE-17).
//!
//! It reads `.chough/`, the files Git tracks under `.chough/` and `knowledge/` (and their
//! bytes at HEAD), the archives' `finalization.json` and `.specsync/sdd.json`; it never
//! calls SpecSync, never writes `.specsync/sdd.json`, and never records a human decision,
//! approval, review or consent.

use crate::checkpoint::{self};
use crate::markdown::Kind as RecordKind;
use crate::records::{local_path, print_json};
use crate::sessions;
use anyhow::{Result, bail, ensure};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(clap::Args)]
pub(super) struct DoctorArgs {
    /// Report each finding and the fix it would make; change nothing (the default).
    #[arg(long, conflicts_with = "fix")]
    dry_run: bool,
    /// Apply the fixes. A fix that is a decision also needs --reason.
    #[arg(long)]
    fix: bool,
    /// Why the decision fixes are right; without it they are reported as skipped.
    #[arg(long)]
    reason: Option<String>,
    /// An open, unfinished session to close for abandoned work (repeatable; a decision).
    #[arg(long = "close")]
    close: Vec<String>,
    /// Report only the `leak` and `oversize` findings and change nothing; exits 1 on
    /// any, for the verify lane's `leaks` step.
    #[arg(long, conflicts_with_all = ["fix", "close", "reason"])]
    leaks: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Unmigrated,
    Leak,
    StalePointer,
    FinishedSession,
    UnfinishedArchive,
    StaleGates,
    UnfinishedSession,
    TimelineRewritten,
    AliasConflict,
    Oversize,
    Unreadable,
    RecordsInVerification,
    MergeDrivers,
    MergeDriverOutdated,
    ConflictMarkers,
    GitOperation,
    StaleBugLinks,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Self::Unmigrated => "unmigrated",
            Self::Leak => "leak",
            Self::StalePointer => "stale_pointer",
            Self::FinishedSession => "finished_session",
            Self::UnfinishedArchive => "unfinished_archive",
            Self::StaleGates => "stale_gates",
            Self::UnfinishedSession => "unfinished_session",
            Self::TimelineRewritten => "timeline_rewritten",
            Self::AliasConflict => "alias_conflict",
            Self::Oversize => "oversize",
            Self::Unreadable => "unreadable",
            Self::RecordsInVerification => "records_in_verification",
            Self::MergeDrivers => "merge_drivers",
            Self::MergeDriverOutdated => "merge_driver_outdated",
            Self::ConflictMarkers => "conflict_markers",
            Self::GitOperation => "git_operation",
            Self::StaleBugLinks => "stale_bug_links",
        }
    }
}

struct Finding {
    kind: Kind,
    target: String,
    fix: String,
    decision: bool,
    status: &'static str,
    reason: Option<String>,
    /// The `--reason` a decision fix was applied with (REQ-history-012, #250).
    decision_reason: Option<String>,
    /// Why the fix cannot run even with `--fix`, when known before fixing.
    blocked: Option<String>,
    /// Advice only: nothing doctor can fix, and nothing that should fail a run (#347).
    advisory: bool,
    /// Fields the finding adds to its JSON, such as the missing entries.
    detail: Option<Value>,
}

impl Finding {
    fn new(kind: Kind, target: &str, fix: String, decision: bool) -> Self {
        Self {
            kind,
            target: target.to_string(),
            fix,
            decision,
            status: "found",
            reason: None,
            decision_reason: None,
            blocked: None,
            advisory: false,
            detail: None,
        }
    }

    fn skip(&mut self, reason: String) {
        self.status = "skipped";
        self.reason = Some(reason);
    }

    fn json(&self) -> Value {
        let mut value = json!({
            "kind": self.kind.name(),
            "target": self.target,
            "fix": self.fix,
            "decision": self.decision,
            "status": self.status,
        });
        if let Some(reason) = &self.reason {
            value["reason"] = json!(reason);
        }
        if self.advisory {
            value["advisory"] = json!(true);
        }
        if let Some(reason) = &self.decision_reason {
            value["decision_reason"] = json!(reason);
        }
        if let Some(Value::Object(detail)) = &self.detail {
            for (key, field) in detail {
                value[key] = field.clone();
            }
        }
        value
    }
}

/// Whether `change` is archived on HEAD, from its `finalization.json` (no native read).
fn archived(root: &Path, change: Option<&str>) -> bool {
    change.is_some_and(|change| crate::library::finalized_at(root, change).is_some())
}

/// The 0.5.x records found, as one finding naming the first and listing up to 20: the
/// project is not migrated (REQ-records-010). Doctor never migrates; the finding names
/// the command.
fn unmigrated(root: &Path) -> Result<Option<Finding>> {
    let paths = crate::migrate::legacy_paths(root)?;
    let Some(first) = paths.first() else {
        return Ok(None);
    };
    let mut finding = Finding::new(Kind::Unmigrated, first, "chough migrate".into(), false);
    finding.blocked = Some(format!(
        "{} 0.5.x record files, such as {first}; run `chough migrate` to see the plan, which doctor never runs itself",
        paths.len()
    ));
    finding.detail = Some(json!({
        "paths": paths.iter().take(20).collect::<Vec<_>>(),
        "total": paths.len(),
    }));
    Ok(Some(finding))
}

/// Where finished sessions' stories live (REQ-records-002).
const STORIES: &str = ".chough/archive/sessions";

/// Run Git in `root`; its stdout when it succeeds.
fn git(root: &Path, arguments: &[&str]) -> Option<Vec<u8>> {
    let output = crate::records::run_owned(
        std::process::Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["-c", "core.quotepath=off"])
            .args(arguments),
        None,
    )
    .ok()?;
    output.status.success().then_some(output.stdout)
}

/// A Git merge or rebase in progress, or unmerged paths left by one (REQ-cli-069).
#[derive(Clone)]
pub(super) struct GitOperation {
    /// `merge`, `rebase`, `cherry-pick` or `revert`, or None when Git only reports
    /// unmerged paths, as after a conflicted `git stash pop`.
    operation: Option<&'static str>,
    /// Every unmerged path, project-relative and sorted.
    unmerged: Vec<String>,
}

/// How many unmerged paths a view or a refusal names.
const UNMERGED_SHOWN: usize = 10;

impl GitOperation {
    /// The unmerged paths under `.chough/`.
    fn chough_unmerged(&self) -> Vec<&str> {
        self.unmerged
            .iter()
            .map(String::as_str)
            .filter(|path| path.starts_with(".chough/"))
            .collect()
    }

    /// Whether a path under `.chough/` is unmerged, so writes refuse.
    pub(super) fn blocks_writes(&self) -> bool {
        !self.chough_unmerged().is_empty()
    }

    fn what(&self) -> &'static str {
        match self.operation {
            Some("merge") => "a git merge is in progress",
            Some("rebase") => "a git rebase is in progress",
            Some("cherry-pick") => "a git cherry-pick is in progress",
            Some(_) => "a git revert is in progress",
            None => "git reports unmerged paths",
        }
    }

    fn named(paths: &[&str]) -> String {
        let shown: Vec<&str> = paths.iter().take(UNMERGED_SHOWN).copied().collect();
        let more = paths.len().saturating_sub(shown.len());
        if more > 0 {
            format!("{} and {more} more", shown.join(", "))
        } else {
            shown.join(", ")
        }
    }

    /// One line: what is in progress, the unmerged paths under `.chough/`, and what to do.
    pub(super) fn text(&self) -> String {
        let chough = self.chough_unmerged();
        let others = self.unmerged.len() - chough.len();
        let mut text = self.what().to_string();
        if !chough.is_empty() {
            text.push_str(&format!(
                ", with {} unmerged under .chough/: {}",
                chough.len(),
                Self::named(&chough)
            ));
        } else if others > 0 {
            text.push_str(&format!(", with {others} unmerged outside .chough/"));
        }
        text.push_str("; finish or abort it first");
        text
    }

    /// The JSON `next`, `status` and `doctor` give it.
    pub(super) fn view(&self) -> Value {
        let chough = self.chough_unmerged();
        json!({
            "operation": self.operation,
            "unmerged": chough.iter().take(UNMERGED_SHOWN).collect::<Vec<_>>(),
            "unmerged_total": chough.len(),
            "unmerged_outside_total": self.unmerged.len() - chough.len(),
            "text": self.text(),
        })
    }
}

/// The merge or rebase in progress in the project's Git work tree, read once per
/// process; None outside a work tree or when there is nothing to report (REQ-cli-069).
pub(super) fn git_operation(root: &Path) -> Option<GitOperation> {
    static READ: std::sync::OnceLock<Option<GitOperation>> = std::sync::OnceLock::new();
    READ.get_or_init(|| read_git_operation(root)).clone()
}

fn read_git_operation(root: &Path) -> Option<GitOperation> {
    let dir = git(root, &["rev-parse", "--absolute-git-dir"])?;
    let dir = std::path::PathBuf::from(String::from_utf8_lossy(&dir).trim());
    // A rebase is in progress while its folder is there, as `git status` reads it: Git
    // can leave `REBASE_HEAD` behind after a rebase has finished (#330 review).
    let operation = if dir.join("MERGE_HEAD").is_file() {
        Some("merge")
    } else if dir.join("rebase-merge").is_dir() || dir.join("rebase-apply").is_dir() {
        Some("rebase")
    } else if dir.join("CHERRY_PICK_HEAD").is_file() {
        Some("cherry-pick")
    } else if dir.join("REVERT_HEAD").is_file() {
        Some("revert")
    } else {
        None
    };
    // `<mode> <object> <stage>\t<path>`, one entry per stage of each unmerged path.
    let listed = git(root, &["ls-files", "-u", "-z"]).unwrap_or_default();
    let unmerged: std::collections::BTreeSet<String> = listed
        .split(|byte| *byte == 0)
        .filter_map(|entry| {
            let tab = entry.iter().position(|byte| *byte == b'\t')?;
            Some(String::from_utf8_lossy(&entry[tab + 1..]).into_owned())
        })
        .filter(|path| !path.is_empty())
        .collect();
    if operation.is_none() && unmerged.is_empty() {
        return None;
    }
    Some(GitOperation {
        operation,
        unmerged: unmerged.into_iter().collect(),
    })
}

/// Refuse a write while a path under `.chough/` is unmerged, naming the paths, before
/// anything is written (REQ-cli-069).
pub(super) fn refuse_if_unmerged(root: &Path) -> Result<()> {
    let Some(git) = git_operation(root) else {
        return Ok(());
    };
    let paths = git.chough_unmerged();
    if paths.is_empty() {
        return Ok(());
    }
    bail!(
        "refused:unmerged: {}, so Chough writes none of its records: {} under .chough/ {} unmerged ({}). Finish or abort the Git operation first (`git status` names its next step), then run the command again; nothing was written",
        git.what(),
        paths.len(),
        if paths.len() == 1 { "is" } else { "are" },
        GitOperation::named(&paths)
    )
}

/// The `git_operation` finding: a merge or rebase in progress, which has no Chough fix.
fn git_operation_finding(root: &Path) -> Option<Finding> {
    let git = git_operation(root)?;
    let mut finding = Finding::new(
        Kind::GitOperation,
        git.operation.unwrap_or("unmerged"),
        "none: finish the Git operation (resolve the conflicts, then commit or continue it) or abort it (`git merge --abort`, `git rebase --abort`, `git cherry-pick --abort`, `git revert --abort`, or after a conflicted `git stash pop`, `git checkout -- <path>` or `git reset`), then run chough doctor again".into(),
        false,
    );
    finding.blocked = Some(git.text());
    finding.detail = Some(git.view());
    Some(finding)
}

/// NUL-separated Git output as paths.
fn paths_of(output: Option<Vec<u8>>) -> Vec<String> {
    output
        .unwrap_or_default()
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path).into_owned())
        .collect()
}

/// Every file Git tracks under `.chough/` and the knowledge directory, sorted.
fn tracked(root: &Path) -> Result<Vec<String>> {
    let dir = crate::prefs::effective(root, "library-dir")?.0;
    let mut found = paths_of(git(root, &["ls-files", "-z", "--", ".chough", &dir]));
    found.sort();
    found.dedup();
    Ok(found)
}

/// The text of a tracked regular file, or None for a link, a missing file or one that is
/// not UTF-8 (an image: its metadata is stripped when it is written, REQ-records-013).
fn tracked_text(root: &Path, path: &str) -> Option<String> {
    let full = local_path(root, path).ok()?;
    if !std::fs::symlink_metadata(&full).ok()?.is_file() {
        return None;
    }
    String::from_utf8(std::fs::read(full).ok()?).ok()
}

/// The text the leak scan reads from a tracked file: a macOS metadata file's bytes with
/// each invalid UTF-8 sequence replaced, since a binary AppleDouble `._*` file can hold
/// a leak in its extended attributes (REQ-cli-068); any other file only when it is valid
/// UTF-8, as before, so no other binary file, such as a screenshot, is read as text.
/// Nothing is ever written from it.
fn leak_scan_text(root: &Path, path: &str) -> Option<String> {
    let metadata = path
        .rsplit('/')
        .next()
        .is_some_and(crate::evidence::is_os_metadata);
    if !metadata {
        return tracked_text(root, path);
    }
    let full = local_path(root, path).ok()?;
    if !std::fs::symlink_metadata(&full).ok()?.is_file() {
        return None;
    }
    Some(String::from_utf8_lossy(&std::fs::read(full).ok()?).into_owned())
}

/// Each leak in the files Git tracks under `.chough/` and the knowledge directory: the
/// file, the line and the kind, never the text (REQ-records-006).
fn leak_findings(root: &Path, files: &[String]) -> Vec<Finding> {
    let local = crate::leaks::Local::current();
    let mut found = vec![];
    for path in files {
        let Some(text) = leak_scan_text(root, path) else {
            continue;
        };
        // REQ-records-006: a temporary path in a case definition is a deliberate test
        // setting, not a leak; everything else in it is checked as anywhere else.
        let case_definition = path.starts_with(".chough/cases/") && path.ends_with(".cases.toml");
        let ids = if path.ends_with(".md") {
            id_lines(&text)
        } else {
            Default::default()
        };
        for (number, line) in text.lines().enumerate() {
            let matches = if case_definition {
                crate::leaks::find_in_case_definition(local, line)
            } else {
                crate::leaks::find(local, line)
            };
            let mut kinds: Vec<&str> = matches.iter().map(|matched| matched.kind.name()).collect();
            kinds.sort_unstable();
            kinds.dedup();
            for kind in kinds {
                let mut finding = Finding::new(
                    Kind::Leak,
                    path,
                    "chough doctor --fix --redact".into(),
                    false,
                );
                finding.detail = Some(json!({"line": number + 1, "match": kind}));
                // REQ-history-012 (#320): an ID is never redacted; the finding names
                // the recovery instead.
                if ids.contains(&number) {
                    let recovery = id_leak_recovery(path);
                    finding.fix = format!("none: {recovery}");
                    finding.blocked = Some(format!(
                        "line {} gives an ID, which `--redact` never rewrites since a redacted ID no longer reads; {recovery}",
                        number + 1
                    ));
                }
                found.push(finding);
            }
        }
    }
    found
}

/// Front matter keys whose values are a record's ID or IDs it references (REQ-records-003).
/// A leak on such a line is never redacted, since a redacted ID no longer reads
/// (REQ-history-012, #320).
const ID_KEYS: [&str; 14] = [
    "id",
    "aliases",
    "native_change",
    "modules",
    "criteria",
    "requirements",
    "cases",
    "observations",
    "bugs",
    "evidence",
    "related",
    "duplicate_of",
    "regression_baseline",
    "case",
];

/// The zero-based numbers of `text`'s front matter lines that give an ID: a line whose
/// key is an ID key, a block-list item under one, and an ID key inside a list of maps.
fn id_lines(text: &str) -> std::collections::BTreeSet<usize> {
    let mut ids = std::collections::BTreeSet::new();
    let mut lines = text.lines().enumerate();
    if !matches!(lines.next(), Some((_, "---"))) {
        return ids;
    }
    let key_of = |line: &str| line.split_once(':').map(|(key, _)| key.trim().to_string());
    let mut under_id_key = false;
    for (number, line) in lines {
        if line == "---" {
            break;
        }
        let continued = line.starts_with(' ') || line.starts_with('-');
        if !continued {
            under_id_key = key_of(line).is_some_and(|key| ID_KEYS.contains(&key.as_str()));
            if under_id_key {
                ids.insert(number);
            }
            continue;
        }
        let item = line.trim_start().trim_start_matches("- ");
        let inner_id = key_of(item).is_some_and(|key| ID_KEYS.contains(&key.as_str()));
        if under_id_key || inner_id {
            ids.insert(number);
        }
    }
    ids
}

/// What to do about a leak in an ID of the record at `path`, which `--redact` never
/// rewrites (#320): shelve a session, otherwise remove it in a reviewed hand edit.
fn id_leak_recovery(path: &str) -> String {
    match path.split('/').collect::<Vec<_>>().as_slice() {
        [".chough", "sessions", id, "session.md"] => format!(
            "set the session aside with `chough session shelve {id}` and continue in `chough session new <key>`"
        ),
        _ => "remove it from the record in a reviewed hand edit".to_string(),
    }
}

/// `text` with its leaks redacted, except on the front matter lines in `keep`: the body
/// as a whole, so a block that spans lines is found, and each other front matter line.
fn redact_except(text: &str, keep: &std::collections::BTreeSet<usize>) -> (String, usize) {
    let local = crate::leaks::Local::current();
    if keep.is_empty() {
        return crate::leaks::redact(local, text);
    }
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let close = lines
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, line)| line.trim_end_matches(['\n', '\r']) == "---")
        .map(|(number, _)| number)
        .unwrap_or(lines.len().saturating_sub(1));
    let mut out = String::with_capacity(text.len());
    let mut count = 0;
    for (number, line) in lines.iter().enumerate().take(close + 1) {
        if number == 0 || number == close || keep.contains(&number) {
            out.push_str(line);
        } else {
            let (redacted, found) = crate::leaks::redact(local, line);
            count += found;
            out.push_str(&redacted);
        }
    }
    let body: String = lines.iter().skip(close + 1).copied().collect();
    let (body, found) = crate::leaks::redact(local, &body);
    out.push_str(&body);
    (out, count + found)
}

/// The ID of a Timeline entry, from its `^t-xxxxxx` anchor, or its position.
fn entry_name(entry: &str, position: usize) -> String {
    entry
        .lines()
        .next()
        .and_then(|line| line.rsplit_once('^'))
        .map(|(_, anchor)| anchor.trim().to_string())
        .filter(|anchor| anchor.starts_with("t-"))
        .unwrap_or_else(|| format!("entry {}", position + 1))
}

/// A record's Timeline entries as written: each `- ` line with the lines under it.
fn timeline_entries(path: &str, text: &str) -> Vec<String> {
    let Ok(record) = crate::markdown::parse(path, text) else {
        return vec![];
    };
    let Some(section) = record.section(crate::session_md::TIMELINE) else {
        return vec![];
    };
    let mut entries: Vec<String> = vec![];
    for line in section.lines().filter(|line| !line.starts_with("## ")) {
        if line.starts_with("- ") {
            entries.push(line.to_string());
        } else if line.starts_with([' ', '\t'])
            && !line.trim().is_empty()
            && let Some(last) = entries.last_mut()
        {
            // An entry continues only on its indented sub-items; a paragraph after the
            // list, such as a story's `Closed <date>: <outcome>.`, is not part of it.
            last.push('\n');
            last.push_str(line);
        }
    }
    entries
}

/// Each session or story whose Timeline at HEAD is not an exact prefix of its working
/// Timeline (REQ-history-002); a session archived since HEAD is compared with its story.
fn timeline_findings(root: &Path) -> Vec<Finding> {
    let mut found = vec![];
    let at_head = paths_of(git(
        root,
        &[
            "ls-tree",
            "-r",
            "-z",
            "--name-only",
            "HEAD",
            "--",
            sessions::DIR,
            STORIES,
        ],
    ));
    for path in at_head {
        let story = path
            .strip_prefix(&format!("{}/", STORIES))
            .is_some_and(|name| !name.contains('/') && name.ends_with(".md"));
        let folder_id = sessions::id_of(&path);
        if !story && folder_id.is_none() {
            continue;
        }
        let working = match &folder_id {
            Some(id) if !local_path(root, &path).is_ok_and(|full| full.is_file()) => {
                sessions::archive_file(id)
            }
            _ => path.clone(),
        };
        let Some(text) = tracked_text(root, &working) else {
            continue;
        };
        let Some(head) = git(root, &["show", &format!("HEAD:{path}")]) else {
            continue;
        };
        let head = timeline_entries(&path, &String::from_utf8_lossy(&head));
        let now = timeline_entries(&working, &text);
        let differing: Vec<String> = head
            .iter()
            .enumerate()
            .filter(|(position, entry)| now.get(*position) != Some(*entry))
            .map(|(position, entry)| entry_name(entry, position))
            .collect();
        if differing.is_empty() {
            continue;
        }
        let mut finding = Finding::new(
            Kind::TimelineRewritten,
            &working,
            format!("none: restore the Timeline entries from `git show HEAD:{path}` by hand"),
            false,
        );
        finding.blocked = Some(
            "a Timeline is append-only and doctor never rewrites one; restore the entries that differ by hand".into(),
        );
        finding.detail = Some(json!({"entries": differing}));
        found.push(finding);
    }
    found
}

/// Every record in `dir` (one level), or every `session.md` under it, with its path.
fn records_in(
    root: &Path,
    dir: &str,
    session_files: bool,
) -> Vec<(String, crate::markdown::Record)> {
    let Ok(entries) = local_path(root, dir).and_then(|path| Ok(std::fs::read_dir(path)?)) else {
        return vec![];
    };
    let mut names: Vec<String> = entries
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| !name.starts_with('.'))
        .collect();
    names.sort();
    names
        .into_iter()
        .filter_map(|name| {
            let path = if session_files {
                format!("{dir}/{name}/session.md")
            } else if name.ends_with(".md") {
                format!("{dir}/{name}")
            } else {
                return None;
            };
            let full = local_path(root, &path).ok()?;
            if !std::fs::symlink_metadata(&full).ok()?.is_file() {
                return None;
            }
            let text =
                crate::records::read_bounded_record(&full, crate::markdown::SESSION_BOUND).ok()?;
            Some((path.clone(), crate::markdown::parse(&path, &text).ok()?))
        })
        .collect()
}

/// Each ID that is the `id` or an alias of two records of one kind (REQ-records-003).
fn alias_findings(root: &Path) -> Vec<Finding> {
    let mut groups: Vec<Vec<(String, crate::markdown::Record)>> = vec![
        records_in(root, crate::bugs::BUGS_DIR, false),
        records_in(root, STORIES, false),
        records_in(root, sessions::DIR, true),
    ];
    for id in sessions::open_ids(root).unwrap_or_default() {
        groups.push(records_in(
            root,
            &format!("{}/{id}/evidence", sessions::DIR),
            false,
        ));
    }
    let mut found = vec![];
    for group in groups {
        let mut holders: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (path, record) in &group {
            let mut names: Vec<String> = record.list("aliases");
            if let Some(id) = record.scalar("id").filter(|id| !id.is_empty()) {
                names.push(id.to_string());
            }
            names.sort();
            names.dedup();
            for name in names {
                holders.entry(name).or_default().push(path.clone());
            }
        }
        for (name, paths) in holders {
            if paths.len() < 2 {
                continue;
            }
            let mut finding = Finding::new(
                Kind::AliasConflict,
                &name,
                "none: keep the ID on one record by hand".into(),
                false,
            );
            finding.blocked = Some(format!(
                "{name} names {}; doctor never rewrites a record's IDs, so decide which keeps it",
                paths.join(" and ")
            ));
            finding.detail = Some(json!({"records": paths}));
            found.push(finding);
        }
    }
    found
}

/// Each conflict marker in the body of a committed record (REQ-records-001, #265): one
/// finding per marker line, naming the file, the line and the marker. Only records are
/// read, the files `bound_of` gives a record's bound; a case definition, a README or a
/// generated index is not one. A conflict is resolved by hand, so there is no fix.
fn conflict_marker_findings(root: &Path, files: &[String]) -> Result<Vec<Finding>> {
    let knowledge = crate::prefs::effective(root, "library-dir")?.0;
    let mut found = vec![];
    // The generated knowledge index has a bound but is not a record.
    let index = format!("{}/index.md", knowledge.trim_end_matches('/'));
    for path in files.iter().filter(|path| path.ends_with(".md")) {
        if *path == index || !matches!(bound_of(path, &knowledge), Some((_, false))) {
            continue;
        }
        let Some(text) = tracked_text(root, path) else {
            continue;
        };
        for (line, marker) in crate::records::conflict_markers(&text) {
            let mut finding = Finding::new(
                Kind::ConflictMarkers,
                path,
                "none: resolve the conflict in the record by hand in a reviewed change, then run chough doctor again".into(),
                false,
            );
            finding.blocked = Some(format!(
                "{path} line {line} holds the Git conflict marker `{marker}`; doctor never rewrites a record"
            ));
            finding.detail = Some(json!({"line": line, "marker": marker}));
            found.push(finding);
        }
    }
    Ok(found)
}

/// The bound of one key screenshot and of all of a session's (REQ-records-013).
const SCREENSHOT_BOUND: u64 = 524_288;
const SESSION_SCREENSHOTS_BOUND: u64 = 2_097_152;
/// The bound of a generated knowledge file (REQ-cli-044).
const KNOWLEDGE_BOUND: u64 = 65_536;

/// A tracked file's bound, by where it lives, and whether it is a screenshot. macOS
/// metadata in `.chough/` is not a record, so it has none and is never reported as
/// `unreadable` or `oversize` (REQ-cli-068); the leak scan still reads it.
fn bound_of(path: &str, knowledge: &str) -> Option<(u64, bool)> {
    if path.starts_with(".chough/") && path.split('/').any(crate::evidence::is_os_metadata) {
        return None;
    }
    let image = [".png", ".jpg", ".jpeg"]
        .iter()
        .any(|extension| path.to_ascii_lowercase().ends_with(extension));
    let parts: Vec<&str> = path.split('/').collect();
    let record = |kind: RecordKind| Some((kind.bound(), false));
    match parts.as_slice() {
        [".chough", "sessions", _, "session.md"] => record(RecordKind::Session),
        [".chough", "sessions", _, "evidence", _] if image => Some((SCREENSHOT_BOUND, true)),
        [".chough", "sessions", _, "evidence", name] if name.ends_with(".md") => {
            record(RecordKind::Evidence)
        }
        [".chough", "sessions", _, "agent", "inbox", .., name] if name.ends_with(".md") => {
            record(RecordKind::Inbox)
        }
        [".chough", "archive", "sessions", name] if name.ends_with(".md") => {
            record(RecordKind::Story)
        }
        [".chough", "archive", "sessions", _, _] if image => Some((SCREENSHOT_BOUND, true)),
        [".chough", "archive", "migrations", name] if name.ends_with(".md") => {
            record(RecordKind::Receipt)
        }
        [".chough", "bugs", name] if name.ends_with(".md") => record(RecordKind::Bug),
        _ => {
            let inside = path.strip_prefix(&format!("{knowledge}/"))?;
            match inside.split('/').collect::<Vec<_>>().as_slice() {
                ["index.md"] | ["journal", "feed.xml"] => Some((KNOWLEDGE_BOUND, false)),
                ["journal", name] if name.ends_with(".md") => record(RecordKind::Journal),
                _ => None,
            }
        }
    }
}

/// Each committed record, evidence entry or screenshot over its bound, and each
/// session whose screenshots together are (REQ-records-001, -005, -013).
fn oversize_findings(root: &Path, files: &[String]) -> Result<Vec<Finding>> {
    let knowledge = crate::prefs::effective(root, "library-dir")?.0;
    let mut found = vec![];
    let mut screenshots: BTreeMap<String, u64> = BTreeMap::new();
    let mut flag = |target: &str, bytes: u64, bound: u64| {
        let mut finding = Finding::new(
            Kind::Oversize,
            target,
            "none: shorten the record, or crop or compress the screenshot, in a reviewed change"
                .into(),
            false,
        );
        finding.blocked = Some(format!(
            "{target} is {bytes} bytes, over its bound of {bound}; doctor never rewrites a record to fit"
        ));
        finding.detail = Some(json!({"bytes": bytes, "bound": bound}));
        found.push(finding);
    };
    for path in files {
        let Some((bound, screenshot)) = bound_of(path, &knowledge) else {
            continue;
        };
        let Ok(metadata) = std::fs::symlink_metadata(local_path(root, path)?) else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }
        if metadata.len() > bound {
            flag(path, metadata.len(), bound);
        }
        if screenshot && let Some((folder, _)) = path.rsplit_once('/') {
            *screenshots.entry(folder.to_string()).or_default() += metadata.len();
        }
    }
    for (folder, bytes) in screenshots {
        if bytes > SESSION_SCREENSHOTS_BOUND {
            flag(&format!("{folder}/"), bytes, SESSION_SCREENSHOTS_BOUND);
        }
    }
    Ok(found)
}

/// Each committed Chough record under `.chough/` that its reader refuses, such as front
/// matter outside the subset, with the file and the reader's reason (REQ-history-012).
/// It has no fix: a person fixes the line in a reviewed change, or sets a session aside
/// with `chough session shelve`. A record over its bound is `oversize` instead.
fn unreadable_findings(root: &Path, files: &[String]) -> Result<Vec<Finding>> {
    let knowledge = crate::prefs::effective(root, "library-dir")?.0;
    let mut found = vec![];
    for path in files.iter().filter(|path| path.starts_with(".chough/")) {
        let Some((bound, false)) = bound_of(path, &knowledge) else {
            continue;
        };
        let Some(text) = tracked_text(root, path) else {
            continue;
        };
        if text.len() as u64 > bound {
            continue;
        }
        let parts: Vec<&str> = path.split('/').collect();
        let parsed = crate::markdown::parse(path, &text);
        let mut refused = parsed
            .as_ref()
            .err()
            .map(|error| anyhow::anyhow!("{error:#}"));
        if refused.is_none()
            && matches!(
                parts.as_slice(),
                [".chough", "sessions", _, "session.md"] | [".chough", "archive", "sessions", _]
            )
        {
            refused = sessions::read_at(root, path).err();
        }
        // An archived story's receipts, in its `## Receipts` section or an older story's
        // front matter, are read too (REQ-history-006).
        if refused.is_none()
            && matches!(parts.as_slice(), [".chough", "archive", "sessions", _])
            && let Ok(record) = &parsed
        {
            refused = record.story_receipts(path).err();
        }
        let Some(error) = refused else {
            continue;
        };
        let fix = match parts.as_slice() {
            [".chough", "sessions", id, "session.md"] => format!(
                "none: fix the line by hand in a reviewed change, or set the session aside with `chough session shelve {id}`"
            ),
            _ => "none: fix the line by hand in a reviewed change".into(),
        };
        let mut finding = Finding::new(Kind::Unreadable, path, fix, false);
        finding.blocked = Some(format!(
            "{path} cannot be read; doctor never rewrites a record"
        ));
        finding.detail = Some(json!({"error": format!("{error:#}")}));
        found.push(finding);
    }
    Ok(found)
}

/// The SDD policy whose `verification_ignored_paths` keeps Chough's records out of
/// SpecSync's verification (REQ-records-014).
pub(super) const SDD_POLICY: &str = ".specsync/sdd.json";
/// The folders Chough writes while work is in progress, which hold only its records.
pub(super) const RECORD_PREFIXES: [&str; 3] =
    [".chough/sessions/", ".chough/archive/", ".chough/bugs/"];
/// The first SpecSync release that honours `verification_ignored_paths`.
pub(super) const EXCLUSIONS_SINCE: &str = "6.1.2";

/// What `.specsync/sdd.json` says about Chough's record folders (REQ-records-014). Chough
/// reads the policy at full size and never writes it.
pub(super) struct Exclusions {
    /// No `.specsync/sdd.json`: no SDD policy, so no native verification digests them.
    pub(super) no_policy: bool,
    /// The policy could not be read or parsed as JSON.
    pub(super) unreadable: Option<String>,
    /// Record prefixes the list does not name exactly.
    pub(super) missing: Vec<&'static str>,
    /// Listed entries broader than a record prefix, such as `.chough/`, which also hide
    /// case definitions and preferences that a change's verification needs.
    pub(super) broader: Vec<String>,
}

impl Exclusions {
    /// Whether every record folder is left out of verification, exactly or by a broader
    /// entry, so Chough's writes after `change check --commit` stale nothing.
    pub(super) fn in_force(&self) -> bool {
        !self.no_policy
            && self.unreadable.is_none()
            && RECORD_PREFIXES.iter().all(|prefix| {
                !self.missing.contains(prefix)
                    || self
                        .broader
                        .iter()
                        .any(|entry| prefix.starts_with(entry.as_str()))
            })
    }

    /// The advice `chough doctor` and `chough init` give while a record prefix is not
    /// listed exactly, or None when all three are.
    pub(super) fn advice(&self) -> Option<String> {
        if self.no_policy {
            return None;
        }
        if let Some(error) = &self.unreadable {
            return Some(format!(
                "{SDD_POLICY} cannot be read ({error}), so Chough cannot tell whether {} are left out of SpecSync's verification; fix the policy in an ordinary reviewed change. Chough never writes it.",
                RECORD_PREFIXES.join(", ")
            ));
        }
        if self.missing.is_empty() {
            return None;
        }
        let mut advice = format!(
            "List {} in verification_ignored_paths in {SDD_POLICY} (SpecSync {EXCLUSIONS_SINCE} and later), so a Chough record written after `specsync change check --commit` leaves the recorded verification current; adding them is an ordinary reviewed change to {SDD_POLICY}, and Chough never writes it.",
            self.missing.join(", ")
        );
        if !self.broader.is_empty() {
            advice.push_str(&format!(
                " {} also leaves .chough/cases/ and .chough/preferences.json out of verification, which a change's verification needs; list the three record folders instead.",
                self.broader.join(", ")
            ));
        }
        Some(advice)
    }

    /// When Chough's records are safe to write, for `chough next` (REQ-records-014).
    pub(super) fn writes(&self) -> Value {
        let (state, text) = if self.no_policy {
            (
                "any-point",
                format!(
                    "no {SDD_POLICY}, so no native verification digests Chough's records; they may be written and committed at any point"
                ),
            )
        } else if self.in_force() {
            (
                "any-point",
                format!(
                    "{SDD_POLICY} leaves {} out of verification, so Chough's records may be written and committed at any point of the native order",
                    RECORD_PREFIXES.join(", ")
                ),
            )
        } else {
            (
                "commit-first",
                format!(
                    "commit Chough's records before `specsync change check --commit` and write nothing under .chough/ until finalize: {SDD_POLICY} does not leave {} out of verification (`chough doctor` explains)",
                    if self.missing.is_empty() {
                        RECORD_PREFIXES.join(", ")
                    } else {
                        self.missing.join(", ")
                    }
                ),
            )
        };
        json!({"state": state, "text": text, "missing": self.missing, "policy": SDD_POLICY})
    }
}

/// Read `.specsync/sdd.json`'s `verification_ignored_paths` (REQ-records-014).
pub(super) fn exclusions(root: &Path) -> Exclusions {
    let mut found = Exclusions {
        no_policy: false,
        unreadable: None,
        missing: vec![],
        broader: vec![],
    };
    let path = match local_path(root, SDD_POLICY) {
        Ok(path) => path,
        Err(error) => {
            found.unreadable = Some(format!("{error:#}"));
            return found;
        }
    };
    if !path.exists() {
        found.no_policy = true;
        return found;
    }
    let policy = crate::records::read_native_text(&path)
        .and_then(|text| serde_json::from_str::<Value>(&text).map_err(anyhow::Error::from));
    let policy = match policy {
        Ok(policy) => policy,
        Err(error) => {
            found.unreadable = Some(format!("{error:#}"));
            return found;
        }
    };
    let listed: Vec<String> = policy["verification_ignored_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.as_str().map(str::to_string))
        .collect();
    found.missing = RECORD_PREFIXES
        .into_iter()
        .filter(|prefix| !listed.iter().any(|entry| entry == prefix))
        .collect();
    found.broader = listed
        .into_iter()
        .filter(|entry| {
            RECORD_PREFIXES
                .iter()
                .any(|prefix| prefix.len() > entry.len() && prefix.starts_with(entry.as_str()))
        })
        .collect();
    found
}

/// The verification-exclusions finding, while `.specsync/sdd.json` exists and does not
/// list every record prefix (REQ-records-014). Doctor never writes the policy, so the
/// finding is never fixed by `--fix`.
fn records_in_verification(root: &Path) -> Option<Finding> {
    let exclusions = exclusions(root);
    let advice = exclusions.advice()?;
    let mut finding = Finding::new(
        Kind::RecordsInVerification,
        SDD_POLICY,
        format!(
            "add {} to verification_ignored_paths in {SDD_POLICY} in a reviewed change",
            if exclusions.missing.is_empty() {
                RECORD_PREFIXES.join(", ")
            } else {
                exclusions.missing.join(", ")
            }
        ),
        false,
    );
    finding.blocked = Some(format!(
        "Chough never writes {SDD_POLICY}; adding the entries is an ordinary reviewed change to it"
    ));
    finding.detail = Some(json!({
        "missing": exclusions.missing,
        "specsync": format!("{EXCLUSIONS_SINCE} and later"),
        "advice": advice,
    }));
    Some(finding)
}

/// The merge-setup finding (REQ-cli-047, DRIVE-12): in a Git work tree, the merge
/// drivers Git has no command for in this clone, as a fresh clone has none, and the
/// `.gitattributes` lines no line declares. `--fix` sets the drivers in the clone's local
/// config; it never writes `.gitattributes`, a committed file a person reviews, so the
/// missing lines leave the finding open, naming them.
fn merge_drivers(root: &Path) -> Result<Option<Finding>> {
    if !crate::library::in_git_work_tree(root) {
        return Ok(None);
    }
    let drivers = crate::library::missing_merge_drivers(root);
    let lines = crate::library::missing_merge_lines(root)?;
    if drivers.is_empty() && lines.is_empty() {
        return Ok(None);
    }
    let declare = "add the missing lines to .gitattributes (`chough init` appends exactly those, changing no line) and commit it";
    let (target, fix) = match (drivers.first(), lines.is_empty()) {
        (Some(driver), true) => (
            format!("merge.{driver}.driver"),
            "chough doctor --fix".to_string(),
        ),
        (Some(_), false) => (
            ".gitattributes".to_string(),
            format!("chough doctor --fix, then {declare}"),
        ),
        (None, _) => (".gitattributes".to_string(), declare.to_string()),
    };
    let mut finding = Finding::new(Kind::MergeDrivers, &target, fix, false);
    if drivers.is_empty() {
        finding.blocked = Some(format!("doctor never writes .gitattributes; {declare}"));
    }
    finding.detail = Some(json!({"missing_drivers": drivers, "missing_lines": lines}));
    Ok(Some(finding))
}

/// The command the README's Install section gives for Chough itself, which its
/// toolchain section points to.
const INSTALL_CHOUGH: &str = "cargo install chough --locked";

/// The outdated-driver finding (REQ-history-012, F13): the merge drivers are set, and the
/// `chough` they run, found through `PATH` as Git finds it, is missing or reports a
/// version older than the running Chough, so a merge would run older code on these
/// records. It has no fix, since Chough never installs itself; it names both versions
/// and the install command.
fn merge_driver_outdated(root: &Path) -> Option<Finding> {
    if !crate::library::in_git_work_tree(root) {
        return None;
    }
    let running = env!("CARGO_PKG_VERSION");
    let current = crate::library::version_triple(running)?;
    let mut programs: Vec<(String, Vec<&str>)> = vec![];
    for (driver, program) in crate::library::chough_driver_programs(root) {
        match programs.iter_mut().find(|(known, _)| *known == program) {
            Some((_, drivers)) => drivers.push(driver),
            None => programs.push((program, vec![driver])),
        }
    }
    for (program, drivers) in programs {
        let found = crate::library::version_as_git_runs(root, &program);
        if found
            .as_deref()
            .and_then(crate::library::version_triple)
            .is_some_and(|version| version >= current)
        {
            continue;
        }
        let says = match &found {
            Some(version) => format!("reports {version}"),
            None => "is not found or reports no version".to_string(),
        };
        let mut finding = Finding::new(
            Kind::MergeDriverOutdated,
            &program,
            format!(
                "install Chough {running} where Git finds `{program}` on PATH: {INSTALL_CHOUGH}"
            ),
            false,
        );
        finding.blocked = Some(format!(
            "Chough never installs itself; the `{program}` the merge drivers run {says}, older than Chough {running}, so install Chough {running} ({INSTALL_CHOUGH}) and run chough doctor again"
        ));
        finding.detail = Some(json!({
            "drivers": drivers,
            "running_version": running,
            "path_version": found,
            "install": INSTALL_CHOUGH,
        }));
        return Some(finding);
    }
    None
}

/// The findings the verify lane's `leaks` step fails on: `leak` and `oversize`.
fn leak_and_oversize(root: &Path) -> Result<Vec<Finding>> {
    let files = tracked(root)?;
    let mut found = leak_findings(root, &files);
    found.extend(oversize_findings(root, &files)?);
    Ok(found)
}

/// Every finding, in the order the fixes run (REQ-history-012).
fn findings(root: &Path, args: &DoctorArgs) -> Result<Vec<Finding>> {
    let mut found = vec![];
    let unmigrated = unmigrated(root)?;
    let migrated = unmigrated.is_none();
    found.extend(unmigrated);
    let files = tracked(root)?;
    found.extend(leak_findings(root, &files));
    // A 0.5.x tree has no 0.6.0 sessions to check; `chough migrate` comes first.
    if migrated && sessions::enabled(root)? {
        found.extend(session_findings(root, args)?);
        found.extend(timeline_findings(root));
        found.extend(alias_findings(root));
    }
    found.extend(oversize_findings(root, &files)?);
    if migrated {
        found.extend(unreadable_findings(root, &files)?);
    }
    found.extend(records_in_verification(root));
    found.extend(merge_drivers(root)?);
    found.extend(merge_driver_outdated(root));
    found.extend(conflict_marker_findings(root, &files)?);
    found.extend(git_operation_finding(root));
    if migrated {
        found.extend(stale_bug_link_findings(root));
    }
    Ok(found)
}

/// REQ-history-012 (#347): one finding per bug whose body links an evidence entry that
/// left with its archived session; the fix appends a Log line linking the archived
/// story, and rewrites nothing.
fn stale_bug_link_findings(root: &Path) -> Vec<Finding> {
    crate::bugs::stale_evidence_links(root)
        .into_iter()
        .map(|found| {
            let (path, stale) = (found.path, found.links);
            // #347 (62-F9): a bug too near its size bound cannot take the line, so the
            // finding says where the evidence is and offers no fix that never applies.
            let by_hand: Vec<String> = stale
                .iter()
                .map(|(entry, session)| {
                    format!(
                        "../sessions/{session}/evidence/{entry}.md with ../archive/sessions/{session}.md"
                    )
                })
                .collect();
            let fix = if found.fits {
                "chough doctor --fix".to_string()
            } else {
                format!(
                    "none: the bug is too near its size bound for one more Log line; to clear this, edit {path} by hand and replace the link target {}",
                    by_hand.join(", and ")
                )
            };
            let mut finding = Finding::new(Kind::StaleBugLinks, &path, fix, false);
            if !found.fits {
                finding.advisory = true;
                finding.blocked = Some(format!(
                    "{path} is too near its 65,536-byte bound to take one more Log line, so doctor leaves it as it is; the linked evidence is in the archived story"
                ));
            }
            finding.detail = Some(json!({
                "links": stale
                    .iter()
                    .map(|(entry, session)| json!({
                        "entry": entry,
                        "story": format!(".chough/archive/sessions/{session}.md"),
                    }))
                    .collect::<Vec<_>>(),
            }));
            finding
        })
        .collect()
}

/// The session findings, in fix order: stale pointer, finished sessions, unfinished
/// archives, stale gates and the sessions named with `--close`.
fn session_findings(root: &Path, args: &DoctorArgs) -> Result<Vec<Finding>> {
    let mut found = vec![];
    let pointer = sessions::pointed(root);
    let open = sessions::open_ids(root)?;
    match &pointer {
        Ok(Some(key)) if !open.contains(key) => found.push(Finding::new(
            Kind::StalePointer,
            key,
            "chough doctor --fix".into(),
            false,
        )),
        Err(_) => found.push(Finding::new(
            Kind::StalePointer,
            sessions::POINTER,
            "chough doctor --fix".into(),
            false,
        )),
        _ => {}
    }
    let closing: Vec<&String> = args.close.iter().collect();
    let mut gates = vec![];
    let mut finished = vec![];
    for key in sessions::keys_in(root, sessions::DIR)? {
        // An unreadable session is `session shelve`'s; the doctor leaves it.
        let Ok(session) = sessions::read_at(root, &sessions::session_file(&key)) else {
            continue;
        };
        match session_check(root, &key, &session, || sessions::unfolded(root, &key))? {
            Some(finding) if finding.kind == Kind::StaleGates => gates.push(finding),
            Some(finding) => finished.push(finding),
            None => {}
        }
    }
    found.extend(finished);
    let mut archives = vec![];
    // REQ-history-012: a session folder whose story exists still holds files the
    // archive could not remove because they were not committed (REQ-records-012).
    for id in sessions::unfinished_archives(root)? {
        let mut finding = Finding::new(
            Kind::UnfinishedArchive,
            &id,
            "chough doctor --fix".into(),
            false,
        );
        let uncommitted = sessions::uncommitted_files(root, &id)?;
        if !uncommitted.is_empty() {
            finding.fix = format!(
                "commit {}, then chough doctor --fix",
                uncommitted.join(", ")
            );
            finding.blocked = Some(format!(
                "{} not committed at HEAD; commit it first, so Git holds every byte removed",
                uncommitted.join(", ")
            ));
        }
        archives.push(finding);
    }
    found.extend(archives);
    found.extend(gates);
    for name in closing {
        let key = &sessions::resolve_open(root, name)
            .map_err(|error| anyhow::anyhow!("--close {name}: {error}"))?;
        let file = sessions::session_file(key);
        let session = sessions::read_at(root, &file)?;
        ensure!(
            !sessions::finished(root, &session),
            "--close {key}: session {key} is finished; `chough doctor --fix` archives it"
        );
        found.push(Finding::new(
            Kind::UnfinishedSession,
            key,
            format!("chough session close {key} --reason <text>"),
            true,
        ));
    }
    Ok(found)
}

/// Doctor's check of one open session (REQ-cli-055): a finished session that holds open
/// gates on its archived native change is `stale_gates`, a decision; another finished one
/// is `finished_session`, blocked while `unfolded` counts inbox entries; an unfinished
/// one has no finding. `chough migrate` runs it on the records it plans
/// ([`session_follow_ups`], REQ-records-009), so on a tree just migrated doctor reports
/// exactly the sessions the follow-ups named.
fn session_check(
    root: &Path,
    key: &str,
    session: &checkpoint::Checkpoint,
    unfolded: impl FnOnce() -> Result<usize>,
) -> Result<Option<Finding>> {
    if !sessions::finished(root, session) {
        return Ok(None);
    }
    if !session.awaiting_human.is_empty() && archived(root, session.native_change.as_deref()) {
        return Ok(Some(Finding::new(
            Kind::StaleGates,
            key,
            "chough doctor --fix --reason <text>".into(),
            true,
        )));
    }
    let mut finding = Finding::new(
        Kind::FinishedSession,
        key,
        "chough doctor --fix".into(),
        false,
    );
    if unfolded()? > 0 {
        finding.fix = format!("chough session use {key} && chough inbox fold");
        finding.blocked = Some(format!(
            "session {key} has unfolded inbox entries; fold them first"
        ));
    }
    Ok(Some(finding))
}

/// The follow-ups a migration plan gives for what [`session_check`] finds on the migrated
/// tree (REQ-records-009, #246): in `planned`, each open session the plan writes, with
/// its record and its unfolded inbox entries, then each session already open that the
/// plan leaves, read as doctor reads it. One follow-up names the finished sessions that
/// `chough doctor --fix` archives, and one each session whose open gates are on an
/// archived native change, with each gate's text and the decision it needs.
pub(super) fn session_follow_ups(
    root: &Path,
    planned: &[(String, checkpoint::Checkpoint, usize)],
) -> Result<Vec<String>> {
    let mut checked = vec![];
    for (key, session, unfolded) in planned {
        if let Some(finding) = session_check(root, key, session, || Ok(*unfolded))? {
            checked.push((finding, session.clone()));
        }
    }
    for key in sessions::keys_in(root, sessions::DIR)? {
        if planned.iter().any(|(planned, _, _)| *planned == key) {
            continue;
        }
        let Ok(session) = sessions::read_at(root, &sessions::session_file(&key)) else {
            continue;
        };
        if let Some(finding) =
            session_check(root, &key, &session, || sessions::unfolded(root, &key))?
        {
            checked.push((finding, session));
        }
    }
    let mut finished = vec![];
    let mut gated = vec![];
    for (finding, session) in &checked {
        let key = &finding.target;
        if finding.kind == Kind::StaleGates {
            let gates: Vec<String> = session
                .awaiting_human
                .iter()
                .map(|gate| format!("\"{gate}\""))
                .collect();
            gated.push(format!(
                "session `{key}`, whose native change `{}` is archived, waits on {}",
                session.native_change.as_deref().unwrap_or_default(),
                gates.join(" and ")
            ));
        } else if finding.blocked.is_some() {
            finished.push(format!(
                "`{key}`, once its inbox entries are folded with `{}`",
                finding.fix
            ));
        } else {
            finished.push(format!("`{key}`"));
        }
    }
    let mut follow_ups = vec![];
    if !finished.is_empty() {
        follow_ups.push(format!(
            "After the apply, `chough doctor` reports {} finished, at done or with its native change archived (`finished_session`, REQ-cli-055): {}. Run `chough doctor --fix`, which archives each into its story, then commit the result.",
            if finished.len() == 1 {
                "this session"
            } else {
                "these sessions"
            },
            finished.join("; ")
        ));
    }
    if !gated.is_empty() {
        follow_ups.push(format!(
            "After the apply, `chough doctor` reports open human gates on archived native changes (`stale_gates`, REQ-cli-055): {}. Withdrawing a gate is the human's decision, so ask them in the same round as this consent; then run `chough doctor --fix --reason \"<their decision>\"`, which withdraws the gates as `chough checkpoint --clear-awaiting-human` does, recording no human decision, and archives the session.",
            gated.join("; ")
        ));
    }
    Ok(follow_ups)
}

/// Apply one finding's fix (REQ-history-012).
fn apply(root: &Path, finding: &Finding, _args: &DoctorArgs, reason: Option<&str>) -> Result<()> {
    match finding.kind {
        Kind::Unmigrated => bail!("run `chough migrate`; doctor never migrates"),
        Kind::RecordsInVerification => bail!("Chough never writes {SDD_POLICY}"),
        Kind::MergeDrivers => {
            let drivers = finding.detail.as_ref().map_or_else(Vec::new, |detail| {
                detail["missing_drivers"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                    .collect::<Vec<_>>()
            });
            crate::library::set_merge_drivers(root, &drivers)?;
            let lines = crate::library::missing_merge_lines(root)?;
            ensure!(
                lines.is_empty(),
                "the drivers are set; doctor never writes .gitattributes, so add {} to it (`chough init` appends exactly those, changing no line) and commit it",
                lines.join(", ")
            );
        }
        Kind::TimelineRewritten
        | Kind::AliasConflict
        | Kind::Oversize
        | Kind::Unreadable
        | Kind::MergeDriverOutdated
        | Kind::ConflictMarkers
        | Kind::GitOperation => {
            bail!("{} has no fix", finding.kind.name())
        }
        Kind::Leak => {
            // `--redact` replaces each match in the file in place; a file an earlier
            // finding already redacted has nothing left to replace.
            let path = local_path(root, &finding.target)?;
            // A file that is not valid UTF-8, such as a binary AppleDouble `._*` file,
            // is reported but never rewritten: writing back decoded text would corrupt
            // it (REQ-cli-068).
            let Some(text) = tracked_text(root, &finding.target) else {
                bail!(
                    "{} is not valid UTF-8, so doctor leaves it unchanged; remove it by hand, for example with `git rm`",
                    finding.target
                );
            };
            // REQ-history-012 (#320): a record's ID lines are never rewritten; its other
            // lines still are, whichever of its findings comes first.
            let ids = if finding.target.ends_with(".md") {
                id_lines(&text)
            } else {
                Default::default()
            };
            let (redacted, count) = redact_except(&text, &ids);
            if count > 0 {
                crate::records::write_replace(root, &finding.target, &redacted)?;
            }
            ensure!(path.is_file(), "{} is gone", finding.target);
            if let Some(blocked) = &finding.blocked {
                bail!("{blocked}");
            }
        }
        Kind::StaleBugLinks => crate::bugs::relink_archived_evidence(root, &finding.target)?,
        Kind::StalePointer => {
            let path = local_path(root, sessions::POINTER)?;
            if path.exists() {
                std::fs::remove_file(path)?;
            }
        }
        Kind::StaleGates => {
            sessions::pin(root, &sessions::session_file(&finding.target));
            let result = checkpoint::withdraw_gates(root, reason.unwrap_or_default());
            sessions::unpin(root);
            result?;
        }
        Kind::FinishedSession => sessions::archive_open(root, &finding.target)?,
        Kind::UnfinishedArchive => {
            let kept = sessions::remove_archived_folder(root, &finding.target)?;
            ensure!(
                kept.is_empty(),
                "{} changed since it was checked; commit it, then run chough doctor --fix again",
                kept.join(", ")
            );
        }
        Kind::UnfinishedSession => {
            sessions::close_session(root, &finding.target, reason.unwrap_or_default())?;
        }
    }
    Ok(())
}

/// Run the doctor; returns whether any finding is left unresolved (exit 1).
pub(super) fn run(root: &Path, args: DoctorArgs) -> Result<bool> {
    let reason = args.reason.as_deref().map(str::trim);
    ensure!(
        reason.is_none_or(|reason| !reason.is_empty()),
        "--reason must not be blank"
    );
    let mode = if args.fix {
        "fix"
    } else if args.dry_run {
        "dry-run"
    } else {
        "report"
    };
    if args.leaks {
        let found = leak_and_oversize(root)?;
        print_json(&json!({
            "schema_version": 1,
            "mode": "leaks",
            "findings": found.iter().map(Finding::json).collect::<Vec<_>>(),
        }))?;
        return Ok(!found.is_empty());
    }
    if args.fix {
        sessions::lock_project(root)?;
    }
    let mut found = findings(root, &args)?;
    if args.fix {
        let mut extra = vec![];
        let mut index = 0;
        while index < found.len() {
            let finding = &mut found[index];
            index += 1;
            if let Some(blocked) = finding.blocked.take() {
                finding.skip(blocked);
                continue;
            }
            if finding.kind == Kind::Leak && !crate::leaks::redacting() {
                finding.skip(
                    "run `chough doctor --fix --redact` to replace each match with `<redacted:<kind>>` in place".into(),
                );
                continue;
            }
            if finding.decision && reason.is_none() {
                finding.skip(format!(
                    "a decision: run `{}` with a reason to apply it",
                    finding.fix
                ));
                continue;
            }
            let finding = &found[index - 1];
            match apply(root, finding, &args, reason) {
                Ok(()) => {
                    found[index - 1].status = "fixed";
                    // REQ-history-012 (#250): a decision the fix applied echoes the
                    // reason it was given.
                    if found[index - 1].decision
                        && let Some(reason) = reason
                    {
                        // What was written: under `--redact` the reason is echoed as
                        // redacted, never raw (REQ-records-006).
                        let mut echoed = reason.to_string();
                        let _ = crate::leaks::guard(
                            crate::leaks::Local::current(),
                            "reason",
                            &mut echoed,
                        );
                        found[index - 1].decision_reason = Some(echoed);
                    }
                    // A withdrawn gate leaves a finished session, archived next.
                    if found[index - 1].kind == Kind::StaleGates {
                        let key = found[index - 1].target.clone();
                        let mut archive = Finding::new(
                            Kind::FinishedSession,
                            &key,
                            "chough doctor --fix".into(),
                            false,
                        );
                        match sessions::unfolded(root, &key) {
                            Ok(0) => match sessions::archive_open(root, &key) {
                                Ok(()) => archive.status = "fixed",
                                Err(error) => archive.skip(format!("{error:#}")),
                            },
                            Ok(_) => {
                                archive.fix =
                                    format!("chough session use {key} && chough inbox fold");
                                archive.skip(format!(
                                    "session {key} has unfolded inbox entries; fold them first"
                                ));
                            }
                            Err(error) => archive.skip(format!("{error:#}")),
                        }
                        extra.push(archive);
                    }
                }
                Err(error) => found[index - 1].skip(format!("{error:#}")),
            }
        }
        found.extend(extra);
        // A pointer to a session this run archived is stale now.
        if let Ok(Some(key)) = sessions::pointer(root)
            && !local_path(root, &sessions::session_file(&key))?.is_file()
        {
            std::fs::remove_file(local_path(root, sessions::POINTER)?)?;
            let mut cleared = Finding::new(
                Kind::StalePointer,
                &key,
                "chough doctor --fix".into(),
                false,
            );
            cleared.status = "fixed";
            found.push(cleared);
        }
    }
    // An advisory finding is said, and never fails the run (#347).
    let unresolved = found
        .iter()
        .any(|finding| finding.status != "fixed" && !finding.advisory);
    print_json(&json!({
        "schema_version": 1,
        "mode": mode,
        "findings": found.iter().map(Finding::json).collect::<Vec<_>>(),
    }))?;
    Ok(unresolved)
}
