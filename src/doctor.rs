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
    /// Why the fix cannot run even with `--fix`, when known before fixing.
    blocked: Option<String>,
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
            blocked: None,
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

/// Each leak in the files Git tracks under `.chough/` and the knowledge directory: the
/// file, the line and the kind, never the text (REQ-records-006).
fn leak_findings(root: &Path, files: &[String]) -> Vec<Finding> {
    let local = crate::leaks::Local::current();
    let mut found = vec![];
    for path in files {
        let Some(text) = tracked_text(root, path) else {
            continue;
        };
        // REQ-records-006: a temporary path in a case definition is a deliberate test
        // setting, not a leak; everything else in it is checked as anywhere else.
        let case_definition = path.starts_with(".chough/cases/") && path.ends_with(".cases.toml");
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
                found.push(finding);
            }
        }
    }
    found
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

/// The bound of one key screenshot and of all of a session's (REQ-records-013).
const SCREENSHOT_BOUND: u64 = 524_288;
const SESSION_SCREENSHOTS_BOUND: u64 = 2_097_152;
/// The bound of a generated knowledge file (REQ-cli-044).
const KNOWLEDGE_BOUND: u64 = 65_536;

/// A tracked file's bound, by where it lives, and whether it is a screenshot.
fn bound_of(path: &str, knowledge: &str) -> Option<(u64, bool)> {
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
    Ok(found)
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
        if !sessions::finished(root, &session) {
            continue;
        }
        if !session.awaiting_human.is_empty() && archived(root, session.native_change.as_deref()) {
            gates.push(Finding::new(
                Kind::StaleGates,
                &key,
                "chough doctor --fix --reason <text>".into(),
                true,
            ));
            continue;
        }
        let mut finding = Finding::new(
            Kind::FinishedSession,
            &key,
            "chough doctor --fix".into(),
            false,
        );
        if sessions::unfolded(root, &key)? > 0 {
            finding.fix = format!("chough session use {key} && chough inbox fold");
            finding.blocked = Some(format!(
                "session {key} has unfolded inbox entries; fold them first"
            ));
        }
        finished.push(finding);
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
        | Kind::MergeDriverOutdated => {
            bail!("{} has no fix", finding.kind.name())
        }
        Kind::Leak => {
            // `--redact` replaces each match in the file in place; a file an earlier
            // finding already redacted has nothing left to replace.
            let path = local_path(root, &finding.target)?;
            let Some(text) = tracked_text(root, &finding.target) else {
                bail!("{} is not a readable text file", finding.target);
            };
            let (redacted, count) = crate::leaks::redact(crate::leaks::Local::current(), &text);
            if count > 0 {
                crate::records::write_replace(root, &finding.target, &redacted)?;
            }
            ensure!(path.is_file(), "{} is gone", finding.target);
        }
        Kind::StalePointer => {
            let path = local_path(root, sessions::POINTER)?;
            if path.exists() {
                std::fs::remove_file(path)?;
            }
        }
        Kind::StaleGates => {
            sessions::pin(root, &sessions::session_file(&finding.target));
            let result = checkpoint::withdraw_gates(root);
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
    let unresolved = found.iter().any(|finding| finding.status != "fixed");
    print_json(&json!({
        "schema_version": 1,
        "mode": mode,
        "findings": found.iter().map(Finding::json).collect::<Vec<_>>(),
    }))?;
    Ok(unresolved)
}
