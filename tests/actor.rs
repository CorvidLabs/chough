//! The acting agent on Timeline entries and evidence entries (REQ-records-007). Bugs
//! carry it too; their test is with the bugs (tests/bugs.rs).

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
    (newest["by"].clone(), stderr, next)
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
    // The entry carries it as its `by` field in session.md (REQ-history-002).
    let text = fixture.read(&fixture.session_file());
    let entry = timeline(&text).last().unwrap().clone();
    assert!(
        text.contains("  - by: SYNTHETIC-agent (SYNTHETIC-model, hc t901)\n"),
        "{entry}\n{text}"
    );
    let shown = fixture.run_with_env(&["next"], &[]);
    let shown = String::from_utf8_lossy(&shown.stdout).to_string();
    assert!(
        shown.contains("Last actor: SYNTHETIC-agent (SYNTHETIC-model, hc t901)"),
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
    assert!(!fixture.read(&fixture.session_file()).contains("- by:"));
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

/// The active session's evidence entry `id`, as text.
fn entry_text(fixture: &Fixture, id: &str) -> String {
    fixture.read(&format!(
        ".chough/sessions/{}/evidence/{id}.md",
        fixture.active_id()
    ))
}

/// TC-records-actor-recorded, the evidence part (REQ-records-007, #201): an evidence
/// entry and a case run record the acting agent as front matter `actor`. With the three
/// override variables they record that agent, model and terminal; with only `CLAUDECODE`
/// and `HC_TERMINAL_ID` they record `claude-code` and that terminal; with none, no
/// `actor`. A terminal holding a home path is left out with a warning and the command
/// succeeds, and `status` shows each listed entry's actor.
#[test]
fn the_evidence_entry_names_the_acting_agent() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"It prints ok\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\npaths = [\"src/input.txt\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"it prints ok\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo ok\"]\nexpect_output = [\"ok\"]\n",
    );
    fixture.init();
    fixture.commit("start");
    let add = |summary: &str, environment: &[(&str, &str)]| -> (String, String) {
        let output = fixture.run_with_env(
            &[
                "evidence",
                "add",
                "--criterion",
                "LIFE-1",
                "--requirement",
                "REQ-cli-001",
                "--contract",
                "specs/cli/cli.spec.md",
                "--kind",
                "product-test",
                "--outcome",
                "passed",
                "--environment",
                "local",
                "--summary",
                summary,
            ],
            environment,
        );
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        assert!(output.status.success(), "{stderr}");
        let added: Value = serde_json::from_slice(&output.stdout).unwrap();
        (added["observation"].as_str().unwrap().to_string(), stderr)
    };
    let overrides = [
        ("CHOUGH_ACTOR", "SYNTHETIC-agent"),
        ("CHOUGH_ACTOR_MODEL", "SYNTHETIC-model"),
        ("CHOUGH_HC_TERMINAL", "t999"),
        ("CLAUDECODE", "1"),
        ("HC_TERMINAL_ID", "t111"),
    ];
    let full =
        "\nactor:\n  - agent: SYNTHETIC-agent\n    model: SYNTHETIC-model\n    terminal: t999\n";
    let (overridden, _) = add("SYNTHETIC overridden actor", &overrides);
    let text = entry_text(&fixture, &overridden);
    assert!(text.contains(full), "{text}");

    // A case run records it too.
    let output = fixture.run_with_env(
        &["cases", "run", "TC-cli-0001", "--environment", "local"],
        &overrides,
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let ran: Value = serde_json::from_slice(&output.stdout).unwrap();
    let ran = ran["observation"].as_str().unwrap().to_string();
    let text = entry_text(&fixture, &ran);
    assert!(text.contains(full), "{text}");

    // Only Claude Code and hc: claude-code and that terminal.
    let (inferred, _) = add(
        "SYNTHETIC inferred actor",
        &[("CLAUDECODE", "1"), ("HC_TERMINAL_ID", "t111")],
    );
    let text = entry_text(&fixture, &inferred);
    assert!(
        text.contains("\nactor:\n  - agent: claude-code\n    terminal: t111\n"),
        "{text}"
    );

    // None set: no actor.
    let (unknown, _) = add("SYNTHETIC unknown actor", &[]);
    let text = entry_text(&fixture, &unknown);
    assert!(!text.contains("\nactor:"), "{text}");

    // A terminal holding a home path is left out with a warning; the add succeeds.
    let (pathy, stderr) = add(
        "SYNTHETIC home path terminal",
        &[
            ("CHOUGH_ACTOR", "SYNTHETIC-agent"),
            ("CHOUGH_HC_TERMINAL", "/Users/SYNTHETIC-user/terminal"),
        ],
    );
    let text = entry_text(&fixture, &pathy);
    assert!(!text.contains("/Users/SYNTHETIC-user"), "{text}");
    assert!(
        text.contains("\nactor:\n  - agent: SYNTHETIC-agent\n"),
        "{text}"
    );
    assert!(!stderr.is_empty(), "no warning for the left-out terminal");

    // Status shows each listed entry's actor.
    let status = fixture.ok(&["status", "--environment", "local"]);
    let actor_of = |id: &str| {
        status["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["record"]["id"] == id)
            .unwrap_or_else(|| panic!("{id}: {status:#}"))["record"]["actor"]
            .clone()
    };
    assert!(
        actor_of(&overridden)
            .to_string()
            .contains("SYNTHETIC-agent"),
        "{status:#}"
    );
    assert!(
        actor_of(&ran).to_string().contains("SYNTHETIC-agent"),
        "{status:#}"
    );
    assert!(actor_of(&unknown).is_null(), "{status:#}");
}
