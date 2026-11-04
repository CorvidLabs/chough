//! A failed native command is shown with its result in plain words (REQ-cli-003, #87).

mod common;

use common::Fixture;

/// A project whose native change status read runs `script` as `specsync`.
#[cfg(unix)]
fn project(script: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "failing-change"]);
    fixture.write("bin/specsync", &format!("#!/bin/sh\n{script}\n"));
    fixture.executable("bin/specsync");
    fixture
}

fn status_content(fixture: &Fixture) -> String {
    let status = fixture.ok(&["status"]);
    assert_eq!(status["native_specsync"]["state"], "failed", "{status:#}");
    status["native_specsync"]["content"]
        .as_str()
        .unwrap_or_default()
        .to_string()
}

fn next_text(fixture: &Fixture) -> String {
    let output = fixture.run(&["next"]);
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

#[cfg(unix)]
#[test]
fn a_failed_native_read_shows_its_exit_in_plain_words() {
    let fixture = project("echo 'status failed' >&2; exit 2");
    let content = status_content(&fixture);
    assert!(content.starts_with("exit 2\n"), "{content:?}");
    assert!(!content.contains("Some("), "{content:?}");
    let next = next_text(&fixture);
    assert!(next.contains("(failed): exit 2"), "{next}");
    assert!(!next.contains("Some("), "{next}");
}

#[cfg(unix)]
#[test]
fn a_native_read_killed_by_a_signal_says_so() {
    let fixture = project("kill -TERM $$");
    let content = status_content(&fixture);
    assert!(content.starts_with("killed by signal 15\n"), "{content:?}");
    assert!(!content.contains("None"), "{content:?}");
}
