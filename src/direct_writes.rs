//! Direct lifecycle writes noted (REQ-cli-060, REQ-history-013, DRIVE-21): the
//! lifecycle records SpecSync writes for a native change are compared with an
//! untracked baseline that Chough's own change steps keep current. A difference that
//! Chough did not make is a direct write. It stays pending in
//! `.chough/local/native-writes/<change>.json` until the next successful stage move
//! records it as a `direct-native-write` history event. `next` and `status` only show
//! it, and nothing is recorded between `change check --commit` and finalize.

use crate::records::local_path;
use crate::stage::{DirectWrite, WrittenFile};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

/// The lifecycle records SpecSync writes in `.specsync/changes/<id>/`.
pub(super) const WATCHED: [&str; 6] = [
    "state.json",
    "approvals.json",
    "review.json",
    "review-attempts.json",
    "verification.json",
    "verification-attempts.json",
];

/// The records as they are: each present file's SHA-256, and the change's state.
#[derive(Clone, Default, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct Fingerprint {
    files: BTreeMap<String, String>,
    state: Option<String>,
}

/// The untracked baseline and the findings not yet recorded.
#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Local {
    schema_version: u32,
    baseline: Fingerprint,
    pending: Vec<DirectWrite>,
}

/// Set once a stage move in this process has recorded the findings, so the steps of
/// a batched advance record them once.
static RECORDED: AtomicBool = AtomicBool::new(false);

/// Whether a change ID can name a file: a nonempty lowercase slug.
fn safe(change: &str) -> bool {
    !change.is_empty()
        && change.len() <= 200
        && change
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn local_relative(change: &str) -> String {
    format!(".chough/local/native-writes/{change}.json")
}

fn fingerprint(root: &Path, change: &str) -> Fingerprint {
    let dir = root.join(".specsync/changes").join(change);
    let mut files = BTreeMap::new();
    let mut state = None;
    for name in WATCHED {
        let Ok(bytes) = fs::read(dir.join(name)) else {
            continue;
        };
        if name == "state.json" {
            state = serde_json::from_slice::<Value>(&bytes)
                .ok()
                .and_then(|value| value["state"].as_str().map(String::from));
        }
        files.insert(name.to_string(), format!("{:x}", Sha256::digest(&bytes)));
    }
    Fingerprint { files, state }
}

/// The difference from `baseline` to `current`, or None when there is none.
fn difference(change: &str, baseline: &Fingerprint, current: &Fingerprint) -> Option<DirectWrite> {
    let mut files = vec![];
    for name in WATCHED {
        let change = match (baseline.files.get(name), current.files.get(name)) {
            (None, Some(_)) => "added",
            (Some(_), None) => "removed",
            (Some(before), Some(after)) if before != after => "changed",
            _ => continue,
        };
        files.push(WrittenFile {
            path: name.into(),
            change: change.into(),
        });
    }
    (!files.is_empty()).then(|| DirectWrite {
        change: change.into(),
        files,
        before_state: baseline.state.clone(),
        after_state: current.state.clone(),
    })
}

fn read_local(root: &Path, change: &str) -> Result<Option<Local>> {
    let path = local_path(root, &local_relative(change))?;
    if !path.exists() {
        return Ok(None);
    }
    let bytes = fs::read(&path)?;
    let local: Local = serde_json::from_slice(&bytes)
        .with_context(|| format!("{} is not a direct-write baseline", local_relative(change)))?;
    Ok(Some(local))
}

/// Write the local file whole: a temporary file renamed into place. It is written
/// only once `.chough/local/` is ignored (the checkpoint writer creates its
/// `.gitignore`), so the baseline is never a tracked file and nothing else is written;
/// until then there is no baseline, and the first one is first sight.
fn write_local(root: &Path, change: &str, local: &Local) -> Result<()> {
    if !local_path(root, ".chough/local/.gitignore")?.exists() {
        return Ok(());
    }
    let path = local_path(root, &local_relative(change))?;
    let dir = path.parent().context("the baseline has no directory")?;
    fs::create_dir_all(dir)?;
    let temporary = dir.join(format!(".{change}.json.{}.tmp", std::process::id()));
    fs::write(&temporary, serde_json::to_vec_pretty(local)?)?;
    fs::rename(&temporary, &path)?;
    Ok(())
}

/// What a `chough change` step saw before its SpecSync step: the records then, and
/// any difference from the baseline, which no step of Chough's made.
pub(super) struct Before {
    change: String,
    finding: Option<DirectWrite>,
    local: Option<Local>,
}

/// Before a `chough change` step for `change`. Nothing is written.
pub(super) fn before_step(root: &Path, change: &str) -> Option<Before> {
    // A change with no workspace has no lifecycle files to baseline, so a step SpecSync
    // refuses for an unknown ID leaves nothing under `.chough/` (REQ-cli-057).
    if !safe(change)
        || !local_path(root, &format!(".specsync/changes/{change}")).is_ok_and(|path| path.is_dir())
    {
        return None;
    }
    let local = read_local(root, change).ok().flatten();
    let finding = local
        .as_ref()
        .and_then(|local| difference(change, &local.baseline, &fingerprint(root, change)));
    Some(Before {
        change: change.into(),
        finding,
        local,
    })
}

/// After a `chough change` step's SpecSync step, whatever its exit: keep the earlier
/// difference pending, and set the baseline to the records as they are now. With no
/// baseline before, this is first sight and nothing is pending.
pub(super) fn after_step(root: &Path, before: Before) -> Result<()> {
    let mut local = before.local.unwrap_or(Local {
        schema_version: 1,
        ..Local::default()
    });
    local.pending.extend(before.finding);
    local.baseline = fingerprint(root, &before.change);
    write_local(root, &before.change, &local)
}

/// After `chough change new` created `change`: its baseline is the records it wrote.
pub(super) fn created(root: &Path, change: &str) -> Result<()> {
    if !safe(change) {
        return Ok(());
    }
    let local = Local {
        schema_version: 1,
        baseline: fingerprint(root, change),
        pending: vec![],
    };
    write_local(root, change, &local)
}

/// The findings a stage move in `change` records: the pending ones and the current
/// difference. None without a baseline (first sight), or once this process has
/// recorded them.
pub(super) fn to_record(root: &Path, change: Option<&str>) -> Vec<DirectWrite> {
    let Some(change) = change.filter(|change| safe(change)) else {
        return vec![];
    };
    if RECORDED.load(Ordering::SeqCst) {
        return vec![];
    }
    let Ok(Some(local)) = read_local(root, change) else {
        return vec![];
    };
    let mut findings = local.pending;
    findings.extend(difference(
        change,
        &local.baseline,
        &fingerprint(root, change),
    ));
    RECORDED.store(true, Ordering::SeqCst);
    findings
}

/// After a successful stage move is saved: nothing is pending, and the baseline is the
/// records as they are. Without a baseline, this sets the first one.
pub(super) fn settle(root: &Path, change: Option<&str>) -> Result<()> {
    let Some(change) = change.filter(|change| safe(change)) else {
        return Ok(());
    };
    let local = Local {
        schema_version: 1,
        baseline: fingerprint(root, change),
        pending: vec![],
    };
    write_local(root, change, &local)
}

/// What `next` and `status` show: the findings a stage move would record now, and
/// whether a baseline exists. Nothing is written.
pub(super) fn view(root: &Path, change: Option<&str>) -> Value {
    let Some(change) = change.filter(|change| safe(change)) else {
        return Value::Null;
    };
    match read_local(root, change) {
        Ok(None) => json!({"baseline": false, "pending": []}),
        Ok(Some(local)) => {
            let mut pending = local.pending;
            pending.extend(difference(
                change,
                &local.baseline,
                &fingerprint(root, change),
            ));
            json!({"baseline": true, "pending": pending})
        }
        Err(error) => json!({"unknown": error.to_string()}),
    }
}

/// The `next` line for pending findings, or None.
pub(super) fn next_line(view: &Value) -> Option<String> {
    let pending = view["pending"].as_array()?;
    if pending.is_empty() {
        return None;
    }
    let files: Vec<String> = pending
        .iter()
        .flat_map(|finding| finding["files"].as_array().cloned().unwrap_or_default())
        .map(|file| {
            format!(
                "{} {}",
                file["path"].as_str().unwrap_or_default(),
                file["change"].as_str().unwrap_or_default()
            )
        })
        .collect();
    Some(format!(
        "Direct lifecycle writes: {} not yet recorded ({}); the next stage move records them",
        pending.len(),
        files.join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_difference_names_each_added_changed_and_removed_record() {
        let mut before = Fingerprint::default();
        before.files.insert("state.json".into(), "a".into());
        before.files.insert("review.json".into(), "b".into());
        before.state = Some("approved".into());
        let mut after = Fingerprint::default();
        after.files.insert("state.json".into(), "c".into());
        after.files.insert("approvals.json".into(), "d".into());
        after.state = Some("verifying".into());
        let found = difference("d1", &before, &after).unwrap();
        let names: Vec<(String, String)> = found
            .files
            .iter()
            .map(|file| (file.path.clone(), file.change.clone()))
            .collect();
        assert_eq!(
            names,
            [
                ("state.json".into(), "changed".into()),
                ("approvals.json".into(), "added".into()),
                ("review.json".into(), "removed".into()),
            ]
        );
        assert_eq!(found.before_state.as_deref(), Some("approved"));
        assert_eq!(found.after_state.as_deref(), Some("verifying"));
        assert!(difference("d1", &after, &after).is_none());
    }

    #[test]
    fn only_slug_change_ids_name_a_baseline() {
        assert!(safe("flag-direct-writes-183"));
        for unsafe_id in ["", "../x", "A", "a/b", "a.b"] {
            assert!(!safe(unsafe_id), "{unsafe_id}");
        }
    }
}
