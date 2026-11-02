//! Checkpoint schema 2, explicit migration and stage routing (REQ-cli-007, REQ-cli-008).

mod common;

use common::Fixture;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

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
    .unwrap()
    // Windows checkouts may convert line endings; headings are compared as lines.
    .replace("\r\n", "\n");
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
        next["next_valid_action"]["action"]
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
    fixture.no_spec_approval("add-thing");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("refused:native-draft"), "{refused}");
    assert!(refused.contains("specsync change status add-thing"));
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["checkpoint"]["gate"]["allowed"], false);
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["gate"]["allowed"], false);
    assert!(
        next["next_valid_action"]["action"]
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

#[cfg(unix)]
#[test]
fn done_archived_change_recommends_new_cycle_and_preserves_precedence() {
    const NEW_CYCLE: &str = "Start a new cycle: chough stage reset --to intent --reason \"<why>\" (or --to plan when the approved intent already covers the new work), then specsync change new \"<outcome>\" --spec <module> --path <path>, then chough checkpoint --native-change <id> --next-action \"<next step>\".";
    const RESET: &str = "chough stage reset --to intent --reason \"<why>\"";

    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("archived");
    fixture.init();
    let stale = "specsync change review archived-change, then ship";
    fixture.ok(&["checkpoint", "--native-change", "archived-change"]);
    fixture.ok(&[
        "stage",
        "reset",
        "--to",
        "done",
        "--reason",
        "Archived native closeout",
    ]);
    fixture.ok(&["checkpoint", "--next-action", stale]);
    assert_eq!(stage(&fixture), "done");

    let assert_action = |command: &str, expected: &str, session_bytes: &str| {
        let mut report = fixture.ok(&[command, "--json"]);
        if command == "next" {
            report["native_summary"] = json!({
                "state": report["native"]["state"],
                "next_action": report["native"]["next_action"],
            });
            report["next_valid_reason"] = report["next_valid_action"]["reason"].clone();
            report["next_valid_command"] = report["next_valid_action"]["command"].clone();
            report["next_valid_action"] = report["next_valid_action"]["action"].clone();
        } else {
            assert_eq!(report["session_next_action"], stale, "{command}");
        }
        assert_eq!(report["next_valid_action"], expected, "{command}");
        assert_eq!(
            fixture.read(".chough/session.json"),
            session_bytes,
            "{command} rewrote the session"
        );
        report
    };

    fixture.write(".stub-count", "");
    let session_bytes = fixture.read(".chough/session.json");
    for command in ["next", "status"] {
        let report = assert_action(command, NEW_CYCLE, &session_bytes);
        assert_eq!(fixture.native_calls(), 1, "{command} native reads");
        assert_eq!(report["native_summary"]["state"], "archived");
        assert_eq!(
            report["native_summary"]["next_action"],
            "stub next action for archived"
        );
        let action = report["next_valid_action"].as_str().unwrap();
        assert!(!action.contains(stale), "{command}: {action}");
        if command == "next" {
            assert_eq!(report["next_valid_command"], RESET);
            let reason = report["next_valid_reason"].as_str().unwrap();
            assert!(
                reason.contains("The cycle for archived-change is closed: native state archived, terminal evidence authenticated-history."),
                "{reason}"
            );
            assert!(
                reason
                    .contains("Native next action, check it first: stub next action for archived."),
                "{reason}"
            );
        }
    }

    fixture.ok(&["stage", "pause", "--reason", "Waiting for a person"]);
    fixture.ok(&["checkpoint", "--awaiting-human", "Human scope decision"]);
    let session_bytes = fixture.read(".chough/session.json");
    for command in ["next", "status"] {
        assert_action(
            command,
            "The done stage is paused. Check native state and freshness, then `chough stage resume`.",
            &session_bytes,
        );
    }

    fixture.ok(&["stage", "resume"]);
    let session_bytes = fixture.read(".chough/session.json");
    for command in ["next", "status"] {
        assert_action(
            command,
            "Waiting for a human decision: Human scope decision. Do not record it on their behalf; continue only independent work within scope.",
            &session_bytes,
        );
    }

    fixture.ok(&["checkpoint", "--clear-awaiting-human"]);
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nprintf '%s\\n' '{\"summary\":{\"state\":\"archived\",\"terminal_evidence\":{\"validity\":\"authenticated-history\"}}}'\n",
    );
    let session_bytes = fixture.read(".chough/session.json");
    for command in ["next", "status"] {
        let report = assert_action(command, NEW_CYCLE, &session_bytes);
        assert_eq!(report["native_summary"]["state"], "archived");
        assert!(report["native_summary"]["next_action"].is_null());
    }

    for (label, next_action) in [
        ("null", Value::Null),
        ("whitespace-only", json!(" \t\n ")),
        ("non-string number", json!(42)),
        ("non-string boolean", json!(false)),
        ("non-string object", json!({"action": "nested"})),
        ("non-string array", json!(["nested"])),
    ] {
        let response = json!({
            "summary": {
                "state": "archived",
                "next_action": next_action.clone(),
                "terminal_evidence": {"validity": "authenticated-history"},
            }
        });
        fixture.write(
            "bin/specsync",
            &format!("#!/bin/sh\nprintf '%s\\n' '{response}'\n"),
        );
        for command in ["next", "status"] {
            let report = assert_action(command, NEW_CYCLE, &session_bytes);
            assert_eq!(report["native_summary"]["state"], "archived");
            let shown_action = if command == "next" && !next_action.is_string() {
                Value::Null
            } else {
                next_action.clone()
            };
            assert_eq!(
                report["native_summary"]["next_action"], shown_action,
                "{command}: {label}"
            );
        }
    }

    let native_action = " \tstub next action for archived \t";
    let response = json!({
        "summary": {
            "state": "archived",
            "next_action": native_action,
            "terminal_evidence": {"validity": "authenticated-history"},
        }
    });
    fixture.write(
        "bin/specsync",
        &format!("#!/bin/sh\nprintf '%s\\n' '{response}'\n"),
    );
    for command in ["next", "status"] {
        let report = assert_action(command, NEW_CYCLE, &session_bytes);
        assert_eq!(report["native_summary"]["state"], "archived");
        assert_eq!(report["native_summary"]["next_action"], native_action);
    }

    fixture.stubs();
    fixture.write(".stub-fail", "");
    for command in ["next", "status"] {
        let report = fixture.ok(&[command, "--json"]);
        let action = if command == "next" {
            report["next_valid_action"]["action"].clone()
        } else {
            assert_eq!(report["session_next_action"], stale, "{command}");
            report["next_valid_action"].clone()
        };
        let action = action.as_str().unwrap();
        assert!(
            action.starts_with("Gate not open (refused:native-unknown):"),
            "{command}: {action}"
        );
        assert_eq!(
            fixture.read(".chough/session.json"),
            session_bytes,
            "{command} rewrote the session"
        );
        if command == "next" {
            assert_eq!(report["native"]["read"], "failed");
            assert!(report["native"]["state"].is_null());
        } else {
            assert_eq!(report["native_specsync"]["state"], "failed");
            assert!(
                report["native_specsync"]["content"]
                    .as_str()
                    .unwrap()
                    .contains("status failed"),
                "{command}: {}",
                report["native_specsync"]["content"]
            );
            assert!(report["native_summary"].is_null());
        }
    }

    fs::remove_file(fixture.root().join(".stub-fail")).unwrap();
    let next = fixture.ok(&["next", "--json"]);
    let command = next["next_valid_action"]["command"].as_str().unwrap();
    assert_eq!(command, RESET);
    let arguments = words(&command.replace("<why>", "Start approved follow-up"));
    assert_eq!(arguments[0], "chough");
    let arguments: Vec<&str> = arguments[1..].iter().map(String::as_str).collect();
    fixture.ok(&arguments);
    assert_eq!(
        fixture.session()["next_action"],
        "New cycle from done after reset: Start approved follow-up. Create or record the native change (specsync change new, then chough checkpoint --native-change <id>) and set the next action."
    );
    assert_eq!(
        transitions(&fixture).pop().unwrap()["reason"],
        "Start approved follow-up"
    );
}

#[cfg(unix)]
#[test]
fn large_native_status_still_gates_by_its_summary_state() {
    let fixture = Fixture::new();
    fixture.stubs();
    // An archived SpecSync change lists every delivery file, so real status output
    // exceeds the 16 KiB display bound; over 4 MiB it is refused as oversized.
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nstate=$(cat .stub-state 2>/dev/null || echo draft)\nsize=20000\n[ -f .stub-huge ] && size=4200000\npad=$(head -c $size /dev/zero | tr '\\0' x)\nprintf '{\"summary\":{\"state\":\"%s\",\"next_action\":\"stub next action\",\"approval_valid\":true,\"terminal_evidence\":{\"validity\":\"authenticated-history\"}},\"change\":{\"affected_specs\":[],\"no_spec_change\":true},\"acceptance_entries\":\"%s\"}\\n' \"$state\" \"$pad\"\n",
    );
    fixture.executable("bin/specsync");
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "large-change"]);
    fixture.no_spec_approval("large-change");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.write(".stub-huge", "");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("refused:native-unknown"), "{refused}");
    assert!(refused.contains("oversized"), "{refused}");
    fs::remove_file(fixture.root().join(".stub-huge")).unwrap();
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "accept");
    fixture.write(".stub-state", "archived\n");
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["native_specsync"]["state"], "observed");
    assert_eq!(status["native_summary"]["state"], "archived");
    let shown = status["native_specsync"]["content"].as_str().unwrap();
    assert!(shown.contains("display only"), "{shown}");
    assert!(shown.len() < 17_000);
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:archived"
    );
    assert_eq!(stage(&fixture), "done");
}

#[test]
fn clear_flags_apply_before_new_entries() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Old gate",
        "--blocker",
        "Old blocker",
    ]);
    let updated = fixture.ok(&[
        "checkpoint",
        "--clear-awaiting-human",
        "--awaiting-human",
        "New gate",
        "--clear-blockers",
        "--blocker",
        "New blocker",
    ]);
    assert_eq!(updated["checkpoint"]["awaiting_human"], json!(["New gate"]));
    assert_eq!(updated["checkpoint"]["blockers"], json!(["New blocker"]));
    assert_eq!(fixture.session()["awaiting_human"], json!(["New gate"]));
}

#[cfg(unix)]
#[test]
fn invalid_native_approval_refuses_every_route_into_build_and_later_stages() {
    // BUG-0006: an eligible state name is not enough; approval_valid must be true.
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "edited-scope"]);
    fixture.no_spec_approval("edited-scope");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.write(".stub-approval", "false\n");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(
        refused.contains("refused:native-approval-invalid"),
        "{refused}"
    );
    assert!(refused.contains("approval_valid false"), "{refused}");
    assert!(
        refused.contains("stub next action for approved"),
        "{refused}"
    );
    let routes: [&[&str]; 4] = [
        &["stage", "skip", "--reason", "Skip ahead"],
        &["stage", "reset", "--to", "build", "--reason", "Restart"],
        &["stage", "reset", "--to", "test", "--reason", "Retest"],
        &["stage", "reset", "--to", "accept", "--reason", "Accept"],
    ];
    for route in routes {
        let refused = fixture.fails(route);
        assert!(
            refused.contains("refused:native-approval-invalid"),
            "{route:?}: {refused}"
        );
    }
    assert_eq!(stage(&fixture), "plan");
    fixture.write(".stub-approval", "none\n");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("approval_valid missing")
    );
    fixture.write(".stub-approval", "true\n");
    fixture.write(".stub-correction", "false\n");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-correction-invalid")
    );
    fixture.write(".stub-correction", "true\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
    // An approval that becomes invalid during build stops the move to test.
    fixture.write(".stub-state", "implementing\n");
    fixture.write(".stub-approval", "false\n");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-approval-invalid")
    );
    assert_eq!(stage(&fixture), "build");
}

#[cfg(unix)]
#[test]
fn reset_back_and_release_meet_their_destination_prerequisites() {
    // BUG-0007: reset, back and release reach a stage only when its prerequisite holds.
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "draft-change"]);
    for target in ["done", "accept", "test", "build"] {
        let refused = fixture.fails(&["stage", "reset", "--to", target, "--reason", "Jump"]);
        assert!(
            refused.contains("refused:native-draft"),
            "{target}: {refused}"
        );
    }
    assert_eq!(stage(&fixture), "intent");
    for target in ["plan", "design", "intent"] {
        fixture.ok(&["stage", "reset", "--to", target, "--reason", "Rework"]);
    }
    fixture.write(".stub-state", "archived\n");
    for target in ["test", "accept", "done"] {
        let moved = fixture.ok(&["stage", "reset", "--to", target, "--reason", "Closeout"]);
        assert_eq!(moved["applied"]["to"], target);
    }
    assert!(
        fixture
            .fails(&["stage", "reset", "--to", "build", "--reason", "More work"])
            .contains("refused:native-archived")
    );
    fixture.write(".stub-validity", "corrupt-history\n");
    assert!(
        fixture
            .fails(&["stage", "back", "--to", "accept", "--reason", "Recheck"])
            .contains("refused:native-archive-corrupt-history")
    );
    fixture.ok(&["stage", "reset", "--to", "plan", "--reason", "Again"]);
    fixture.write(".stub-validity", "none\n");
    assert!(
        fixture
            .fails(&["stage", "reset", "--to", "done", "--reason", "Close"])
            .contains("refused:native-archive-unreported")
    );
    fixture.write(".stub-validity", "authenticated-history\n");
    fixture.ok(&["stage", "reset", "--to", "done", "--reason", "Close"]);
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "SYNTHETIC test consent; authorizes nothing real",
    ]);
    fixture.ok(&["stage", "enter", "release", "--reason", "Release routing"]);
    fixture.write(".stub-validity", "corrupt-history\n");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-archive-corrupt-history")
    );
    fixture.write(".stub-validity", "authenticated-history\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:archived"
    );
}

#[cfg(unix)]
#[test]
fn return_from_a_side_stage_meets_the_caller_prerequisite() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "side-trip"]);
    fixture.no_spec_approval("side-trip");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "enter", "bugs", "--reason", "A failing case"]);
    fixture.write(".stub-approval", "false\n");
    let refused = fixture.fails(&["stage", "return"]);
    assert!(
        refused.contains("refused:native-approval-invalid"),
        "{refused}"
    );
    // Every reset Chough suggests carries the reason clap requires.
    assert!(
        refused.contains("chough stage reset --to <stage> --reason"),
        "{refused}"
    );
    assert_eq!(stage(&fixture), "bugs");
    fixture.write(".stub-approval", "true\n");
    assert_eq!(fixture.ok(&["stage", "return"])["applied"]["to"], "test");
}

#[cfg(unix)]
#[test]
fn sdd_policy_is_read_with_specsync_semantics() {
    // BUG-0008: an invalid policy refuses; only a valid disabled or absent one is ungated.
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "policy-change"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    for (policy, why) in [
        ("{ \"enabled\": true, ", "truncated"),
        ("{\"enabled\": \"yes\"}", "enabled as a string"),
        ("{\"enabled\": null}", "enabled as null"),
        ("{\"meaningful_paths\": \"src/\"}", "paths as a string"),
        ("{\"enabled\": true, \"enabled\": false}", "duplicate key"),
        ("true", "not an object"),
    ] {
        fixture.write(".specsync/sdd.json", policy);
        let refused = fixture.fails(&["stage", "advance"]);
        assert!(
            refused.contains("refused:sdd-policy-invalid"),
            "{why}: {refused}"
        );
        assert!(refused.contains(".specsync/sdd.json"), "{why}: {refused}");
    }
    // As in SpecSync, a missing `enabled` means enabled, and unknown fields are accepted.
    fixture.write(
        ".specsync/sdd.json",
        "{\"version\": 2, \"future_field\": 1}\n",
    );
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-draft")
    );
    fixture.write(".specsync/sdd.json", "{\"enabled\": false}\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "ungated:sdd-disabled"
    );
    fixture.ok(&["stage", "back", "--to", "plan", "--reason", "Again"]);
    fs::remove_file(fixture.root().join(".specsync/sdd.json")).unwrap();
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "ungated:sdd-disabled"
    );
}

/// Run a shell script in the fixture with a local Git identity, as native tools commit.
#[cfg(unix)]
fn sh(fixture: &Fixture, script: &str) -> String {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .current_dir(fixture.root())
        .env("CHOUGH", env!("CARGO_BIN_EXE_chough"))
        .env(
            "SAMPLE",
            concat!(env!("CARGO_MANIFEST_DIR"), "/examples/shipping"),
        )
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "script failed: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[cfg(unix)]
#[test]
fn real_specsync_approval_reported_invalid_refuses_build() {
    // BUG-0006 against real SpecSync. The approver is SYNTHETIC and approves nothing real.
    let available = std::process::Command::new("specsync")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success());
    if !available {
        eprintln!("skipped: real SpecSync is not on PATH (the Fledge CI job runs this test)");
        return;
    }
    let fixture = Fixture::new();
    sh(
        &fixture,
        r##"set -eu
rm -rf ./* .chough .specsync
cp -R "$SAMPLE/." .
rm -f .specsync/changes/.keep .specsync/archive/changes/.keep
git -c init.defaultBranch=main init -q
git config user.name "Chough test"; git config user.email test@example.invalid; git config commit.gpgsign false
"$CHOUGH" init --objective "Synthetic approval validity regression" --intent-scope CHECKOUT --module shipping >/dev/null
specsync change adopt --json >/dev/null
git add -A; git commit -qm seed
id=free-shipping-at-the-threshold
specsync change new "Free shipping at the threshold" --spec shipping --path src/lib.rs --path tests/ --json >/dev/null
specsync change answer $id acceptance_criteria '["An order of exactly 5000 cents ships free"]' --json >/dev/null
specsync change answer $id public_contract yes --json >/dev/null
specsync change answer $id architecture_risk no --json >/dev/null
specsync change answer $id constraints none --json >/dev/null 2>&1 || true
for f in .specsync/changes/$id/*.md; do
  name=$(basename "$f" .md)
  [ "$name" = change ] && continue
  printf -- '---\nchange: %s\nartifact: %s\n---\n\n# %s\n\n' "$id" "$name" "$name" >"$f"
  case "$name" in
    tasks) printf -- '- [x] Add the threshold rule.\n' >>"$f" ;;
    plan) printf '## Approach\n\nAdd the rule.\n\n## Out of scope\n\nNone.\n\n## Steps\n\n1. Add it.\n\n## Risks\n\nOff by one.\n\n## Constraints consulted\n\nNone.\n' >>"$f" ;;
    *) printf 'Orders of 5000 cents or more ship free.\n' >>"$f" ;;
  esac
done
mkdir -p .specsync/changes/$id/deltas
printf '## ADDED\n\n### REQUIREMENT REQ-shipping-002\n\nThe module SHALL charge no fee at or above the threshold. Trace: CHECKOUT-1.\n\nAcceptance Criteria\n- tests/threshold.rs asserts a zero fee at the threshold.\n' >.specsync/changes/$id/deltas/shipping.md
git add -A; git commit -qm draft
specsync change approve $id --actor "SYNTHETIC-test-approver-not-a-human" >/dev/null
"$CHOUGH" checkpoint --native-change $id >/dev/null
"$CHOUGH" stage advance >/dev/null
"$CHOUGH" stage skip --reason "No visuals" >/dev/null
cp .specsync/changes/$id/state.json state.approved
sed 's#"tests/"#"tests/", "docs/"#' state.approved >.specsync/changes/$id/state.json
specsync change status $id --json
"##,
    )
    .contains("\"approval_valid\": false")
    .then_some(())
    .expect("real SpecSync should report the changed scope as an invalid approval");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(
        refused.contains("refused:native-approval-invalid"),
        "{refused}"
    );
    sh(
        &fixture,
        "cp state.approved .specsync/changes/free-shipping-at-the-threshold/state.json",
    );
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
}

#[test]
fn native_change_ids_up_to_the_specsync_ceiling_are_kept_across_processes() {
    // BUG-0013: SpecSync mints IDs up to 120 bytes and reads up to 255.
    let fixture = Fixture::new();
    fixture.init();
    let id = "prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package";
    assert_eq!(id.len(), 117);
    fixture.ok(&["checkpoint", "--native-change", id]);
    assert_eq!(fixture.session()["native_change"], id);
    assert_eq!(
        fixture.ok(&["checkpoint"])["checkpoint"]["native_change"],
        id
    );
    let ceiling = "a".repeat(255);
    fixture.ok(&["checkpoint", "--native-change", &ceiling]);
    let too_long = "a".repeat(256);
    assert!(
        fixture
            .fails(&["checkpoint", "--native-change", &too_long])
            .contains("invalid native change ID")
    );
    for malformed in ["has space", "dot.dot", "../escape", ""] {
        assert!(
            fixture
                .fails(&["checkpoint", "--native-change", malformed])
                .contains("invalid native change ID"),
            "{malformed:?}"
        );
    }
    assert_eq!(fixture.session()["native_change"], ceiling);
}

#[cfg(unix)]
/// Split a suggested command line into arguments, keeping double-quoted words whole.
fn words(command: &str) -> Vec<String> {
    let mut words = vec![];
    let mut word = String::new();
    let mut quoted = false;
    for character in command.chars() {
        match character {
            '"' => quoted = !quoted,
            ' ' if !quoted => {
                if !word.is_empty() {
                    words.push(std::mem::take(&mut word));
                }
            }
            _ => word.push(character),
        }
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
}

/// A fixture in stage done whose recorded native change is archived.
#[cfg(unix)]
fn done_fixture(id: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("archived");
    fixture.write(".stub-validity", "exact\n");
    fixture.write(".stub-next", "merge the PR on GitHub if it is still open\n");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", id]);
    fixture.ok(&["stage", "reset", "--to", "done", "--reason", "Closeout"]);
    fixture.ok(&[
        "checkpoint",
        "--next-action",
        "specsync change review old-change --reviewer 0xLeif, then ship",
    ]);
    fixture
}

#[cfg(unix)]
#[test]
fn done_with_an_archived_change_recommends_a_new_cycle_with_a_reason() {
    let fixture = done_fixture("closed-change");
    let next = fixture.ok(&["next", "--json"]);
    let action = &next["next_valid_action"];
    let text = action["action"].as_str().unwrap();
    assert!(
        text.starts_with("Start a new cycle: chough stage reset --to intent --reason \"<why>\""),
        "{text}"
    );
    for part in [
        "--to plan",
        "specsync change new \"<outcome>\" --spec <module> --path <path>",
        "chough checkpoint --native-change <id> --next-action \"<next step>\"",
    ] {
        assert!(text.contains(part), "missing {part:?}: {text}");
    }
    assert!(!text.contains("specsync change review"), "{text}");
    let reason = action["reason"].as_str().unwrap();
    assert!(
        reason.contains(
            "The cycle for closed-change is closed: native state archived, terminal evidence exact."
        ),
        "{reason}"
    );
    assert!(
        reason.contains(
            "Native next action, check it first: merge the PR on GitHub if it is still open."
        ),
        "{reason}"
    );
    assert_eq!(
        action["command"],
        "chough stage reset --to intent --reason \"<why>\""
    );
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["next_valid_action"], action["action"]);
    assert_eq!(
        status["session_next_action"],
        "specsync change review old-change --reviewer 0xLeif, then ship"
    );
    let shown = String::from_utf8(fixture.run(&["next"]).stdout).unwrap();
    assert!(shown.contains("Next: Start a new cycle"), "{shown}");
    assert!(
        shown.contains("Native next action: merge the PR on GitHub"),
        "{shown}"
    );

    // The suggested reset, with its placeholder filled, succeeds and replaces the stale
    // next action; the transition keeps the reason.
    let command = action["command"]
        .as_str()
        .unwrap()
        .replace("<why>", "New work: a focused view");
    let arguments = words(&command);
    assert_eq!(arguments[0], "chough");
    let arguments: Vec<&str> = arguments[1..].iter().map(String::as_str).collect();
    let reset = fixture.ok(&arguments);
    assert_eq!(reset["applied"]["to"], "intent");
    assert_eq!(
        fixture.session()["next_action"],
        "New cycle from done after reset: New work: a focused view. Create or record the native change (specsync change new, then chough checkpoint --native-change <id>) and set the next action."
    );
    let last = transitions(&fixture).pop().unwrap();
    assert_eq!(last["kind"], "reset");
    assert_eq!(last["reason"], "New work: a focused view");

    // Back continues the same cycle and keeps the next action.
    fixture.ok(&["checkpoint", "--next-action", "Keep this step"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "back", "--to", "intent", "--reason", "Refine"]);
    assert_eq!(fixture.session()["next_action"], "Keep this step");
}

#[test]
fn reset_replaces_the_next_action_and_back_keeps_it() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["checkpoint", "--next-action", "Finish the old plan"]);
    fixture.ok(&["stage", "back", "--to", "design", "--reason", "Revisit"]);
    assert_eq!(fixture.session()["next_action"], "Finish the old plan");
    fixture.ok(&[
        "stage",
        "reset",
        "--to",
        "plan",
        "--reason",
        "Different work",
    ]);
    assert_eq!(
        fixture.session()["next_action"],
        "New cycle from design after reset: Different work. Create or record the native change (specsync change new, then chough checkpoint --native-change <id>) and set the next action."
    );
    let last = transitions(&fixture).pop().unwrap();
    assert_eq!(last["kind"], "reset");
    assert_eq!(last["reason"], "Different work");

    // The next action keeps the checkpoint's own 4096-byte bound and stays one line,
    // while the transition keeps the reason whole.
    let reason = format!("Line one\nline two {}", "r".repeat(4_096 - 18));
    assert_eq!(reason.len(), 4_096);
    fixture.ok(&["stage", "reset", "--to", "intent", "--reason", &reason]);
    let next_action = fixture.session()["next_action"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(next_action.len() <= 4_096, "{} bytes", next_action.len());
    assert!(
        next_action.starts_with("New cycle from plan after reset: Line one line two rrr"),
        "{next_action:.80}"
    );
    assert!(!next_action.contains('\n'));
    assert!(next_action.ends_with("and set the next action."));
    assert_eq!(transitions(&fixture).pop().unwrap()["reason"], reason);
    fixture.ok(&["checkpoint", "--next-action", &next_action]);
}

#[cfg(unix)]
#[test]
fn done_keeps_the_gate_refusal_for_an_archive_that_does_not_authenticate() {
    let fixture = done_fixture("unproven-archive");
    fixture.write(".stub-validity", "corrupt-history\n");
    let next = fixture.ok(&["next", "--json"]);
    let action = next["next_valid_action"]["action"].as_str().unwrap();
    assert!(
        action.contains("refused:native-archive-corrupt-history"),
        "{action}"
    );
    assert!(!action.contains("Start a new cycle"), "{action}");
}

#[cfg(unix)]
#[test]
fn done_without_a_native_change_or_with_sdd_disabled_recommends_the_reset() {
    let fixture = done_fixture("forgotten");
    let mut session = fixture.session();
    session["native_change"] = Value::Null;
    fixture.write(".chough/session.json", &session.to_string());
    let next = fixture.ok(&["next", "--json"]);
    let action = &next["next_valid_action"];
    assert!(
        action["action"]
            .as_str()
            .unwrap()
            .contains("specsync change new"),
        "{action}"
    );
    assert!(
        action["reason"]
            .as_str()
            .unwrap()
            .contains("No native change is recorded"),
        "{action}"
    );
    assert_eq!(
        action["command"],
        "chough stage reset --to intent --reason \"<why>\""
    );

    fixture.write(".specsync/sdd.json", "{\"enabled\": false}\n");
    let next = fixture.ok(&["next", "--json"]);
    let text = next["next_valid_action"]["action"].as_str().unwrap();
    assert!(
        text.starts_with("Start a new cycle: chough stage reset --to intent --reason"),
        "{text}"
    );
    assert!(!text.contains("specsync change new"), "{text}");
}

// ---------------------------------------------------------------------------
// The approved-delta guard (REQ-cli-022, BUG-0002).

/// SpecSync 6.1.1's approved-delta digest: SHA-256 over the length-framed domain,
/// module name and delta body, with CRLF normalized to LF.
fn delta_digest(module: &str, body: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let body = String::from_utf8_lossy(body).replace("\r\n", "\n");
    let mut hash = Sha256::new();
    for (tag, value) in [
        ("domain", "specsync.approved-delta.v1".as_bytes()),
        ("module", module.as_bytes()),
        ("body", body.as_bytes()),
    ] {
        hash.update((tag.len() as u64).to_be_bytes());
        hash.update(tag.as_bytes());
        hash.update((value.len() as u64).to_be_bytes());
        hash.update(value);
    }
    format!("{:x}", hash.finalize())
}

#[cfg(unix)]
const DELTA: &str = "## ADDED\n\n### REQUIREMENT REQ-cli-090\n\nThe helper SHALL do the thing.\n";

/// A fixture at plan whose native change `id` is approved with a `cli` delta.
#[cfg(unix)]
fn approved_delta(id: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.write(".stub-affected", "\"cli\"\n");
    fixture.write(".stub-no-spec", "false\n");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", id]);
    fixture.write(&format!(".specsync/changes/{id}/deltas/cli.md"), DELTA);
    fixture.write(
        &format!(".specsync/changes/{id}/approvals.json"),
        &json!({
            "approvals": [
                {"gate": "definition", "actor": "SYNTHETIC", "approved_delta_digests": {"cli": "0".repeat(64)}},
                {"gate": "review", "actor": "SYNTHETIC"},
                {"gate": "definition", "actor": "SYNTHETIC", "approved_delta_digests": {"cli": delta_digest("cli", DELTA.as_bytes())}},
            ],
            "reopenings": [],
        })
        .to_string(),
    );
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture
}

#[cfg(unix)]
/// The session and transition bytes, which a refused move must leave unchanged.
fn records(fixture: &Fixture) -> (Vec<u8>, Vec<u8>) {
    (
        fs::read(fixture.root().join(".chough/session.json")).unwrap(),
        fs::read(fixture.root().join(".chough/transitions.jsonl")).unwrap(),
    )
}

/// Assert that `route` is refused with `label`, naming `expected`, and changes nothing.
#[cfg(unix)]
fn refused_unchanged(fixture: &Fixture, route: &[&str], label: &str, expected: &str) {
    let before = records(fixture);
    let refused = fixture.fails(route);
    assert!(refused.contains(label), "{route:?}: {refused}");
    assert!(refused.contains(expected), "{route:?}: {refused}");
    assert!(records(fixture) == before, "{route:?} changed the records");
}

#[test]
fn recomputed_digests_match_every_archived_approval_in_this_repository() {
    let archive =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".specsync/archive/changes");
    let mut compared = 0;
    for entry in fs::read_dir(&archive).unwrap() {
        let directory = entry.unwrap().path();
        let ledger: Value =
            serde_json::from_str(&fs::read_to_string(directory.join("approvals.json")).unwrap())
                .unwrap();
        let approval = ledger["approvals"]
            .as_array()
            .unwrap()
            .iter()
            .rev()
            .find(|approval| approval["gate"] == "definition")
            .unwrap();
        for (module, digest) in approval["approved_delta_digests"].as_object().unwrap() {
            let body = fs::read(directory.join(format!("deltas/{module}.md"))).unwrap();
            assert_eq!(
                delta_digest(module, &body),
                digest.as_str().unwrap(),
                "{}",
                directory.display()
            );
            compared += 1;
        }
    }
    assert!(compared >= 7, "only {compared} archived digests compared");
}

#[cfg(unix)]
#[test]
fn the_guard_accepts_every_archived_approval_in_this_repository() {
    // Each archived ledger and its deltas, presented as an approved change, pass the
    // guard: Chough's digests match the ones SpecSync recorded.
    let archive =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".specsync/archive/changes");
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    let mut checked = 0;
    for entry in fs::read_dir(&archive).unwrap() {
        let directory = entry.unwrap().path();
        let finalization: Value =
            serde_json::from_str(&fs::read_to_string(directory.join("finalization.json")).unwrap())
                .unwrap();
        let id = finalization["change_id"].as_str().unwrap().to_string();
        let target = format!(".specsync/changes/{id}");
        let ledger = fs::read_to_string(directory.join("approvals.json")).unwrap();
        fixture.write(&format!("{target}/approvals.json"), &ledger);
        if let Ok(deltas) = fs::read_dir(directory.join("deltas")) {
            for delta in deltas {
                let delta = delta.unwrap().path();
                let name = delta.file_name().unwrap().to_str().unwrap().to_string();
                fixture.write(
                    &format!("{target}/deltas/{name}"),
                    &fs::read_to_string(&delta).unwrap(),
                );
            }
        }
        let ledger: Value = serde_json::from_str(&ledger).unwrap();
        let scope = &ledger["approvals"]
            .as_array()
            .unwrap()
            .iter()
            .rev()
            .find(|approval| approval["gate"] == "definition")
            .unwrap()["approved_scope"];
        let affected: Vec<String> = scope["affected_specs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|module| format!("\"{}\"", module.as_str().unwrap()))
            .collect();
        fixture.write(".stub-affected", &affected.join(","));
        fixture.write(".stub-no-spec", &scope["no_spec_change"].to_string());
        fixture.ok(&["checkpoint", "--native-change", &id]);
        let next = fixture.ok(&["next", "--json"]);
        assert_eq!(next["gate"]["label"], "native:approved", "{id}: {next}");
        let deltas = next["native"]["approved_deltas"].as_str().unwrap();
        assert!(
            deltas.starts_with("unchanged: cli") || deltas.starts_with("no semantic deltas"),
            "{id}: {deltas}"
        );
        checked += 1;
    }
    assert!(checked >= 10, "only {checked} archived changes checked");
}

#[cfg(unix)]
#[test]
fn a_changed_added_or_deleted_delta_refuses_every_route_with_native_delta_changed() {
    let id = "guarded-change";
    let fixture = approved_delta(id);
    let delta = format!(".specsync/changes/{id}/deltas/cli.md");
    let changed = "refused:native-delta-changed";
    // An approved delta edited after the approval, while status still reports
    // approval_valid true (BUG-0002).
    fixture.write(
        &delta,
        &format!("{DELTA}\n### REQUIREMENT REQ-cli-091\n\nMore.\n"),
    );
    let routes: [&[&str]; 5] = [
        &["stage", "advance"],
        &["stage", "skip", "--reason", "Skip ahead"],
        &["stage", "reset", "--to", "build", "--reason", "Restart"],
        &["stage", "reset", "--to", "test", "--reason", "Retest"],
        &["stage", "reset", "--to", "accept", "--reason", "Accept"],
    ];
    for route in routes {
        refused_unchanged(&fixture, route, changed, "cli");
    }
    let refused = fixture.fails(&["stage", "advance"]);
    for part in [
        "specsync change approve guarded-change --actor \"<their name>\"",
        "specsync change check guarded-change",
        "specsync.approved-delta.v1",
    ] {
        assert!(refused.contains(part), "missing {part:?}: {refused}");
    }
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["gate"]["label"], changed);
    assert!(
        next["native"]["approved_deltas"]
            .as_str()
            .unwrap()
            .starts_with("changed: cli"),
        "{next}"
    );
    assert_eq!(
        fixture.ok(&["status", "--json"])["checkpoint"]["gate"]["label"],
        changed
    );

    // A line-ending-only edit and the restored delta pass.
    fixture.write(&delta, &DELTA.replace('\n', "\r\n"));
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
    fixture.write(&delta, DELTA);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "test");

    // A module added after the approval.
    fixture.write(".stub-affected", "\"cli\",\"docs\"\n");
    fixture.write(&format!(".specsync/changes/{id}/deltas/docs.md"), DELTA);
    refused_unchanged(&fixture, &["stage", "advance"], changed, "docs");
    refused_unchanged(
        &fixture,
        &["stage", "back", "--to", "build", "--reason", "Fix"],
        changed,
        "docs",
    );
    fixture.write(".stub-affected", "\"cli\"\n");
    fs::remove_file(
        fixture
            .root()
            .join(format!(".specsync/changes/{id}/deltas/docs.md")),
    )
    .unwrap();

    // A deleted delta.
    fs::remove_file(fixture.root().join(&delta)).unwrap();
    refused_unchanged(&fixture, &["stage", "advance"], changed, "cli");
    fixture.write(&delta, DELTA);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "accept");

    // Module names SpecSync 6.1.1 accepts, such as `a.b` and `Módulo`, are compared
    // like any other: approved deltas pass and an edited one refuses.
    let modules = ["a.b", "Módulo"];
    fixture.write(".stub-affected", "\"cli\",\"a.b\",\"Módulo\"\n");
    let mut digests = serde_json::Map::new();
    digests.insert("cli".into(), json!(delta_digest("cli", DELTA.as_bytes())));
    for module in modules {
        fixture.write(&format!(".specsync/changes/{id}/deltas/{module}.md"), DELTA);
        digests.insert(module.into(), json!(delta_digest(module, DELTA.as_bytes())));
    }
    fixture.write(
        &format!(".specsync/changes/{id}/approvals.json"),
        &json!({"approvals": [{"gate": "definition", "approved_delta_digests": digests}], "reopenings": []})
            .to_string(),
    );
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        next["native"]["approved_deltas"], "unchanged: Módulo, a.b, cli",
        "{next}"
    );
    fixture.write(
        &format!(".specsync/changes/{id}/deltas/a.b.md"),
        &format!("{DELTA}\nMore.\n"),
    );
    refused_unchanged(
        &fixture,
        &["stage", "reset", "--to", "build", "--reason", "Restart"],
        changed,
        "a.b (edited)",
    );
    fixture.write(&format!(".specsync/changes/{id}/deltas/a.b.md"), DELTA);
    fixture.ok(&["stage", "reset", "--to", "build", "--reason", "Restart"]);
    assert_eq!(stage(&fixture), "build");
}

#[cfg(unix)]
#[test]
fn a_comparison_that_cannot_be_made_refuses_with_native_delta_unverifiable() {
    let id = "unverifiable-change";
    let fixture = approved_delta(id);
    let ledger = format!(".specsync/changes/{id}/approvals.json");
    let approved = fixture.read(&ledger);
    let unverifiable = "refused:native-delta-unverifiable";
    let status = format!("specsync change status {id}");
    let digest = delta_digest("cli", DELTA.as_bytes());
    // An approval without digests can only be cleared by a new approval, so its
    // refusal names that rather than a status read.
    let approve = format!("specsync change approve {id} --actor \"<their name>\"");
    let cases: Vec<(&str, String, &str)> = vec![
        ("malformed", "{\"approvals\": [".into(), &status),
        ("malformed", "{\"approvals\": \"none\"}".into(), &status),
        (
            "no definition approval",
            json!({"approvals": [{"gate": "review"}]}).to_string(),
            &status,
        ),
        (
            "approved_delta_digests",
            json!({"approvals": [{"gate": "definition"}]}).to_string(),
            &approve,
        ),
        (
            "malformed",
            json!({"approvals": [{"gate": "definition", "approved_delta_digests": {"cli": "short"}}]})
                .to_string(),
            &status,
        ),
        (
            "malformed",
            json!({"approvals": [{"gate": "definition", "approved_delta_digests": {"../cli": digest}}]})
                .to_string(),
            &status,
        ),
        (
            "4194304",
            format!(
                "{{\"approvals\":[{{\"gate\":\"definition\",\"approved_delta_digests\":{{\"cli\":\"{digest}\"}}}}],\"padding\":\"{}\"}}",
                "x".repeat(4 * 1_048_576)
            ),
            &status,
        ),
    ];
    for (why, content, remedy) in cases {
        fixture.write(&ledger, &content);
        refused_unchanged(&fixture, &["stage", "advance"], unverifiable, why);
        refused_unchanged(&fixture, &["stage", "advance"], unverifiable, remedy);
    }
    fs::remove_file(fixture.root().join(&ledger)).unwrap();
    refused_unchanged(&fixture, &["stage", "advance"], unverifiable, "missing");
    fixture.write(&ledger, &approved);
    // A module name that is not one plain path segment is never read as a path.
    for module in ["../cli", "a/b", "nested\\cli", ".", "trailing."] {
        fixture.write(".stub-affected", &format!("{}\n", json!(module)));
        refused_unchanged(
            &fixture,
            &["stage", "reset", "--to", "build", "--reason", "Restart"],
            unverifiable,
            "invalid module name",
        );
    }
    fixture.write(".stub-affected", "none\n");
    refused_unchanged(
        &fixture,
        &["stage", "reset", "--to", "build", "--reason", "Restart"],
        unverifiable,
        "affected_specs",
    );
    let next = fixture.ok(&["next", "--json"]);
    assert!(
        next["native"]["approved_deltas"]
            .as_str()
            .unwrap()
            .starts_with("unverifiable:"),
        "{next}"
    );
    fixture.write(".stub-affected", "\"cli\"\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
}

fn merge_base() -> Value {
    json!({
        "schema_version": 2,
        "objective": "Merge parallel checkpoint edits",
        "intent_scope": "FLOW-13",
        "modules": ["cli"],
        "stage": {"current": "build", "status": "active", "callers": [], "iteration": 1},
        "native_change": "merge-checkpoints",
        "references": {
            "criteria": ["FLOW-13"],
            "requirements": ["REQ-cli-007"],
            "cases": [],
            "design": [],
            "files": []
        },
        "decisions": [],
        "open_questions": [],
        "completed": [],
        "observations": [],
        "bugs": [],
        "blockers": [],
        "awaiting_human": [],
        "run_consents": [],
        "next_action": "Merge compatible edits"
    })
}

fn merge_json(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap() + "\n"
}

fn merge_paths(fixture: &Fixture) -> [String; 3] {
    ["base.json", "ours.json", "theirs.json"].map(|name| {
        fixture
            .root()
            .join("merge-inputs")
            .join(name)
            .to_string_lossy()
            .into_owned()
    })
}

fn run_merge_text(fixture: &Fixture, base: &str, ours: &str, theirs: &str) -> Output {
    fixture.write("merge-inputs/base.json", base);
    fixture.write("merge-inputs/ours.json", ours);
    fixture.write("merge-inputs/theirs.json", theirs);
    let paths = merge_paths(fixture);
    fixture.run(&["checkpoint", "--merge", &paths[0], &paths[1], &paths[2]])
}

fn merge_record_snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, directory: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) {
        if !directory.exists() {
            return;
        }
        for entry in fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                visit(root, &path, files);
            } else {
                files.insert(
                    path.strip_prefix(root).unwrap().to_owned(),
                    fs::read(&path).unwrap(),
                );
            }
        }
    }

    let mut files = BTreeMap::new();
    visit(root, &root.join(".chough"), &mut files);
    files
}

fn assert_merge_refusal(fixture: &Fixture, base: &str, ours: &str, theirs: &str, reason: &str) {
    let before = [base.as_bytes(), ours.as_bytes(), theirs.as_bytes()];
    let records = merge_record_snapshot(fixture.root());
    let output = run_merge_text(fixture, base, ours, theirs);
    assert!(
        !output.status.success(),
        "merge unexpectedly passed: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(reason), "missing {reason:?}: {stderr}");
    for (name, expected) in ["base.json", "ours.json", "theirs.json"].iter().zip(before) {
        assert_eq!(
            fs::read(fixture.root().join("merge-inputs").join(name)).unwrap(),
            expected
        );
    }
    assert_eq!(merge_record_snapshot(fixture.root()), records);
}

fn assert_merge_ok(fixture: &Fixture, base: &Value, ours: &Value, theirs: &Value) -> Value {
    let records = merge_record_snapshot(fixture.root());
    let output = run_merge_text(
        fixture,
        &merge_json(base),
        &merge_json(ours),
        &merge_json(theirs),
    );
    assert!(
        output.status.success(),
        "merge failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(fixture.read("merge-inputs/base.json"), merge_json(base));
    assert_eq!(fixture.read("merge-inputs/theirs.json"), merge_json(theirs));
    assert_eq!(merge_record_snapshot(fixture.root()), records);
    serde_json::from_str(&fixture.read("merge-inputs/ours.json")).unwrap()
}

#[test]
fn checkpoint_merge_combines_every_exact_string_collection_in_order() {
    let fixture = Fixture::new();
    fixture.init();
    let collections = [
        (
            "references",
            "criteria",
            "FLOW-1",
            "FLOW-2",
            "FLOW-3",
            "FLOW-4",
            "FLOW-5",
            "FLOW-6",
        ),
        (
            "references",
            "requirements",
            "REQ-cli-001",
            "REQ-cli-002",
            "REQ-cli-003",
            "REQ-cli-004",
            "REQ-cli-005",
            "REQ-cli-006",
        ),
        (
            "references",
            "cases",
            "CASE-a",
            "CASE-b",
            "CASE-c",
            "CASE-d",
            "CASE-e",
            "CASE-f",
        ),
        (
            "references",
            "design",
            "docs/a.md",
            "docs/b.md",
            "docs/c.md",
            "docs/d.md",
            "docs/e.md",
            "docs/f.md",
        ),
        (
            "references",
            "files",
            "src/a.rs",
            "src/b.rs",
            "src/c.rs",
            "src/d.rs",
            "src/e.rs",
            "src/f.rs",
        ),
        (
            "",
            "decisions",
            "keep",
            "remove ours",
            "remove theirs",
            "ours",
            "theirs",
            "shared",
        ),
        (
            "",
            "open_questions",
            "keep",
            "remove ours",
            "remove theirs",
            "ours",
            "theirs",
            "shared",
        ),
        (
            "",
            "completed",
            "keep",
            "remove ours",
            "remove theirs",
            "ours",
            "theirs",
            "shared",
        ),
        (
            "",
            "observations",
            "OBS-a",
            "OBS-b",
            "OBS-c",
            "OBS-d",
            "OBS-e",
            "OBS-f",
        ),
        (
            "", "bugs", "BUG-a", "BUG-b", "BUG-c", "BUG-d", "BUG-e", "BUG-f",
        ),
    ];
    for (parent, field, keep, remove_ours, remove_theirs, ours_add, theirs_add, shared) in
        collections
    {
        let mut base = merge_base();
        let mut ours = merge_base();
        let mut theirs = merge_base();
        fn slot<'a>(session: &'a mut Value, parent: &str, field: &str) -> &'a mut Value {
            if parent.is_empty() {
                &mut session[field]
            } else {
                &mut session[parent][field]
            }
        }
        *slot(&mut base, parent, field) = json!([keep, remove_ours, remove_theirs]);
        *slot(&mut ours, parent, field) = json!([keep, remove_theirs, ours_add, shared]);
        *slot(&mut theirs, parent, field) = json!([keep, remove_ours, theirs_add, shared]);
        let merged = assert_merge_ok(&fixture, &base, &ours, &theirs);
        let actual = if parent.is_empty() {
            &merged[field]
        } else {
            &merged[parent][field]
        };
        assert_eq!(
            actual,
            &json!([keep, ours_add, shared, theirs_add]),
            "{parent}.{field}"
        );
    }

    let mut base = merge_base();
    let mut ours = merge_base();
    let mut theirs = merge_base();
    base["decisions"] = json!([]);
    ours["decisions"] = json!(["Case", "case ", "case!", "shared"]);
    theirs["decisions"] = json!(["case", "case ", "shared", "case."]);
    let merged = assert_merge_ok(&fixture, &base, &ours, &theirs);
    assert_eq!(
        merged["decisions"],
        json!(["Case", "case ", "case!", "shared", "case", "case."])
    );
}

#[test]
fn checkpoint_merge_selects_the_current_cycle_as_one_group() {
    let fixture = Fixture::new();
    fixture.init();
    let changes = [
        ("objective", json!("A different objective")),
        ("intent_scope", json!("LIFE-3")),
        ("modules", json!(["cli", "docs"])),
        (
            "stage",
            json!({"current": "design", "status": "paused", "callers": [], "iteration": 2}),
        ),
        ("native_change", json!("another-change")),
        ("next_action", json!("Wait for the selected cycle")),
        (
            "run_consents",
            json!([{"key": "validation", "source": "human", "objective": "Merge parallel checkpoint edits", "granted_at_unix_seconds": 123}]),
        ),
        ("blockers", json!(["A selected blocker"])),
        ("awaiting_human", json!(["A selected human gate"])),
    ];
    for (field, value) in changes {
        let base = merge_base();
        let mut ours = base.clone();
        ours[field] = value.clone();
        let merged = assert_merge_ok(&fixture, &base, &ours, &base);
        assert_eq!(merged[field], value, "ours field {field}");

        let mut theirs = base.clone();
        theirs[field] = value.clone();
        let merged = assert_merge_ok(&fixture, &base, &base, &theirs);
        assert_eq!(merged[field], value, "theirs field {field}");

        let merged = assert_merge_ok(&fixture, &base, &ours, &theirs);
        assert_eq!(merged[field], value, "identical field {field}");
    }

    let base = merge_base();
    let mut changed = base.clone();
    changed["objective"] = json!("Identical new cycle");
    changed["next_action"] = json!("Continue the identical cycle");
    changed["blockers"] = json!(["Same blocker"]);
    let merged = assert_merge_ok(&fixture, &base, &changed, &changed);
    assert_eq!(merged, changed);

    let mut ours = base.clone();
    let mut theirs = base.clone();
    ours["objective"] = json!("Ours cycle");
    theirs["next_action"] = json!("Theirs next action");
    assert_merge_refusal(
        &fixture,
        &merge_json(&base),
        &merge_json(&ours),
        &merge_json(&theirs),
        "current-cycle conflict",
    );
    let output = run_merge_text(
        &fixture,
        &merge_json(&base),
        &merge_json(&ours),
        &merge_json(&theirs),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("objective"), "{stderr}");
    assert!(stderr.contains("next_action"), "{stderr}");

    let mut ours = base.clone();
    let mut theirs = base.clone();
    ours["native_change"] = json!("ours-change");
    theirs["native_change"] = json!("theirs-change");
    assert_merge_refusal(
        &fixture,
        &merge_json(&base),
        &merge_json(&ours),
        &merge_json(&theirs),
        "native_change",
    );

    for (field, ours_value, theirs_value) in [
        (
            "run_consents",
            json!([{"key": "validation", "source": "human", "objective": "Merge parallel checkpoint edits", "granted_at_unix_seconds": 123}]),
            json!([{"key": "release", "source": "human", "objective": "Merge parallel checkpoint edits", "granted_at_unix_seconds": 456}]),
        ),
        (
            "blockers",
            json!(["Ours blocker"]),
            json!(["Theirs blocker"]),
        ),
        (
            "awaiting_human",
            json!(["Ours gate"]),
            json!(["Theirs gate"]),
        ),
    ] {
        let mut ours = base.clone();
        let mut theirs = base.clone();
        ours[field] = ours_value;
        theirs[field] = theirs_value;
        assert_merge_refusal(
            &fixture,
            &merge_json(&base),
            &merge_json(&ours),
            &merge_json(&theirs),
            "current-cycle conflict",
        );
    }

    let records = merge_record_snapshot(fixture.root());
    let output = run_merge_text(
        &fixture,
        &merge_json(&base),
        &base.to_string(),
        &merge_json(&base),
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(merge_record_snapshot(fixture.root()), records);
}

#[test]
fn checkpoint_merge_rejects_invalid_schemas_json_and_unknown_fields() {
    let fixture = Fixture::new();
    fixture.init();
    let valid = merge_json(&merge_base());
    let schema_1 = "{\n  \"schema_version\": 1,\n  \"objective\": \"Old session\",\n  \"intent_scope\": \"LIFE\",\n  \"modules\": [\"cli\"],\n  \"decisions\": [],\n  \"open_questions\": [],\n  \"next_action\": \"Migrate explicitly\"\n}\n";
    let mut unknown_top = merge_base();
    unknown_top["unknown"] = json!(true);
    let mut unknown_nested = merge_base();
    unknown_nested["stage"]["unknown"] = json!(true);
    let mut unsupported = merge_base();
    unsupported["schema_version"] = json!(3);
    let invalid = [
        schema_1.to_string(),
        "{\n".to_string(),
        merge_json(&unknown_top),
        merge_json(&unknown_nested),
        merge_json(&unsupported),
    ];
    for content in invalid {
        for position in 0..3 {
            let mut inputs = [valid.clone(), valid.clone(), valid.clone()];
            inputs[position] = content.clone();
            assert_merge_refusal(&fixture, &inputs[0], &inputs[1], &inputs[2], "session");
        }
    }
    assert!(!fixture.exists(".chough/session.v1.json"));
}

#[test]
fn checkpoint_merge_enforces_input_collection_and_output_byte_bounds() {
    let fixture = Fixture::new();
    fixture.init();
    let valid = merge_json(&merge_base());
    let oversized = format!("{}{}", valid, " ".repeat(65_536));
    for position in 0..3 {
        let mut inputs = [valid.clone(), valid.clone(), valid.clone()];
        inputs[position] = oversized.clone();
        assert_merge_refusal(&fixture, &inputs[0], &inputs[1], &inputs[2], "65536");
    }

    let base = merge_base();
    let mut ours = base.clone();
    let mut theirs = base.clone();
    ours["decisions"] = json!(
        (0..150)
            .map(|index| format!("ours-{index}"))
            .collect::<Vec<_>>()
    );
    theirs["decisions"] = json!(
        (0..150)
            .map(|index| format!("theirs-{index}"))
            .collect::<Vec<_>>()
    );
    assert_merge_refusal(
        &fixture,
        &merge_json(&base),
        &merge_json(&ours),
        &merge_json(&theirs),
        "too many",
    );

    ours["decisions"] = json!(
        (0..100)
            .map(|index| format!("ours-{index}-{}", "o".repeat(320)))
            .collect::<Vec<_>>()
    );
    theirs["decisions"] = json!(
        (0..100)
            .map(|index| format!("theirs-{index}-{}", "t".repeat(320)))
            .collect::<Vec<_>>()
    );
    assert_merge_refusal(
        &fixture,
        &merge_json(&base),
        &merge_json(&ours),
        &merge_json(&theirs),
        "65536",
    );

    ours["decisions"] = json!(
        (0..100)
            .map(|index| format!("ours-{index}"))
            .collect::<Vec<_>>()
    );
    theirs["decisions"] = json!(
        (0..100)
            .map(|index| format!("theirs-{index}"))
            .collect::<Vec<_>>()
    );
    let merged = assert_merge_ok(&fixture, &base, &ours, &theirs);
    assert_eq!(merged["decisions"].as_array().unwrap().len(), 200);

    ours["run_consents"] = json!(
        (0..200)
            .map(|index| {
                json!({
                    "key": format!("consent-{index}"),
                    "source": "test",
                    "objective": base["objective"].clone(),
                    "granted_at_unix_seconds": 1
                })
            })
            .collect::<Vec<_>>()
    );
    theirs = base.clone();
    let merged = assert_merge_ok(&fixture, &base, &ours, &theirs);
    assert_eq!(merged["run_consents"].as_array().unwrap().len(), 200);
    ours["run_consents"].as_array_mut().unwrap().push(json!({
        "key": "consent-200",
        "source": "test",
        "objective": base["objective"].clone(),
        "granted_at_unix_seconds": 1
    }));
    assert_merge_refusal(
        &fixture,
        &merge_json(&base),
        &merge_json(&ours),
        &merge_json(&theirs),
        "too many run consents",
    );
}

#[cfg(unix)]
#[test]
fn checkpoint_merge_closed_stdout_does_not_turn_a_successful_write_into_failure() {
    let fixture = Fixture::new();
    fixture.init();
    let base = merge_base();
    let mut ours = base.clone();
    ours["decisions"] = json!(["merge succeeded before reporting"]);
    let theirs = base.clone();
    fixture.write("merge-inputs/base.json", &merge_json(&base));
    fixture.write("merge-inputs/ours.json", &merge_json(&ours));
    fixture.write("merge-inputs/theirs.json", &merge_json(&theirs));
    let paths = merge_paths(&fixture);
    let output = Command::new("sh")
        .arg("-c")
        .arg("exec 1>&-; \"$@\"")
        .arg("sh")
        .arg(env!("CARGO_BIN_EXE_chough"))
        .arg("checkpoint")
        .arg("--merge")
        .args(paths)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "closed stdout changed the result: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let merged: Value = serde_json::from_str(&fixture.read("merge-inputs/ours.json")).unwrap();
    assert_eq!(
        merged["decisions"],
        json!(["merge succeeded before reporting"])
    );
    assert_eq!(fixture.read("merge-inputs/base.json"), merge_json(&base));
    assert_eq!(
        fixture.read("merge-inputs/theirs.json"),
        merge_json(&theirs)
    );
}

#[test]
fn checkpoint_merge_requires_distinct_regular_files_and_exact_arguments() {
    let fixture = Fixture::new();
    fixture.init();
    let valid = merge_json(&merge_base());
    fixture.write("merge-inputs/base.json", &valid);
    fixture.write("merge-inputs/ours.json", &valid);
    fixture.write("merge-inputs/theirs.json", &valid);
    fs::create_dir_all(fixture.root().join("merge-inputs/directory")).unwrap();
    let paths = merge_paths(&fixture);
    let records = merge_record_snapshot(fixture.root());
    for arguments in [
        vec!["checkpoint", "--merge", &paths[0], &paths[1]],
        vec![
            "checkpoint",
            "--merge",
            &paths[0],
            &paths[1],
            &paths[2],
            &paths[0],
        ],
        vec![
            "checkpoint",
            "--merge",
            &paths[0],
            &paths[1],
            &paths[2],
            "--next-action",
            "No",
        ],
        vec!["checkpoint", "--merge", &paths[0], &paths[1], &paths[1]],
        vec!["checkpoint", "--merge", &paths[0], &paths[0], &paths[2]],
    ] {
        assert!(
            !fixture.run(&arguments).status.success(),
            "{arguments:?} unexpectedly passed"
        );
        assert_eq!(merge_record_snapshot(fixture.root()), records);
    }
    let directory = fixture
        .root()
        .join("merge-inputs/directory")
        .to_string_lossy()
        .into_owned();
    assert!(
        !fixture
            .run(&["checkpoint", "--merge", &paths[0], &directory, &paths[2]])
            .status
            .success()
    );
    let missing = fixture
        .root()
        .join("merge-inputs/missing.json")
        .to_string_lossy()
        .into_owned();
    assert!(
        !fixture
            .run(&["checkpoint", "--merge", &paths[0], &missing, &paths[2]])
            .status
            .success()
    );
    assert_eq!(fixture.read("merge-inputs/base.json"), valid);
    assert_eq!(fixture.read("merge-inputs/ours.json"), valid);
    assert_eq!(fixture.read("merge-inputs/theirs.json"), valid);

    #[cfg(unix)]
    for position in 0..3 {
        let link = fixture.root().join("merge-inputs/link.json");
        std::os::unix::fs::symlink(fixture.root().join("merge-inputs/base.json"), &link).unwrap();
        let mut operands = paths.clone();
        operands[position] = link.to_string_lossy().into_owned();
        assert!(
            !fixture
                .run(&[
                    "checkpoint",
                    "--merge",
                    &operands[0],
                    &operands[1],
                    &operands[2]
                ])
                .status
                .success()
        );
        fs::remove_file(link).unwrap();
    }
}

fn isolated_git(root: &Path, args: &[&str]) -> Output {
    let binary = Path::new(env!("CARGO_BIN_EXE_chough"));
    let mut paths = vec![binary.parent().unwrap().to_owned()];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    fs::create_dir_all(root.join("isolated-home")).unwrap();
    Command::new("git")
        .current_dir(root)
        .args(args)
        .env("HOME", root.join("isolated-home"))
        .env("GIT_CONFIG_GLOBAL", root.join("isolated-global-config"))
        .env("GIT_CONFIG_SYSTEM", root.join("isolated-system-config"))
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("PATH", std::env::join_paths(paths).unwrap())
        .output()
        .unwrap()
}

fn git_ok(root: &Path, args: &[&str]) {
    let output = isolated_git(root, args);
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_merge_fixture(
    driver: bool,
    current_cycle_conflict: bool,
) -> (Fixture, Output, Value, Value, Value) {
    let fixture = Fixture::new();
    fixture.init();
    fixture.write(
        ".gitattributes",
        ".chough/session.json merge=chough-session\n",
    );
    let base = merge_base();
    let mut ours = base.clone();
    let mut theirs = base.clone();
    let mut base = base;
    base["decisions"] = json!(["base keep", "remove ours", "remove theirs"]);
    ours["decisions"] = json!([
        "base keep",
        "remove theirs",
        "ours addition",
        "shared addition"
    ]);
    theirs["decisions"] = json!([
        "base keep",
        "remove ours",
        "theirs addition",
        "shared addition"
    ]);
    if current_cycle_conflict {
        ours["objective"] = json!("Ours current cycle");
        theirs["next_action"] = json!("Theirs independent next action");
    }
    fixture.write(".chough/session.json", &merge_json(&base));
    git_ok(fixture.root(), &["init", "--initial-branch=main"]);
    git_ok(fixture.root(), &["config", "user.name", "Checkpoint Test"]);
    git_ok(
        fixture.root(),
        &["config", "user.email", "checkpoint@example.invalid"],
    );
    git_ok(fixture.root(), &["config", "commit.gpgsign", "false"]);
    if driver {
        git_ok(
            fixture.root(),
            &[
                "config",
                "merge.chough-session.driver",
                "chough checkpoint --merge %O %A %B",
            ],
        );
    }
    git_ok(fixture.root(), &["add", "."]);
    git_ok(fixture.root(), &["commit", "-m", "base checkpoint"]);
    git_ok(fixture.root(), &["checkout", "-b", "ours"]);
    fixture.write(".chough/session.json", &merge_json(&ours));
    git_ok(fixture.root(), &["add", ".chough/session.json"]);
    git_ok(fixture.root(), &["commit", "-m", "ours checkpoint"]);
    git_ok(fixture.root(), &["checkout", "main"]);
    git_ok(fixture.root(), &["checkout", "-b", "theirs"]);
    fixture.write(".chough/session.json", &merge_json(&theirs));
    git_ok(fixture.root(), &["add", ".chough/session.json"]);
    git_ok(fixture.root(), &["commit", "-m", "theirs checkpoint"]);
    git_ok(fixture.root(), &["checkout", "ours"]);
    let output = isolated_git(fixture.root(), &["merge", "--no-edit", "theirs"]);
    (fixture, output, base, ours, theirs)
}

fn print_git_merge(output: &Output, checkpoint: &[u8]) {
    println!("command: git merge --no-edit theirs");
    println!("exit status: {}", output.status);
    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    println!(
        "resulting .chough/session.json bytes:\n{}",
        String::from_utf8_lossy(checkpoint)
    );
}

#[test]
fn real_git_without_driver_conflict() {
    let (fixture, output, _, _, _) = git_merge_fixture(false, false);
    let checkpoint = fs::read(fixture.root().join(".chough/session.json")).unwrap();
    print_git_merge(&output, &checkpoint);
    assert!(!output.status.success());
    assert!(
        !isolated_git(fixture.root(), &["ls-files", "-u"])
            .stdout
            .is_empty()
    );
    assert!(String::from_utf8_lossy(&checkpoint).contains("<<<<<<<"));
}

#[test]
fn real_git_driver_merges_compatible_checkpoint_changes() {
    let (fixture, output, base, ours, theirs) = git_merge_fixture(true, false);
    let checkpoint = fs::read(fixture.root().join(".chough/session.json")).unwrap();
    print_git_merge(&output, &checkpoint);
    assert!(output.status.success(), "configured driver did not merge");
    let expected = r#"{
  "schema_version": 2,
  "objective": "Merge parallel checkpoint edits",
  "intent_scope": "FLOW-13",
  "modules": [
    "cli"
  ],
  "stage": {
    "current": "build",
    "status": "active",
    "callers": [],
    "iteration": 1
  },
  "native_change": "merge-checkpoints",
  "references": {
    "criteria": [
      "FLOW-13"
    ],
    "requirements": [
      "REQ-cli-007"
    ],
    "cases": [],
    "design": [],
    "files": []
  },
  "decisions": [
    "base keep",
    "ours addition",
    "shared addition",
    "theirs addition"
  ],
  "open_questions": [],
  "completed": [],
  "observations": [],
  "bugs": [],
  "blockers": [],
  "awaiting_human": [],
  "run_consents": [],
  "next_action": "Merge compatible edits"
}
"#;
    assert_eq!(checkpoint, expected.as_bytes());
    assert!(
        isolated_git(fixture.root(), &["ls-files", "-u"])
            .stdout
            .is_empty()
    );
    assert!(
        isolated_git(fixture.root(), &["status", "--porcelain"])
            .stdout
            .is_empty()
    );
    assert_eq!(merge_record_snapshot(fixture.root()).len(), 1);

    fixture.write("merge-inputs/base.json", &merge_json(&base));
    fixture.write("merge-inputs/ours.json", &merge_json(&ours));
    fixture.write("merge-inputs/theirs.json", &merge_json(&theirs));
    let paths = merge_paths(&fixture);
    let direct = fixture.run(&["checkpoint", "--merge", &paths[0], &paths[1], &paths[2]]);
    assert!(
        direct.status.success(),
        "{}",
        String::from_utf8_lossy(&direct.stderr)
    );
    assert_eq!(fixture.read("merge-inputs/ours.json"), expected);
    assert_eq!(fixture.read("merge-inputs/base.json"), merge_json(&base));
    assert_eq!(
        fixture.read("merge-inputs/theirs.json"),
        merge_json(&theirs)
    );
    assert_eq!(merge_record_snapshot(fixture.root()).len(), 1);
}

#[test]
fn real_git_current_cycle_conflict_and_direct_fallback_preserve_ours() {
    let (fixture, output, base, ours, theirs) = git_merge_fixture(true, true);
    let checkpoint = fs::read(fixture.root().join(".chough/session.json")).unwrap();
    print_git_merge(&output, &checkpoint);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("current-cycle conflict"));
    assert_eq!(checkpoint, merge_json(&ours).as_bytes());
    assert!(!String::from_utf8_lossy(&checkpoint).contains("<<<<<<<"));
    assert!(
        !isolated_git(fixture.root(), &["ls-files", "-u"])
            .stdout
            .is_empty()
    );
    assert_eq!(merge_record_snapshot(fixture.root()).len(), 1);
    git_ok(fixture.root(), &["merge", "--abort"]);

    fixture.write("merge-inputs/base.json", &merge_json(&base));
    fixture.write("merge-inputs/ours.json", &merge_json(&ours));
    fixture.write("merge-inputs/theirs.json", &merge_json(&theirs));
    let paths = merge_paths(&fixture);
    let direct = fixture.run(&["checkpoint", "--merge", &paths[0], &paths[1], &paths[2]]);
    assert!(!direct.status.success());
    assert!(String::from_utf8_lossy(&direct.stderr).contains("current-cycle conflict"));
    assert_eq!(fixture.read("merge-inputs/ours.json"), merge_json(&ours));
    assert_eq!(merge_record_snapshot(fixture.root()).len(), 1);
}
