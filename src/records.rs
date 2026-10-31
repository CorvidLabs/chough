//! Bounded, project-confined reading and writing shared by every command.

use anyhow::{Context, Result, bail, ensure};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) const LIMIT: usize = 16_384;
pub(super) const RECORD_LIMIT: u64 = 65_536;
const LOG_TAIL_LIMIT: u64 = 65_536;

#[derive(Serialize)]
pub(super) struct Section {
    pub(super) source: String,
    pub(super) state: String,
    pub(super) content: String,
}

pub(super) fn print_json(value: &Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

pub(super) fn now() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

pub(super) fn valid_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 100
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

/// Reject blank text and text that would make a record unreadable.
pub(super) fn ensure_text(label: &str, value: &str) -> Result<()> {
    ensure!(!value.trim().is_empty(), "{label} must not be blank");
    ensure!(
        value.len() <= 4_096,
        "{label} exceeds 4096 bytes; link a file instead"
    );
    Ok(())
}

// Reject even in-project symlinks so later retargeting cannot redirect a write.
pub(super) fn local_path(root: &Path, relative: &str) -> Result<PathBuf> {
    let relative = Path::new(relative);
    ensure!(!relative.as_os_str().is_empty(), "empty source path");
    let mut path = root.to_path_buf();
    for component in relative.components() {
        match component {
            Component::Normal(part) => path.push(part),
            _ => bail!(
                "path must be relative without traversal: {}",
                relative.display()
            ),
        }
        if let Ok(metadata) = fs::symlink_metadata(&path) {
            ensure!(
                !metadata.file_type().is_symlink(),
                "symlink is not an allowed source: {}",
                path.display()
            );
        }
    }
    Ok(path)
}

/// Create a file only when it does not exist; returns false when it was preserved.
pub(super) fn write_new(root: &Path, relative: &str, content: &str) -> Result<bool> {
    ensure!(
        content.len() as u64 <= RECORD_LIMIT,
        "managed file exceeds 65536 bytes: {relative}"
    );
    write_new_bytes(root, relative, content.as_bytes())
}

/// Create a file of any bounded size only when it does not exist.
pub(super) fn write_new_bytes(root: &Path, relative: &str, content: &[u8]) -> Result<bool> {
    let path = local_path(root, relative)?;
    if path.exists() {
        ensure!(path.is_file(), "expected a file: {}", path.display());
        return Ok(false);
    }
    fs::create_dir_all(path.parent().context("file has no parent")?)?;
    local_path(root, relative)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    file.write_all(content)?;
    Ok(true)
}

/// Replace a managed record atomically: write a sibling temporary file, then rename.
pub(super) fn write_replace(root: &Path, relative: &str, content: &str) -> Result<()> {
    ensure!(
        content.len() as u64 <= RECORD_LIMIT,
        "managed file exceeds 65536 bytes: {relative}"
    );
    let path = local_path(root, relative)?;
    if path.exists() {
        ensure!(path.is_file(), "expected a file: {}", path.display());
    }
    let parent = path.parent().context("file has no parent")?;
    fs::create_dir_all(parent)?;
    local_path(root, relative)?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("record name must be UTF-8")?;
    let temporary = parent.join(format!(".{name}.{}.tmp", std::process::id()));
    let result = (|| -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
        fs::rename(&temporary, &path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result.with_context(|| format!("writing {relative}"))
}

/// Append one line to an append-only managed log.
pub(super) fn append_line(root: &Path, relative: &str, line: &str) -> Result<()> {
    ensure!(
        !line.contains('\n') && line.len() as u64 <= RECORD_LIMIT,
        "log entries are single bounded lines"
    );
    let path = local_path(root, relative)?;
    fs::create_dir_all(path.parent().context("file has no parent")?)?;
    local_path(root, relative)?;
    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}

/// Read the complete lines at the end of a log, bounded to 64 KiB.
pub(super) fn read_log_tail(path: &Path, lines: usize) -> Result<(Vec<String>, bool)> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok((vec![], false));
        }
        Err(error) => return Err(error.into()),
    };
    let length = file.metadata()?.len();
    let truncated = length > LOG_TAIL_LIMIT;
    if truncated {
        file.seek(SeekFrom::Start(length - LOG_TAIL_LIMIT))?;
    }
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;
    let text = String::from_utf8_lossy(&bytes);
    let mut all: Vec<&str> = text.lines().filter(|line| !line.is_empty()).collect();
    if truncated && !all.is_empty() {
        all.remove(0);
    }
    let start = all.len().saturating_sub(lines);
    Ok((
        all[start..]
            .iter()
            .map(|line| (*line).to_string())
            .collect(),
        truncated || start > 0,
    ))
}

pub(super) fn read_record(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut content = String::new();
    file.take(RECORD_LIMIT + 1).read_to_string(&mut content)?;
    ensure!(
        content.len() as u64 <= RECORD_LIMIT,
        "structured record exceeds 65536 bytes: {}",
        path.display()
    );
    Ok(content)
}

pub(super) fn bounded(value: &[u8]) -> String {
    let mut result = String::from_utf8_lossy(&value[..value.len().min(LIMIT)]).into_owned();
    if value.len() > LIMIT {
        result.push_str("\n[truncated at 16384 bytes; read the source when relevant]");
    }
    result
}

pub(super) fn digest_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(super) fn file_digest(root: &Path, relative: &str) -> Result<String> {
    let path = local_path(root, relative)?;
    let mut file = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = [0_u8; 8192];
    loop {
        let size = file.read(&mut buffer)?;
        if size == 0 {
            break;
        }
        hash.update(&buffer[..size]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

pub(super) fn is_digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub(super) fn file_section(root: &Path, name: &str) -> Result<Section> {
    let path = local_path(root, name)?;
    Ok(match File::open(&path) {
        Ok(file) => {
            let mut bytes = vec![];
            file.take((LIMIT + 1) as u64).read_to_end(&mut bytes)?;
            Section {
                source: name.into(),
                state: "present".into(),
                content: bounded(&bytes),
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Section {
            source: name.into(),
            state: "missing".into(),
            content: "Source is missing; do not infer its contents.".into(),
        },
        Err(error) => return Err(error).with_context(|| format!("reading {}", path.display())),
    })
}

pub(super) fn native(root: &Path, program: &str, arguments: &[&str]) -> Section {
    match Command::new(program)
        .args(arguments)
        .current_dir(root)
        .output()
    {
        Ok(output) => Section {
            source: format!("{} {}", program, arguments.join(" ")),
            state: if output.status.success() {
                "observed"
            } else {
                "failed"
            }
            .into(),
            content: if output.status.success() {
                bounded(&output.stdout)
            } else {
                format!(
                    "exit={:?}\n{}\n{}",
                    output.status.code(),
                    bounded(&output.stdout),
                    bounded(&output.stderr)
                )
            },
        },
        Err(error) => Section {
            source: format!("{} {}", program, arguments.join(" ")),
            state: "unavailable".into(),
            content: format!("Unknown: {error}"),
        },
    }
}

/// Largest native JSON response parsed in full (SpecSync's own artifact limit).
pub(super) const PARSE_LIMIT: usize = 4 * 1_048_576;

/// A native command's bounded display section and, when it succeeded with a JSON
/// object or array within the parse bound, the complete decoded value.
pub(super) struct NativeJson {
    pub(super) section: Section,
    pub(super) value: Option<Value>,
}

/// Run a native command whose JSON output decides state. The value is parsed from the
/// complete stdout (up to 4 MiB); the section shown to people stays within 16 KiB.
pub(super) fn native_json(root: &Path, program: &str, arguments: &[&str]) -> NativeJson {
    let source = format!("{} {}", program, arguments.join(" "));
    let output = match Command::new(program)
        .args(arguments)
        .current_dir(root)
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            return NativeJson {
                section: Section {
                    source,
                    state: "unavailable".into(),
                    content: format!("Unknown: {error}"),
                },
                value: None,
            };
        }
    };
    if !output.status.success() {
        return NativeJson {
            section: Section {
                source,
                state: "failed".into(),
                content: format!(
                    "exit={:?}\n{}\n{}",
                    output.status.code(),
                    bounded(&output.stdout),
                    bounded(&output.stderr)
                ),
            },
            value: None,
        };
    }
    let length = output.stdout.len();
    if length > PARSE_LIMIT {
        return NativeJson {
            section: Section {
                source,
                state: "oversized".into(),
                content: format!(
                    "Lifecycle state is unknown: output of {length} bytes exceeds the {PARSE_LIMIT}-byte parse bound.\nBounded raw output:\n{}",
                    bounded(&output.stdout)
                ),
            },
            value: None,
        };
    }
    match serde_json::from_slice::<Value>(&output.stdout) {
        Ok(value) if value.is_object() || value.is_array() => {
            let mut content = bounded(&output.stdout);
            if length > LIMIT {
                content.push_str(&format!(
                    "\n[display only: the complete {length} bytes were parsed]"
                ));
            }
            NativeJson {
                section: Section {
                    source,
                    state: "observed".into(),
                    content,
                },
                value: Some(value),
            }
        }
        _ => NativeJson {
            section: Section {
                source,
                state: "invalid-output".into(),
                content: format!(
                    "Lifecycle state is unknown: successful output is not a complete JSON object or array.\nBounded raw output:\n{}",
                    bounded(&output.stdout)
                ),
            },
            value: None,
        },
    }
}

/// The current Git HEAD when one exists; recorded with transitions as context only.
pub(super) fn head_revision(root: &Path) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// List JSON record names in a managed directory in a deterministic order.
pub(super) fn record_names(root: &Path, directory: &str) -> Result<Vec<String>> {
    let path = local_path(root, directory)?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let mut names = vec![];
    for entry in fs::read_dir(path)? {
        let name = entry?
            .file_name()
            .to_str()
            .context("invalid record filename")?
            .to_string();
        ensure!(
            name.ends_with(".json"),
            "unexpected file in {directory}: {name}"
        );
        names.push(name);
    }
    names.sort();
    Ok(names)
}
