//! Bounded, project-confined reading and writing shared by every command.

use anyhow::{Context, Result, bail, ensure};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Output, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub(super) const LIMIT: usize = 16_384;
pub(super) const RECORD_LIMIT: u64 = 65_536;

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
/// The variable that names the timing report's whole-report native budget; a read
/// capped by that budget names it when it times out (REQ-reporting-002).
pub(super) const TIMING_BUDGET_VARIABLE: &str = "CHOUGH_TIMING_NATIVE_BUDGET_SECONDS";
/// `CHOUGH_TRACE=1` turns on the opt-in stderr trace (REQ-reporting-002).
const TRACE_VARIABLE: &str = "CHOUGH_TRACE";
/// Longest trace line written, in bytes.
const TRACE_LINE_LIMIT: usize = 1_024;

#[derive(Serialize)]
pub(super) struct Section {
    pub(super) source: String,
    pub(super) state: String,
    pub(super) content: String,
}

/// Whether `CHOUGH_TRACE=1` is set; read once per process.
fn tracing() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os(TRACE_VARIABLE).is_some_and(|value| value == "1"))
}

/// Write one bounded trace line to stderr when tracing is on. A trace never changes
/// standard output, a record or the exit code, so a failed write is ignored.
pub(super) fn trace(line: &str) {
    if !tracing() {
        return;
    }
    let mut text = format!("chough-trace: {}", line.replace(['\n', '\r'], " "));
    if text.len() > TRACE_LINE_LIMIT {
        let mut cut = TRACE_LINE_LIMIT - 3;
        while !text.is_char_boundary(cut) {
            cut -= 1;
        }
        text.truncate(cut);
        text.push_str("...");
    }
    let _ = writeln!(io::stderr().lock(), "{text}");
}

/// Trace one named phase of a command that started at `started`.
pub(super) fn trace_phase(name: &str, started: Instant) {
    trace(&format!(
        "phase {name} elapsed_ms={}",
        started.elapsed().as_millis()
    ));
}

fn trace_native(section: &Section, started: Instant) {
    trace(&native_trace_line(
        &section.source,
        &section.state,
        started.elapsed().as_millis(),
    ));
}

/// One native trace line: the mandatory state and elapsed fields come first, and only
/// the command text after them is shortened, so the whole line (with its prefix) fits
/// the trace bound and never loses its outcome.
fn native_trace_line(source: &str, state: &str, elapsed_ms: u128) -> String {
    let head = format!("native state={state} elapsed_ms={elapsed_ms} command=");
    let room = TRACE_LINE_LIMIT
        .saturating_sub("chough-trace: ".len() + head.len())
        .saturating_sub(3);
    let source = source.replace(['\n', '\r'], " ");
    if source.len() <= room + 3 {
        return format!("{head}{source}");
    }
    let mut cut = room;
    while !source.is_char_boundary(cut) {
        cut -= 1;
    }
    format!("{head}{}...", &source[..cut])
}

pub(super) fn print_json(value: &Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

pub(super) fn now() -> Result<u64> {
    #[cfg(test)]
    if let Some(seconds) = PINNED_CLOCK.with(std::cell::Cell::get) {
        return Ok(seconds);
    }
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

#[cfg(test)]
thread_local! {
    static PINNED_CLOCK: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
    static NATIVE_STDOUT: std::cell::RefCell<Vec<(String, String)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// Runs `run` with `now()` pinned to `seconds` on this thread. Test builds only, so no
/// runtime switch can forge an observation time (issue #30).
#[cfg(test)]
pub(super) fn with_pinned_clock<T>(seconds: u64, run: impl FnOnce() -> T) -> T {
    let previous = PINNED_CLOCK.with(|slot| slot.replace(Some(seconds)));
    let result = run();
    PINNED_CLOCK.with(|slot| slot.set(previous));
    result
}

/// Runs `run` with `native()` and `native_json()` answering `program` with `stdout` on
/// this thread instead of spawning it, so unit tests do not depend on a host install.
/// Test builds only.
#[cfg(test)]
pub(super) fn with_native_stdout<T>(program: &str, stdout: &str, run: impl FnOnce() -> T) -> T {
    NATIVE_STDOUT.with(|slot| slot.borrow_mut().push((program.into(), stdout.into())));
    let result = run();
    NATIVE_STDOUT.with(|slot| slot.borrow_mut().pop());
    result
}

pub(super) fn valid_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 100
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

/// A SpecSync module ID, accepted exactly when SpecSync 6.1.2's `validate_module_name`
/// accepts it: one portable path segment such as `chat.service` or `identity-wipe.service`,
/// with no separator, Windows-invalid or control character, no `.` or `..`, no leading or
/// trailing space, no trailing dot, no Windows reserved device basename, and at most
/// 247 bytes so `<module>.spec.md` fits in one 255-byte path component.
pub(super) fn valid_module_name(value: &str) -> bool {
    let mut components = Path::new(value).components();
    let single =
        matches!(components.next(), Some(Component::Normal(_))) && components.next().is_none();
    let clean = !value.chars().any(|c| {
        matches!(c, '/' | '\\' | '<' | '>' | ':' | '"' | '|' | '?' | '*') || c.is_control()
    });
    if !single || !clean || value != value.trim() || value.ends_with('.') {
        return false;
    }
    if value.len() + ".spec.md".len() > 255 {
        return false;
    }
    let basename = value
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    let numbered = basename.len() == 4
        && (basename.starts_with("COM") || basename.starts_with("LPT"))
        && matches!(basename.as_bytes()[3], b'1'..=b'9');
    !(numbered || matches!(basename.as_str(), "CON" | "PRN" | "AUX" | "NUL"))
}

/// A HI criterion ID, accepted exactly when hi 0.8.0's `Id::parse` accepts it: a family of
/// `A-Z` then `A-Z`, `0-9` or `_`, a hyphen, then levels separated by single dots that
/// alternate number and lowercase letters, starting with a number, such as `LIFE-3`,
/// `UX-12.a` or `SEND-1.a.1.b`. A number level fits in 32 bits and has no leading zero.
/// Like hi, it sets no length limit; the record bounds still apply.
pub(super) fn hi_criterion_id(value: &str) -> bool {
    let Some((family, rest)) = value.split_once('-') else {
        return false;
    };
    let family_ok = family.starts_with(|c: char| c.is_ascii_uppercase())
        && family
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_');
    if !family_ok || rest.is_empty() {
        return false;
    }
    rest.split('.').enumerate().all(|(index, part)| {
        if index % 2 == 0 {
            !part.is_empty()
                && part.bytes().all(|b| b.is_ascii_digit())
                && !(part.len() > 1 && part.starts_with('0'))
                && part.parse::<u32>().is_ok()
        } else {
            !part.is_empty() && part.bytes().all(|b| b.is_ascii_lowercase())
        }
    })
}

/// A criterion ID Chough records: any ID hi accepts (DRIVE-1), and any plain ID that
/// `valid_name` accepted before hi sub-criteria were supported, such as `UX-012`, so no
/// existing checkpoint or bug becomes unreadable. Dotted IDs hi rejects stay refused.
pub(super) fn valid_criterion_id(value: &str) -> bool {
    hi_criterion_id(value) || valid_name(value)
}

/// A requirement or case ID that may embed a dotted SpecSync module, such as
/// `REQ-chat.service-1` or `TC-chat.service-sends-hybrid`: the characters of `valid_name`
/// plus `.`, within the same 100 bytes, with at least one letter or digit. These IDs are
/// never used as paths.
pub(super) fn valid_reference_id(value: &str) -> bool {
    value.bytes().any(|byte| byte.is_ascii_alphanumeric())
        && value.len() <= 100
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
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
    write_new_bounded(root, relative, content, RECORD_LIMIT)
}

/// Create a file only when it does not exist, within an explicit byte bound.
pub(super) fn write_new_bounded(
    root: &Path,
    relative: &str,
    content: &str,
    limit: u64,
) -> Result<bool> {
    ensure!(
        content.len() as u64 <= limit,
        "managed file exceeds {limit} bytes: {relative}"
    );
    write_new_bytes(root, relative, content.as_bytes())
}

/// Create a file of any bounded size only when it does not exist. The content is written
/// to a temporary file first (see [`temporary_for`]) and linked into place only when it
/// is whole, so a write that fails partway, on a full disk or past a file-size limit,
/// leaves no partial file (#260); the temporary is removed either way. A file that
/// appears under the name meanwhile is kept, as when it existed before.
pub(super) fn write_new_bytes(root: &Path, relative: &str, content: &[u8]) -> Result<bool> {
    let path = local_path(root, relative)?;
    if path.exists() {
        ensure!(path.is_file(), "expected a file: {}", path.display());
        return Ok(false);
    }
    fs::create_dir_all(path.parent().context("file has no parent")?)?;
    local_path(root, relative)?;
    let temporary = temporary_for(root, relative, &path)?;
    let created = (|| -> std::io::Result<bool> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(content)?;
        file.sync_all()?;
        match fs::hard_link(&temporary, &path) {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
            Err(error) => Err(error),
        }
    })();
    let _ = fs::remove_file(&temporary);
    Ok(created?)
}

/// Replace a managed record atomically: write a sibling temporary file, then rename.
pub(super) fn write_replace(root: &Path, relative: &str, content: &str) -> Result<()> {
    write_replace_bounded(root, relative, content, RECORD_LIMIT)
}

/// Replace a managed record atomically within an explicit byte bound.
pub(super) fn write_replace_bounded(
    root: &Path,
    relative: &str,
    content: &str,
    limit: u64,
) -> Result<()> {
    ensure!(
        content.len() as u64 <= limit,
        "managed file exceeds {limit} bytes: {relative}"
    );
    let path = local_path(root, relative)?;
    if path.exists() {
        ensure!(path.is_file(), "expected a file: {}", path.display());
    }
    let parent = path.parent().context("file has no parent")?;
    fs::create_dir_all(parent)?;
    local_path(root, relative)?;
    let temporary = temporary_for(root, relative, &path)?;
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

/// Where a write's temporary file goes before its rename or link: for a record under
/// `.chough/`, the Git-ignored `.chough/local/`, so no temporary ever sits beside a
/// committed record and no ignore file is needed among them (REQ-records-002,
/// REQ-history-007); anything else, beside its target. Both are on the project's file
/// system, so the rename stays atomic.
pub(super) fn temporary_for(root: &Path, relative: &str, target: &Path) -> Result<PathBuf> {
    let name = target
        .file_name()
        .and_then(|name| name.to_str())
        .context("record name must be UTF-8")?;
    let pid = std::process::id();
    if relative.starts_with(".chough/") && !relative.starts_with(".chough/local/") {
        let local = local_path(root, ".chough/local")?;
        fs::create_dir_all(&local)?;
        let _ = write_new(root, ".chough/local/.gitignore", "*\n");
        let digest = format!("{:x}", sha2::Sha256::digest(relative.as_bytes()));
        return Ok(local.join(format!(".{}.{name}.{pid}.tmp", &digest[..8])));
    }
    let parent = target.parent().context("file has no parent")?;
    Ok(parent.join(format!(".{name}.{pid}.tmp")))
}

pub(super) fn read_record(path: &Path) -> Result<String> {
    read_bounded_record(path, RECORD_LIMIT)
}

/// Read a native SpecSync spec file under `specs/` at any size. Spec files are the
/// project's contracts, not Chough records, so Chough's record bounds do not apply
/// (Leif, 2026-11-03 10:05 AM MT: "Spec files can grow to whatever number they want").
///
/// This reader does no path checks of its own: callers must resolve `path` through
/// [`local_path`] first, which refuses paths outside the project root and symlinks.
pub(super) fn read_spec_file(path: &Path) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

/// Read a structured record within an explicit byte bound.
pub(super) fn read_bounded_record(path: &Path, limit: u64) -> Result<String> {
    let file = File::open(path)?;
    let mut content = String::new();
    file.take(limit + 1).read_to_string(&mut content)?;
    ensure!(
        content.len() as u64 <= limit,
        "structured record exceeds {limit} bytes: {}",
        path.display()
    );
    Ok(content)
}

/// Largest native record file read, SpecSync 6.1.1's own change-artifact bound
/// (MAX_CHANGE_ARTIFACT_BYTES); native approval ledgers outgrow the 64 KiB record bound.
pub(super) const NATIVE_RECORD_LIMIT: u64 = 4 * 1_048_576;

/// Read a native SpecSync file, such as an archived `finalization.json` or `change.md` or
/// `.specsync/sdd.json`, as text within SpecSync's own 4 MiB change-artifact bound,
/// never Chough's 64 KiB record bound: native files are not Chough records, and a Chough
/// bound must never decide a gate or push anyone to shorten them (LIFE-15).
pub(super) fn read_native_text(path: &Path) -> Result<String> {
    let bytes = read_native_record(path, NATIVE_RECORD_LIMIT)?.with_context(|| {
        format!(
            "native file exceeds SpecSync's {NATIVE_RECORD_LIMIT}-byte artifact bound: {}",
            path.display()
        )
    })?;
    String::from_utf8(bytes)
        .with_context(|| format!("native file is not UTF-8: {}", path.display()))
}

/// Read a native record file read-only within `limit` bytes; `Ok(None)` when it is
/// larger. A missing file is an `io::ErrorKind::NotFound` error.
pub(super) fn read_native_record(path: &Path, limit: u64) -> io::Result<Option<Vec<u8>>> {
    let mut bytes = vec![];
    File::open(path)?.take(limit + 1).read_to_end(&mut bytes)?;
    Ok((bytes.len() as u64 <= limit).then_some(bytes))
}

pub(super) fn bounded(value: &[u8]) -> String {
    bounded_capture(value, value.len() as u64)
}

/// The marker after a display section cut at 16 KiB. The cut is Chough's display bound
/// only: the source is complete, and nothing should shorten it to fit (LIFE-15).
pub(super) const DISPLAY_TRUNCATED: &str = "\n[truncated at 16384 bytes for display only; read the complete source when relevant, and never shorten the source to fit a Chough display bound]";

/// Display a retained prefix of a stream that wrote `total` bytes, within 16 KiB.
fn bounded_capture(retained: &[u8], total: u64) -> String {
    let mut result = String::from_utf8_lossy(&retained[..retained.len().min(LIMIT)]).into_owned();
    if total > LIMIT as u64 {
        result.push_str(DISPLAY_TRUNCATED);
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
    /// On Unix, the signal that ended the command; None when it exited or timed out.
    pub(super) signal: Option<i32>,
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

/// Whether this process made itself a child subreaper (Linux; the binary does at
/// startup). A stop then also collects Chough's own children: orphans of a stopped
/// tree are re-parented here instead of to init (#81).
#[cfg(target_os = "linux")]
static SUBREAPER: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Make this process the subreaper of what it starts, so a descendant whose parent exits
/// (a double fork) is re-parented to Chough and a stop can still find it (Linux; called
/// once by the binary, never by unit tests, which share one process).
pub(super) fn become_subreaper() {
    #[cfg(target_os = "linux")]
    {
        // SAFETY: prctl(2) with PR_SET_CHILD_SUBREAPER takes plain integers.
        if unsafe { libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0) } == 0 {
            SUBREAPER.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
}

/// Every process below `roots` by parent links in `/proc` (Linux), roots included. This
/// finds descendants that left the process group with `setsid` (#81, FLOW-14).
#[cfg(target_os = "linux")]
fn descendants(roots: &[u32]) -> Vec<i32> {
    let mut links: Vec<(i32, i32)> = vec![];
    for entry in fs::read_dir("/proc").into_iter().flatten().flatten() {
        let Some(pid) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse::<i32>().ok())
        else {
            continue;
        };
        let Ok(stat) = fs::read_to_string(entry.path().join("stat")) else {
            continue;
        };
        // The command name is in parentheses and may hold spaces; the state and the
        // parent ID follow the last ')'.
        let Some(rest) = stat.rfind(')').map(|end| &stat[end + 1..]) else {
            continue;
        };
        if let Some(parent) = rest
            .split_whitespace()
            .nth(1)
            .and_then(|field| field.parse::<i32>().ok())
        {
            links.push((pid, parent));
        }
    }
    let mut found: Vec<i32> = roots
        .iter()
        .filter_map(|root| i32::try_from(*root).ok())
        .collect();
    let mut next = 0;
    while next < found.len() {
        let parent = found[next];
        for (pid, of) in &links {
            if *of == parent && !found.contains(pid) {
                found.push(*pid);
            }
        }
        next += 1;
    }
    found
}

/// Stop the command and its descendants: signal the process group it leads and, on
/// Linux, every process below it by parent links, including ones that left the group
/// and, when Chough is their subreaper, orphans of the tree. Returns true when the
/// group was signalled.
#[cfg(unix)]
fn stop_tree(child: &mut Child) -> bool {
    #[cfg(target_os = "linux")]
    let tree = {
        let mut roots = vec![child.id()];
        if SUBREAPER.load(std::sync::atomic::Ordering::SeqCst) {
            roots.push(std::process::id());
        }
        descendants(&roots)
    };
    let stopped = i32::try_from(child.id()).is_ok_and(|group| {
        // SAFETY: kill(2) takes plain integers and has no memory-safety preconditions.
        // The negative ID names the process group the child leads (process_group(0)).
        unsafe { libc::kill(-group, libc::SIGKILL) == 0 }
    });
    #[cfg(target_os = "linux")]
    {
        let own = i32::try_from(std::process::id()).unwrap_or(-1);
        for pid in tree.into_iter().filter(|pid| *pid != own && *pid > 1) {
            // SAFETY: as above; a process that already ended makes kill(2) fail harmlessly.
            unsafe {
                libc::kill(pid, libc::SIGKILL);
            }
        }
    }
    // std never signals a child it has already reaped.
    let _ = child.kill();
    if stopped {
        settle_group(child);
    }
    stopped
}

/// How long [`settle_group`] keeps signalling a stopped command's process group.
#[cfg(unix)]
const GROUP_SETTLE: Duration = Duration::from_millis(500);

/// Signal the process group `child` led until no live member is left (#132). One
/// SIGKILL to a group does not reach a member the group gains while it is being
/// signalled: on macOS a shell forking its background `sleep` as the stop arrived left
/// that `sleep` running after the shell died (3 of 320 loaded trials, none of 320 with
/// this loop). The loop ends when kill(2) finds no live member it may signal: ESRCH,
/// or EPERM when only zombies are left (macOS), or after [`GROUP_SETTLE`]. The leader
/// is not reaped here, so its ID stays taken and is never reused as another group's
/// while this loop signals it; the caller's own wait reaps it.
#[cfg(unix)]
fn settle_group(child: &Child) {
    let Ok(group) = i32::try_from(child.id()) else {
        return;
    };
    let deadline = Instant::now() + GROUP_SETTLE;
    loop {
        // SAFETY: kill(2) takes plain integers and has no memory-safety preconditions;
        // the negative ID names the command's process group.
        if unsafe { libc::kill(-group, libc::SIGKILL) } != 0 || Instant::now() >= deadline {
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
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
///
/// On Unix, while the command runs, SIGINT, SIGTERM and SIGHUP are noted instead of
/// ending Chough at once; a noted signal stops the command and its descendants, and
/// Chough then ends by that signal before any caller can record a partial result
/// (FLOW-14). A signal inherited as ignored stays ignored.
pub(super) fn run_bounded(
    root: &Path,
    argv: &[impl AsRef<OsStr>],
    env: &[(String, String)],
    limits: &Limits,
) -> io::Result<Captured> {
    let _watch = interrupt::watch();
    run_watched(root, argv, env, limits)
}

fn run_watched(
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
        if interrupt::noted() {
            stop_tree(&mut child);
            let _ = child.wait();
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Chough was stopped by a signal",
            ));
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
        signal: if timed_out {
            None
        } else {
            status.and_then(terminating_signal)
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

/// Stop signals while Chough owns a child or saves what one reported (FLOW-14,
/// REQ-cli-011).
#[cfg(unix)]
mod interrupt {
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::{Mutex, PoisonError};

    /// The signals that stop Chough normally: Ctrl-C, a plain kill and a hangup.
    const SIGNALS: [libc::c_int; 3] = [libc::SIGINT, libc::SIGTERM, libc::SIGHUP];
    /// The last stop signal noted while a watch was active; 0 when none.
    static NOTED: AtomicI32 = AtomicI32::new(0);

    /// Only an atomic store, which is async-signal-safe.
    extern "C" fn note(signal: libc::c_int) {
        NOTED.store(signal, Ordering::SeqCst);
    }

    /// Active watches, and the dispositions the first one replaced; the last to end
    /// restores them, so nested or overlapping watches never keep the handler.
    static WATCHING: Mutex<(usize, Vec<(libc::c_int, libc::sigaction)>)> =
        Mutex::new((0, Vec::new()));

    /// One watch. Dropping the last one restores the previous dispositions and, when a
    /// stop signal arrived meanwhile, ends Chough by that signal; during a panic it
    /// only restores them.
    pub(super) struct Watch(());

    /// Note stop signals instead of ending at once. A signal inherited as ignored, or
    /// as blocked in the signal mask, keeps its inherited disposition.
    pub(super) fn watch() -> Watch {
        let mut watching = WATCHING.lock().unwrap_or_else(PoisonError::into_inner);
        watching.0 += 1;
        if watching.0 > 1 {
            return Watch(());
        }
        // SAFETY: pthread_sigmask(3) only reads the calling thread's mask into a
        // zeroed sigset_t.
        let mut blocked: libc::sigset_t = unsafe { std::mem::zeroed() };
        let mask_read = unsafe {
            libc::pthread_sigmask(libc::SIG_BLOCK, std::ptr::null(), &raw mut blocked) == 0
        };
        let previous = &mut watching.1;
        for signal in SIGNALS {
            // SAFETY: sigaction(2) with zeroed, then filled, plain C structs; the
            // handler only stores to an atomic. sigismember reads the set read above.
            unsafe {
                if !mask_read || libc::sigismember(&raw const blocked, signal) == 1 {
                    continue;
                }
                let mut old: libc::sigaction = std::mem::zeroed();
                if libc::sigaction(signal, std::ptr::null(), &raw mut old) != 0
                    || old.sa_sigaction == libc::SIG_IGN
                {
                    continue;
                }
                let mut action: libc::sigaction = std::mem::zeroed();
                action.sa_sigaction = note as extern "C" fn(libc::c_int) as libc::sighandler_t;
                action.sa_flags = libc::SA_RESTART;
                libc::sigemptyset(&raw mut action.sa_mask);
                if libc::sigaction(signal, &raw const action, std::ptr::null_mut()) == 0 {
                    previous.push((signal, old));
                }
            }
        }
        Watch(())
    }

    /// Whether a stop signal arrived while a watch was active.
    pub(super) fn noted() -> bool {
        NOTED.load(Ordering::SeqCst) != 0
    }

    /// End Chough now when a stop signal has arrived: the last point at which a stop
    /// leaves nothing recorded. After it, a stop lets the save finish and then ends
    /// Chough when the last watch ends.
    pub(super) fn stop_if_noted() {
        let signal = NOTED.load(Ordering::SeqCst);
        if signal != 0 {
            restore(&mut WATCHING.lock().unwrap_or_else(PoisonError::into_inner));
            end_by(signal);
        }
    }

    fn restore(watching: &mut (usize, Vec<(libc::c_int, libc::sigaction)>)) {
        for (signal, old) in watching.1.drain(..) {
            // SAFETY: restores a disposition read by sigaction(2) in `watch`.
            unsafe {
                libc::sigaction(signal, &raw const old, std::ptr::null_mut());
            }
        }
    }

    /// End Chough by `signal`, as if it had not been caught.
    fn end_by(signal: libc::c_int) -> ! {
        // SAFETY: plain signal calls with a valid signal number; raise(3) ends the
        // process with the default disposition, so the exit below is a fallback.
        unsafe {
            libc::signal(signal, libc::SIG_DFL);
            let mut set: libc::sigset_t = std::mem::zeroed();
            libc::sigemptyset(&raw mut set);
            libc::sigaddset(&raw mut set, signal);
            libc::pthread_sigmask(libc::SIG_UNBLOCK, &raw const set, std::ptr::null_mut());
            libc::raise(signal);
        }
        std::process::exit(128 + signal);
    }

    impl Drop for Watch {
        fn drop(&mut self) {
            {
                let mut watching = WATCHING.lock().unwrap_or_else(PoisonError::into_inner);
                watching.0 = watching.0.saturating_sub(1);
                if watching.0 > 0 {
                    return;
                }
                restore(&mut watching);
            }
            if std::thread::panicking() {
                return;
            }
            let signal = NOTED.swap(0, Ordering::SeqCst);
            if signal != 0 {
                end_by(signal);
            }
        }
    }
}

/// Without Unix signals there is nothing to watch.
#[cfg(not(unix))]
mod interrupt {
    pub(super) struct Watch(());
    pub(super) fn watch() -> Watch {
        Watch(())
    }
    pub(super) fn noted() -> bool {
        false
    }
    pub(super) fn stop_if_noted() {}
}

/// A stop watch held across a whole operation, such as a case run and its save
/// (FLOW-14): stop signals are noted, and dropping it ends Chough by a noted signal.
pub(super) struct StopWatch {
    _watch: interrupt::Watch,
}

pub(super) fn stop_watch() -> StopWatch {
    StopWatch {
        _watch: interrupt::watch(),
    }
}

/// End Chough now by a stop signal noted under a [`StopWatch`]: the last point at
/// which a stop records nothing.
pub(super) fn stop_if_noted() {
    interrupt::stop_if_noted();
}

/// Run one of Chough's own commands (git) like `Command::output`, with stdin closed
/// unless the caller set it, but under the stop watch: on Unix it leads its own
/// process group, and a stop signal stops that group and ends Chough by the signal
/// before the caller sees a result (FLOW-14). `write` is sent to its stdin when given.
pub(super) fn run_owned(command: &mut Command, write: Option<&[u8]>) -> io::Result<Output> {
    let _watch = interrupt::watch();
    if write.is_none() {
        command.stdin(Stdio::null());
    } else {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let mut child = command.spawn()?;
    // Each reader and the stdin writer report when they end, so the loop below can
    // keep watching for stop signals until every pipe has closed.
    let (done, finished) = mpsc::channel::<()>();
    let read_all = |stream: Option<Box<dyn Read + Send>>, done: mpsc::Sender<()>| {
        thread::Builder::new()
            .name("chough-git".into())
            .spawn(move || {
                let mut bytes = vec![];
                let read = match stream {
                    Some(mut stream) => stream.read_to_end(&mut bytes).map(|_| ()),
                    None => Ok(()),
                };
                let _ = done.send(());
                read.map(|()| bytes)
            })
    };
    let stdout = read_all(
        child
            .stdout
            .take()
            .map(|s| Box::new(s) as Box<dyn Read + Send>),
        done.clone(),
    );
    let stderr = read_all(
        child
            .stderr
            .take()
            .map(|s| Box::new(s) as Box<dyn Read + Send>),
        done.clone(),
    );
    let (stdout, stderr) = match (stdout, stderr) {
        (Ok(stdout), Ok(stderr)) => (stdout, stderr),
        (Err(error), _) | (_, Err(error)) => {
            stop_tree(&mut child);
            let _ = child.wait();
            return Err(error);
        }
    };
    // Stdin is written on its own thread so a large write to a stalled command never
    // holds off the stop check below; the pipe closes when the writer ends.
    let mut open = 2_u8;
    let writer = match (write, child.stdin.take()) {
        (Some(bytes), Some(mut stdin)) => {
            let bytes = bytes.to_vec();
            let done = done.clone();
            match thread::Builder::new()
                .name("chough-git-stdin".into())
                .spawn(move || {
                    let written = stdin.write_all(&bytes);
                    drop(stdin);
                    let _ = done.send(());
                    written
                }) {
                Ok(writer) => {
                    open += 1;
                    Some(writer)
                }
                Err(error) => {
                    stop_tree(&mut child);
                    let _ = child.wait();
                    return Err(error);
                }
            }
        }
        _ => None,
    };
    drop(done);
    // Watch for stop signals until the command has exited and every pipe has closed. A
    // descendant that keeps a pipe open after the command exits is given `EXIT_GRACE`,
    // then the command's group is stopped; readers still open a second later are
    // abandoned, never awaited.
    let mut status: Option<ExitStatus> = None;
    let mut exited_at: Option<Instant> = None;
    loop {
        if interrupt::noted() {
            stop_tree(&mut child);
            let _ = child.wait();
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Chough was stopped by a signal",
            ));
        }
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
        if exited_at.is_some_and(|at| at.elapsed() >= EXIT_GRACE) {
            stop_tree(&mut child);
            // Say so (#81): the stopped process would otherwise go unnoticed.
            eprintln!(
                "chough: stopped a process that kept Git's output open {} s after Git exited",
                EXIT_GRACE.as_secs()
            );
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
    if open > 0 {
        return Err(io::Error::other(
            "a process Chough started kept its output open after it was stopped",
        ));
    }
    let Some(status) = status else {
        return Err(io::Error::other("the command's exit status was not read"));
    };
    if let Some(writer) = writer {
        writer
            .join()
            .unwrap_or_else(|_| Err(io::Error::other("the stdin writer panicked")))?;
    }
    let joined = |reader: thread::JoinHandle<io::Result<Vec<u8>>>| {
        reader
            .join()
            .unwrap_or_else(|_| Err(io::Error::other("a reader panicked")))
    };
    Ok(Output {
        status,
        stdout: joined(stdout)?,
        stderr: joined(stderr)?,
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

/// The signal that ended a command, on Unix.
#[cfg(unix)]
fn terminating_signal(status: std::process::ExitStatus) -> Option<i32> {
    std::os::unix::process::ExitStatusExt::signal(&status)
}

#[cfg(not(unix))]
fn terminating_signal(_: std::process::ExitStatus) -> Option<i32> {
    None
}

/// How a command ended, in plain words (REQ-cli-003, #87).
fn ended(output: &Captured) -> String {
    match (output.exit_code, output.signal) {
        (Some(code), _) => format!("exit {code}"),
        (None, Some(signal)) => format!("killed by signal {signal}"),
        (None, None) if output.timed_out => "timed out".into(),
        (None, None) => "no exit status".into(),
    }
}

fn failure(output: &Captured) -> String {
    format!(
        "{}\n{}\n{}",
        ended(output),
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
    cap: Option<Duration>,
) -> Result<Captured, Section> {
    let unknown = |state: &str, content: String| Section {
        source: source.into(),
        state: state.into(),
        content,
    };
    let per_read = native_deadline(std::env::var_os(NATIVE_TIMEOUT_VARIABLE).as_deref())
        .map_err(|error| unknown("unavailable", format!("Unknown: {error}")))?;
    // A caller's remaining whole-report budget can only shorten the per-read deadline.
    let capped = cap.filter(|cap| *cap < per_read);
    let deadline = capped.unwrap_or(per_read);
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
        let within = match capped {
            Some(cap) => format!(
                "the remaining timing native budget of {:.1} s ({TIMING_BUDGET_VARIABLE})",
                cap.as_secs_f64()
            ),
            None => format!("{} s ({NATIVE_TIMEOUT_VARIABLE})", deadline.as_secs()),
        };
        return Err(unknown(
            "timed-out",
            format!(
                "Unknown: no response within {within}; {stopped}.\n{}\n{}",
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
    #[cfg(test)]
    if let Some(content) = NATIVE_STDOUT.with(|slot| {
        slot.borrow()
            .iter()
            .rev()
            .find(|(name, _)| name == program)
            .map(|(_, stdout)| stdout.clone())
    }) {
        return Section {
            source: format!("{} {}", program, arguments.join(" ")),
            state: "observed".into(),
            content,
        };
    }
    let started = Instant::now();
    let section = native_section(root, program, arguments);
    trace_native(&section, started);
    section
}

fn native_section(root: &Path, program: &str, arguments: &[&str]) -> Section {
    let source = format!("{} {}", program, arguments.join(" "));
    match native_capture(root, &source, program, arguments, None) {
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
    native_json_within(root, program, arguments, None)
}

/// `native_json` with the per-read deadline shortened to `cap` when that is smaller:
/// the timing report passes its remaining whole-report budget (REQ-reporting-002). Gated
/// moves, `next` and `status` use `native_json` and never pass a cap.
pub(super) fn native_json_within(
    root: &Path,
    program: &str,
    arguments: &[&str],
    cap: Option<Duration>,
) -> NativeJson {
    #[cfg(test)]
    if let Some(stdout) = NATIVE_STDOUT.with(|slot| {
        slot.borrow()
            .iter()
            .rev()
            .find(|(name, _)| name == program)
            .map(|(_, stdout)| stdout.clone())
    }) {
        return NativeJson {
            section: Section {
                source: format!("{} {}", program, arguments.join(" ")),
                state: "observed".into(),
                content: bounded(stdout.as_bytes()),
            },
            value: serde_json::from_str::<Value>(&stdout)
                .ok()
                .filter(|value| value.is_object() || value.is_array()),
        };
    }
    let started = Instant::now();
    let response = native_json_read(root, program, arguments, cap);
    trace_native(&response.section, started);
    response
}

fn native_json_read(
    root: &Path,
    program: &str,
    arguments: &[&str],
    cap: Option<Duration>,
) -> NativeJson {
    let source = format!("{} {}", program, arguments.join(" "));
    let output = match native_capture(root, &source, program, arguments, cap) {
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

/// The current Git HEAD when one exists; recorded with transitions as context only. A
/// failure other than no repository or a HEAD with no commit is said on stderr (#81).
pub(super) fn head_revision(root: &Path) -> Option<String> {
    // Git's messages are matched below, so ask for them untranslated.
    let output = match run_owned(
        Command::new("git")
            .args(["rev-parse", "HEAD"])
            .env("LC_ALL", "C")
            .env("LANGUAGE", "C")
            .current_dir(root),
        None,
    ) {
        Ok(output) => output,
        Err(error) => {
            eprintln!(
                "chough: could not read the Git revision ({error}); the record is saved without one"
            );
            return None;
        }
    };
    if output.status.success() {
        return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let expected = stderr.contains("not a git repository")
        || stderr.contains("unknown revision")
        || stderr.contains("ambiguous argument 'HEAD'");
    if !expected {
        let reason = stderr
            .lines()
            .find(|line| !line.trim().is_empty())
            .map_or_else(|| "no message".to_string(), |line| cut(line.trim(), 200));
        let code = output
            .status
            .code()
            .map_or_else(|| "no exit code".to_string(), |code| format!("exit {code}"));
        eprintln!(
            "chough: could not read the Git revision ({code}: {reason}); the record is saved without one"
        );
    }
    None
}

/// At most `limit` bytes of `text`, cut on a character boundary.
fn cut(text: &str, limit: usize) -> String {
    let mut end = text.len().min(limit);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    text[..end].to_string()
}

/// The process ID the project's writer lock file records (REQ-cli-040): the writer
/// that holds the lock writes it there. None when the file names none.
pub(super) fn lock_holder(path: &Path) -> Option<u32> {
    fs::read_to_string(path).ok()?.trim().parse().ok()
}

/// Record this process as the holder of the writer lock `file`, which it has just
/// taken. The file is rewritten in place, never replaced, so every writer keeps
/// locking the same file.
pub(super) fn record_lock_holder(mut file: &File) -> io::Result<()> {
    use std::io::{Seek, SeekFrom};
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(format!("{}\n", std::process::id()).as_bytes())?;
    file.flush()
}

/// Whether process `pid` is running, where that can be told.
fn process_running(pid: u32) -> Option<bool> {
    #[cfg(unix)]
    {
        let pid = libc::pid_t::try_from(pid).ok()?;
        // SAFETY: signal 0 sends nothing; kill only checks that the process exists and
        // that this process may signal it.
        if unsafe { libc::kill(pid, 0) } == 0 {
            return Some(true);
        }
        Some(io::Error::last_os_error().raw_os_error() == Some(libc::EPERM))
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        None
    }
}

/// What a busy refusal says of the lock's holder (REQ-cli-040, #132): the process the
/// lock file names, and, when it is not running, that a process it forked may still
/// hold the lock through an inherited descriptor.
pub(super) fn lock_holder_text(holder: Option<u32>) -> String {
    match holder {
        None => "the lock file names no holder".into(),
        Some(pid) => match process_running(pid) {
            Some(true) => format!("held by process {pid}"),
            Some(false) => format!(
                "the lock file names process {pid}, which is not running, so a process it forked may still hold the lock through an inherited descriptor (#132)"
            ),
            None => format!("the lock file names process {pid}"),
        },
    }
}

/// Trace how long a writer waited for the project's writer lock (REQ-cli-040), so a
/// slow or refused writer in a loaded run shows what it waited on.
pub(super) fn trace_lock_wait(started: Instant) {
    trace(&format!(
        "checkpoint-lock waited_ms={}",
        started.elapsed().as_millis()
    ));
}

/// A wait for people: whole seconds, or milliseconds under one second.
pub(super) fn wait_text(wait: Duration) -> String {
    if wait.as_secs() > 0 {
        format!("{} seconds", wait.as_secs())
    } else {
        format!("{} ms", wait.as_millis())
    }
}

/// Where the `toolchain` task (tools/toolchain.sh) leaves its verdict, under the
/// Git-ignored local logs (REQ-history-007, REQ-records-005).
pub(super) const TOOLCHAIN_LOG: &str = ".chough/local/logs/toolchain.log";

// Plan step 4's evidence writer calls it; until then only its tests do.
#[cfg_attr(not(test), allow(dead_code))]
/// The limitation evidence states when the last `toolchain` run found an installed tool
/// that differs from the CI pins (REQ-cli-019, BUG-0022): `recorded on a toolchain that
/// differs from CI: <each difference>`. None when it matched, never ran or cannot be read.
pub(super) fn toolchain_limitation(root: &Path) -> Option<String> {
    let path = local_path(root, TOOLCHAIN_LOG).ok()?;
    let text = read_bounded_record(&path, 65_536).ok()?;
    let differences = text.lines().next()?.strip_prefix("differs: ")?.trim();
    (!differences.is_empty())
        .then(|| format!("recorded on a toolchain that differs from CI: {differences}"))
}

#[cfg(test)]
mod tests {
    /// BUG-0022: the verdict the toolchain task records becomes an evidence limitation
    /// only when it says the toolchain differs.
    #[test]
    fn a_differing_toolchain_verdict_becomes_an_evidence_limitation() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        assert_eq!(super::toolchain_limitation(root), None);
        let log = root.join(super::TOOLCHAIN_LOG);
        std::fs::create_dir_all(log.parent().unwrap()).unwrap();
        for (verdict, expected) in [
            ("matches\n2026-11-11T00:00:00Z\n", None),
            ("differs: \n", None),
            ("SYNTHETIC unreadable\n", None),
            (
                "differs: Fledge 1.7.2 (CI 1.8.0); HI not installed (CI 0.8.0)\n2026-11-11T00:00:00Z\nFledge: installed 1.7.2, CI pins 1.8.0\n",
                Some(
                    "recorded on a toolchain that differs from CI: Fledge 1.7.2 (CI 1.8.0); HI not installed (CI 0.8.0)",
                ),
            ),
        ] {
            std::fs::write(&log, verdict).unwrap();
            assert_eq!(
                super::toolchain_limitation(root).as_deref(),
                expected,
                "{verdict:?}"
            );
        }
    }

    #[test]
    fn a_long_native_command_keeps_the_trace_state_and_elapsed_fields_within_the_bound() {
        let source = format!("specsync change status {} --json", "é".repeat(4_000));
        let line = format!(
            "chough-trace: {}",
            super::native_trace_line(&source, "timed-out", 120_000)
        );
        assert!(
            line.len() <= super::TRACE_LINE_LIMIT,
            "{} bytes",
            line.len()
        );
        assert!(
            line.starts_with("chough-trace: native state=timed-out elapsed_ms=120000 command=specsync change status "),
            "{line}"
        );
        assert!(line.ends_with("..."), "a shortened command is marked");
        let short = super::native_trace_line("hi export", "observed", 7);
        assert_eq!(
            short,
            "native state=observed elapsed_ms=7 command=hi export"
        );
    }

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
