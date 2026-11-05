//! The native check in `chough next` (REQ-cli-039, #86): the latest attempt SpecSync
//! recorded for the open native change, and a plain note when the check has not passed
//! for this tree. It reads one local file and runs no native command.

use crate::records::{NATIVE_RECORD_LIMIT, local_path, read_native_record, valid_change_id};
use anyhow::Result;
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::Path;

/// Failing commands shown for an attempt; the rest are counted.
const COMMAND_LIMIT: usize = 3;

#[derive(Deserialize)]
struct Attempts {
    attempts: Vec<Attempt>,
}

#[derive(Deserialize)]
struct Attempt {
    timestamp: u64,
    commit: String,
    passed: bool,
    #[serde(default)]
    commands: Vec<Command>,
}

#[derive(Deserialize)]
struct Command {
    command: String,
    success: bool,
    exit_code: Option<i64>,
}

/// The `native_check` field of the next view: `null` with no recorded native change or
/// no workspace under `.specsync/changes/` (an archived change), otherwise the latest
/// recorded attempt, whether the check has not passed for this tree, and why it is
/// unknown when the attempts record cannot be read. `summary` is the `summary` of the
/// native status read `next` already made: its `state`, and for `verifying` its
/// `handoff.readiness`, which SpecSync sets to `safe` only while its last check still
/// matches this tree (#91 review). No other native read is made.
pub(super) fn view(root: &Path, change: Option<&str>, summary: Option<&Value>) -> Result<Value> {
    let Some(change) = change.filter(|change| valid_change_id(change)) else {
        return Ok(Value::Null);
    };
    let workspace = format!(".specsync/changes/{change}");
    if !local_path(root, &workspace)?.is_dir() {
        return Ok(Value::Null);
    }
    let state = summary
        .and_then(|summary| summary.get("state"))
        .and_then(Value::as_str);
    let handoff = summary.and_then(|summary| summary.get("handoff"));
    let readiness = handoff
        .and_then(|handoff| handoff.get("readiness"))
        .and_then(Value::as_str);
    let check = format!("run `specsync change check {change}`");
    let (not_passed, note) = match (state, readiness) {
        (Some("approved" | "implementing"), _) => (
            true,
            Some(format!(
                "The native check has not passed for this tree. SpecSync keeps no reason when it refuses before verifying (for example incomplete tasks or missing requirement evidence); {check} to see it."
            )),
        ),
        (Some("verifying"), Some("safe")) => (false, None),
        (Some("verifying"), Some(_)) => {
            let reason = handoff
                .and_then(|handoff| handoff.get("reason"))
                .and_then(Value::as_str)
                .map_or_else(
                    || "its last check no longer matches this tree".to_string(),
                    crate::cut,
                );
            (
                true,
                Some(format!(
                    "The native check has not passed for this tree: SpecSync says {reason}; {check}."
                )),
            )
        }
        (Some("verifying"), None) => (
            false,
            Some(format!(
                "Whether the last native check matches this tree is unknown: SpecSync's status gave no readiness; {check} to be sure."
            )),
        ),
        _ => (false, None),
    };
    let (last_attempt, unknown) =
        match latest(root, &format!("{workspace}/verification-attempts.json")) {
            Ok(attempt) => (attempt, None),
            Err(reason) => (None, Some(reason)),
        };
    Ok(json!({
        "last_attempt": last_attempt,
        "not_passed_for_tree": not_passed,
        "note": note,
        "unknown": unknown,
    }))
}

/// The attempt with the greatest timestamp (ties go to the later one in the file), or
/// None when SpecSync has recorded none.
fn latest(root: &Path, relative: &str) -> std::result::Result<Option<Value>, String> {
    let path = local_path(root, relative).map_err(|error| format!("{relative}: {error:#}"))?;
    let bytes = match read_native_record(&path, NATIVE_RECORD_LIMIT) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("{relative} cannot be read: {error}")),
        Ok(None) => return Err(format!("{relative} exceeds {NATIVE_RECORD_LIMIT} bytes")),
        Ok(Some(bytes)) => bytes,
    };
    let record: Attempts = serde_json::from_slice(&bytes)
        .map_err(|error| format!("{relative} is not a valid attempts record: {error}"))?;
    let Some(attempt) = record
        .attempts
        .iter()
        .enumerate()
        .max_by_key(|(index, attempt)| (attempt.timestamp, *index))
        .map(|(_, attempt)| attempt)
    else {
        return Ok(None);
    };
    let failed: Vec<&Command> = attempt
        .commands
        .iter()
        .filter(|command| !command.success)
        .collect();
    Ok(Some(json!({
        "passed": attempt.passed,
        "at_unix_seconds": attempt.timestamp,
        "commit": attempt.commit.chars().take(12).collect::<String>(),
        "failed_commands": {
            "total": failed.len(),
            "items": failed
                .iter()
                .take(COMMAND_LIMIT)
                .map(|command| json!({"command": crate::cut(&command.command), "exit_code": command.exit_code}))
                .collect::<Vec<_>>(),
        },
    })))
}

/// The text lines for the next view's `native_check`; none when it is `null`.
pub(super) fn lines(check: &Value) -> Vec<String> {
    if check.is_null() {
        return vec![];
    }
    let mut lines = vec![];
    let last = &check["last_attempt"];
    if let Some(reason) = check["unknown"].as_str() {
        lines.push(format!("Native check: unknown ({reason})"));
    } else if last.is_null() {
        lines.push("Native check: no attempt recorded".into());
    } else {
        let at = last["at_unix_seconds"]
            .as_u64()
            .map_or_else(|| "unknown time".into(), crate::timing::utc);
        let commit: String = last["commit"]
            .as_str()
            .unwrap_or_default()
            .chars()
            .take(7)
            .collect();
        if last["passed"] == true {
            lines.push(format!(
                "Native check: last attempt passed at {at} on {commit}"
            ));
        } else {
            let failed = &last["failed_commands"];
            let mut shown: Vec<String> = failed["items"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|command| {
                    let code = command["exit_code"]
                        .as_i64()
                        .map_or_else(|| "no exit code".into(), |code| format!("exit {code}"));
                    format!(
                        "`{}` {code}",
                        command["command"].as_str().unwrap_or_default()
                    )
                })
                .collect();
            let total = failed["total"].as_u64().unwrap_or(0);
            if total > shown.len() as u64 {
                shown.push(format!("{} more", total - shown.len() as u64));
            }
            if shown.is_empty() {
                shown.push("no failing command recorded".into());
            }
            lines.push(format!(
                "Native check: last attempt failed at {at} on {commit}: {}",
                shown.join("; ")
            ));
        }
    }
    if let Some(note) = check["note"].as_str() {
        lines.push(format!("  {note}"));
    }
    lines
}
