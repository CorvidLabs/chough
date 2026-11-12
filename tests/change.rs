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
fn no_change_command_writes_a_tracked_file_under_dot_chough() {
    // REQ-cli-057 as modified by REQ-cli-060: no tracked file under `.chough/` changes,
    // and under `.chough/local/` a step writes only its change's direct-write baseline.
    let fixture = fixture();
    fixture.init();
    // A checkpoint save, as in any project past init: the writer's lock creates the
    // `.gitignore` that keeps `.chough/local/` untracked.
    fixture.ok(&["checkpoint", "--next-action", "Run the change steps"]);
    fixture.write(".stub-out", "{}\n");
    let split = |files: BTreeMap<String, Vec<u8>>| {
        let (local, tracked): (BTreeMap<_, _>, BTreeMap<_, _>) = files
            .into_iter()
            .partition(|(name, _)| name.starts_with(".chough/local/"));
        (tracked, local)
    };
    let (tracked_before, local_before) = split(dot_chough(fixture.root()));
    assert!(!tracked_before.is_empty(), "the fixture has a checkpoint");
    let baseline = ".chough/local/native-writes/free-shipping.json";
    for code in ["0", "1"] {
        fixture.write(".stub-exit", code);
        for (chough, specsync) in steps() {
            let _ = fs::remove_file(fixture.root().join(".stub-argv"));
            run_env(&fixture, &chough, &[]);
            // The step ran, so an unchanged .chough/ is not just a refusal.
            assert_eq!(argv(&fixture), specsync, "{chough:?} did not run SpecSync");
            let (tracked, local) = split(dot_chough(fixture.root()));
            assert_eq!(
                tracked, tracked_before,
                "{chough:?} changed a tracked file under .chough/ (exit {code})"
            );
            for (name, bytes) in &local {
                assert!(
                    local_before.get(name) == Some(bytes) || name == baseline,
                    "{chough:?} wrote {name} under .chough/local/ (exit {code})"
                );
            }
        }
    }
    // `answer` and `check` name the change, so its baseline now exists.
    assert!(
        fixture.exists(baseline),
        "no direct-write baseline after chough's own steps"
    );
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
            error.contains("`cargo install --locked specsync@6.3.0`"),
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
    assert!(common::alive(&pid));
    let sent = Command::new("kill")
        .args(["-s", "TERM", &child.id().to_string()])
        .status()
        .unwrap();
    assert!(sent.success());
    let status = child.wait().unwrap();
    assert!(!status.success(), "chough ended normally after SIGTERM");
    let started = Instant::now();
    while common::alive(&pid) {
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

// REQ-cli-059 (#179, DRIVE-20 slice 2): `chough change ship` ships after the review,
// closes the review gate with its relay and moves the session to done.

const SHIP_ID: &str = "a5";
const SHIP_KEY: &str = "ship-a5";
const REVIEW_GATE: &str = "Implementation review of a5";

/// A stand-in for `specsync change ship` and `change status`. Ship records each
/// argument on its own line in `.stub-ship-calls` (appending), writes its PID to
/// `.stub-pid`, sleeps `.stub-ship-sleep` seconds when that file exists, prints
/// `.stub-ship-out` and `.stub-ship-err`, and exits with `.stub-ship-exit` (0 when
/// absent). Status reports an archived change, or exits 2 while `.stub-status-fail`
/// exists.
const SHIP_STUB: &str = r#"#!/bin/sh
if [ "$2" = ship ]; then
    for argument in "$@"; do printf '%s\n' "$argument"; done >>.stub-ship-calls
    echo $$ >.stub-pid
    if [ -f .stub-ship-sleep ]; then sleep "$(cat .stub-ship-sleep)"; fi
    if [ -f .stub-ship-out ]; then cat .stub-ship-out; fi
    if [ -f .stub-ship-err ]; then cat .stub-ship-err >&2; fi
    exit "$(cat .stub-ship-exit 2>/dev/null || echo 0)"
fi
if [ "$2" = review ] || [ "$2" = approve ]; then
    echo "$*" >>.stub-native-calls
    if [ -f .stub-review-exit ]; then echo 'review refused' >&2; exit "$(cat .stub-review-exit)"; fi
    exit 0
fi
if [ -f .stub-status-fail ]; then echo 'status failed' >&2; exit 2; fi
printf '{"summary":{"id":"%s","state":"archived","approval_valid":true,"correction_valid":true,"terminal_evidence":{"validity":"authenticated-history"}},"acceptance_entries":[{"path":"specs/cli/cli.spec.md","kind":"file"}]}\n' "$3"
"#;

/// A session for native change `a5` at `stage`, its review gate open, with the
/// archive files the move into done reads (as `tests/advance_to.rs` builds them).
fn shipping_session_at(stage: &str) -> Fixture {
    shipping_session(stage, true, &[REVIEW_GATE])
}

/// As [`shipping_session_at`], with or without the native change recorded and with
/// `gates` open.
fn shipping_session(stage: &str, native: bool, gates: &[&str]) -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(".specsync/sdd.json", "{\"enabled\": true}\n");
    fixture.write("bin/specsync", SHIP_STUB);
    fixture.executable("bin/specsync");
    fixture.hi_stand_in();
    let dir = ".specsync/archive/changes/2026-11-01-a5";
    fixture.write(
        &format!("{dir}/finalization.json"),
        &format!(
            "{{\"schema_version\": 2, \"change_id\": \"a5\", \"timestamp\": 500, \"implementation_commit\": \"{}\"}}\n",
            "1".repeat(40)
        ),
    );
    fixture.write(
        &format!("{dir}/verification.json"),
        &serde_json::to_string_pretty(&serde_json::json!({
            "commit": "2".repeat(40),
            "passed": true,
            "requirement_ids": ["REQ-cli-001"],
            "acceptance_manifest": {"schema_version": 1, "entries": [
                {"path": "specs/cli/cli.spec.md", "kind": "file"}
            ]},
        }))
        .unwrap(),
    );
    fixture.write(
        &format!("{dir}/change.md"),
        "---\nid: a5\nstate: archived\n---\n\n# Title of a5\n\n## Intent\n\nx\n\n## Acceptance Criteria\n\n- a5 works\n",
    );
    fixture.ok(&["session", "new", SHIP_KEY, "--objective", "Ship a5"]);
    // The review gate drives the native review as the decided human (REQ-records-008).
    fixture.write(
        ".chough/local/identity.toml",
        "[[human]]\nname = \"SYNTHETIC Human\"\naliases = [\"SYNTHETIC-human\"]\nspecsync_actor = \"SYNTHETIC-reviewer\"\n",
    );
    fixture.ok(&["checkpoint", "--native-change", SHIP_ID]);
    fixture.ok(&["stage", "reset", "--to", stage, "--reason", "Ready to ship"]);
    for gate in gates {
        fixture.ok(&["checkpoint", "--awaiting-human", gate]);
    }
    if !native {
        // A session that lost its native change record; reaching accept needs one, so
        // the record is cleared after the reset.
        fixture.ok(&["session", "use", SHIP_KEY]);
        fixture.set_front("native_change", "");
    }
    fixture.commit("fixture");
    fixture
}

/// The ship session's checkpoint, open or archived.
fn ship_session(fixture: &Fixture) -> serde_json::Value {
    fixture.session_json(SHIP_KEY)
}

/// The four relay values for the review gate.
fn review_relay() -> Vec<&'static str> {
    vec![
        "--close-gate",
        REVIEW_GATE,
        "--decided-by",
        "SYNTHETIC-human",
        "--words",
        "pass",
        "--where",
        "a test",
        "--recorded-by",
        "SYNTHETIC-agent",
    ]
}

fn ship_with<'a>(id: &'a str, rest: &[&'a str]) -> Vec<&'a str> {
    let mut args = vec!["change", "ship", id];
    args.extend_from_slice(rest);
    args
}

fn ship_calls(fixture: &Fixture) -> Vec<String> {
    if !fixture.exists(".stub-ship-calls") {
        return vec![];
    }
    fixture
        .read(".stub-ship-calls")
        .lines()
        .map(String::from)
        .collect()
}

/// Every file under `.chough/` except the lock directory, with its bytes.
fn records(root: &Path) -> BTreeMap<String, Vec<u8>> {
    dot_chough(root)
        .into_iter()
        .filter(|(name, _)| !name.starts_with(".chough/local"))
        .collect()
}

/// Whether the archived story's Timeline records the review gate closed with the four
/// values. After the move into done there is no open session, so the story is read
/// directly.
fn gate_closed_with_relay(fixture: &Fixture) -> bool {
    let expected = serde_json::json!({
        "decided_by": "SYNTHETIC-human", "words": "pass", "where": "a test",
        "recorded_by": "SYNTHETIC-agent"
    });
    // The session may still be open (a refused done move) or archived.
    let file = if fixture.is_archived(SHIP_KEY) {
        fixture.story_of(SHIP_KEY)
    } else {
        fixture.session_file_of(SHIP_KEY)
    };
    let story = fixture.read(&file);
    let mut gate_closed = false;
    for line in story.lines() {
        if line.starts_with("- ") {
            gate_closed = line.contains("· gate-closed ·");
        } else if gate_closed && let Some(payload) = line.strip_prefix("  - relayed_by: ") {
            let relay: serde_json::Value = serde_json::from_str(payload).unwrap();
            let four = serde_json::json!({
                "decided_by": relay["decided_by"], "words": relay["words"],
                "where": relay["where"], "recorded_by": relay["recorded_by"]
            });
            if four == expected {
                return true;
            }
        }
    }
    false
}

#[test]
fn ship_with_the_review_relay_ships_closes_the_gate_and_moves_to_done() {
    let fixture = shipping_session_at("accept");
    fixture.write(".stub-ship-out", "SHIPPED a5\n");
    let output = run_env(&fixture, &ship_with(SHIP_ID, &review_relay()), &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(ship_calls(&fixture), ["change", "ship", SHIP_ID]);
    // REQ-records-008: the review ran once, as the decided human, before the ship.
    assert_eq!(
        fixture.read(".stub-native-calls"),
        format!("change review {SHIP_ID} --reviewer SYNTHETIC-reviewer --verdict pass\n")
    );
    assert!(
        output.stdout.starts_with(b"SHIPPED a5\n"),
        "SpecSync's output does not come first: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        gate_closed_with_relay(&fixture),
        "no relayed gate-closed event"
    );
    let session = ship_session(&fixture);
    assert_eq!(session["stage"]["current"], "done", "{session:#}");
    assert_eq!(session["awaiting_human"], serde_json::json!([]));
}

/// REQ-cli-059, REQ-records-008: a review gate's relay naming no configured human is
/// refused before SpecSync runs; a refused review stops the ship with its output and
/// exit code.
#[test]
fn ship_refuses_an_unknown_human_and_stops_on_a_refused_review() {
    let fixture = shipping_session_at("accept");
    let before = records(fixture.root());
    let mut relay = review_relay();
    relay[3] = "nobody";
    let output = run_env(&fixture, &ship_with(SHIP_ID, &relay), &[]);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("refused:human-identity-unknown"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!fixture.exists(".stub-native-calls"));
    assert!(ship_calls(&fixture).is_empty());
    assert_eq!(records(fixture.root()), before);
    fixture.write(".stub-review-exit", "3");
    let output = run_env(&fixture, &ship_with(SHIP_ID, &review_relay()), &[]);
    assert_eq!(output.status.code(), Some(3));
    assert_eq!(String::from_utf8_lossy(&output.stderr), "review refused\n");
    assert!(
        ship_calls(&fixture).is_empty(),
        "the ship ran after a refused review"
    );
    assert_eq!(records(fixture.root()), before);
}

#[test]
fn a_specsync_ship_refusal_passes_through_and_records_nothing() {
    let fixture = shipping_session_at("accept");
    let stdout: &[u8] = b"Ship readiness: blocked\r\n\xff";
    let stderr: &[u8] = b"Error: the scoped review is stale\nrun `specsync change review`";
    fs::write(fixture.root().join(".stub-ship-out"), stdout).unwrap();
    fs::write(fixture.root().join(".stub-ship-err"), stderr).unwrap();
    fixture.write(".stub-ship-exit", "1");
    let before = records(fixture.root());
    let output = run_env(&fixture, &ship_with(SHIP_ID, &review_relay()), &[]);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, stdout);
    assert_eq!(output.stderr, stderr);
    assert_eq!(ship_calls(&fixture), ["change", "ship", SHIP_ID]);
    assert_eq!(records(fixture.root()), before, ".chough/ changed");
}

#[test]
fn ship_refuses_before_running_without_the_relay_or_outside_accept() {
    let relay = review_relay();
    let incomplete: Vec<&str> = relay[..8].to_vec();
    let mut wrong_gate = review_relay();
    wrong_gate[1] = "Scope approval of a5";
    let long_words = "w".repeat(5_000);
    let mut too_long = review_relay();
    too_long[5] = &long_words;
    let values_only: Vec<&str> = relay[2..].to_vec();
    let docs_gate = "Docs check of a5";
    let full = format!("chough change ship {SHIP_ID} --close-gate \"{REVIEW_GATE}\"");
    let bare = format!("chough change ship {SHIP_ID}`");
    let close_docs: Vec<&str> = [
        "checkpoint",
        "--close-gate",
        docs_gate,
        "--decided-by",
        "SYNTHETIC-human",
        "--words",
        "fine",
        "--where",
        "a test",
        "--recorded-by",
        "SYNTHETIC-agent",
        // A docs check held at accept drives the review by default, and is not one.
        "--no-native",
    ]
    .to_vec();
    // Each case: the arguments; the stage, whether the native change is recorded and
    // the open gates; the command the refusal must name; the steps that recovery
    // takes; and the ship that then succeeds.
    struct Case<'a> {
        args: Vec<&'a str>,
        stage: &'a str,
        native: bool,
        gates: Vec<&'a str>,
        names: String,
        recover: Vec<Vec<&'a str>>,
        then: Vec<&'a str>,
    }
    let with_relay = ship_with(SHIP_ID, &relay);
    let without_relay = ship_with(SHIP_ID, &[]);
    let review = vec![REVIEW_GATE];
    let cases = vec![
        Case {
            args: without_relay.clone(),
            stage: "accept",
            native: true,
            gates: review.clone(),
            names: full.clone(),
            recover: vec![],
            then: with_relay.clone(),
        },
        Case {
            args: ship_with(SHIP_ID, &incomplete),
            stage: "accept",
            native: true,
            gates: review.clone(),
            names: full.clone(),
            recover: vec![],
            then: with_relay.clone(),
        },
        Case {
            args: ship_with(SHIP_ID, &wrong_gate),
            stage: "accept",
            native: true,
            gates: review.clone(),
            names: full.clone(),
            recover: vec![],
            then: with_relay.clone(),
        },
        Case {
            args: ship_with(SHIP_ID, &too_long),
            stage: "accept",
            native: true,
            gates: review.clone(),
            names: full.clone(),
            recover: vec![],
            then: with_relay.clone(),
        },
        Case {
            args: ship_with(SHIP_ID, &values_only),
            stage: "accept",
            native: true,
            gates: review.clone(),
            names: full.clone(),
            recover: vec![],
            then: with_relay.clone(),
        },
        Case {
            args: ship_with("b6", &relay),
            stage: "accept",
            native: true,
            gates: review.clone(),
            names: format!("chough change ship {SHIP_ID}"),
            recover: vec![],
            then: with_relay.clone(),
        },
        Case {
            args: with_relay.clone(),
            stage: "accept",
            native: false,
            gates: review.clone(),
            names: format!("chough checkpoint --native-change {SHIP_ID}`"),
            recover: vec![vec!["checkpoint", "--native-change", SHIP_ID]],
            then: with_relay.clone(),
        },
        Case {
            args: with_relay.clone(),
            stage: "accept",
            native: true,
            gates: vec![REVIEW_GATE, docs_gate],
            names: format!("chough checkpoint --close-gate \"{docs_gate}\""),
            recover: vec![close_docs.clone()],
            then: with_relay.clone(),
        },
        Case {
            args: without_relay.clone(),
            stage: "test",
            native: true,
            gates: vec![],
            names: "chough stage advance --to accept`".into(),
            recover: vec![vec!["stage", "advance", "--to", "accept"]],
            then: without_relay.clone(),
        },
        Case {
            args: with_relay.clone(),
            stage: "accept",
            native: true,
            gates: vec![],
            names: bare.clone(),
            recover: vec![],
            then: without_relay.clone(),
        },
        Case {
            args: ship_with(SHIP_ID, &values_only),
            stage: "accept",
            native: true,
            gates: vec![],
            names: bare.clone(),
            recover: vec![],
            then: without_relay.clone(),
        },
    ];
    for case in cases {
        let fixture = shipping_session(case.stage, case.native, &case.gates);
        let before = records(fixture.root());
        let output = run_env(&fixture, &case.args, &[]);
        let error = String::from_utf8_lossy(&output.stderr);
        let label = format!("{:?} at {} with {:?}", case.args, case.stage, case.gates);
        assert_eq!(output.status.code(), Some(1), "{label}: {error}");
        assert!(ship_calls(&fixture).is_empty(), "{label}: SpecSync ran");
        assert_eq!(records(fixture.root()), before, "{label}: .chough/ changed");
        assert!(
            error.contains(&format!("`{}", case.names)),
            "{label}: the refusal does not name `{}`: {error}",
            case.names
        );
        // The recovery it names succeeds, and then so does the ship.
        for step in &case.recover {
            fixture.ok(step);
        }
        let retried = run_env(&fixture, &case.then, &[]);
        assert!(
            retried.status.success(),
            "{label}: the recovery failed: {}",
            String::from_utf8_lossy(&retried.stderr)
        );
        assert_eq!(ship_calls(&fixture), ["change", "ship", SHIP_ID], "{label}");
        assert_eq!(
            ship_session(&fixture)["stage"]["current"],
            "done",
            "{label}"
        );
    }
}

#[test]
fn a_refused_done_move_after_ship_names_the_remaining_step() {
    let fixture = shipping_session_at("accept");
    fixture.write(".stub-status-fail", "");
    let output = run_env(&fixture, &ship_with(SHIP_ID, &review_relay()), &[]);
    let error = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{error}");
    assert_eq!(ship_calls(&fixture), ["change", "ship", SHIP_ID]);
    assert!(error.contains("shipped"), "{error}");
    assert!(
        error.contains("`chough stage advance --to done`"),
        "the refusal names no remaining step: {error}"
    );
    assert!(gate_closed_with_relay(&fixture));
    assert_eq!(ship_session(&fixture)["stage"]["current"], "accept");
    // The step it names succeeds once native status answers.
    fs::remove_file(fixture.root().join(".stub-status-fail")).unwrap();
    fixture.ok(&["stage", "advance", "--to", "done"]);
    assert_eq!(ship_session(&fixture)["stage"]["current"], "done");
}

#[test]
fn ship_outlives_the_native_read_bound() {
    let fixture = shipping_session_at("accept");
    fixture.write(".stub-ship-sleep", "2");
    let output = run_env(
        &fixture,
        &ship_with(SHIP_ID, &review_relay()),
        &[("CHOUGH_NATIVE_TIMEOUT_SECONDS", "1")],
    );
    assert!(
        output.status.success(),
        "ship was stopped by the read bound: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(ship_session(&fixture)["stage"]["current"], "done");
}

#[test]
fn the_walkthrough_and_closeout_run_chough_change() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for name in [
        "docs/walkthrough.md",
        "skills/chough-workflow/references/accept.md",
    ] {
        let text = fs::read_to_string(root.join(name)).unwrap();
        for wanted in [
            "chough change check",
            "chough change ship",
            "specsync change review",
        ] {
            assert!(text.contains(wanted), "{name} does not name `{wanted}`");
        }
        for line in text.lines() {
            assert!(
                !line.contains("specsync change new"),
                "{name} still runs specsync change new: {line}"
            );
            assert!(
                !(line.contains("specsync change check") && line.contains("--commit")),
                "{name} still runs specsync change check --commit: {line}"
            );
            for direct in [
                "specsync change ship ",
                "specsync change ship`",
                "specsync change finalize",
            ] {
                assert!(!line.contains(direct), "{name} still runs {direct}: {line}");
            }
        }
    }
    let walkthrough = fs::read_to_string(root.join("docs/walkthrough.md")).unwrap();
    assert!(walkthrough.contains("chough change new"));
}

/// #189 (REQ-cli-057 as modified): `chough change new` passes `--no-spec-change` and
/// its `--rationale` through, and refuses the first without the second before SpecSync
/// runs.
#[test]
fn change_new_passes_no_spec_change_with_its_rationale() {
    let fixture = fixture();
    let output = run_env(
        &fixture,
        &[
            "change",
            "new",
            "Docs only",
            "--spec",
            "cli",
            "--no-spec-change",
            "--rationale",
            "Only the README changes",
        ],
        &[],
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        argv(&fixture),
        [
            "change",
            "new",
            "Docs only",
            "--spec",
            "cli",
            "--no-spec-change",
            "--rationale",
            "Only the README changes",
            "--json"
        ]
    );
    // Without a rationale it is refused before SpecSync runs, naming --rationale.
    fs::remove_file(fixture.root().join(".stub-argv")).unwrap();
    let refused = run_env(
        &fixture,
        &["change", "new", "Docs only", "--no-spec-change"],
        &[],
    );
    assert!(!refused.status.success());
    assert!(!fixture.exists(".stub-argv"), "SpecSync ran");
    let error = String::from_utf8_lossy(&refused.stderr);
    assert!(error.contains("--rationale"), "{error}");
    // The command it names succeeds.
    let retried = run_env(
        &fixture,
        &[
            "change",
            "new",
            "Docs only",
            "--no-spec-change",
            "--rationale",
            "Only the README changes",
        ],
        &[],
    );
    assert!(retried.status.success());
}

/// REQ-history-010, REQ-cli-059, TC-cli-a-ship-records-the-review-it-ran (A3-1): when
/// `chough change ship` runs the review as the decided human, the gate's `gate-closed`
/// entry names that SpecSync command as `native` and the actor it ran as as `actor`,
/// as a close that drives the review itself does.
#[test]
fn a_ship_that_runs_the_review_records_it_on_the_gate_closed_relay() {
    let fixture = shipping_session_at("accept");
    fixture.write(".stub-ship-out", "SHIPPED a5\n");
    let output = run_env(&fixture, &ship_with(SHIP_ID, &review_relay()), &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let file = if fixture.is_archived(SHIP_KEY) {
        fixture.story_of(SHIP_KEY)
    } else {
        fixture.session_file_of(SHIP_KEY)
    };
    let story = fixture.read(&file);
    let mut gate_closed = false;
    let mut relay = serde_json::Value::Null;
    for line in story.lines() {
        if line.starts_with("- ") {
            gate_closed = line.contains("· gate-closed ·");
        } else if gate_closed && let Some(payload) = line.strip_prefix("  - relayed_by: ") {
            relay = serde_json::from_str(payload).unwrap();
        }
    }
    assert_eq!(
        relay["native"],
        format!("specsync change review {SHIP_ID}"),
        "{story}"
    );
    assert_eq!(relay["actor"], "SYNTHETIC-reviewer", "{story}");
}
