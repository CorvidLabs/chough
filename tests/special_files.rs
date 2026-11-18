//! A FIFO or other non-regular file in a record's place (#262): opening one for reading
//! blocks until a writer appears, so no reader opens it. An entry in `.chough/bugs/`
//! that is not a regular file is not a record, as `chough bug list` reads the folder;
//! a session whose `session.md` is one is a session that cannot be read, listed with the
//! reason and the recovery (REQ-history-006).
#![cfg(unix)]

mod common;

use common::Fixture;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

/// Run `chough` with the fixture's isolation and a deadline: a command that has not
/// finished after 20 seconds is killed and fails the test, so a blocked reader never
/// hangs the suite.
fn run_within_deadline(fixture: &Fixture, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
    command
        .arg("--root")
        .arg(fixture.root())
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for name in common::ACTOR_VARIABLES {
        command.env_remove(name);
    }
    command.env("XDG_CONFIG_HOME", fixture.root().join(common::USER_CONFIG));
    let mut child = command.spawn().unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("chough {args:?} was still running after 20 seconds");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    child.wait_with_output().unwrap()
}

fn mkfifo(fixture: &Fixture, relative: &str) {
    let path = fixture.root().join(relative);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let status = Command::new("mkfifo").arg(&path).status().unwrap();
    assert!(status.success(), "mkfifo {relative}");
}

fn said(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// #262, TC-history-a-fifo-in-the-bugs-folder-blocks-no-reader: a FIFO named like a bug
/// record is not a record. `status` and `next` answer as `bug list` does.
#[test]
fn a_fifo_in_the_bugs_folder_blocks_no_reader() {
    let fixture = Fixture::new();
    fixture.init();
    mkfifo(&fixture, ".chough/bugs/2026-11-17-a-synthetic-fifo.md");
    for args in [
        &["status"][..],
        &["next"][..],
        &["next", "--json"][..],
        &["bug", "list"][..],
    ] {
        let output = run_within_deadline(&fixture, args);
        assert!(output.status.success(), "{args:?}: {}", said(&output));
    }
}

/// REQ-history-006 (#262), TC-history-a-session-whose-record-is-a-fifo-is-listed: a
/// session whose `session.md` is a FIFO cannot be read. `session list` reports it with
/// the reason and the recovery, another session still works, and the recovery moves the
/// folder without reading the FIFO.
#[test]
fn a_session_whose_record_is_a_fifo_is_listed_with_the_reason_and_recovery() {
    let fixture = Fixture::new();
    fixture.init();
    let broken = fixture.active_id();
    fixture.ok(&[
        "session",
        "new",
        "SYNTHETIC-other",
        "--objective",
        "Other work",
    ]);
    let other = fixture.active_id();
    assert_ne!(broken, other);
    let record = format!(".chough/sessions/{broken}/session.md");
    std::fs::remove_file(fixture.root().join(&record)).unwrap();
    mkfifo(&fixture, &record);

    let output = run_within_deadline(&fixture, &["session", "list"]);
    assert!(output.status.success(), "{}", said(&output));
    let listed: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let entry = listed["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|session| session["id"] == broken.as_str())
        .unwrap_or_else(|| panic!("the session with a FIFO record is not listed: {listed:#}"));
    assert_eq!(entry["readable"], false, "{entry:#}");
    assert!(
        entry["reason"]
            .as_str()
            .unwrap_or_default()
            .contains("not a regular file"),
        "{entry:#}"
    );
    assert_eq!(
        entry["recovery"],
        format!("chough session shelve {broken}"),
        "{entry:#}"
    );
    assert_eq!(listed["active"], other.as_str(), "{listed:#}");

    // The other session's commands are not stopped.
    for args in [&["next"][..], &["status"][..]] {
        let output = run_within_deadline(&fixture, args);
        assert!(output.status.success(), "{args:?}: {}", said(&output));
    }

    // The recovery moves the folder without opening the FIFO, and says what it is.
    let output = run_within_deadline(&fixture, &["session", "shelve", &broken]);
    assert!(output.status.success(), "{}", said(&output));
    assert!(!fixture.exists(&format!(".chough/sessions/{broken}")));
    let note = fixture.read(&format!(".chough/archive/shelved/{broken}/shelved.md"));
    assert!(
        note.contains("- `session.md` not a regular file, not read"),
        "{note}"
    );
    let output = run_within_deadline(&fixture, &["session", "list"]);
    let listed = said(&output);
    assert!(
        output.status.success() && !listed.contains(&broken),
        "{listed}"
    );
}
