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

/// Every file under `.chough` with its bytes.
fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
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
    walk(root, &root.join(".chough"), &mut files);
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
    let _ = fs::remove_file(fixture.root().join("running"));
    let before = snapshot(fixture.root());
    let (mut child, outputs) = start(fixture, args, Inherit::Defaults);
    wait_for_marker(fixture, &mut child);
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
        snapshot(fixture.root()),
        before,
        "{signal}: an interrupted run changed .chough"
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
