//! Authenticated archive reuse (REQ-cli-035). SpecSync stays the authority: Chough only
//! replays SpecSync's own `authenticated-history` verdict for an archive whose committed
//! inputs are byte-identical to the ones SpecSync authenticated, and reads live otherwise.

use crate::records::{NativeJson, Section, native_json_within};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const ARCHIVES: &str = ".specsync/archive/changes";
const BASELINE: &str = ".specsync/archive/legacy-baseline.json";
const INPUTS: [&str; 3] = [
    ".specsync/sdd.json",
    ".specsync/config.toml",
    ".specsync/config.json",
];
/// The most archive input bytes hashed before reading live instead.
const INPUT_LIMIT: u64 = 64 * 1024 * 1024;
/// The largest recorded entry read back: a 4 MiB native response plus its envelope.
const ENTRY_LIMIT: u64 = 8 * 1024 * 1024;
/// Shown bytes of a reused response, like a live read's section.
const SHOWN: usize = 16 * 1024;
const SCHEMA: u64 = 1;
const PROOF: &str = "specsync change status reported change.state and summary.state archived with summary.terminal_evidence.validity authenticated-history";

/// `specsync change status <id> --json`, or SpecSync's recorded verdict for the same
/// committed archive inputs. Every gate and report read of one change goes through here;
/// retention keeps its own live reads.
pub(super) fn change_status(root: &Path, id: &str, cap: Option<Duration>) -> NativeJson {
    let key = Key::of(root, id);
    if let Some(key) = &key
        && let Some(hit) = key.reuse()
    {
        return hit;
    }
    let live = native_json_within(root, "specsync", &["change", "status", id, "--json"], cap);
    if let (Some(key), Some(value)) = (&key, &live.value)
        && authenticated(value, id)
    {
        key.record(value);
    }
    live
}

/// Whether a response proves what the cache may replay.
fn authenticated(value: &Value, id: &str) -> bool {
    value.pointer("/summary/state").and_then(Value::as_str) == Some("archived")
        && value.pointer("/change/state").and_then(Value::as_str) == Some("archived")
        && value
            .pointer("/summary/terminal_evidence/validity")
            .and_then(Value::as_str)
            == Some("authenticated-history")
        && value
            .pointer("/change/id")
            .and_then(Value::as_str)
            .is_none_or(|reported| reported == id)
}

/// Everything a recorded verdict is keyed by.
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
        {
            return None;
        }
        let archive = archive_dir(root, id)?;
        let directory = root.join(&archive);
        if !regular(&directory.join("accepted-state.json")) {
            return None;
        }
        let verification: Value =
            serde_json::from_slice(&bounded(&directory.join("verification.json"), ENTRY_LIMIT)?)
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
        for input in std::iter::once(BASELINE).chain(INPUTS) {
            let path = root.join(input);
            match path.symlink_metadata() {
                Ok(meta) if meta.is_file() => files.push(path),
                Ok(_) => return None,
                Err(_) => {}
            }
        }
        let mut total = 0u64;
        for file in &files {
            let relative = file.strip_prefix(root).ok()?.to_str()?.replace('\\', "/");
            let bytes = bounded(file, INPUT_LIMIT - total)?;
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

    /// The recorded verdict, when every key part still matches.
    fn reuse(&self) -> Option<NativeJson> {
        let entry: Value = serde_json::from_slice(&bounded(&self.entry, ENTRY_LIMIT)?).ok()?;
        let commit = entry["commit"].as_str()?;
        if entry["schema_version"] != SCHEMA
            || entry["change_id"] != self.id.as_str()
            || entry["archive"] != self.archive.as_str()
            || entry["digest"] != self.digest.as_str()
            || entry["tree"] != self.tree.as_str()
            || entry["specsync"] != self.binary
            || !commit.bytes().all(|byte| byte.is_ascii_hexdigit())
            || !authenticated(&entry["response"], &self.id)
        {
            return None;
        }
        // The commit that carried these bytes must still be reachable: SpecSync
        // authenticates archives against reachable history.
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
        let proof = json!({
            "proved": PROOF,
            "specsync": self.binary,
            "authenticated_at_unix_seconds": entry["authenticated_at_unix_seconds"],
            "digest": self.digest,
            "archive": self.archive,
            "tree": self.tree,
            "commit": commit,
        });
        let mut value = entry["response"].clone();
        value.as_object_mut()?.insert(MARKER.into(), proof);
        let mut content = serde_json::to_string_pretty(&value).ok()?;
        if content.len() > SHOWN {
            let mut end = SHOWN;
            while !content.is_char_boundary(end) {
                end -= 1;
            }
            content.truncate(end);
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

    /// Record a live authenticated verdict, only when the archive is committed as read.
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
        let (Some(clean), Some(commit)) = (clean, commit) else {
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
        let entry = json!({
            "schema_version": SCHEMA,
            "change_id": self.id,
            "archive": self.archive,
            "digest": self.digest,
            "tree": self.tree,
            "commit": commit,
            "specsync": self.binary,
            "proved": PROOF,
            "authenticated_at_unix_seconds": at,
            "response": response,
        });
        let Some(parent) = self.entry.parent() else {
            return;
        };
        if fs::create_dir_all(parent).is_err() {
            return;
        }
        let temporary = parent.join(format!(".{}.{}.tmp", std::process::id(), at));
        if fs::write(&temporary, entry.to_string()).is_err()
            || fs::rename(&temporary, &self.entry).is_err()
        {
            let _ = fs::remove_file(&temporary);
        }
    }
}

/// The key a reused response carries so gates can say what was proved; a live
/// response never carries it.
pub(super) const MARKER: &str = "chough_archive_authentication";

fn plain(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 255
        && !id.starts_with('-')
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

/// The one archive directory, `<id>` or `YYYY-MM-DD-<id>`, as a relative path.
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
    path.symlink_metadata().is_ok_and(|meta| meta.is_file())
}

/// Every regular file under `directory`, sorted; any symlink or other kind fails.
fn walk(directory: &Path, files: &mut Vec<PathBuf>) -> Option<()> {
    let mut entries = fs::read_dir(directory)
        .ok()?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    entries.sort();
    for path in entries {
        let meta = path.symlink_metadata().ok()?;
        if meta.is_dir() {
            walk(&path, files)?;
        } else if meta.is_file() {
            files.push(path);
        } else {
            return None;
        }
    }
    Some(())
}

/// A file's bytes when it is a regular file of at most `limit` bytes.
fn bounded(path: &Path, limit: u64) -> Option<Vec<u8>> {
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

/// The SpecSync program `specsync` resolves to on PATH, by path, size and modification
/// time, so a replaced or upgraded SpecSync never reuses another binary's verdict.
fn specsync_binary() -> Option<Value> {
    let names: &[&str] = if cfg!(windows) {
        &["specsync.exe", "specsync.cmd", "specsync.bat", "specsync"]
    } else {
        &["specsync"]
    };
    for directory in std::env::split_paths(&std::env::var_os("PATH")?) {
        for name in names {
            let candidate = directory.join(name);
            if let Ok(meta) = fs::metadata(&candidate)
                && meta.is_file()
            {
                let path = fs::canonicalize(&candidate).ok()?;
                let modified = meta.modified().ok()?.duration_since(UNIX_EPOCH).ok()?;
                return Some(json!({
                    "path": path.to_str()?,
                    "size": meta.len(),
                    "modified_unix_nanos": modified.as_nanos().to_string(),
                }));
            }
        }
    }
    None
}

/// The Git common directory of the worktree at `root`, found without running Git.
fn git_common_dir(root: &Path) -> Option<PathBuf> {
    let dot = root.join(".git");
    let meta = dot.symlink_metadata().ok()?;
    let git_dir = if meta.is_dir() {
        dot
    } else if meta.is_file() {
        let text = String::from_utf8(bounded(&dot, 4096)?).ok()?;
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
    let common = match bounded(&git_dir.join("commondir"), 4096) {
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

/// Trimmed stdout of a successful Git command at `root`.
fn git(root: &Path, arguments: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
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
