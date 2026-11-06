//! Stopping Chough stops what it started and records nothing partial (FLOW-14,
//! REQ-cli-011). Unix only: SIGINT, SIGTERM and SIGHUP; SIGKILL is out of scope.
#![cfg(unix)]

mod common;

use common::Fixture;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

const HEADER: &str = "profile = \"chough-testing/1\"\nmodule = \"cli\"\n";

fn command_case(id: &str, argv: &str, expect: &str) -> String {
    format!(
        "\n[[case]]\nid = \"{id}\"\ntitle = \"Automated {id}\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\npaths = [\"src/\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"The bound command passes\"]\n\n[case.bind]\nadapter = \"command\"\nargv = {argv}\nexpect_output = [\"{expect}\"]\ntimeout_seconds = 300\n"
    )
}

/// Every file under each of `dirs` that exists, with its bytes.
fn snapshot_dirs(root: &Path, dirs: &[&str]) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(base, &path, files);
            } else {
                let name = path.strip_prefix(base).unwrap().display().to_string();
                files.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    let mut files = BTreeMap::new();
    for dir in dirs {
        if root.join(dir).is_dir() {
            walk(root, &root.join(dir), &mut files);
        }
    }
    files
}

/// How chough inherits SIGHUP and SIGTERM when it starts.
#[derive(Clone, Copy, PartialEq)]
enum Inherit {
    Defaults,
    /// SIGHUP ignored, as under `nohup`.
    IgnoredHup,
    /// SIGTERM blocked in the signal mask.
    BlockedTerm,
}

/// Start chough in the background with its output in files, inheriting `inherit`.
fn start(fixture: &Fixture, args: &[&str], inherit: Inherit) -> (Child, tempfile::TempDir) {
    let outputs = tempfile::tempdir().unwrap();
    let mut paths = vec![fixture.root().join("bin")];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
    // The test runner may itself ignore some of these signals (a task runner or
    // `nohup`), and an ignored signal is inherited; start chough from known defaults.
    // SAFETY: signal(2) is async-signal-safe, as pre_exec requires.
    unsafe {
        command.pre_exec(move || {
            for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
                libc::signal(signal, libc::SIG_DFL);
            }
            if inherit == Inherit::IgnoredHup {
                libc::signal(libc::SIGHUP, libc::SIG_IGN);
            }
            if inherit == Inherit::BlockedTerm {
                let mut set: libc::sigset_t = std::mem::zeroed();
                libc::sigemptyset(&raw mut set);
                libc::sigaddset(&raw mut set, libc::SIGTERM);
                libc::sigprocmask(libc::SIG_BLOCK, &raw const set, std::ptr::null_mut());
            }
            Ok(())
        });
    }
    let child = command
        .arg("--root")
        .arg(fixture.root())
        .args(args)
        .env("PATH", std::env::join_paths(paths).unwrap())
        .stdin(Stdio::null())
        .stdout(File::create(outputs.path().join("stdout")).unwrap())
        .stderr(File::create(outputs.path().join("stderr")).unwrap())
        .spawn()
        .unwrap();
    (child, outputs)
}

/// Wait until the started command has written its marker file.
fn wait_for_marker(fixture: &Fixture, child: &mut Child) {
    let started = Instant::now();
    while !fixture.exists("running") {
        assert!(
            child.try_wait().unwrap().is_none(),
            "chough ended before its command started"
        );
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "the command never started"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn send(child: &Child, signal: &str) {
    let status = Command::new("kill")
        .args(["-s", signal, &child.id().to_string()])
        .status()
        .unwrap();
    assert!(status.success(), "kill -s {signal} failed");
}

fn wait_exit(child: &mut Child) -> ExitStatus {
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            return status;
        }
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "chough did not end after the signal"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// Whether a process whose command line contains `pattern` is still running after
/// about three seconds.
fn still_running(pattern: &str) -> bool {
    for _ in 0..30 {
        let found = Command::new("pgrep")
            .args(["-f", pattern])
            .output()
            .unwrap();
        if !found.status.success() {
            return false;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    true
}

/// Stop chough with `signal` once its command runs; it must end by that signal, leave
/// no process matching `pattern` and leave `.chough` byte-identical.
fn assert_stopped(fixture: &Fixture, args: &[&str], signal: &str, number: i32, pattern: &str) {
    assert_stopped_in(fixture, args, signal, number, pattern, &[".chough"]);
}

/// [`assert_stopped`], with every directory in `dirs` left byte-identical.
fn assert_stopped_in(
    fixture: &Fixture,
    args: &[&str],
    signal: &str,
    number: i32,
    pattern: &str,
    dirs: &[&str],
) {
    assert_stopped_after(fixture, args, signal, number, pattern, dirs, Duration::ZERO);
}

/// [`assert_stopped_in`], with the signal sent `delay` after the marker appears.
fn assert_stopped_after(
    fixture: &Fixture,
    args: &[&str],
    signal: &str,
    number: i32,
    pattern: &str,
    dirs: &[&str],
    delay: Duration,
) {
    let _ = fs::remove_file(fixture.root().join("running"));
    let before = snapshot_dirs(fixture.root(), dirs);
    let (mut child, outputs) = start(fixture, args, Inherit::Defaults);
    wait_for_marker(fixture, &mut child);
    std::thread::sleep(delay);
    send(&child, signal);
    let status = wait_exit(&mut child);
    let survived = still_running(pattern);
    if survived {
        // Do not leave the fixture's command behind when the assertion fails.
        let _ = Command::new("pkill").args(["-f", pattern]).status();
    }
    assert!(!survived, "{signal}: the command chough started survived");
    assert_eq!(
        status.signal(),
        Some(number),
        "{signal}: {status:?} {}",
        fs::read_to_string(outputs.path().join("stderr")).unwrap()
    );
    assert_eq!(
        snapshot_dirs(fixture.root(), dirs),
        before,
        "{signal}: an interrupted run changed {dirs:?}"
    );
}

#[test]
fn stopping_a_case_run_stops_its_command_and_records_nothing() {
    for (signal, number, seconds) in [
        ("INT", libc::SIGINT, 611),
        ("TERM", libc::SIGTERM, 612),
        ("HUP", libc::SIGHUP, 613),
    ] {
        let fixture = Fixture::new();
        fixture.hi_stand_in();
        let argv = format!("[\"sh\", \"-c\", \"touch running; sleep {seconds} & wait\"]");
        fixture.write(
            ".chough/cases/cli.cases.toml",
            &format!("{HEADER}{}", command_case("TC-cli-0001", &argv, "never")),
        );
        fixture.init();
        fixture.commit("fixture");
        assert_stopped(
            &fixture,
            &["cases", "run", "TC-cli-0001", "--environment", "local"],
            signal,
            number,
            &format!("sleep {seconds}"),
        );
    }

    // A signal inherited as ignored stays ignored: the run finishes and records.
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    let argv = "[\"sh\", \"-c\", \"touch running; sleep 2; echo finished\"]";
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!("{HEADER}{}", command_case("TC-cli-0001", argv, "finished")),
    );
    fixture.init();
    fixture.commit("fixture");
    let (mut child, outputs) = start(
        &fixture,
        &["cases", "run", "TC-cli-0001", "--environment", "local"],
        Inherit::IgnoredHup,
    );
    wait_for_marker(&fixture, &mut child);
    send(&child, "HUP");
    let status = wait_exit(&mut child);
    assert!(
        status.success(),
        "{status:?} {}",
        fs::read_to_string(outputs.path().join("stderr")).unwrap()
    );
    let report: Value =
        serde_json::from_slice(&fs::read(outputs.path().join("stdout")).unwrap()).unwrap();
    assert_eq!(report["outcome"], "passed", "{report}");
    let observation = report["observation"].as_str().unwrap();
    assert!(fixture.exists(&format!(".chough/evidence/{observation}.json")));
}

#[test]
fn stopping_a_native_read_stops_specsync_and_records_nothing() {
    for (signal, number, seconds) in [
        ("INT", libc::SIGINT, 621),
        ("TERM", libc::SIGTERM, 622),
        ("HUP", libc::SIGHUP, 623),
    ] {
        let fixture = Fixture::new();
        fixture.stubs();
        fixture.native("approved");
        fixture.init();
        fixture.ok(&["checkpoint", "--native-change", "stop-read"]);
        fixture.write(
            "bin/specsync",
            &format!("#!/bin/sh\ntouch running\nsleep {seconds} &\nwait\n"),
        );
        fixture.executable("bin/specsync");
        fixture.write(".gitignore", ".stub-*\nrunning\n");
        fixture.commit("fixture");
        assert_stopped(
            &fixture,
            &["next", "--json"],
            signal,
            number,
            &format!("sleep {seconds}"),
        );
    }
}

/// The path of the real `git`, for a stand-in that delays and then runs it.
fn real_git() -> String {
    let found = Command::new("sh")
        .args(["-c", "command -v git"])
        .output()
        .unwrap();
    assert!(found.status.success(), "git is required");
    String::from_utf8(found.stdout).unwrap().trim().to_string()
}

/// A stop after the bound command exits, while Chough saves what it saw (its Git
/// snapshot runs here), stops Chough's own git and records nothing: no artifact and
/// no observation (Chough bot review on #79).
#[test]
fn stopping_a_case_run_while_it_saves_stops_git_and_records_nothing() {
    for (signal, number, seconds) in [
        ("INT", libc::SIGINT, 631),
        ("TERM", libc::SIGTERM, 632),
        ("HUP", libc::SIGHUP, 633),
    ] {
        let fixture = Fixture::new();
        fixture.hi_stand_in();
        let argv = "[\"sh\", \"-c\", \"touch ran; echo finished\"]";
        fixture.write(
            ".chough/cases/cli.cases.toml",
            &format!("{HEADER}{}", command_case("TC-cli-0001", argv, "finished")),
        );
        fixture.write(".gitignore", "ran\nrunning\n");
        fixture.init();
        fixture.commit("fixture");
        // Once the bound command has run, every git call marks itself and stalls.
        fixture.write(
            "bin/git",
            &format!(
                "#!/bin/sh\nif [ -e ran ]; then touch running; sleep {seconds}; fi\nexec '{}' \"$@\"\n",
                real_git()
            ),
        );
        fixture.executable("bin/git");
        assert_stopped(
            &fixture,
            &["cases", "run", "TC-cli-0001", "--environment", "local"],
            signal,
            number,
            &format!("sleep {seconds}"),
        );
    }
}

/// A stop signal inherited as blocked stays blocked: the run finishes and records,
/// as it did before stop handling.
#[test]
fn a_stop_signal_inherited_as_blocked_stays_blocked() {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    let argv = "[\"sh\", \"-c\", \"touch running; sleep 2; echo finished\"]";
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!("{HEADER}{}", command_case("TC-cli-0001", argv, "finished")),
    );
    fixture.write(".gitignore", "running\n");
    fixture.init();
    fixture.commit("fixture");
    let (mut child, outputs) = start(
        &fixture,
        &["cases", "run", "TC-cli-0001", "--environment", "local"],
        Inherit::BlockedTerm,
    );
    wait_for_marker(&fixture, &mut child);
    send(&child, "TERM");
    let status = wait_exit(&mut child);
    assert!(
        status.success(),
        "{status:?} {}",
        fs::read_to_string(outputs.path().join("stderr")).unwrap()
    );
    let report: Value =
        serde_json::from_slice(&fs::read(outputs.path().join("stdout")).unwrap()).unwrap();
    assert_eq!(report["outcome"], "passed", "{report}");
    let observation = report["observation"].as_str().unwrap();
    assert!(fixture.exists(&format!(".chough/evidence/{observation}.json")));
}

/// A `git` stand-in that, once `armed` exists, marks itself running and stalls in
/// `sleep {seconds}` before running the real Git.
fn stall_git(fixture: &Fixture, seconds: u32) {
    fixture.write(
        "bin/git",
        &format!(
            "#!/bin/sh\nif [ -e armed ]; then touch running; sleep {seconds}; fi\nexec '{}' \"$@\"\n",
            real_git()
        ),
    );
    fixture.executable("bin/git");
    fixture.write("armed", "");
}

/// `stage backfill` checks its commit with Git before recording anything; a stop then
/// stops that Git and records nothing (Chough bot review 5762745045 on #79).
#[test]
fn stopping_a_backfill_stops_its_git_and_records_nothing() {
    stop_backfills();
}

/// #144: the backfill stop test finds and reaps only its own fixture's processes; a
/// `sleep 641` to `sleep 643` from another worktree's run of the same test is neither
/// taken for a survivor nor killed.
#[cfg(target_os = "linux")]
#[test]
fn the_backfill_stop_test_leaves_another_runs_processes_alone() {
    struct Decoys(Vec<Child>);
    impl Drop for Decoys {
        fn drop(&mut self) {
            for decoy in &mut self.0 {
                let _ = decoy.kill();
                let _ = decoy.wait();
            }
        }
    }
    let mut decoys = Decoys(
        ["641", "642", "643"]
            .iter()
            .map(|seconds| Command::new("sleep").arg(seconds).spawn().unwrap())
            .collect(),
    );
    stop_backfills();
    for decoy in &mut decoys.0 {
        assert!(
            decoy.try_wait().unwrap().is_none(),
            "the stop test killed another run's sleep"
        );
    }
}

fn stop_backfills() {
    for (signal, number, seconds) in [
        ("INT", libc::SIGINT, 641),
        ("TERM", libc::SIGTERM, 642),
        ("HUP", libc::SIGHUP, 643),
    ] {
        let fixture = Fixture::new();
        fixture.stubs();
        fixture.native("approved");
        fixture.init();
        fixture.ok(&["stage", "advance"]);
        fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
        fixture.ok(&["checkpoint", "--native-change", "add-thing"]);
        fixture.no_spec_approval("add-thing");
        // Move the log into the past so a backfill at now minus 5,000 s fits after it.
        let shifted: String = fixture
            .read(".chough/transitions.jsonl")
            .lines()
            .enumerate()
            .map(|(index, line)| {
                let mut record: Value = serde_json::from_str(line).unwrap();
                let at = record["at_unix_seconds"].as_u64().unwrap();
                record["at_unix_seconds"] = Value::from(at - 10_000 + 100 * index as u64);
                format!("{record}\n")
            })
            .collect();
        fixture.write(".chough/transitions.jsonl", &shifted);
        fixture.write(".gitignore", ".stub-*\narmed\nrunning\n");
        fixture.commit("base");
        let head = String::from_utf8(
            Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(fixture.root())
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string();
        let at = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - 5_000)
            .to_string();
        stall_git(&fixture, seconds);
        assert_stopped(
            &fixture,
            &[
                "stage",
                "backfill",
                "--at",
                &at,
                "--commit",
                &head,
                "--quote",
                "Leif: yes",
            ],
            signal,
            number,
            &format!("sleep {seconds}"),
        );
    }
}

/// `library snapshot` reads the Git HEAD for its manifest; a stop then stops that Git
/// and leaves the library unchanged, with no copied files and no manifest (Chough bot
/// review 5762745045 on #79).
#[test]
fn stopping_a_library_snapshot_stops_git_and_records_nothing() {
    for (signal, number, seconds) in [
        ("INT", libc::SIGINT, 651),
        ("TERM", libc::SIGTERM, 652),
        ("HUP", libc::SIGHUP, 653),
    ] {
        let fixture = Fixture::new();
        fixture.hi_stand_in();
        fixture.init();
        fixture.write("notes.md", "a design note\n");
        fixture.write(".gitignore", "armed\nrunning\n");
        fixture.commit("base");
        fixture.ok(&["library", "index"]);
        stall_git(&fixture, seconds);
        assert_stopped_in(
            &fixture,
            &[
                "library",
                "snapshot",
                "notes.md",
                "--reason",
                "keep the note",
            ],
            signal,
            number,
            &format!("sleep {seconds}"),
            &[".chough", "knowledge"],
        );
    }
}

/// A `git` stand-in that, once `armed` exists, leaves `sleep {seconds}` running in the
/// background holding its output pipes, runs the real Git, marks itself and exits.
fn hold_git(fixture: &Fixture, seconds: impl std::fmt::Display) {
    fixture.write(
        "bin/git",
        &format!(
            "#!/bin/sh\n[ -e armed ] && sleep {seconds} &\n'{}' \"$@\"\nstatus=$?\n[ -e armed ] && touch running\nexit $status\n",
            real_git()
        ),
    );
    fixture.executable("bin/git");
    fixture.write("armed", "");
}

fn snapshot_fixture() -> Fixture {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.write("notes.md", "a design note\n");
    fixture.write(".gitignore", "armed\nrunning\n");
    fixture.commit("base");
    fixture.ok(&["library", "index"]);
    fixture
}

/// After Chough's own git exits, a descendant that still holds its output does not
/// hide a stop: Chough keeps watching for stop signals until the pipes close, stops
/// the group and ends by the signal, recording nothing (Leif's thread 4463155937 and
/// Chough bot review 5762997733 on #79).
#[test]
fn stopping_while_a_descendant_of_git_holds_its_output_stops_both() {
    for (signal, number, seconds) in [
        ("INT", libc::SIGINT, 661),
        ("TERM", libc::SIGTERM, 662),
        ("HUP", libc::SIGHUP, 663),
    ] {
        let fixture = snapshot_fixture();
        hold_git(&fixture, seconds);
        // The marker is written as the stand-in exits; the signal follows once git has
        // exited and well inside the grace for the held pipes.
        assert_stopped_after(
            &fixture,
            &[
                "library",
                "snapshot",
                "notes.md",
                "--reason",
                "keep the note",
            ],
            signal,
            number,
            &format!("sleep {seconds}"),
            &[".chough", "knowledge"],
            Duration::from_millis(500),
        );
    }
}

/// Without a stop, a descendant of git that holds its output is stopped after a
/// bounded grace once git exits, and the command finishes with git's output.
#[test]
fn a_descendant_of_git_holding_its_output_is_stopped_after_a_bounded_grace() {
    let fixture = snapshot_fixture();
    hold_git(&fixture, 671);
    let started = Instant::now();
    let (mut child, outputs) = start(
        &fixture,
        &[
            "library",
            "snapshot",
            "notes.md",
            "--reason",
            "keep the note",
        ],
        Inherit::Defaults,
    );
    let started_at = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if started_at.elapsed() > Duration::from_secs(30) {
            let _ = child.kill();
            let _ = Command::new("pkill").args(["-f", "sleep 671"]).status();
            panic!("chough waited on the held pipes with no limit");
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let survived = still_running("sleep 671");
    if survived {
        let _ = Command::new("pkill").args(["-f", "sleep 671"]).status();
    }
    assert!(!survived, "the descendant holding git's output survived");
    assert!(
        status.success(),
        "{status:?} {}",
        fs::read_to_string(outputs.path().join("stderr")).unwrap()
    );
    assert!(started.elapsed() < Duration::from_secs(30));
}

/// A `sleep` argument only this fixture uses: `base` seconds plus a fraction made of this
/// test process's ID, a per-process counter and the clock's nanoseconds, each fixed width
/// so no identity is a prefix of another. The tests below find, check and reap their
/// processes by `sleep <identity>`, so the same tests running in another worktree at the
/// same time, and any other `sleep`, are never matched (#95 review).
fn identity(base: u32) -> String {
    static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let next = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    format!(
        "{base}.{:010}{:04}{nanos:09}",
        std::process::id(),
        next % 10_000
    )
}

/// Kills the processes whose command line contains this fixture's `sleep <identity>`
/// when dropped, so a failing assertion never leaves the test's sleep behind.
struct Reap(String);

impl Drop for Reap {
    fn drop(&mut self) {
        let _ = Command::new("pkill")
            .args(["-KILL", "-f", &self.0])
            .status();
    }
}

/// Issue #81 item 1 (REQ-cli-011, FLOW-14): a descendant that left the process group
/// with `setsid` is stopped too, on Linux.
#[cfg(target_os = "linux")]
#[test]
fn stopping_a_case_run_stops_a_descendant_that_left_the_group() {
    stop_a_descendant_that_left_the_group();
}

/// #95 review: a stop test finds and reaps only its own fixture's processes. A process
/// whose command line is `sleep 691`, as when this suite runs in a second worktree at the
/// same time, is neither taken for a survivor nor killed.
#[cfg(target_os = "linux")]
#[test]
fn a_stop_test_leaves_another_runs_processes_alone() {
    struct Decoy(Child);
    impl Drop for Decoy {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let mut decoy = Decoy(Command::new("sleep").arg("691").spawn().unwrap());
    stop_a_descendant_that_left_the_group();
    assert!(
        decoy.0.try_wait().unwrap().is_none(),
        "the stop test killed another run's sleep 691"
    );
}

#[cfg(target_os = "linux")]
fn stop_a_descendant_that_left_the_group() {
    let sleep = format!("sleep {}", identity(691));
    let _reap = Reap(sleep.clone());
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    let argv = format!("[\"sh\", \"-c\", \"setsid {sleep} & touch running; sleep 692 & wait\"]");
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!("{HEADER}{}", command_case("TC-cli-0001", &argv, "never")),
    );
    fixture.init();
    fixture.commit("fixture");
    assert_stopped(
        &fixture,
        &["cases", "run", "TC-cli-0001", "--environment", "local"],
        "TERM",
        libc::SIGTERM,
        &sleep,
    );
}

/// Issue #81 item 1: a `setsid` descendant whose parent exited at once (a double fork)
/// is orphaned; Chough is its subreaper on Linux, so a stop finds and stops it too.
#[cfg(target_os = "linux")]
#[test]
fn stopping_a_case_run_stops_an_orphan_that_left_the_group() {
    let sleep = format!("sleep {}", identity(695));
    let _reap = Reap(sleep.clone());
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    let argv = format!("[\"sh\", \"-c\", \"(setsid {sleep} &); touch running; sleep 696 & wait\"]");
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!("{HEADER}{}", command_case("TC-cli-0001", &argv, "never")),
    );
    fixture.init();
    fixture.commit("fixture");
    assert_stopped(
        &fixture,
        &["cases", "run", "TC-cli-0001", "--environment", "local"],
        "TERM",
        libc::SIGTERM,
        &sleep,
    );
}

/// Issue #81 item 3 (REQ-cli-011): when a descendant of Chough's own Git holds its
/// output past the grace period and is stopped, Chough says so on stderr.
#[test]
fn a_grace_kill_after_git_is_reported() {
    let seconds = identity(681);
    let _reap = Reap(format!("sleep {seconds}"));
    let fixture = snapshot_fixture();
    hold_git(&fixture, &seconds);
    let output = fixture.run(&[
        "library",
        "snapshot",
        "notes.md",
        "--reason",
        "keep the note",
    ]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains("stopped a process that kept Git's output open"),
        "{stderr}"
    );
}

/// Issue #81 item 2 (REQ-cli-011): a failed `git rev-parse HEAD` is reported on stderr
/// and the move is still recorded; with no repository at all nothing is said.
#[test]
fn a_failed_head_revision_is_reported() {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.commit("fixture");
    fixture.write(
        "bin/git",
        &format!(
            "#!/bin/sh\nif [ \"$1\" = rev-parse ] && [ \"$2\" = HEAD ]; then echo 'fatal: simulated failure' >&2; exit 1; fi\nexec '{}' \"$@\"\n",
            real_git()
        ),
    );
    fixture.executable("bin/git");
    let output = fixture.run(&["stage", "advance"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains("could not read the Git revision") && stderr.contains("simulated failure"),
        "{stderr}"
    );

    let plain = Fixture::new();
    plain.hi_stand_in();
    plain.init();
    let output = plain.run(&["stage", "advance"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(!stderr.contains("Git revision"), "no repository: {stderr}");
}
