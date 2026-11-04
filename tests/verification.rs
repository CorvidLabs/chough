//! Saying when native verification runs none of the project's commands, and pointing
//! the test stage to recorded test evidence before accept (REQ-cli-038, FLOW-15, #85).

mod common;

use common::Fixture;
use serde_json::Value;

const NO_COMMANDS: &str = "{\"enabled\": true}\n";
const WITH_COMMANDS: &str = "{\"enabled\": true, \"verification_commands\": [\"cargo test\"]}\n";
const DISABLED: &str = "{\"enabled\": false}\n";
const WARNING_KEY: &str = "native_verification_warning";

/// A project at the test stage of an approved native change, with `policy` as its
/// `.specsync/sdd.json`, or with no native configuration at all when `policy` is None.
#[cfg(unix)]
fn at_test(policy: Option<&str>) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    if let Some(policy) = policy {
        fixture.native("approved");
        fixture.write(".specsync/sdd.json", policy);
    }
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "verify-change"]);
    fixture.no_spec_approval("verify-change");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(
        fixture.ok(&["stage", "show"])["stage"]["current"],
        "test",
        "the fixture must reach the test stage"
    );
    fixture
}

fn text(fixture: &Fixture, args: &[&str]) -> String {
    let output = fixture.run(args);
    assert!(
        output.status.success(),
        "{args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// The warning each report carries: `next --json`, `status` and `stage show`.
fn warnings(fixture: &Fixture) -> [Value; 3] {
    let next = fixture.ok(&["next", "--json"]);
    let status = fixture.ok(&["status"]);
    let stage = fixture.ok(&["stage", "show"]);
    for (name, report) in [("next --json", &next), ("status", &status), ("stage show", &stage)] {
        assert!(
            report.get(WARNING_KEY).is_some(),
            "{name} has no {WARNING_KEY} key: {report:#}"
        );
    }
    [
        next[WARNING_KEY].clone(),
        status[WARNING_KEY].clone(),
        stage[WARNING_KEY].clone(),
    ]
}

#[cfg(unix)]
#[test]
fn a_project_whose_native_verification_runs_no_commands_is_told_so() {
    let fixture = at_test(Some(NO_COMMANDS));
    for warning in warnings(&fixture) {
        let warning = warning.as_str().unwrap_or_default();
        assert!(
            warning.contains("runs none of the project's commands")
                && warning.contains("verification_commands")
                && warning.contains(".specsync/sdd.json"),
            "{warning:?}"
        );
    }
    let next = text(&fixture, &["next"]);
    assert!(
        next.lines().any(|line| line.starts_with("Warning:")
            && line.contains("runs none of the project's commands")),
        "{next}"
    );
}

#[cfg(unix)]
#[test]
fn no_warning_with_verification_commands_disabled_sdd_or_no_native_configuration() {
    for (why, policy) in [
        ("with a verification command", Some(WITH_COMMANDS)),
        ("with SDD disabled", Some(DISABLED)),
        ("with no native configuration", None),
    ] {
        let fixture = at_test(policy);
        for warning in warnings(&fixture) {
            assert_eq!(warning, Value::Null, "{why}");
        }
        let next = text(&fixture, &["next"]);
        assert!(
            !next.contains("runs none of the project's commands"),
            "{why}: {next}"
        );
    }
}

#[cfg(unix)]
#[test]
fn the_test_stage_points_to_recorded_test_evidence_before_accept() {
    for policy in [NO_COMMANDS, WITH_COMMANDS] {
        let fixture = at_test(Some(policy));
        let next = fixture.ok(&["next", "--json"]);
        let action = &next["next_valid_action"];
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
                .contains("recorded test evidence before accept"),
            "{action:#}"
        );
    }
}

/// The warning is advice: the same move is allowed with and without it, it adds no
/// native read, and the stage report that the move prints carries it.
#[cfg(unix)]
#[test]
fn the_warning_changes_no_gate_and_adds_no_native_read() {
    let mut calls = vec![];
    for policy in [NO_COMMANDS, WITH_COMMANDS] {
        let fixture = at_test(Some(policy));
        fixture.write(".stub-count", "");
        fixture.native_calls();
        fixture.ok(&["next", "--json"]);
        let next_calls = fixture.native_calls();
        let moved = fixture.ok(&["stage", "advance"]);
        let advance_calls = fixture.native_calls();
        assert_eq!(moved["applied"]["to"], "accept", "{moved:#}");
        assert_eq!(
            moved[WARNING_KEY].is_string(),
            policy == NO_COMMANDS,
            "{moved:#}"
        );
        calls.push((next_calls, advance_calls));
    }
    assert_eq!(calls[0], calls[1], "native reads with and without the warning");
}
