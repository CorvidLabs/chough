//! Read-only reuse of SpecSync's authenticated archived status (REQ-cli-053).

use crate::records::{NativeJson, Section, bounded, native as native_text, native_json, run_owned};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const ARCHIVES: &str = ".specsync/archive/changes";
const INPUTS: [&str; 7] = [
    ".specsync/workflow-v2-baseline.json",
    ".specsync/archive/legacy-baseline.json",
    ".specsync/sdd.json",
    ".specsync/config.toml",
    ".specsync/config.json",
    ".specsync/registry.toml",
    ".specsync/version",
];
const INPUT_LIMIT: u64 = 64 * 1024 * 1024;
const BINARY_LIMIT: u64 = 256 * 1024 * 1024;
const ENTRY_LIMIT: u64 = 8 * 1024 * 1024;
const SCHEMA: u64 = 1;
pub(super) const MARKER: &str = "chough_archive_authentication";

/// The report-only native status path. Lifecycle commands call `native_json` directly.
pub(super) fn report_status(root: &Path, id: &str) -> NativeJson {
    let key = Key::of(root, id);
    if let Some(key) = &key
        && let Some(hit) = key.reuse()
    {
        return hit;
    }
    let live = native_json(root, "specsync", &["change", "status", id, "--json"]);
    let Some(value) = &live.value else {
        return live;
    };
    if !authenticated(value, id) {
        return live;
    }
    if let Some(before) = key
        && let Some(after) = Key::of(root, id)
        && before.digest == after.digest
    {
        after.record(value);
    }
    live
}

fn authenticated(value: &Value, id: &str) -> bool {
    value.pointer("/summary/state").and_then(Value::as_str) == Some("archived")
        && value.pointer("/change/state").and_then(Value::as_str) == Some("archived")
        && value.pointer("/change/id").and_then(Value::as_str) == Some(id)
        && value
            .pointer("/summary/terminal_evidence/validity")
            .and_then(Value::as_str)
            == Some("authenticated-history")
}

struct Key {
    root: PathBuf,
    id: String,
    archive: String,
    entry: PathBuf,
    digest: String,
    tree: String,
    binary: Value,
}

impl Key {
    fn of(root: &Path, id: &str) -> Option<Self> {
        if !plain(id)
            || root
                .join(".specsync/changes")
                .join(id)
                .symlink_metadata()
                .is_ok()
            || !default_history(root)?
        {
            return None;
        }
        let archive = archive_dir(root, id)?;
        let directory = root.join(&archive);
        if !regular(&directory.join("accepted-state.json")) {
            return None;
        }
        let verification: Value = serde_json::from_slice(&bounded_file(
            &directory.join("verification.json"),
            ENTRY_LIMIT,
        )?)
        .ok()?;
        if verification
            .get("acceptance_manifest")
            .is_none_or(Value::is_null)
        {
            return None;
        }
        let common = git_common_dir(root)?;
        let binary = specsync_binary()?;
        let tree = git(
            root,
            &["rev-parse", "--verify", "-q", &format!("HEAD:{archive}")],
        )?;
        let mut hasher = Sha256::new();
        hasher.update(b"chough-archive-auth/1\0");
        hasher.update(binary.to_string().as_bytes());
        hasher.update(b"\0");
        hasher.update(tree.as_bytes());
        hasher.update(b"\0");
        let mut files = Vec::new();
        walk(&directory, &mut files)?;
        for input in INPUTS {
            let path = root.join(input);
            match path.symlink_metadata() {
                Ok(meta) if meta.is_file() && !meta.file_type().is_symlink() => files.push(path),
                Ok(_) => return None,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(_) => return None,
            }
        }
        let mut total = 0_u64;
        for file in &files {
            let relative = file.strip_prefix(root).ok()?.to_str()?.replace('\\', "/");
            let bytes = bounded_file(file, INPUT_LIMIT.checked_sub(total)?)?;
            total += bytes.len() as u64;
            hasher.update(relative.as_bytes());
            hasher.update(b"\0");
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(&bytes);
        }
        let name = hex(&Sha256::digest(id.as_bytes()));
        Some(Self {
            root: root.to_path_buf(),
            id: id.to_string(),
            archive,
            entry: common
                .join("chough/archive-auth")
                .join(format!("{name}.json")),
            digest: format!("sha256:{}", hex(&hasher.finalize())),
            tree,
            binary,
        })
    }

    fn reuse(&self) -> Option<NativeJson> {
        if !regular(&self.entry) {
            return None;
        }
        let entry: Value = serde_json::from_slice(&bounded_file(&self.entry, ENTRY_LIMIT)?).ok()?;
        let commit = entry["commit"].as_str()?;
        if entry["schema_version"] != SCHEMA
            || entry["change_id"] != self.id.as_str()
            || entry["archive"] != self.archive.as_str()
            || entry["digest"] != self.digest.as_str()
            || entry["tree"] != self.tree.as_str()
            || entry["specsync"]["path"] != self.binary["path"]
            || entry["specsync"]["sha256"] != self.binary["sha256"]
            || entry["specsync"]["version"].as_str()?.is_empty()
            || !commit.bytes().all(|byte| byte.is_ascii_hexdigit())
            || !authenticated(&entry["response"], &self.id)
            || !intact(&entry)
        {
            return None;
        }
        let carried = git(
            &self.root,
            &[
                "rev-parse",
                "--verify",
                "-q",
                &format!("{commit}:{}", self.archive),
            ],
        )?;
        if carried != self.tree
            || !git_ok(&self.root, &["merge-base", "--is-ancestor", commit, "HEAD"])
        {
            return None;
        }
        let value = entry["response"].clone();
        let mut content = serde_json::to_string_pretty(&value).ok()?;
        if content.len() > crate::records::LIMIT {
            content = bounded(content.as_bytes());
            content.push_str("\n[display only: the recorded response is longer]");
        }
        Some(NativeJson {
            section: Section {
                source: format!(
                    "specsync change status {} --json (authenticated archive reused for {})",
                    self.id, self.digest
                ),
                state: "cached".into(),
                content,
            },
            value: Some(value),
        })
    }

    fn record(&self, response: &Value) {
        let clean = git(
            &self.root,
            &[
                "status",
                "--porcelain",
                "--untracked-files=all",
                "--",
                &self.archive,
            ],
        );
        let commit = git(
            &self.root,
            &["log", "-1", "--format=%H", "--", &self.archive],
        );
        let version = specsync_version(&self.root, &self.binary);
        let (Some(clean), Some(commit), Some(version)) = (clean, commit, version) else {
            return;
        };
        if !clean.is_empty() || commit.is_empty() {
            return;
        }
        let at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_secs());
        let mut response = response.clone();
        if let Some(object) = response.as_object_mut() {
            object.remove(MARKER);
        }
        let mut binary = self.binary.clone();
        binary["version"] = Value::from(version);
        let mut entry = json!({
            "schema_version": SCHEMA,
            "change_id": self.id,
            "archive": self.archive,
            "digest": self.digest,
            "tree": self.tree,
            "commit": commit,
            "specsync": binary,
            "authenticated_at_unix_seconds": at,
            "response": response,
        });
        let integrity = envelope_digest(&entry);
        entry["integrity"] = Value::from(integrity);
        let content = entry.to_string();
        if content.len() as u64 > ENTRY_LIMIT {
            return;
        }
        let Some(parent) = self.entry.parent() else {
            return;
        };
        if fs::create_dir_all(parent).is_err()
            || !parent
                .symlink_metadata()
                .is_ok_and(|meta| meta.is_dir() && !meta.file_type().is_symlink())
        {
            return;
        }
        let temporary = parent.join(format!(".{}.{}.tmp", std::process::id(), at));
        let written = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .and_then(|mut file| {
                file.write_all(content.as_bytes())?;
                file.sync_all()
            });
        if written.is_err() || fs::rename(&temporary, &self.entry).is_err() {
            let _ = fs::remove_file(&temporary);
        }
    }
}

fn intact(entry: &Value) -> bool {
    entry["integrity"].as_str() == Some(envelope_digest(entry).as_str())
}

fn envelope_digest(entry: &Value) -> String {
    let mut value = entry.clone();
    if let Some(object) = value.as_object_mut() {
        object.remove("integrity");
    }
    format!(
        "sha256:{}",
        hex(&Sha256::digest(value.to_string().as_bytes()))
    )
}

fn plain(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 255
        && !id.starts_with('-')
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

fn archive_dir(root: &Path, id: &str) -> Option<String> {
    let mut found = None;
    for entry in fs::read_dir(root.join(ARCHIVES)).ok()? {
        let entry = entry.ok()?;
        let name = entry.file_name().into_string().ok()?;
        if archived_id(&name) != id {
            continue;
        }
        if !entry.file_type().ok()?.is_dir() || found.is_some() {
            return None;
        }
        found = Some(format!("{ARCHIVES}/{name}"));
    }
    found
}

fn archived_id(directory: &str) -> &str {
    let bytes = directory.as_bytes();
    let dated = bytes.len() > 11
        && bytes[..10].iter().enumerate().all(|(index, byte)| {
            if matches!(index, 4 | 7) {
                *byte == b'-'
            } else {
                byte.is_ascii_digit()
            }
        })
        && bytes[10] == b'-';
    if dated { &directory[11..] } else { directory }
}

fn regular(path: &Path) -> bool {
    path.symlink_metadata()
        .is_ok_and(|meta| meta.is_file() && !meta.file_type().is_symlink())
}

fn walk(directory: &Path, files: &mut Vec<PathBuf>) -> Option<()> {
    let mut entries = fs::read_dir(directory)
        .ok()?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    entries.sort();
    for path in entries {
        let meta = path.symlink_metadata().ok()?;
        if meta.is_dir() && !meta.file_type().is_symlink() {
            walk(&path, files)?;
        } else if meta.is_file() && !meta.file_type().is_symlink() {
            files.push(path);
        } else {
            return None;
        }
    }
    Some(())
}

fn bounded_file(path: &Path, limit: u64) -> Option<Vec<u8>> {
    if !regular(path) {
        return None;
    }
    let mut bytes = Vec::new();
    fs::File::open(path)
        .ok()?
        .take(limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .ok()?;
    (bytes.len() as u64 <= limit).then_some(bytes)
}

fn specsync_binary() -> Option<Value> {
    for directory in std::env::split_paths(&std::env::var_os("PATH")?) {
        let candidate = directory.join("specsync");
        if let Ok(meta) = candidate.symlink_metadata()
            && meta.is_file()
        {
            let path = candidate.canonicalize().ok()?;
            let mut file = fs::File::open(&path).ok()?;
            let mut hasher = Sha256::new();
            let mut buffer = [0_u8; 65_536];
            let mut total = 0_u64;
            loop {
                let size = file.read(&mut buffer).ok()?;
                total = total.checked_add(size as u64)?;
                if total > BINARY_LIMIT {
                    return None;
                }
                if size == 0 {
                    break;
                }
                hasher.update(&buffer[..size]);
            }
            return Some(json!({
                "path": path.to_str()?,
                "sha256": format!("sha256:{}", hex(&hasher.finalize())),
            }));
        }
    }
    None
}

fn specsync_version(root: &Path, binary: &Value) -> Option<String> {
    let path = binary["path"].as_str()?;
    let response = native_text(root, path, &["--version"]);
    if response.state != "observed" || response.content.len() > 4096 {
        return None;
    }
    let version = response.content.trim().to_string();
    (!version.is_empty()).then_some(version)
}

fn default_history(root: &Path) -> Option<bool> {
    for name in ["shallow", "info/grafts", "objects/info/alternates"] {
        let path = git(root, &["rev-parse", "--git-path", name])?;
        if root.join(path).symlink_metadata().is_ok() {
            return Some(false);
        }
    }
    Some(
        git(
            root,
            &["for-each-ref", "--format=%(refname)", "refs/replace/"],
        )?
        .is_empty(),
    )
}

fn git_common_dir(root: &Path) -> Option<PathBuf> {
    let dot = root.join(".git");
    let meta = dot.symlink_metadata().ok()?;
    let git_dir = if meta.is_dir() {
        dot
    } else if meta.is_file() {
        let text = String::from_utf8(bounded_file(&dot, 4096)?).ok()?;
        let target = text.strip_prefix("gitdir:")?.trim();
        let target = Path::new(target);
        if target.is_absolute() {
            target.to_path_buf()
        } else {
            root.join(target)
        }
    } else {
        return None;
    };
    let common = match bounded_file(&git_dir.join("commondir"), 4096) {
        Some(bytes) => {
            let text = String::from_utf8(bytes).ok()?;
            let path = Path::new(text.trim());
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                git_dir.join(path)
            }
        }
        None => git_dir,
    };
    common.is_dir().then_some(common)
}

fn git(root: &Path, arguments: &[&str]) -> Option<String> {
    let output = run_owned(
        Command::new("git").arg("-C").arg(root).args(arguments),
        None,
    )
    .ok()?;
    if !output.status.success() || output.stdout.len() > crate::records::PARSE_LIMIT {
        return None;
    }
    Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
}

fn git_ok(root: &Path, arguments: &[&str]) -> bool {
    git(root, arguments).is_some()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
