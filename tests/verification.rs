//! Saying when native verification runs none of the project's commands, judged from the
//! effective commands SpecSync reports, and pointing the test stage to recorded test
//! evidence before accept (REQ-cli-038, FLOW-15, #85).

mod common;

use common::Fixture;
use serde_json::Value;

const ONLY_SPECSYNC: &str = "[\"specsync check --spec cli\"]\n";
const WITH_PROJECT: &str = "[\"specsync check --spec cli\", \"cargo test --locked\"]\n";
const ENABLED: &str = "{\"enabled\": true}\n";
const CONFIGURED: &str =
    "{\"enabled\": true, \"verification_commands\": [\"cargo test --locked\"]}\n";
const DISABLED: &str = "{\"enabled\": false}\n";
const KEY: &str = "native_verification_warning";

/// A project at the test stage of an approved native change. `policy` is its
/// `.specsync/sdd.json` (None: no native configuration) and `effective` the
/// `summary.verification_commands` the stub reports (None: the reply has no list).
#[cfg(unix)]
fn at_test(policy: Option<&str>, effective: Option<&str>) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    if let Some(policy) = policy {
        fixture.native("approved");
        fixture.write(".specsync/sdd.json", policy);
    }
    if let Some(effective) = effective {
        fixture.write(".stub-verification", effective);
    }
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "verify-change"]);
    fixture.no_spec_approval("verify-change");
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.ok(&["stage", "skip", "--json", "--reason", "No visuals"]);
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(
        fixture.ok(&["stage", "show"])["stage"]["current"],
        "test",
        "the fixture must reach the test stage"
    );
    fixture
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

/// The warning `next --json` and `status` carry; each must have the key.
fn warnings(fixture: &Fixture) -> [Value; 2] {
    let next = fixture.ok(&["next", "--json"]);
    let status = fixture.ok(&["status"]);
    for (name, report) in [("next --json", &next), ("status", &status)] {
        assert!(report.get(KEY).is_some(), "{name} has no {KEY}: {report:#}");
    }
    [next[KEY].clone(), status[KEY].clone()]
}

#[cfg(unix)]
#[test]
fn a_check_that_runs_only_specsync_commands_is_reported() {
    let fixture = at_test(Some(ENABLED), Some(ONLY_SPECSYNC));
    for warning in warnings(&fixture) {
        let warning = warning.as_str().unwrap_or_default();
        assert!(
            warning.contains("runs none of the project's commands")
                && warning.contains("specsync check --spec cli"),
            "{warning:?}"
        );
        assert!(
            !warning.contains("not run"),
            "nothing configured: {warning:?}"
        );
    }
    let next = text(&fixture);
    assert!(
        next.lines().any(|line| line.starts_with("Warning:")
            && line.contains("runs none of the project's commands")),
        "{next}"
    );
}

/// The #89 review regression: configuring a command SpecSync does not run must not hide
/// the warning.
#[cfg(unix)]
#[test]
fn configured_commands_the_check_does_not_run_are_counted() {
    let fixture = at_test(Some(CONFIGURED), Some(ONLY_SPECSYNC));
    for warning in warnings(&fixture) {
        let warning = warning.as_str().unwrap_or_default();
        assert!(
            warning.contains("runs none of the project's commands")
                && warning.contains("1 configured command")
                && warning.contains("not run"),
            "{warning:?}"
        );
    }
}

#[cfg(unix)]
#[test]
fn no_warning_when_a_project_command_runs_or_the_list_is_unknown() {
    for (why, policy, effective) in [
        (
            "a project command runs",
            Some(CONFIGURED),
            Some(WITH_PROJECT),
        ),
        ("no effective list", Some(ENABLED), None),
        ("SDD disabled", Some(DISABLED), Some(ONLY_SPECSYNC)),
        ("no native configuration", None, Some(ONLY_SPECSYNC)),
    ] {
        let fixture = at_test(policy, effective);
        for warning in warnings(&fixture) {
            assert_eq!(warning, Value::Null, "{why}");
        }
        assert!(
            !text(&fixture).contains("runs none of the project's commands"),
            "{why}"
        );
    }
}

#[cfg(unix)]
#[test]
fn the_test_stage_points_to_recorded_test_evidence_before_accept() {
    for effective in [ONLY_SPECSYNC, WITH_PROJECT] {
        let fixture = at_test(Some(ENABLED), Some(effective));
        let action = fixture.ok(&["next", "--json"])["next_valid_action"].clone();
        assert!(
            action["action"]
                .as_str()
                .unwrap_or_default()
                .starts_with("Continue test:"),
            "{action:#}"
        );
        assert!(
            action["command"]
                .as_str()
                .unwrap_or_default()
                .contains("chough cases run"),
            "{action:#}"
        );
        assert!(
            action["reason"]
                .as_str()
                .unwrap_or_default()
                .contains("Recorded test evidence is needed before accept"),
            "{action:#}"
        );
    }
}

/// The warning is advice: the same move is allowed with and without it, and it adds no
/// native read.
#[cfg(unix)]
#[test]
fn the_warning_changes_no_gate_and_adds_no_native_read() {
    let mut calls = vec![];
    for effective in [ONLY_SPECSYNC, WITH_PROJECT] {
        let fixture = at_test(Some(ENABLED), Some(effective));
        fixture.write(".stub-count", "");
        fixture.native_calls();
        let next = fixture.ok(&["next", "--json"]);
        let next_calls = fixture.native_calls();
        assert_eq!(
            next[KEY].is_string(),
            effective == ONLY_SPECSYNC,
            "{next:#}"
        );
        let moved = fixture.ok(&["stage", "advance", "--json"]);
        let advance_calls = fixture.native_calls();
        assert_eq!(moved["applied"]["to"], "accept", "{moved:#}");
        calls.push((next_calls, advance_calls));
    }
    assert_eq!(
        calls[0], calls[1],
        "native reads with and without the warning"
    );
}

/// The #92 review: the warning names effective commands, but a huge or long list must not
/// push `next` past its 16384-byte bound (REQ-cli-021); at most 3 commands are named,
/// each cut like other record strings, with a count for the rest.
#[cfg(unix)]
#[test]
fn the_warning_stays_within_the_next_bound() {
    let huge = format!("specsync check --spec {}", "x".repeat(20_000));
    let fixture = at_test(
        Some(ENABLED),
        Some(&(serde_json::to_string(&[huge.as_str()]).unwrap() + "\n")),
    );
    let json = fixture.run(&["next", "--json"]);
    assert!(json.status.success());
    assert!(
        json.stdout.len() <= 16_384,
        "next --json is {} bytes",
        json.stdout.len()
    );
    let view: Value = serde_json::from_slice(&json.stdout).unwrap();
    let warning = view[KEY].as_str().unwrap_or_default();
    assert!(
        warning.contains("runs none of the project's commands") && warning.contains("[cut]"),
        "{warning:.300}"
    );
    let shown = text(&fixture);
    assert!(shown.len() <= 16_384, "next is {} bytes", shown.len());
    assert!(shown.contains("runs none of the project's commands"));

    let many: Vec<String> = (1..=10)
        .map(|n| format!("specsync check --spec module{n}"))
        .collect();
    let fixture = at_test(
        Some(ENABLED),
        Some(&(serde_json::to_string(&many).unwrap() + "\n")),
    );
    let warning = fixture.ok(&["next", "--json"])[KEY].clone();
    let warning = warning.as_str().unwrap_or_default();
    assert!(
        warning.contains("`specsync check --spec module3`"),
        "{warning}"
    );
    assert!(!warning.contains("module4`"), "{warning}");
    assert!(warning.contains("7 more"), "{warning}");
}
