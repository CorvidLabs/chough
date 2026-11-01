//! Bounded, project-confined reading and writing shared by every command.

use anyhow::{Context, Result, bail, ensure};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub(super) const LIMIT: usize = 16_384;
pub(super) const RECORD_LIMIT: u64 = 65_536;
const LOG_TAIL_LIMIT: u64 = 65_536;

/// How often a bounded run checks its child and its readers.
const POLL: Duration = Duration::from_millis(25);
/// How long output may stay open after the command exits before its tree is stopped.
const EXIT_GRACE: Duration = Duration::from_secs(2);
/// The shortest wait after an exit at the deadline, so readers can report a normal end.
const EXIT_SETTLE: Duration = Duration::from_millis(200);
/// How long to wait for readers after stopping a tree before abandoning them.
const READER_WAIT: Duration = Duration::from_secs(1);
/// Default deadline for native reads, and the variable that overrides it.
const NATIVE_DEADLINE: Duration = Duration::from_secs(120);
const NATIVE_TIMEOUT_VARIABLE: &str = "CHOUGH_NATIVE_TIMEOUT_SECONDS";

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

/// Longest native change ID accepted, matching SpecSync's own read ceiling (one path
/// component); SpecSync mints IDs of up to 120 bytes from a change description.
pub(super) const CHANGE_ID_LIMIT: usize = 255;

/// A native change ID: the characters of `valid_name`, up to SpecSync's ceiling.
pub(super) fn valid_change_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= CHANGE_ID_LIMIT
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
    bounded_capture(value, value.len() as u64)
}

/// Display a retained prefix of a stream that wrote `total` bytes, within 16 KiB.
fn bounded_capture(retained: &[u8], total: u64) -> String {
    let mut result = String::from_utf8_lossy(&retained[..retained.len().min(LIMIT)]).into_owned();
    if total > LIMIT as u64 {
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

/// Bounds for one external command run through [`run_bounded`].
pub(super) struct Limits {
    pub(super) stdout_cap: usize,
    pub(super) stderr_cap: usize,
    pub(super) deadline: Duration,
}

/// What a bounded run observed. Each retained stream is a prefix of at most its cap;
/// the totals count every byte read from the stream.
pub(super) struct Captured {
    /// The exit code; None when the command timed out or ended by a signal.
    pub(super) exit_code: Option<i32>,
    pub(super) timed_out: bool,
    pub(super) stdout: Vec<u8>,
    pub(super) stdout_total: u64,
    pub(super) stderr: Vec<u8>,
    pub(super) stderr_total: u64,
    /// The command exited, but its output stayed open past the grace period.
    pub(super) held_open: bool,
    /// The command's process tree was confirmed stopped after a deadline or while it
    /// held output open.
    pub(super) descendants_stopped: bool,
    /// A reader was abandoned or failed, so the retained output may be incomplete.
    pub(super) capture_incomplete: bool,
}

#[derive(Default)]
struct Retained {
    bytes: Vec<u8>,
    total: u64,
    failed: bool,
    abandoned: bool,
}

/// Read a stream to its end, keeping at most `cap` bytes and counting every byte.
/// Reading continues beyond the cap so the writer never blocks on a full pipe; an
/// abandoned reader stops at its next chunk.
fn drain(mut stream: impl Read, cap: usize, shared: &Mutex<Retained>) {
    let mut buffer = vec![0_u8; 65_536];
    loop {
        let read = match stream.read(&mut buffer) {
            Ok(0) => return,
            Ok(read) => read,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => {
                shared.lock().unwrap_or_else(PoisonError::into_inner).failed = true;
                return;
            }
        };
        let mut retained = shared.lock().unwrap_or_else(PoisonError::into_inner);
        if retained.abandoned {
            return;
        }
        let keep = read.min(cap.saturating_sub(retained.bytes.len()));
        if keep > 0 {
            let length = retained.bytes.len();
            if retained.bytes.capacity() < length + keep {
                // Grow geometrically, but never reserve beyond the cap.
                let target = (length * 2).max(length + keep).min(cap);
                retained.bytes.reserve_exact(target - length);
            }
            retained.bytes.extend_from_slice(&buffer[..keep]);
        }
        retained.total += read as u64;
    }
}

/// Start a detached reader that reports its end on `done`; it is never joined.
fn spawn_reader(
    stream: Option<impl Read + Send + 'static>,
    cap: usize,
    done: mpsc::Sender<()>,
) -> io::Result<Arc<Mutex<Retained>>> {
    let shared = Arc::new(Mutex::new(Retained::default()));
    let reader = Arc::clone(&shared);
    thread::Builder::new()
        .name("chough-capture".into())
        .spawn(move || {
            if let Some(stream) = stream {
                drain(stream, cap, &reader);
            }
            let _ = done.send(());
        })?;
    Ok(shared)
}

/// Take what a reader retained and tell it to stop retaining.
fn take(shared: &Mutex<Retained>) -> (Vec<u8>, u64, bool) {
    let mut retained = shared.lock().unwrap_or_else(PoisonError::into_inner);
    retained.abandoned = true;
    (
        std::mem::take(&mut retained.bytes),
        retained.total,
        retained.failed,
    )
}

/// Stop the command and its descendants by signalling the process group it leads.
/// Returns true when the group was signalled.
#[cfg(unix)]
fn stop_tree(child: &mut Child) -> bool {
    let stopped = i32::try_from(child.id()).is_ok_and(|group| {
        // SAFETY: kill(2) takes plain integers and has no memory-safety preconditions.
        // The negative ID names the process group the child leads (process_group(0)).
        unsafe { libc::kill(-group, libc::SIGKILL) == 0 }
    });
    // std never signals a child it has already reaped.
    let _ = child.kill();
    stopped
}

/// Stop the command and its descendants with `taskkill /T /F`. Returns true when
/// taskkill reported success; it cannot find descendants whose parent has exited.
#[cfg(windows)]
fn stop_tree(child: &mut Child) -> bool {
    let program = std::env::var_os("SystemRoot").map_or_else(
        || PathBuf::from("taskkill.exe"),
        |root| PathBuf::from(root).join("System32").join("taskkill.exe"),
    );
    let stopped = Command::new(program)
        .args(["/PID", &child.id().to_string(), "/T", "/F"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .and_then(|mut taskkill| {
            let started = Instant::now();
            loop {
                if let Some(status) = taskkill.try_wait()? {
                    return Ok(status);
                }
                if started.elapsed() >= Duration::from_secs(10) {
                    let _ = taskkill.kill();
                    return taskkill.wait();
                }
                thread::sleep(POLL);
            }
        })
        .is_ok_and(|status| status.success());
    let _ = child.kill();
    stopped
}

#[cfg(not(any(unix, windows)))]
fn stop_tree(child: &mut Child) -> bool {
    let _ = child.kill();
    false
}

/// Run an argument vector directly, without a shell, reading both streams concurrently
/// within their caps, and end by the deadline. On the deadline, or when output stays
/// open after the command exits (2 s grace, bounded by the deadline but at least
/// 200 ms), the command and its descendants are stopped (Unix: its process group;
/// Windows: its process tree); readers still blocked a second later are abandoned,
/// never awaited. `env` is set on top of the inherited environment, which is never
/// cleared.
pub(super) fn run_bounded(
    root: &Path,
    argv: &[impl AsRef<OsStr>],
    env: &[(String, String)],
    limits: &Limits,
) -> io::Result<Captured> {
    let Some((program, arguments)) = argv.split_first() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "empty argument vector",
        ));
    };
    let mut command = Command::new(program);
    command
        .args(arguments)
        .envs(env.iter().map(|(name, value)| (name, value)))
        .current_dir(root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        // Lead a new process group so the whole tree can be stopped at once.
        command.process_group(0);
    }
    let mut child = command.spawn()?;
    let (done, finished) = mpsc::channel();
    let stdout = spawn_reader(child.stdout.take(), limits.stdout_cap, done.clone());
    let stderr = spawn_reader(child.stderr.take(), limits.stderr_cap, done);
    let (stdout, stderr) = match (stdout, stderr) {
        (Ok(stdout), Ok(stderr)) => (stdout, stderr),
        (Err(error), _) | (_, Err(error)) => {
            stop_tree(&mut child);
            let _ = child.wait();
            return Err(error);
        }
    };
    let started = Instant::now();
    let mut open = 2_u8;
    let mut status: Option<ExitStatus> = None;
    let mut exited_at: Option<Instant> = None;
    let mut timed_out = false;
    let mut held_open = false;
    let mut descendants_stopped = false;
    loop {
        if status.is_none() {
            match child.try_wait() {
                Ok(Some(exit)) => {
                    status = Some(exit);
                    exited_at = Some(Instant::now());
                }
                Ok(None) => {}
                Err(error) => {
                    stop_tree(&mut child);
                    let _ = child.wait();
                    return Err(error);
                }
            }
        }
        if status.is_some() && open == 0 {
            break;
        }
        let past_deadline = started.elapsed() >= limits.deadline;
        if status.is_none() && past_deadline {
            timed_out = true;
            descendants_stopped = stop_tree(&mut child);
            status = child.wait().ok();
            break;
        }
        if exited_at.is_some_and(|at| {
            at.elapsed() >= EXIT_GRACE || (past_deadline && at.elapsed() >= EXIT_SETTLE)
        }) {
            held_open = true;
            descendants_stopped = stop_tree(&mut child);
            break;
        }
        if open == 0 {
            thread::sleep(POLL);
        } else {
            match finished.recv_timeout(POLL) {
                Ok(()) => open -= 1,
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => open = 0,
            }
        }
    }
    let wait_until = Instant::now() + READER_WAIT;
    while open > 0 {
        match finished.recv_timeout(wait_until.saturating_duration_since(Instant::now())) {
            Ok(()) => open -= 1,
            Err(_) => break,
        }
    }
    let (stdout, stdout_total, stdout_failed) = take(&stdout);
    let (stderr, stderr_total, stderr_failed) = take(&stderr);
    Ok(Captured {
        exit_code: if timed_out {
            None
        } else {
            status.and_then(|status| status.code())
        },
        timed_out,
        stdout,
        stdout_total,
        stderr,
        stderr_total,
        held_open,
        descendants_stopped,
        capture_incomplete: open > 0 || stdout_failed || stderr_failed,
    })
}

/// The deadline for native reads: 120 s unless CHOUGH_NATIVE_TIMEOUT_SECONDS names a
/// whole number of seconds from 1 to 3600. An invalid value is refused, not replaced.
fn native_deadline(value: Option<&OsStr>) -> Result<Duration, String> {
    let Some(value) = value else {
        return Ok(NATIVE_DEADLINE);
    };
    value
        .to_str()
        .and_then(|text| text.parse::<u64>().ok())
        .filter(|seconds| (1..=3_600).contains(seconds))
        .map(Duration::from_secs)
        .ok_or_else(|| {
            format!(
                "{NATIVE_TIMEOUT_VARIABLE} must be a whole number of seconds from 1 to 3600; got {:?}, so the native command was not run",
                value.to_string_lossy().chars().take(64).collect::<String>()
            )
        })
}

fn failure(output: &Captured) -> String {
    format!(
        "exit={:?}\n{}\n{}",
        output.exit_code,
        bounded_capture(&output.stdout, output.stdout_total),
        bounded_capture(&output.stderr, output.stderr_total)
    )
}

/// Run a native read within its bounds: the complete stdout up to the parse bound
/// plus one byte (so "over the bound" is exact), stderr up to the display bound, and
/// the native deadline. A complete capture is returned for the caller to judge; any
/// other result is a section whose state leaves lifecycle state unknown.
fn native_capture(
    root: &Path,
    source: &str,
    program: &str,
    arguments: &[&str],
) -> Result<Captured, Section> {
    let unknown = |state: &str, content: String| Section {
        source: source.into(),
        state: state.into(),
        content,
    };
    let deadline = native_deadline(std::env::var_os(NATIVE_TIMEOUT_VARIABLE).as_deref())
        .map_err(|error| unknown("unavailable", format!("Unknown: {error}")))?;
    let mut argv = vec![program];
    argv.extend_from_slice(arguments);
    let limits = Limits {
        stdout_cap: PARSE_LIMIT + 1,
        stderr_cap: LIMIT,
        deadline,
    };
    let output = run_bounded(root, &argv, &[], &limits)
        .map_err(|error| unknown("unavailable", format!("Unknown: {error}")))?;
    if output.timed_out {
        let stopped = if output.descendants_stopped {
            "the command and its descendants were stopped"
        } else {
            "the command was stopped, but stopping its descendants failed"
        };
        return Err(unknown(
            "timed-out",
            format!(
                "Unknown: no response within {} s ({NATIVE_TIMEOUT_VARIABLE}); {stopped}.\n{}\n{}",
                deadline.as_secs(),
                bounded_capture(&output.stdout, output.stdout_total),
                bounded_capture(&output.stderr, output.stderr_total)
            ),
        ));
    }
    if output.capture_incomplete {
        return Err(unknown(
            "failed",
            format!(
                "Unknown: the command exited but its output stayed open and could not be read completely.\n{}",
                failure(&output)
            ),
        ));
    }
    Ok(output)
}

pub(super) fn native(root: &Path, program: &str, arguments: &[&str]) -> Section {
    let source = format!("{} {}", program, arguments.join(" "));
    match native_capture(root, &source, program, arguments) {
        Ok(output) if output.exit_code == Some(0) => Section {
            source,
            state: "observed".into(),
            content: bounded_capture(&output.stdout, output.stdout_total),
        },
        Ok(output) => Section {
            source,
            state: "failed".into(),
            content: failure(&output),
        },
        Err(section) => section,
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
    let output = match native_capture(root, &source, program, arguments) {
        Ok(output) => output,
        Err(section) => {
            return NativeJson {
                section,
                value: None,
            };
        }
    };
    if output.exit_code != Some(0) {
        return NativeJson {
            section: Section {
                source,
                state: "failed".into(),
                content: failure(&output),
            },
            value: None,
        };
    }
    let length = output.stdout_total;
    if length > PARSE_LIMIT as u64 {
        return NativeJson {
            section: Section {
                source,
                state: "oversized".into(),
                content: format!(
                    "Lifecycle state is unknown: output of {length} bytes exceeds the {PARSE_LIMIT}-byte parse bound.\nBounded raw output:\n{}",
                    bounded_capture(&output.stdout, length)
                ),
            },
            value: None,
        };
    }
    match serde_json::from_slice::<Value>(&output.stdout) {
        Ok(value) if value.is_object() || value.is_array() => {
            let mut content = bounded(&output.stdout);
            if length > LIMIT as u64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn limits(cap: usize, seconds: u64) -> Limits {
        Limits {
            stdout_cap: cap,
            stderr_cap: cap,
            deadline: Duration::from_secs(seconds),
        }
    }

    #[cfg(unix)]
    fn script(text: &str) -> Vec<String> {
        vec!["sh".into(), "-c".into(), text.into()]
    }

    #[cfg(windows)]
    fn script(text: &str) -> Vec<String> {
        vec!["cmd".into(), "/c".into(), text.into()]
    }

    #[test]
    fn a_reader_retains_at_most_its_cap_while_counting_every_byte() {
        for (written, cap) in [
            (0, 8),
            (5, 8),
            (8, 8),
            (200_000, 8),
            (200_000, 0),
            (3_000_000, 1_048_576),
        ] {
            let shared = Mutex::new(Retained::default());
            drain(io::repeat(b'x').take(written as u64), cap, &shared);
            let retained = shared.into_inner().unwrap();
            assert_eq!(retained.total, written as u64, "{written} with cap {cap}");
            assert_eq!(retained.bytes.len(), written.min(cap));
            assert!(retained.bytes.capacity() <= cap, "{written} with cap {cap}");
            assert!(!retained.failed);
        }
    }

    #[test]
    fn an_abandoned_reader_stops_retaining() {
        let shared = Mutex::new(Retained {
            abandoned: true,
            ..Retained::default()
        });
        drain(io::repeat(b'x').take(1_000), 100, &shared);
        let retained = shared.into_inner().unwrap();
        assert!(retained.bytes.is_empty());
        assert_eq!(retained.total, 0);
    }

    #[test]
    fn an_empty_argument_vector_is_refused() {
        let root = tempfile::tempdir().unwrap();
        let empty: [&str; 0] = [];
        assert!(run_bounded(root.path(), &empty, &[], &limits(8, 1)).is_err());
    }

    #[test]
    fn both_streams_are_counted_beyond_their_caps() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("big.txt"), vec![b'x'; 3_000_000]).unwrap();
        #[cfg(unix)]
        let argv = script("cat big.txt; cat big.txt >&2");
        #[cfg(windows)]
        let argv = script("type big.txt & type big.txt 1>&2");
        let output = run_bounded(root.path(), &argv, &[], &limits(4_096, 60)).unwrap();
        assert_eq!(output.exit_code, Some(0));
        assert_eq!(
            (output.stdout.len(), output.stdout_total),
            (4_096, 3_000_000)
        );
        assert_eq!(
            (output.stderr.len(), output.stderr_total),
            (4_096, 3_000_000)
        );
        assert!(!output.timed_out && !output.held_open && !output.capture_incomplete);
    }

    #[test]
    fn a_deadline_stops_descendants_that_hold_the_pipes() {
        let root = tempfile::tempdir().unwrap();
        #[cfg(unix)]
        let argv = script("sleep 30 & sleep 30");
        #[cfg(windows)]
        let argv = script("start /b ping -n 30 127.0.0.1 & ping -n 30 127.0.0.1");
        let started = Instant::now();
        let output = run_bounded(root.path(), &argv, &[], &limits(4_096, 1)).unwrap();
        let elapsed = started.elapsed();
        assert!(elapsed < Duration::from_secs(10), "{elapsed:?}");
        assert!(output.timed_out && output.descendants_stopped);
        assert_eq!(output.exit_code, None);
        assert!(!output.held_open);
        #[cfg(unix)]
        assert!(!output.capture_incomplete);
    }

    #[test]
    fn output_held_open_after_exit_ends_the_run_within_the_grace() {
        let root = tempfile::tempdir().unwrap();
        #[cfg(unix)]
        let argv = script("sleep 30 & echo started");
        #[cfg(windows)]
        let argv = script("start /b ping -n 30 127.0.0.1 & echo started");
        let started = Instant::now();
        let output = run_bounded(root.path(), &argv, &[], &limits(4_096, 20)).unwrap();
        let elapsed = started.elapsed();
        assert!(elapsed < Duration::from_secs(10), "{elapsed:?}");
        assert!(output.held_open && !output.timed_out);
        assert_eq!(output.exit_code, Some(0));
        assert!(String::from_utf8_lossy(&output.stdout).contains("started"));
        // A tree that could not be stopped leaves a reader abandoned, never awaited.
        assert!(output.descendants_stopped || output.capture_incomplete);
        #[cfg(unix)]
        assert!(output.descendants_stopped && !output.capture_incomplete);
    }

    #[cfg(unix)]
    #[test]
    fn a_command_that_closes_its_output_early_is_still_awaited_to_its_exit() {
        let root = tempfile::tempdir().unwrap();
        let argv = script("exec >/dev/null 2>&1; sleep 1; exit 4");
        let output = run_bounded(root.path(), &argv, &[], &limits(64, 20)).unwrap();
        assert_eq!(output.exit_code, Some(4));
        assert!(!output.timed_out && !output.held_open && !output.capture_incomplete);
    }

    #[test]
    fn native_deadline_override_is_validated() {
        assert_eq!(native_deadline(None), Ok(NATIVE_DEADLINE));
        for (value, seconds) in [("1", 1), ("2", 2), ("3600", 3_600)] {
            assert_eq!(
                native_deadline(Some(OsStr::new(value))),
                Ok(Duration::from_secs(seconds))
            );
        }
        for value in ["", "0", "3601", "-5", "1.5", " 2", "two"] {
            let error = native_deadline(Some(OsStr::new(value))).unwrap_err();
            assert!(error.contains("from 1 to 3600"), "{value}: {error}");
            assert!(error.contains("not run"), "{value}: {error}");
        }
    }
}
