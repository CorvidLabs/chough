//! Checkpoint schema 2, explicit migration and stage routing (REQ-cli-007, REQ-cli-008).

mod common;

use common::Fixture;
use serde_json::{Value, json};
use std::fs;

fn transitions(fixture: &Fixture) -> Vec<Value> {
    fixture
        .read(".chough/transitions.jsonl")
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

fn stage(fixture: &Fixture) -> String {
    fixture.session()["stage"]["current"]
        .as_str()
        .unwrap()
        .to_string()
}

#[test]
fn init_writes_a_schema_2_checkpoint_at_intent() {
    let fixture = Fixture::new();
    fixture.init();
    let session = fixture.session();
    assert_eq!(session["schema_version"], 2);
    assert_eq!(session["stage"]["current"], "intent");
    assert_eq!(session["stage"]["status"], "active");
    let shown = fixture.ok(&["stage", "show"]);
    assert!(
        shown["valid_transitions"][0]
            .as_str()
            .unwrap()
            .contains("intent -> design")
    );
}

#[test]
fn every_stage_is_documented_in_the_shared_stage_contract() {
    let fixture = Fixture::new();
    fixture.init();
    let reference = fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("skills/chough-workflow/references/stages.md"),
    )
    .unwrap();
    let stages = fixture.ok(&["stage", "show"])["stages"].clone();
    assert_eq!(stages.as_array().unwrap().len(), 11);
    for stage in stages.as_array().unwrap() {
        let heading = format!("\n## {}\n", stage["name"].as_str().unwrap());
        assert!(reference.contains(&heading), "missing {heading:?}");
    }
}

#[test]
fn migration_is_explicit_and_preserves_schema_1_bytes() {
    let fixture = Fixture::new();
    let original = "{\n  \"schema_version\": 1,\n  \"objective\": \"Keep my old session\",\n  \"intent_scope\": \"LIFE\",\n  \"modules\": [\"cli\"],\n  \"decisions\": [\"Decided A because B\"],\n  \"open_questions\": [\"Q1\"],\n  \"next_action\": \"Do the next thing\"\n}\n";
    fixture.write(".chough/session.json", original);
    // Schema 1 remains readable for context and status, but not for stage changes.
    assert_eq!(
        fixture.ok(&["context", "--json"])["session"]["schema_version"],
        1
    );
    assert!(
        fixture.ok(&["status", "--json"])["checkpoint"]["migration"]
            .as_str()
            .unwrap()
            .contains("chough migrate")
    );
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("chough migrate")
    );
    assert_eq!(fixture.read(".chough/session.json"), original);

    let migrated = fixture.ok(&["migrate", "--stage", "plan"]);
    assert_eq!(migrated["migrated"], true);
    assert_eq!(fixture.read(".chough/session.v1.json"), original);
    let session = fixture.session();
    assert_eq!(session["schema_version"], 2);
    assert_eq!(session["stage"]["current"], "plan");
    assert_eq!(session["decisions"], json!(["Decided A because B"]));
    assert_eq!(session["open_questions"], json!(["Q1"]));
    assert_eq!(session["next_action"], "Do the next thing");
    assert_eq!(fixture.ok(&["migrate"])["migrated"], false);

    // A second schema 1 file never overwrites the preserved backup.
    fixture.write(".chough/session.json", original);
    assert!(fixture.fails(&["migrate"]).contains("already exists"));
    assert_eq!(fixture.read(".chough/session.json"), original);
}

#[test]
fn checkpoint_round_trips_across_processes_and_rejects_unknown_fields() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "Use TOML because lists are first-class",
        "--question",
        "Which viewport matters most?",
        "--criterion",
        "LIFE-3",
        "--requirement",
        "REQ-cli-002",
        "--design",
        "design/prototype/index.html",
        "--awaiting-human",
        "Scope approval of add-thing",
        "--next-action",
        "Wait for approval",
    ]);
    let resumed = fixture.ok(&["context", "--json"])["session"].clone();
    assert_eq!(resumed["references"]["criteria"], json!(["LIFE-3"]));
    assert_eq!(
        resumed["awaiting_human"],
        json!(["Scope approval of add-thing"])
    );
    assert_eq!(resumed["next_action"], "Wait for approval");
    let next = fixture.ok(&["next", "--json"]);
    assert!(
        next["next_valid_action"]
            .as_str()
            .unwrap()
            .contains("Waiting for a human decision")
    );
    fixture.ok(&[
        "checkpoint",
        "--resolve-question",
        "Which viewport matters most?",
        "--clear-awaiting-human",
    ]);
    assert_eq!(fixture.session()["open_questions"], json!([]));
    assert!(
        fixture
            .fails(&["checkpoint", "--design", "../outside.html"])
            .contains("traversal")
    );

    let mut session = fixture.session();
    session["hidden_reasoning"] = json!("must not be accepted");
    fixture.write(".chough/session.json", &session.to_string());
    assert!(fixture.fails(&["context", "--json"]).contains("invalid"));
}

#[test]
fn transitions_cover_every_kind_and_keep_history() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["stage", "advance", "--note", "Intent approved by the human"]);
    assert_eq!(stage(&fixture), "design");
    assert!(
        fixture
            .fails(&["stage", "skip", "--reason", "   "])
            .contains("reason")
    );
    fixture.ok(&[
        "stage",
        "skip",
        "--reason",
        "CLI-only change; nothing visual",
    ]);
    assert_eq!(stage(&fixture), "plan");
    let iterated = fixture.ok(&["stage", "iterate", "--note", "Second estimate"]);
    assert_eq!(iterated["applied"]["iteration"], 2);
    let built = fixture.ok(&["stage", "advance"]);
    assert_eq!(built["applied"]["gate"], "ungated:not-configured");
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "test");

    let back = fixture.ok(&[
        "stage",
        "back",
        "--to",
        "plan",
        "--reason",
        "Test exposed a missing requirement",
    ]);
    let refresh = back["applied"]["refresh"].to_string();
    for word in ["plan:", "build:", "test:"] {
        assert!(refresh.contains(word), "{refresh}");
    }
    assert!(
        fixture
            .fails(&["stage", "back", "--to", "test", "--reason", "x"])
            .contains("earlier")
    );

    fixture.ok(&["stage", "pause", "--reason", "Waiting on the human"]);
    assert!(fixture.fails(&["stage", "advance"]).contains("paused"));
    assert_eq!(
        fixture.ok(&["stage", "show"])["valid_transitions"],
        json!(["chough stage resume"])
    );
    fixture.ok(&["stage", "resume"]);

    fixture.ok(&[
        "stage",
        "enter",
        "librarian",
        "--reason",
        "Record a decision",
    ]);
    assert_eq!(stage(&fixture), "librarian");
    assert_eq!(
        fixture.session()["stage"]["callers"][0]["stage"],
        json!("plan")
    );
    assert!(fixture.fails(&["stage", "advance"]).contains("return"));
    fixture.ok(&["stage", "enter", "bugs", "--reason", "Nested capture"]);
    fixture.ok(&["stage", "return"]);
    fixture.ok(&["stage", "return", "--note", "Decision recorded"]);
    assert_eq!(stage(&fixture), "plan");

    fixture.ok(&["stage", "reset", "--reason", "The human changed direction"]);
    assert_eq!(stage(&fixture), "intent");

    let kinds: Vec<String> = transitions(&fixture)
        .iter()
        .map(|transition| transition["kind"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(
        kinds,
        [
            "advance", "skip", "iterate", "advance", "advance", "back", "pause", "resume", "enter",
            "enter", "return", "return", "reset"
        ]
    );
    assert_eq!(
        transitions(&fixture)[1]["reason"],
        "CLI-only change; nothing visual"
    );
}

#[test]
fn optional_side_stages_and_release_need_authorization() {
    let fixture = Fixture::new();
    fixture.init();
    assert!(
        fixture
            .fails(&["stage", "enter", "validate", "--reason", "Repeat the check"])
            .contains("consent")
    );
    fixture.ok(&[
        "consent",
        "grant",
        "repeat-validation",
        "--source",
        "Human: yes, repeat it this time",
    ]);
    assert!(!fixture.exists(".chough/preferences.json"));
    fixture.ok(&["stage", "enter", "validate", "--reason", "Repeat the check"]);
    fixture.ok(&["stage", "return"]);

    for _ in 0..6 {
        fixture.ok(&["stage", "advance"]);
    }
    assert_eq!(stage(&fixture), "done");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("release is entered only")
    );
    assert!(
        fixture
            .fails(&["stage", "enter", "release", "--reason", "Ship it"])
            .contains("not authorized")
    );
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "Human: please publish 0.2.0",
    ]);
    let entered = fixture.ok(&["stage", "enter", "release", "--reason", "Requested"]);
    assert!(
        entered["applied"]["authority"]
            .as_str()
            .unwrap()
            .starts_with("run consent")
    );
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "done");

    // A new objective ends run consents; standing preferences were never written.
    fixture.ok(&["checkpoint", "--objective", "Next piece of work"]);
    assert_eq!(fixture.session()["run_consents"], json!([]));
    assert!(!fixture.exists(".chough/preferences.json"));
}

#[cfg(unix)]
#[test]
fn native_state_gates_build_and_closing_without_inventing_it() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:no-native-change")
    );
    fixture.ok(&["checkpoint", "--native-change", "add-thing"]);
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("refused:native-draft"), "{refused}");
    assert!(refused.contains("specsync change status add-thing"));
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["checkpoint"]["gate"]["allowed"], false);
    assert!(
        next["next_valid_action"]
            .as_str()
            .unwrap()
            .contains("Gate not open")
    );

    fixture.write(".stub-fail", "");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-unknown")
    );
    fs::remove_file(fixture.root().join(".stub-fail")).unwrap();

    fixture.write(".stub-state", "approved\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
    fixture.write(".stub-state", "verifying\n");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "accept");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-verifying")
    );
    let back = fixture.ok(&["stage", "back", "--to", "plan", "--reason", "Scope gap"]);
    assert!(
        back["applied"]["refresh"]
            .to_string()
            .contains("needs native reapproval")
    );
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.write(".stub-state", "archived\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:archived"
    );
    assert_eq!(stage(&fixture), "done");
}
