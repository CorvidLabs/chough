//! The acting agent on Timeline entries (REQ-records-007). Evidence entries and bugs
//! carry it too; their tests are with evidence and bugs.

mod common;

use common::{Fixture, timeline};
use serde_json::{Value, json};

/// Record one Timeline entry with `environment` and return the newest entry's actor,
/// the command's standard error and `chough next --json`.
fn iterate_with(fixture: &Fixture, environment: &[(&str, &str)]) -> (Value, String, Value) {
    let output = fixture.run_with_env(&["stage", "iterate", "--note", "round"], environment);
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(output.status.success(), "{stderr}");
    let shown = fixture.ok(&["history", "show"]);
    let newest = shown["entries"].as_array().unwrap().last().unwrap().clone();
    let next = fixture.run_with_env(&["next", "--json"], environment);
    assert!(next.status.success());
    let next: Value = serde_json::from_slice(&next.stdout).unwrap();
    (newest["actor"].clone(), stderr, next)
}

#[test]
fn the_override_variables_name_the_agent_model_and_terminal() {
    let fixture = Fixture::new();
    fixture.init();
    let (actor, _, next) = iterate_with(
        &fixture,
        &[
            ("CHOUGH_ACTOR", "SYNTHETIC-agent"),
            ("CHOUGH_ACTOR_MODEL", "SYNTHETIC-model"),
            ("CHOUGH_HC_TERMINAL", "t901"),
            ("CLAUDECODE", "1"),
            ("HC_TERMINAL_ID", "t902"),
        ],
    );
    let expected =
        json!({"agent": "SYNTHETIC-agent", "model": "SYNTHETIC-model", "terminal": "t901"});
    assert_eq!(actor, expected);
    assert_eq!(next["last_actor"], expected);
    // The entry carries it as a one-line field in session.md.
    let text = fixture.read(&fixture.session_file());
    let entry = timeline(&text).last().unwrap().clone();
    assert!(
        text.contains(&format!("  - actor: {expected}\n")),
        "{entry}\n{text}"
    );
    let shown = fixture.run_with_env(&["next"], &[]);
    let shown = String::from_utf8_lossy(&shown.stdout).to_string();
    assert!(
        shown.contains("Last actor: SYNTHETIC-agent (SYNTHETIC-model) in t901"),
        "{shown}"
    );
    let status = fixture.ok(&["status"]);
    assert_eq!(status["last_actor"], expected);
}

#[test]
fn claude_code_and_hc_fill_in_without_overrides() {
    let fixture = Fixture::new();
    fixture.init();
    let (actor, _, _) = iterate_with(&fixture, &[("CLAUDECODE", "1"), ("HC_TERMINAL_ID", "t903")]);
    assert_eq!(actor, json!({"agent": "claude-code", "terminal": "t903"}));
}

#[test]
fn with_nothing_set_no_entry_has_an_actor() {
    let fixture = Fixture::new();
    fixture.init();
    let (actor, stderr, next) = iterate_with(&fixture, &[]);
    assert_eq!(actor, Value::Null);
    assert!(!stderr.contains("actor"), "{stderr}");
    assert_eq!(next["last_actor"], Value::Null);
    assert!(!fixture.read(&fixture.session_file()).contains("- actor:"));
    let shown = String::from_utf8_lossy(&fixture.run(&["next"]).stdout).to_string();
    assert!(!shown.contains("Last actor"), "{shown}");
}

#[test]
fn a_terminal_holding_a_home_path_is_left_out_with_a_warning() {
    let fixture = Fixture::new();
    fixture.init();
    let (actor, stderr, _) = iterate_with(
        &fixture,
        &[
            ("CLAUDECODE", "1"),
            ("HC_TERMINAL_ID", "/Users/SYNTHETIC/t1"),
        ],
    );
    assert_eq!(actor, json!({"agent": "claude-code"}));
    assert!(
        stderr.contains("warning: the actor's terminal holds home-path, so it is left out"),
        "{stderr}"
    );
    assert!(
        !fixture
            .read(&fixture.session_file())
            .contains("SYNTHETIC/t1")
    );
}
