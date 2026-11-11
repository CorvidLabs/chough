//! `chough doctor`: find and fix the Chough debt that stops a fresh worktree from
//! entering Chough (REQ-cli-055, REQ-history-012, DRIVE-17).
//!
//! It reads only `.chough/`, the archives' `finalization.json` and `.specsync/sdd.json`;
//! it never calls SpecSync, never writes `.specsync/sdd.json`, and never records a human
//! decision, approval, review or consent.

use crate::checkpoint::{self};
use crate::records::{local_path, print_json};
use crate::sessions;
use anyhow::{Result, bail, ensure};
use serde_json::{Value, json};
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
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    LegacyCheckpoint,
    StalePointer,
    StaleGates,
    FinishedSession,
    UnfinishedSession,
    UnfinishedArchive,
    RecordsInVerification,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Self::LegacyCheckpoint => "legacy_checkpoint",
            Self::StalePointer => "stale_pointer",
            Self::StaleGates => "stale_gates",
            Self::FinishedSession => "finished_session",
            Self::UnfinishedSession => "unfinished_session",
            Self::UnfinishedArchive => "unfinished_archive",
            Self::RecordsInVerification => "records_in_verification",
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

/// The 0.5.x checkpoint finding, if `.chough/session.json` exists: the project is not
/// migrated (REQ-records-010). Doctor never migrates; the finding names the command.
fn legacy(root: &Path) -> Result<Option<Finding>> {
    let target = checkpoint::SESSION;
    if !local_path(root, target)?.exists() {
        return Ok(None);
    }
    let mut finding = Finding::new(
        Kind::LegacyCheckpoint,
        target,
        "chough migrate".into(),
        false,
    );
    finding.blocked = Some(format!(
        "{target} is a 0.5.x checkpoint; run `chough migrate`, which doctor never runs itself"
    ));
    Ok(Some(finding))
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

/// Every finding, in the order the fixes run (REQ-history-012).
fn findings(root: &Path, args: &DoctorArgs) -> Result<Vec<Finding>> {
    let mut found = vec![];
    if let Some(finding) = legacy(root)? {
        found.push(finding);
    }
    if let Some(finding) = records_in_verification(root) {
        found.push(finding);
    }
    if !sessions::enabled(root)? {
        return Ok(found);
    }
    let pointer = sessions::pointer(root);
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
    found.extend(gates);
    found.extend(finished);
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
        found.push(finding);
    }
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
        Kind::LegacyCheckpoint => bail!(
            "{} is a 0.5.x checkpoint; run `chough migrate`",
            checkpoint::SESSION
        ),
        Kind::RecordsInVerification => bail!("Chough never writes {SDD_POLICY}"),
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
