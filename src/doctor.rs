//! `chough doctor`: find and fix the Chough debt that stops a fresh worktree from
//! entering Chough (REQ-cli-055, REQ-history-012, DRIVE-17).
//!
//! It reads only `.chough/` and the archives' `finalization.json`; it never calls
//! SpecSync and never records a human decision, approval, review or consent.

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
    /// The session key for a legacy `.chough/session.json` with no native change.
    #[arg(long = "adopt-as")]
    adopt_as: Option<String>,
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    LegacyCheckpoint,
    StalePointer,
    StaleGates,
    FinishedSession,
    UnfinishedSession,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Self::LegacyCheckpoint => "legacy_checkpoint",
            Self::StalePointer => "stale_pointer",
            Self::StaleGates => "stale_gates",
            Self::FinishedSession => "finished_session",
            Self::UnfinishedSession => "unfinished_session",
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
        value
    }
}

/// Whether `change` is archived on HEAD, from its `finalization.json` (no native read).
fn archived(root: &Path, change: Option<&str>) -> bool {
    change.is_some_and(|change| crate::library::finalized_at(root, change).is_some())
}

/// The 0.5.x checkpoint finding, if `.chough/session.json` exists: the project is not
/// migrated (REQ-records-010). Doctor never migrates; the finding names the command.
fn legacy(root: &Path, _adopt_as: Option<&str>) -> Result<Option<Finding>> {
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

/// Every finding, in the order the fixes run (REQ-history-012).
fn findings(root: &Path, args: &DoctorArgs) -> Result<Vec<Finding>> {
    let mut found = vec![];
    if let Some(finding) = legacy(root, args.adopt_as.as_deref())? {
        found.push(finding);
    }
    if !sessions::enabled(root)? {
        return Ok(found);
    }
    let pointer = sessions::pointer(root);
    match &pointer {
        Ok(Some(key)) if !local_path(root, &sessions::session_file(key))?.is_file() => found.push(
            Finding::new(Kind::StalePointer, key, "chough doctor --fix".into(), false),
        ),
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
    for key in closing {
        sessions::ensure_key(key)?;
        let file = sessions::session_file(key);
        ensure!(
            local_path(root, &file)?.is_file(),
            "--close {key}: no open session {key}"
        );
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
