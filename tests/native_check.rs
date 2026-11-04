//! The native check in `chough next`: the latest attempt SpecSync recorded, and an honest
//! statement when the check has not passed for this tree (REQ-cli-039, #86).

mod common;

use common::Fixture;
use serde_json::{Value, json};

const CHANGE: &str = "check-change";

#[cfg(unix)]
fn project(state: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native(state);
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", CHANGE]);
    fixture.write(&format!(".specsync/changes/{CHANGE}/state.json"), "{}\n");
    fixture
}

fn attempt(timestamp: u64, commit: &str, commands: &[(&str, bool, i64)]) -> Value {
    json!({
        "timestamp": timestamp,
        "commit": commit,
        "contract_digest": "c",
        "execution_digest": "e",
        "workspace_digest": "w",
        "passed": commands.iter().all(|(_, success, _)| *success),
        "commands": commands
            .iter()
            .map(|(command, success, code)| json!({"command": command, "success": success, "exit_code": code}))
            .collect::<Vec<_>>(),
        "requirement_ids": ["REQ-cli-001"],
    })
}

fn write_attempts(fixture: &Fixture, attempts: &[Value]) {
    fixture.write(
        &format!(".specsync/changes/{CHANGE}/verification-attempts.json"),
        &(json!({"schema_version": 1, "attempts": attempts}).to_string() + "\n"),
    );
}

fn text(fixture: &Fixture) -> String {
    let output = fixture.run(&["next"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[cfg(unix)]
#[test]
fn a_failed_native_attempt_is_shown_in_next() {
    let fixture = project("implementing");
    let failed = attempt(
        1_793_800_000,
        "4100c2aa6afd940da7e986fb37da408ee76d6596",
        &[
            ("specsync check --spec cli", false, 1),
            ("cargo test --locked", true, 0),
            (
                "specsync change check check-change (effective contract: cli)",
                false,
                1,
            ),
        ],
    );
    let older_pass = attempt(
        1_793_700_000,
        "0000000000000000000000000000000000000000",
        &[("specsync check --spec cli", true, 0)],
    );
    write_attempts(&fixture, &[older_pass, failed]);

    let next = fixture.ok(&["next", "--json"]);
    let last = &next["native_check"]["last_attempt"];
    assert_eq!(last["passed"], false, "{next:#}");
    assert_eq!(last["at_unix_seconds"], 1_793_800_000);
    assert_eq!(last["commit"], "4100c2aa6afd");
    assert_eq!(last["failed_commands"]["total"], 2);
    assert_eq!(
        last["failed_commands"]["items"][0],
        json!({"command": "specsync check --spec cli", "exit_code": 1})
    );
    let shown = text(&fixture);
    let line = shown
        .lines()
        .find(|line| line.starts_with("Native check:"))
        .unwrap_or_else(|| panic!("no Native check line: {shown}"));
    assert!(
        line.contains("failed")
            && line.contains("4100c2a")
            && line.contains("specsync check --spec cli")
            && line.contains("exit 1"),
        "{line}"
    );

    // A later passing attempt replaces the failure.
    let failed = fixture.ok(&["next", "--json"])["native_check"]["last_attempt"].clone();
    let passed = attempt(
        1_793_900_000,
        "1111111111111111111111111111111111111111",
        &[("specsync check --spec cli", true, 0)],
    );
    write_attempts(&fixture, &[failed_attempt_value(&failed), passed]);
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        next["native_check"]["last_attempt"]["passed"], true,
        "{next:#}"
    );
    assert_eq!(
        next["native_check"]["last_attempt"]["failed_commands"]["total"],
        0
    );
}

/// Rebuild a raw attempt from the view's summary, enough for ordering by timestamp.
fn failed_attempt_value(view: &Value) -> Value {
    attempt(
        view["at_unix_seconds"].as_u64().unwrap(),
        "4100c2aa6afd940da7e986fb37da408ee76d6596",
        &[("specsync check --spec cli", false, 1)],
    )
}

#[cfg(unix)]
#[test]
fn next_says_when_the_native_check_has_not_passed_for_this_tree() {
    let fixture = project("implementing");
    let next = fixture.ok(&["next", "--json"]);
    let check = &next["native_check"];
    assert_eq!(check["last_attempt"], Value::Null, "{next:#}");
    assert_eq!(check["not_passed_for_tree"], true, "{next:#}");
    let note = check["note"].as_str().unwrap_or_default();
    assert!(
        note.contains("no reason") && note.contains(&format!("specsync change check {CHANGE}")),
        "{note:?}"
    );
    let shown = text(&fixture);
    assert!(
        shown.contains("Native check: no attempt recorded"),
        "{shown}"
    );
    assert!(
        shown.contains(&format!("specsync change check {CHANGE}")),
        "{shown}"
    );

    let verifying = project("verifying");
    let next = verifying.ok(&["next", "--json"]);
    assert_eq!(
        next["native_check"]["not_passed_for_tree"], false,
        "{next:#}"
    );
    assert_eq!(next["native_check"]["note"], Value::Null, "{next:#}");
}

#[cfg(unix)]
#[test]
fn an_unreadable_attempts_record_is_unknown_and_adds_no_native_read() {
    let fixture = project("implementing");
    fixture.write(".stub-count", "");
    fixture.native_calls();
    fixture.ok(&["next", "--json"]);
    let without = fixture.native_calls();

    fixture.write(
        &format!(".specsync/changes/{CHANGE}/verification-attempts.json"),
        "{not json\n",
    );
    let next = fixture.ok(&["next", "--json"]);
    let with = fixture.native_calls();
    let check = &next["native_check"];
    assert!(check["unknown"].is_string(), "{next:#}");
    assert_eq!(check["last_attempt"], Value::Null);
    assert_eq!(with, without, "no extra native read");
    assert_eq!(with, 1, "next makes one native status read");
    let shown = text(&fixture);
    assert!(shown.contains("Native check: unknown"), "{shown}");
}
