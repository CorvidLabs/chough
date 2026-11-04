//! SpecSync's SDD policy, `.specsync/sdd.json`, read only to apply SpecSync's own rules
//! to stage gates (REQ-cli-008, REQ-cli-031) and to say when native verification runs
//! none of the project's commands (REQ-cli-038, FLOW-15).

use crate::records::{local_path, read_record};
use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

pub(super) const POLICY: &str = ".specsync/sdd.json";

/// What `next`, `status` and the `stage` report say when native verification runs none
/// of the project's commands (REQ-cli-038).
const NO_COMMANDS: &str = "Native verification runs none of the project's commands: .specsync/sdd.json lists no verification_commands, so a passing `specsync change check` does not show that the project's tests ran. List them in verification_commands, and record test evidence with `chough cases run` or `chough evidence add`.";

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

/// The warning to show when a native configuration exists and its enabled policy lists
/// no verification commands; read from the local file alone, with no native read.
pub(super) fn verification_warning(root: &Path) -> Result<Option<&'static str>> {
    if !crate::stage::native_configured(root)? {
        return Ok(None);
    }
    Ok(match read(root)? {
        Some(Ok(policy)) if policy.enabled && policy.verification_commands.is_empty() => {
            Some(NO_COMMANDS)
        }
        _ => None,
    })
}
