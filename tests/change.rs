//! REQ-cli-057 (#171, DRIVE-20 slice 1): `chough change new`, `answer`, `check` and
//! `status` run the matching SpecSync step and pass its output, refusals and exit code
//! through unchanged. Unix only: the stand-in `specsync` is a shell script.
#![cfg(unix)]

mod common;

use common::Fixture;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

/// A `specsync` stand-in that records each argument on its own line in `.stub-argv`
/// and its working directory in `.stub-cwd`, writes its PID to `.stub-pid`, sleeps
/// `.stub-sleep` seconds when that file exists, then prints `.stub-out` to stdout and
/// `.stub-err` to stderr byte for byte and exits with `.stub-exit` (0 when absent).
const STUB: &str = r#"#!/bin/sh
for argument in "$@"; do printf '%s\n' "$argument"; done >.stub-argv
pwd -P >.stub-cwd
echo $$ >.stub-pid
if [ -f .stub-sleep ]; then sleep "$(cat .stub-sleep)"; fi
if [ -f .stub-out ]; then cat .stub-out; fi
if [ -f .stub-err ]; then cat .stub-err >&2; fi
exit "$(cat .stub-exit 2>/dev/null || echo 0)"
"#;

fn fixture() -> Fixture {
    let fixture = Fixture::new();
    fixture.write("bin/specsync", STUB);
    fixture.executable("bin/specsync");
    fixture
}

/// `chough --root <root> <args>` with `bin/` first on PATH and `env` set.
fn run_env(fixture: &Fixture, args: &[&str], env: &[(&str, &str)]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
    command.arg("--root").arg(fixture.root()).args(args);
    let mut paths = vec![fixture.root().join("bin")];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    command.env("PATH", std::env::join_paths(paths).unwrap());
    command.env_remove("CHOUGH_NATIVE_TIMEOUT_SECONDS");
    for (name, value) in env {
        command.env(name, value);
    }
    command.output().unwrap()
}

fn argv(fixture: &Fixture) -> Vec<String> {
    fixture
        .read(".stub-argv")
        .lines()
        .map(String::from)
        .collect()
}

fn canonical(path: &Path) -> String {
    fs::canonicalize(path).unwrap().display().to_string()
}

/// Every file under `.chough/` with its bytes.
fn dot_chough(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries {
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

/// The four steps with the arguments each must pass to SpecSync.
fn steps() -> Vec<(Vec<&'static str>, Vec<&'static str>)> {
    vec![
        (
            vec![
                "change",
                "new",
                "Free shipping at the threshold",
                "--kind",
                "feature",
                "--spec",
                "shipping",
                "--spec",
                "cart",
                "--path",
                "src/shipping.rs",
                "--path",
                "tests/",
            ],
            vec![
                "change",
                "new",
                "Free shipping at the threshold",
                "--kind",
                "feature",
                "--spec",
                "shipping",
                "--spec",
                "cart",
                "--path",
                "src/shipping.rs",
                "--path",
                "tests/",
                "--json",
            ],
        ),
        (
            vec![
                "change",
                "answer",
                "free-shipping",
                "constraints",
                "No external services",
            ],
            vec![
                "change",
                "answer",
                "free-shipping",
                "constraints",
                "No external services",
                "--json",
            ],
        ),
        (
            vec!["change", "check", "free-shipping"],
            vec!["change", "check", "free-shipping", "--commit"],
        ),
        (
            vec!["change", "status", "free-shipping"],
            vec!["change", "status", "free-shipping", "--json"],
        ),
        (vec!["change", "status"], vec!["change", "status", "--json"]),
    ]
}

#[test]
fn change_new_answer_check_and_status_run_specsync_with_exact_arguments() {
    let fixture = fixture();
    for (chough, specsync) in steps() {
        let output = run_env(&fixture, &chough, &[]);
        assert!(
            output.status.success(),
            "{chough:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(argv(&fixture), specsync, "{chough:?}");
        assert_eq!(
            fixture.read(".stub-cwd").trim_end(),
            canonical(fixture.root()),
            "{chough:?} ran outside --root"
        );
    }
}

#[test]
fn specsync_output_and_exit_code_pass_through_byte_for_byte() {
    let fixture = fixture();
    // Bytes no text handling would keep as they are: no final newline, a CR, a NUL
    // and invalid UTF-8.
    let stdout: &[u8] = b"{\"summary\":{\"state\":\"draft\"}}\r\nsecond line\0\xff";
    let stderr: &[u8] =
        b"Error: refused: the interview is unfinished\nrun `specsync change answer`";
    fs::write(fixture.root().join(".stub-out"), stdout).unwrap();
    fs::write(fixture.root().join(".stub-err"), stderr).unwrap();
    for code in ["0", "1", "3"] {
        fixture.write(".stub-exit", code);
        for (chough, _) in steps() {
            let output = run_env(&fixture, &chough, &[]);
            assert_eq!(output.stdout, stdout, "{chough:?} stdout, exit {code}");
            assert_eq!(output.stderr, stderr, "{chough:?} stderr, exit {code}");
            assert_eq!(
                output.status.code(),
                Some(code.parse().unwrap()),
                "{chough:?} exit code"
            );
        }
    }
}

#[test]
fn no_change_command_writes_under_dot_chough() {
    let fixture = fixture();
    fixture.init();
    fixture.write(".stub-out", "{}\n");
    let before = dot_chough(fixture.root());
    assert!(!before.is_empty(), "the fixture has a checkpoint");
    for code in ["0", "1"] {
        fixture.write(".stub-exit", code);
        for (chough, specsync) in steps() {
            let _ = fs::remove_file(fixture.root().join(".stub-argv"));
            run_env(&fixture, &chough, &[]);
            // The step ran, so an unchanged .chough/ is not just a refusal.
            assert_eq!(argv(&fixture), specsync, "{chough:?} did not run SpecSync");
            assert_eq!(
                dot_chough(fixture.root()),
                before,
                "{chough:?} changed .chough/ (exit {code})"
            );
        }
    }
}

#[test]
fn check_outlives_the_native_bound_while_status_refuses_past_it() {
    let fixture = fixture();
    fixture.write(".stub-sleep", "2");
    fixture.write(".stub-out", "{\"summary\":{\"state\":\"verifying\"}}\n");
    let bound = [("CHOUGH_NATIVE_TIMEOUT_SECONDS", "1")];

    let check = run_env(&fixture, &["change", "check", "free-shipping"], &bound);
    assert!(
        check.status.success(),
        "check was stopped by the read bound: {}",
        String::from_utf8_lossy(&check.stderr)
    );
    assert_eq!(check.stdout, b"{\"summary\":{\"state\":\"verifying\"}}\n");

    let before = dot_chough(fixture.root());
    let status = run_env(&fixture, &["change", "status", "free-shipping"], &bound);
    assert_eq!(status.status.code(), Some(1), "status past the bound");
    let error = String::from_utf8_lossy(&status.stderr);
    let retry = "CHOUGH_NATIVE_TIMEOUT_SECONDS=";
    assert!(
        error.contains("no response within 1 s") && error.contains("nothing was recorded"),
        "{error}"
    );
    assert!(
        error.contains(&format!("`{retry}"))
            && error.contains("chough change status free-shipping`"),
        "the refusal names no retry: {error}"
    );
    assert_eq!(dot_chough(fixture.root()), before);

    // The retry it names, with a longer bound, succeeds.
    let longer = error
        .split(retry)
        .nth(1)
        .and_then(|rest| rest.split_whitespace().next())
        .unwrap();
    let retried = run_env(
        &fixture,
        &["change", "status", "free-shipping"],
        &[("CHOUGH_NATIVE_TIMEOUT_SECONDS", longer)],
    );
    assert!(
        retried.status.success(),
        "the named retry failed: {}",
        String::from_utf8_lossy(&retried.stderr)
    );
    assert_eq!(retried.stdout, b"{\"summary\":{\"state\":\"verifying\"}}\n");
}

#[test]
fn a_missing_specsync_refuses_naming_install_and_the_step_to_rerun() {
    let fixture = Fixture::new();
    // A PATH with the shell tools and no specsync.
    let without = "/usr/bin:/bin";
    for (chough, _) in steps() {
        let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
        command
            .arg("--root")
            .arg(fixture.root())
            .args(&chough)
            .env("PATH", without);
        let output = command.output().unwrap();
        assert_eq!(output.status.code(), Some(1), "{chough:?}");
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(
            error.contains("`cargo install --locked specsync@6.2.0`"),
            "{chough:?} names no install: {error}"
        );
        let step = format!("`chough {}`", chough[..2].join(" "));
        assert!(
            error.contains(&step),
            "{chough:?} names no step to rerun: {error}"
        );
        assert!(output.stdout.is_empty(), "{chough:?}");
    }
    // With SpecSync installed (the stand-in on PATH), each step then succeeds.
    fixture.write("bin/specsync", STUB);
    fixture.executable("bin/specsync");
    for (chough, specsync) in steps() {
        let output = run_env(&fixture, &chough, &[]);
        assert!(output.status.success(), "{chough:?} after installing");
        assert_eq!(argv(&fixture), specsync);
    }
}

#[test]
fn sigterm_to_change_check_stops_specsync() {
    let fixture = fixture();
    fixture.write(".stub-sleep", "30");
    let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
    command
        .arg("--root")
        .arg(fixture.root())
        .args(["change", "check", "free-shipping"])
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut paths = vec![fixture.root().join("bin")];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    command.env("PATH", std::env::join_paths(paths).unwrap());
    let mut child = command.spawn().unwrap();
    let started = Instant::now();
    while !fixture.exists(".stub-pid") {
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "specsync never started"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    let pid = fixture.read(".stub-pid").trim().to_string();
    let alive = |pid: &str| {
        Command::new("kill")
            .args(["-0", pid])
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success()
    };
    assert!(alive(&pid));
    let sent = Command::new("kill")
        .args(["-s", "TERM", &child.id().to_string()])
        .status()
        .unwrap();
    assert!(sent.success());
    let status = child.wait().unwrap();
    assert!(!status.success(), "chough ended normally after SIGTERM");
    let started = Instant::now();
    while alive(&pid) {
        if started.elapsed() > Duration::from_secs(10) {
            let _ = Command::new("kill").args(["-9", &pid]).status();
            panic!("specsync {pid} kept running after chough was stopped");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn an_invalid_native_bound_refuses_status_naming_a_valid_retry() {
    let fixture = fixture();
    fixture.write(".stub-out", "{}\n");
    for invalid in ["0", "3601", "soon"] {
        let _ = fs::remove_file(fixture.root().join(".stub-argv"));
        let output = run_env(
            &fixture,
            &["change", "status", "free-shipping"],
            &[("CHOUGH_NATIVE_TIMEOUT_SECONDS", invalid)],
        );
        assert_eq!(output.status.code(), Some(1), "{invalid}");
        assert!(!fixture.exists(".stub-argv"), "{invalid}: specsync ran");
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(
            error.contains("from 1 to 3600")
                && error.contains(
                    "`CHOUGH_NATIVE_TIMEOUT_SECONDS=120 chough change status free-shipping`"
                ),
            "{invalid}: {error}"
        );
    }
    // The retry it names succeeds.
    let output = run_env(
        &fixture,
        &["change", "status", "free-shipping"],
        &[("CHOUGH_NATIVE_TIMEOUT_SECONDS", "120")],
    );
    assert!(output.status.success());
    assert_eq!(
        argv(&fixture),
        ["change", "status", "free-shipping", "--json"]
    );
}
