//! Machine-local state that is never committed (REQ-records-002, REQ-records-005):
//! the raw logs of executed cases at `.chough/local/logs/<evidence id>.log` and the
//! external-artifact map `.chough/local/external-artifacts.toml`, which alone holds
//! each external artifact's real path. Everything here is Git-ignored.

use crate::records::local_path;
use anyhow::{Context, Result, bail, ensure};
use std::path::Path;

pub(crate) const LOGS: &str = ".chough/local/logs";
pub(crate) const EXTERNAL: &str = ".chough/local/external-artifacts.toml";

/// The local raw log of an executed case's evidence entry.
pub(crate) fn log_path(id: &str) -> String {
    format!("{LOGS}/{id}.log")
}

/// Write a captured log by create-new; returns the project-relative path.
pub(crate) fn write_log(root: &Path, id: &str, bytes: &[u8]) -> Result<String> {
    let relative = log_path(id);
    let path = local_path(root, &relative)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("creating {LOGS}"))?;
    }
    crate::records::write_new_bytes(root, &relative, bytes)
        .with_context(|| format!("writing the local log for {id}"))?;
    Ok(relative)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct External {
    pub(crate) path: String,
    pub(crate) fingerprint: String,
}

/// Record an external artifact's path and fingerprint under a fresh neutral ID,
/// `ext-<6 lowercase hex>` from one random draw (REQ-records-005); a clash is refused
/// without another draw (REQ-cli-023). Written by create-new. Returns the ID.
pub(crate) fn record_external(root: &Path, path: &str, fingerprint: &str) -> Result<String> {
    let id = crate::identity::suffixed("ext")?;
    let file = local_path(root, EXTERNAL)?;
    let existing = std::fs::read_to_string(&file).unwrap_or_default();
    ensure!(
        !existing.contains(&format!("[{id}]")),
        "refused: {id} is already recorded; nothing was written and no other name was drawn"
    );
    let mut content = existing;
    content.push_str(&format!(
        "[{id}]\npath = \"{path}\"\nfingerprint = \"{fingerprint}\"\n\n"
    ));
    crate::records::write_replace(root, EXTERNAL, &content)
        .with_context(|| format!("writing {EXTERNAL}"))?;
    Ok(id)
}

/// The path and fingerprint a neutral ID holds, when this machine recorded it.
pub(crate) fn external(root: &Path, id: &str) -> Result<Option<External>> {
    let file = local_path(root, EXTERNAL)?;
    let Ok(content) = std::fs::read_to_string(&file) else {
        return Ok(None);
    };
    let header = format!("[{id}]");
    let mut lines = content.lines();
    while let Some(line) = lines.next() {
        if line.trim() == header {
            let mut path = None;
            let mut fingerprint = None;
            for line in lines.by_ref() {
                if line.starts_with('[') {
                    break;
                }
                if let Some(value) = line.strip_prefix("path = ") {
                    path = Some(unquote(value)?);
                } else if let Some(value) = line.strip_prefix("fingerprint = ") {
                    fingerprint = Some(unquote(value)?);
                }
            }
            return Ok(match (path, fingerprint) {
                (Some(path), Some(fingerprint)) => Some(External { path, fingerprint }),
                _ => bail!("{EXTERNAL}: entry {id} lacks a path or fingerprint"),
            });
        }
    }
    Ok(None)
}

fn unquote(value: &str) -> Result<String> {
    let inner = value
        .trim()
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .with_context(|| format!("{EXTERNAL}: a value is not a quoted string"))?;
    Ok(inner.replace("\\\"", "\"").replace("\\\\", "\\"))
}

/// The verify lane's toolchain verdict, recorded at `.chough/local/logs/toolchain.log`
/// by the `toolchain` Fledge task (REQ-cli-019, BUG-0022): its first line is `matches`
/// or `differs: ...`. Returns the limitation an evidence entry then carries.
pub(crate) fn toolchain_limitation(root: &Path) -> Option<String> {
    let path = local_path(root, &format!("{LOGS}/toolchain.log")).ok()?;
    let content = std::fs::read_to_string(path).ok()?;
    let first = content.lines().next()?;
    first.starts_with("differs").then(|| {
        format!(
            "recorded on a toolchain that differs from CI: {}",
            &content[..content.len().min(200)]
        )
    })
}
