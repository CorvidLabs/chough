//! SpecSync's SDD policy, `.specsync/sdd.json`, read only to apply SpecSync's own rules
//! to stage gates (REQ-cli-008, REQ-cli-031), and the warning when native verification
//! runs none of the project's commands (REQ-cli-038, FLOW-15).

use crate::records::{local_path, read_record};
use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;

pub(super) const POLICY: &str = ".specsync/sdd.json";

/// SpecSync 6.1's `SddPolicy` shape, deserialized only to apply SpecSync's own rules:
/// missing fields take these defaults (SDD enabled), unknown fields are accepted, and a
/// wrong type or a duplicate key makes the policy invalid.
#[derive(Deserialize)]
#[serde(default)]
#[allow(dead_code)]
struct PolicyFile {
    version: u32,
    enabled: bool,
    require_change_for_meaningful_files: bool,
    meaningful_paths: Vec<String>,
    ignored_paths: Vec<String>,
    verification_commands: Vec<String>,
    custom_artifacts: std::collections::BTreeMap<String, String>,
    principles_file: Option<String>,
}

impl Default for PolicyFile {
    fn default() -> Self {
        Self {
            version: 2,
            enabled: true,
            require_change_for_meaningful_files: true,
            meaningful_paths: vec![],
            ignored_paths: vec![],
            verification_commands: vec![],
            custom_artifacts: Default::default(),
            principles_file: None,
        }
    }
}

pub(super) enum Policy {
    /// No policy file: SDD is not adopted.
    Absent,
    Enabled(bool),
    Invalid(String),
}

/// The policy file, None when absent, or why it is invalid.
fn read(root: &Path) -> Result<Option<std::result::Result<PolicyFile, String>>> {
    let path = local_path(root, POLICY)?;
    if !path.exists() {
        return Ok(None);
    }
    let text = match read_record(&path) {
        Ok(text) => text,
        Err(error) => return Ok(Some(Err(format!("{POLICY} cannot be read: {error:#}")))),
    };
    Ok(Some(serde_json::from_str::<PolicyFile>(&text).map_err(
        |error| format!("{POLICY} is not a valid SpecSync policy: {error}"),
    )))
}

pub(super) fn sdd_policy(root: &Path) -> Result<Policy> {
    Ok(match read(root)? {
        None => Policy::Absent,
        Some(Ok(policy)) => Policy::Enabled(policy.enabled),
        Some(Err(reason)) => Policy::Invalid(reason),
    })
}

/// The warning for `next` and `status` (REQ-cli-038): `effective` is the change's
/// `summary.verification_commands` from the native status read they already made. When
/// every effective command is SpecSync's own, native verification runs none of the
/// project's commands; configured commands it leaves out are counted, because SpecSync
/// 6.1.2 does not run them (spec-sync#848). Nothing is said when the list is absent,
/// holds a project command, or SDD is not enabled.
pub(super) fn verification_warning(
    root: &Path,
    effective: Option<&Value>,
) -> Result<Option<String>> {
    let Some(effective) = effective.and_then(Value::as_array) else {
        return Ok(None);
    };
    let effective: Vec<&str> = effective.iter().filter_map(Value::as_str).collect();
    if !crate::stage::native_configured(root)? {
        return Ok(None);
    }
    let Some(Ok(policy)) = read(root)? else {
        return Ok(None);
    };
    if !policy.enabled || effective.iter().any(|command| !is_specsync(command)) {
        return Ok(None);
    }
    let ran = if effective.is_empty() {
        "no command at all".to_string()
    } else {
        format!(
            "only {}",
            effective
                .iter()
                .map(|command| format!("`{command}`"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let mut warning = format!(
        "Native verification runs none of the project's commands: SpecSync's change check runs {ran}, so a passing `specsync change check` does not show that the project's tests ran. Record test evidence with `chough cases run` or `chough evidence add`."
    );
    let skipped = policy
        .verification_commands
        .iter()
        .filter(|command| !effective.contains(&command.as_str()))
        .count();
    if skipped > 0 {
        let noun = if skipped == 1 { "command" } else { "commands" };
        warning.push_str(&format!(
            " {POLICY} lists {skipped} configured {noun} that it does not run."
        ));
    }
    Ok(Some(warning))
}

/// SpecSync's own command: `specsync` followed by its arguments.
fn is_specsync(command: &str) -> bool {
    command == "specsync" || command.starts_with("specsync ")
}
