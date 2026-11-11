//! Checkpoint schema 2, explicit migration and stage routing (REQ-cli-007, REQ-cli-008).

mod common;

use common::Fixture;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::process::Command;

/// The active session's Timeline entries, oldest first: `time`, `kind`, `from`, `to`,
/// `text` and `id`, plus each structured field (`  - name: value`) under its name.
fn transitions(fixture: &Fixture) -> Vec<Value> {
    timeline_entries(&fixture.read(&fixture.session_file()))
}

/// The Timeline entries of a session record (REQ-history-002): `- <time> · <kind> ·
/// <from> → <to> · <text> ^<id>`, each followed by its `  - name: value` fields.
fn timeline_entries(text: &str) -> Vec<Value> {
    let Some(start) = text.find("\n## Timeline\n") else {
        return vec![];
    };
    let mut entries: Vec<Value> = vec![];
    for line in text[start + 1..].lines().skip(1) {
        if let Some(item) = line.strip_prefix("- ") {
            let parts: Vec<&str> = item.splitn(4, " · ").collect();
            assert_eq!(parts.len(), 4, "a timeline entry: {line}");
            let (from, to) = parts[2].split_once(" → ").unwrap_or((parts[2], ""));
            let (text, id) = parts[3].rsplit_once(" ^").unwrap_or((parts[3], ""));
            entries.push(json!({
                "time": parts[0], "kind": parts[1], "from": from, "to": to,
                "text": text.trim(), "id": id,
            }));
        } else if let (Some(field), Some(last)) = (line.strip_prefix("  - "), entries.last_mut()) {
            if let Some((name, value)) = field.split_once(": ") {
                last[name] = serde_json::from_str(value).unwrap_or_else(|_| json!(value));
            }
        } else if line.starts_with("## ") {
            break;
        }
    }
    entries
}

fn stage(fixture: &Fixture) -> String {
    fixture.session()["stage"]["current"]
        .as_str()
        .unwrap()
        .to_string()
}

#[test]
fn init_writes_a_schema_2_checkpoint_at_intent() {
    // TC-cli-0015 in the 0.6.0 layout: `init --objective` writes the first session folder,
    // named by date and the objective's slug, points at it and starts at intent
    // (REQ-cli-007, REQ-cli-037). No 0.5.x checkpoint file is written.
    let fixture = Fixture::new();
    fixture.init();
    let id = fixture.active_id();
    assert_eq!(fixture.open_ids(), std::slice::from_ref(&id));
    assert_eq!(id.get(11..), Some("ship-a-focused-change"), "{id}");
    let record = fixture.read(&fixture.session_file());
    assert!(record.starts_with("---\nkind: session\n"), "{record}");
    let stored = common::parse_session(&record);
    assert_eq!(stored["id"], id);
    assert_eq!(stored["objective"], "Ship a focused change");
    assert_eq!(stored["stage"]["current"], "intent");
    assert_eq!(stored["stage"]["status"], "active");
    assert!(!fixture.exists(".chough/session.json"));
    let session = fixture.session();
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
#[ignore = "plan step 10: rewritten with chough migrate"]
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

    // An unknown front matter key is kept (REQ-records-001); a key with the wrong type is
    // refused naming the session file and the key, with nothing written (REQ-cli-007).
    fixture.set_front("iteration", "many");
    let file = fixture.session_file();
    let before = fixture.read(&file);
    let refused = fixture.fails(&["context", "--json"]);
    assert!(
        refused.contains(&file) && refused.contains("iteration"),
        "{refused}"
    );
    assert_eq!(fixture.read(&file), before);
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
        transitions(&fixture)[1]["text"],
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

    // In 0.6.0 the move into done archives the session unless release is authorized
    // first (REQ-history-006), so the consent comes before done and is withdrawn there
    // to show the refusal.
    let release = [
        "consent",
        "grant",
        "release",
        "--source",
        "Human: please publish 0.2.0",
    ];
    fixture.ok(&release);
    for _ in 0..6 {
        fixture.ok(&["stage", "advance"]);
    }
    assert_eq!(stage(&fixture), "done");
    assert!(!fixture.is_archived("Ship a focused change"));
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("release is entered only")
    );
    fixture.ok(&["consent", "revoke", "release"]);
    assert!(
        fixture
            .fails(&["stage", "enter", "release", "--reason", "Ship it"])
            .contains("not authorized")
    );
    fixture.ok(&release);
    let entered = fixture.ok(&["stage", "enter", "release", "--reason", "Requested"]);
    assert!(
        entered["applied"]["authority"]
            .as_str()
            .unwrap()
            .starts_with("run consent")
    );

    // A new objective ends run consents; standing preferences were never written.
    fixture.ok(&["checkpoint", "--objective", "Next piece of work"]);
    assert_eq!(fixture.session()["run_consents"], json!([]));
    assert!(!fixture.exists(".chough/preferences.json"));

    // The move from release into done archives the session at done.
    fixture.ok(&["stage", "advance"]);
    assert!(fixture.is_archived("Ship a focused change"));
    assert_eq!(
        fixture.session_json("Ship a focused change")["stage"]["current"],
        "done"
    );
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
    // The move into done archives the session (REQ-history-006).
    assert!(fixture.is_archived("Ship a focused change"));
    assert_eq!(
        fixture.session_json("Ship a focused change")["stage"]["current"],
        "done"
    );
}

#[cfg(unix)]
#[test]
fn done_archived_change_recommends_new_cycle_and_preserves_precedence() {
    // After done the session is archived, so the next piece of work starts a new
    // session (REQ-cli-021, corrected in the 0.6.0 plan's Delta corrections).
    const NEW_CYCLE: &str = "Start the next piece of work in a new session: chough session new <key> --objective \"<outcome>\", then specsync change new \"<outcome>\" --spec <module> --path <path>, then chough checkpoint --native-change <id> --next-action \"<next step>\".";
    const RESET: &str = "chough session new <key> --objective \"<outcome>\"";

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
            fixture.read(&fixture.session_file()),
            session_bytes,
            "{command} rewrote the session"
        );
        report
    };

    fixture.write(".stub-count", "");
    let session_bytes = fixture.read(&fixture.session_file());
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
    let session_bytes = fixture.read(&fixture.session_file());
    for command in ["next", "status"] {
        assert_action(
            command,
            "The done stage is paused. Check native state and freshness, then `chough stage resume`.",
            &session_bytes,
        );
    }

    fixture.ok(&["stage", "resume"]);
    let session_bytes = fixture.read(&fixture.session_file());
    for command in ["next", "status"] {
        assert_action(
            command,
            "Waiting for a human decision: Human scope decision. Do not decide it for them; continue only independent work within scope. When they decide, record their words with `chough checkpoint --close-gate \"<gate>\" --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"`. `chough stage advance` and `chough stage skip` are refused until it is closed.",
            &session_bytes,
        );
    }

    fixture.ok(&["checkpoint", "--clear-awaiting-human"]);
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nprintf '%s\\n' '{\"summary\":{\"state\":\"archived\",\"terminal_evidence\":{\"validity\":\"authenticated-history\"}}}'\n",
    );
    let session_bytes = fixture.read(&fixture.session_file());
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
            fixture.read(&fixture.session_file()),
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
    let arguments = words(
        &command
            .replace("<key>", "follow-up")
            .replace("<outcome>", "Start approved follow-up"),
    );
    assert_eq!(arguments[0], "chough");
    let arguments: Vec<&str> = arguments[1..].iter().map(String::as_str).collect();
    fixture.ok(&arguments);
    assert_eq!(fixture.session()["objective"], "Start approved follow-up");
    assert_eq!(fixture.session()["stage"]["current"], "intent");
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
        "#!/bin/sh\nstate=$(cat .stub-state 2>/dev/null || echo draft)\nsize=20000\n[ -f .stub-huge ] && size=4200000\npad=$(head -c $size /dev/zero | tr '\\0' x)\nprintf '{\"summary\":{\"id\":\"%s\",\"state\":\"%s\",\"next_action\":\"stub next action\",\"approval_valid\":true,\"terminal_evidence\":{\"validity\":\"authenticated-history\"}},\"change\":{\"affected_specs\":[],\"no_spec_change\":true},\"acceptance_entries\":\"%s\"}\\n' \"$3\" \"$state\" \"$pad\"\n",
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
    // The move into done archives the session (REQ-history-006).
    assert!(fixture.is_archived("Ship a focused change"));
    assert_eq!(
        fixture.session_json("Ship a focused change")["stage"]["current"],
        "done"
    );
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
    // REQ-cli-021, corrected: a new session, not a reset (the plan's Delta corrections).
    assert!(
        text.starts_with(
            "Start the next piece of work in a new session: chough session new <key> --objective \"<outcome>\""
        ),
        "{text}"
    );
    for part in [
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
        "chough session new <key> --objective \"<outcome>\""
    );
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["next_valid_action"], action["action"]);
    assert_eq!(
        status["session_next_action"],
        "specsync change review old-change --reviewer 0xLeif, then ship"
    );
    let shown = String::from_utf8(fixture.run(&["next"]).stdout).unwrap();
    assert!(
        shown.contains("Next: Start the next piece of work in a new session"),
        "{shown}"
    );
    assert!(
        shown.contains("Native next action: merge the PR on GitHub"),
        "{shown}"
    );

    // The suggested command, its placeholders filled, starts the next piece of work in
    // a new session and leaves the finished one as it was.
    let finished = fixture.read(&fixture.session_file());
    let command = action["command"]
        .as_str()
        .unwrap()
        .replace("<key>", "focused-view")
        .replace("<outcome>", "New work: a focused view");
    let arguments = words(&command);
    assert_eq!(arguments[0], "chough");
    let arguments: Vec<&str> = arguments[1..].iter().map(String::as_str).collect();
    let before = fixture.session_file();
    fixture.ok(&arguments);
    assert_eq!(fixture.session()["objective"], "New work: a focused view");
    assert_eq!(fixture.session()["stage"]["current"], "intent");
    assert_eq!(fixture.read(&before), finished);
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
    assert_eq!(last["text"], "Different work");

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
    // A timeline entry is one line (REQ-history-002), so its text keeps the whole reason
    // with the line break folded to a space.
    assert_eq!(
        transitions(&fixture).pop().unwrap()["text"],
        reason.replace('\n', " ")
    );
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
    fixture.set_front("native_change", "\"\"");
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
        "chough session new <key> --objective \"<outcome>\""
    );

    fixture.write(".specsync/sdd.json", "{\"enabled\": false}\n");
    let next = fixture.ok(&["next", "--json"]);
    let text = next["next_valid_action"]["action"].as_str().unwrap();
    assert!(
        text.starts_with("Start the next piece of work in a new session: chough session new <key>"),
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
/// The active session record's bytes (its checkpoint and timeline) and the files under
/// `.chough/`, which a refused move must leave unchanged.
fn records(fixture: &Fixture) -> (Vec<u8>, Vec<String>) {
    fn walk(dir: &Path, root: &Path, out: &mut Vec<String>) {
        for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, root, out);
            } else {
                out.push(path.strip_prefix(root).unwrap().display().to_string());
            }
        }
    }
    let mut files = vec![];
    walk(&fixture.root().join(".chough"), fixture.root(), &mut files);
    files.sort();
    (
        fs::read(fixture.root().join(fixture.session_file())).unwrap(),
        files,
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
        // Unchanged deltas name only modules in this archive's approved scope, which may
        // be history alone (#96, #99), not always cli.
        let modules: Vec<&str> = scope["affected_specs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|module| module.as_str().unwrap())
            .collect();
        let unchanged = deltas
            .strip_prefix("unchanged: ")
            .map(|listed| listed.split(", ").all(|module| modules.contains(&module)));
        assert!(
            unchanged == Some(true) || deltas == "no semantic deltas (no_spec_change)",
            "{id}: {deltas} (approved scope {modules:?})"
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
        "specsync change approve guarded-change --actor \"<agent>-<terminal>\" --note \"Approved under <the human's words> (<source>); covered scope: <scope>; limits: <limits>. Recorded by <agent> (<model>, hc <terminal>).\"",
        "--drives approve",
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
    let approve = format!(
        "specsync change approve {id} --actor \"<agent>-<terminal>\" --note \"Approved under <the human's words> (<source>); covered scope: <scope>; limits: <limits>. Recorded by <agent> (<model>, hc <terminal>).\""
    );
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

/// Issue #38: with a gate awaiting a human, advance and skip are refused at intent,
/// design, plan (SDD disabled) and release, writing nothing; at most 3 gates are
/// named; clearing the gate allows the same move with its usual gate label.
#[test]
fn open_human_gates_refuse_advance_and_skip() {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(".specsync/sdd.json", "{\"enabled\": false}\n");
    fixture.init();
    let refused = |fixture: &Fixture, gate: &str| {
        // The session record holds the checkpoint and the timeline (REQ-history-002).
        let before = fixture.read(&fixture.session_file());
        for route in [
            vec!["stage", "advance"],
            vec!["stage", "skip", "--reason", "not needed"],
        ] {
            let error = fixture.fails(&route);
            for expected in [
                "refused:awaiting-human",
                gate,
                "chough checkpoint --close-gate \"<gate>\" --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"",
                "chough checkpoint --clear-awaiting-human",
            ] {
                assert!(error.contains(expected), "{route:?}: {expected}: {error}");
            }
        }
        let after = fixture.read(&fixture.session_file());
        assert!(before == after, "a refused move wrote records");
    };
    for (stage_name, gate) in [
        ("intent", "Intent approval of LIFE-1"),
        ("design", "Design approval"),
        ("plan", "Scope approval of x"),
    ] {
        assert_eq!(stage(&fixture), stage_name);
        fixture.ok(&["checkpoint", "--awaiting-human", gate]);
        refused(&fixture, gate);
        assert_eq!(stage(&fixture), stage_name);
        fixture.ok(&["checkpoint", "--clear-awaiting-human"]);
        let applied = fixture.ok(&["stage", "advance"]);
        if stage_name == "plan" {
            assert_eq!(applied["applied"]["gate"], "ungated:sdd-disabled");
        }
    }
    assert_eq!(stage(&fixture), "build");

    // Five gates: three named, two counted.
    let mut args = vec!["checkpoint"];
    let gates = ["Gate A", "Gate B", "Gate C", "Gate D", "Gate E"];
    for gate in &gates {
        args.extend(["--awaiting-human", gate]);
    }
    fixture.ok(&args);
    let error = fixture.fails(&["stage", "advance"]);
    assert!(
        error.contains("Gate A; Gate B; Gate C (and 2 more)"),
        "{error}"
    );
    assert!(!error.contains("Gate D"), "{error}");
    fixture.ok(&["checkpoint", "--clear-awaiting-human"]);

    // Release -> done. Release is authorized before the move into done, which otherwise
    // archives the session (REQ-history-006).
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "Human: publish it",
    ]);
    for _ in 0..3 {
        fixture.ok(&["stage", "advance"]);
    }
    assert_eq!(stage(&fixture), "done");
    fixture.ok(&["stage", "enter", "release", "--reason", "Requested"]);
    fixture.ok(&["checkpoint", "--awaiting-human", "Release approval"]);
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:awaiting-human")
    );
    assert_eq!(stage(&fixture), "release");
    fixture.ok(&["checkpoint", "--clear-awaiting-human"]);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(
        fixture.session_json("Ship a focused change")["stage"]["current"],
        "done"
    );
}

/// With a gate open, other moves still work, and next and status give the same
/// waiting action, which names `--close-gate` and says advance and skip are refused
/// until it is cleared.
#[test]
fn open_human_gates_leave_other_moves_and_agree_in_next() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["checkpoint", "--awaiting-human", "Design approval"]);
    let next = fixture.ok(&["next", "--json"]);
    let status = fixture.ok(&["status", "--json"]);
    let action = next["next_valid_action"]["action"].as_str().unwrap();
    assert!(
        action.starts_with("Waiting for a human decision: Design approval"),
        "{action}"
    );
    assert!(
        action.contains(
            "`chough stage advance` and `chough stage skip` are refused until it is closed"
        ),
        "{action}"
    );
    assert!(
        action.contains("`chough checkpoint --close-gate \"<gate>\""),
        "{action}"
    );
    assert_eq!(
        next["next_valid_action"]["action"],
        status["next_valid_action"]
    );
    fixture.ok(&["stage", "iterate", "--note", "Second prototype"]);
    fixture.ok(&["stage", "pause", "--reason", "Waiting"]);
    fixture.ok(&["stage", "resume"]);
    fixture.ok(&["stage", "enter", "bugs", "--reason", "Triage"]);
    fixture.ok(&["stage", "return"]);
    fixture.ok(&["stage", "back", "--to", "intent", "--reason", "Rethink"]);
    assert_eq!(stage(&fixture), "intent");
    fixture.ok(&["stage", "reset", "--reason", "Start over"]);
    assert_eq!(
        fixture.session()["awaiting_human"],
        json!(["Design approval"])
    );
}

/// A fixture at stage test with SDD not configured whose checkpoint references
/// TC-cli-0001 (issue #39).
fn at_test_with_case() -> Fixture {
    let fixture = Fixture::new();
    fixture.init();
    for _ in 0..4 {
        fixture.ok(&["stage", "advance"]);
    }
    assert_eq!(stage(&fixture), "test");
    fixture.ok(&["checkpoint", "--case", "TC-cli-0001"]);
    fixture
}

fn case_observation(fixture: &Fixture, id: &str, case: &str, outcome: &str, observed: u64) {
    let record = json!({
        "schema_version": 2,
        "id": id,
        "criteria": ["LIFE-1"],
        "requirements": ["REQ-cli-001"],
        "contracts": ["specs/cli/cli.spec.md"],
        "kind": "product-test",
        "outcome": outcome,
        "environment": "local",
        "summary": format!("fixture observation {id}"),
        "artifacts": [],
        "artifact_fingerprints": {},
        "observed_at_unix_seconds": observed,
        "snapshot": {"revision": "0".repeat(40), "dirty": false, "fingerprint": "0".repeat(64)},
        "provenance": "fixture",
        "cases": [{"id": case, "digest": "0".repeat(64)}],
    });
    fixture.write(
        &format!(".chough/evidence/{id}.json"),
        &serde_json::to_string(&record).unwrap(),
    );
}

fn linking_bug(fixture: &Fixture, id: &str, status: &str, scope: &str, observation: &str) {
    fixture.bug_md(id, scope, status, &[observation]);
}

const FAILED: &str = "OBS-fail-000000000000000000000001";

/// Issue #39: leaving test is refused, by advance and by skip, while a referenced
/// case's latest observation failed and no bug links it; next and status name the
/// same refusal; a later pass lifts it.
#[test]
fn leaving_test_refuses_an_unlinked_failed_case_until_it_passes() {
    let fixture = at_test_with_case();
    case_observation(
        &fixture,
        "OBS-pass-000000000000000000000001",
        "TC-cli-0001",
        "passed",
        100,
    );
    case_observation(&fixture, FAILED, "TC-cli-0001", "failed", 200);
    for route in [
        vec!["stage", "advance"],
        vec!["stage", "skip", "--reason", "nothing to accept"],
    ] {
        let error = fixture.fails(&route);
        for expected in [
            "refused:unlinked-failure",
            "TC-cli-0001",
            FAILED,
            "chough bug add",
            "chough bug update",
            &format!("--observation {FAILED}"),
        ] {
            assert!(
                error.contains(expected),
                "{route:?}: {expected} missing: {error}"
            );
        }
        assert_eq!(stage(&fixture), "test");
    }
    let next = fixture.ok(&["next", "--json"]);
    let status = fixture.ok(&["status", "--json"]);
    let action = next["next_valid_action"]["action"].as_str().unwrap();
    assert!(action.contains("refused:unlinked-failure"), "{action}");
    assert!(action.contains(FAILED), "{action}");
    assert_eq!(
        next["next_valid_action"]["action"],
        status["next_valid_action"]
    );
    // A later pass of the same case lifts the refusal.
    case_observation(
        &fixture,
        "OBS-pass-000000000000000000000002",
        "TC-cli-0001",
        "passed",
        300,
    );
    let next = fixture.ok(&["next", "--json"]);
    assert!(
        !next["next_valid_action"]["action"]
            .as_str()
            .unwrap()
            .contains("refused:unlinked-failure")
    );
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "accept");
}

/// PR #43 review: a later `not-run` observation executed nothing, so the latest
/// executed (passed or failed) observation decides; a not-run-only case does not
/// refuse, and a later pass still lifts the refusal.
#[test]
fn a_later_not_run_keeps_an_unlinked_failure_refused() {
    let fixture = at_test_with_case();
    case_observation(&fixture, FAILED, "TC-cli-0001", "failed", 100);
    case_observation(
        &fixture,
        "OBS-notrun-000000000000000000000001",
        "TC-cli-0001",
        "not-run",
        200,
    );
    for route in [
        vec!["stage", "advance"],
        vec!["stage", "skip", "--reason", "nothing to accept"],
    ] {
        let error = fixture.fails(&route);
        assert!(
            error.contains("refused:unlinked-failure"),
            "{route:?}: {error}"
        );
        assert!(error.contains(FAILED), "{route:?}: {error}");
        assert_eq!(stage(&fixture), "test");
    }
    let next = fixture.ok(&["next", "--json"]);
    let status = fixture.ok(&["status", "--json"]);
    let action = next["next_valid_action"]["action"].as_str().unwrap();
    assert!(action.contains("refused:unlinked-failure"), "{action}");
    assert!(action.contains(FAILED), "{action}");
    assert_eq!(
        next["next_valid_action"]["action"],
        status["next_valid_action"]
    );
    case_observation(
        &fixture,
        "OBS-pass-000000000000000000000003",
        "TC-cli-0001",
        "passed",
        300,
    );
    case_observation(
        &fixture,
        "OBS-notrun-000000000000000000000002",
        "TC-cli-0001",
        "not-run",
        400,
    );
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "accept");

    let fixture = at_test_with_case();
    case_observation(
        &fixture,
        "OBS-notrun-000000000000000000000003",
        "TC-cli-0001",
        "not-run",
        200,
    );
    let next = fixture.ok(&["next", "--json"]);
    assert!(
        !next["next_valid_action"]["action"]
            .as_str()
            .unwrap()
            .contains("refused:unlinked-failure")
    );
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "accept");
}

/// A bug of any status that links the failed observation lifts the refusal; an open
/// blocking bug keeps its own refusal first; a failure of an unreferenced case and a
/// failure followed by a pass do not refuse; equal times order by observation ID.
#[test]
fn a_linking_bug_lifts_the_unlinked_failure_refusal_and_other_moves_are_unchanged() {
    for status in [
        "open",
        "investigating",
        "fix-pending-verification",
        "verified",
        "deferred",
        "duplicate",
        "wont-fix",
    ] {
        let fixture = at_test_with_case();
        case_observation(&fixture, FAILED, "TC-cli-0001", "failed", 200);
        linking_bug(&fixture, "BUG-0001", status, "in-scope", FAILED);
        fixture.ok(&["stage", "advance"]);
        assert_eq!(stage(&fixture), "accept", "bug status {status}");
    }

    let fixture = at_test_with_case();
    case_observation(&fixture, FAILED, "TC-cli-0001", "failed", 200);
    linking_bug(
        &fixture,
        "BUG-0002",
        "open",
        "blocking",
        "OBS-other-000000000000000000000001",
    );
    let error = fixture.fails(&["stage", "advance"]);
    assert!(error.contains("refused:blocking-bugs"), "{error}");
    assert!(!error.contains("refused:unlinked-failure"), "{error}");

    let fixture = at_test_with_case();
    case_observation(&fixture, FAILED, "TC-cli-0002", "failed", 200);
    case_observation(
        &fixture,
        "OBS-pass-000000000000000000000001",
        "TC-cli-0001",
        "passed",
        100,
    );
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "accept");

    // Equal times: the greater ID is the latest.
    let fixture = at_test_with_case();
    case_observation(
        &fixture,
        "OBS-a-000000000000000000000001",
        "TC-cli-0001",
        "failed",
        200,
    );
    case_observation(
        &fixture,
        "OBS-b-000000000000000000000001",
        "TC-cli-0001",
        "passed",
        200,
    );
    fixture.ok(&["stage", "advance"]);
    let fixture = at_test_with_case();
    case_observation(
        &fixture,
        "OBS-a-000000000000000000000001",
        "TC-cli-0001",
        "passed",
        200,
    );
    case_observation(
        &fixture,
        "OBS-b-000000000000000000000001",
        "TC-cli-0001",
        "failed",
        200,
    );
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("OBS-b-000000000000000000000001")
    );

    // Other moves out of test are not gated by it.
    fixture.ok(&["stage", "back", "--to", "build", "--reason", "fix it"]);
    assert_eq!(stage(&fixture), "build");
}

/// Issue #40: a new project runs `specsync init`, which leaves SDD off. The approved-delta
/// line and the gate detail say to run `specsync change adopt`, so an agent does not take
/// `ungated:sdd-disabled` for a configured lifecycle; an unconfigured project and a
/// malformed policy read as before.
#[cfg(unix)]
#[test]
fn a_configured_project_with_sdd_off_says_to_run_specsync_change_adopt() {
    const HINT: &str =
        "not checked: native SDD is disabled; run `specsync change adopt` to gate stages";
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        next["native"]["approved_deltas"], "not checked: no native lifecycle",
        "{next}"
    );
    assert_eq!(next["gate"]["label"], "ungated:not-configured", "{next}");
    assert!(
        !next.to_string().contains("specsync change adopt"),
        "{next}"
    );

    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    for policy in [None, Some("{\"enabled\": false}\n")] {
        match policy {
            Some(policy) => fixture.write(".specsync/sdd.json", policy),
            None => assert!(!fixture.root().join(".specsync/sdd.json").exists()),
        }
        let next = fixture.ok(&["next", "--json"]);
        assert_eq!(
            next["native"]["approved_deltas"], HINT,
            "{policy:?}: {next}"
        );
        assert_eq!(next["gate"]["label"], "ungated:sdd-disabled", "{next}");
        assert!(
            next["gate"]["detail"]
                .as_str()
                .unwrap()
                .contains("Run `specsync change adopt` to gate stages."),
            "{next}"
        );
        let output = fixture.run(&["next"]);
        assert!(output.status.success());
        let shown = String::from_utf8(output.stdout).unwrap();
        assert!(
            shown.contains(&format!("Approved deltas: {HINT}")),
            "{shown}"
        );
        let status = fixture.ok(&["status"]);
        assert!(
            status.to_string().contains("specsync change adopt"),
            "{status}"
        );
    }

    fixture.write(".specsync/sdd.json", "{\"enabled\": \"yes\"}\n");
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        next["native"]["approved_deltas"], "not checked: SDD policy invalid",
        "{next}"
    );
    assert!(
        !next.to_string().contains("specsync change adopt"),
        "{next}"
    );
}

/// A native next action that already names the status command is not repeated: the
/// refusal names the command once and the `next` main line keeps only the gate's own
/// detail, so a long native next action no longer cuts it (issue #49).
#[cfg(unix)]
#[test]
fn native_next_action_is_shown_once_and_does_not_cut_the_next_line() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.ok(&["checkpoint", "--native-change", "add-thing"]);
    fixture.no_spec_approval("add-thing");
    let native_next = format!(
        "complete selected artifacts {}, then run `specsync change status add-thing`",
        "and their open decisions ".repeat(15)
    );
    fixture.write(".stub-next", &format!("{native_next}\n"));

    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("refused:native-draft"), "{refused}");
    assert!(refused.contains(&native_next), "{refused}");
    assert_eq!(
        refused.matches("specsync change status add-thing").count(),
        1,
        "{refused}"
    );

    let output = fixture.run(&["next"]);
    assert!(output.status.success());
    let shown = String::from_utf8(output.stdout).unwrap();
    let main = shown
        .lines()
        .find(|line| line.starts_with("Next: "))
        .unwrap();
    assert!(
        main.starts_with("Next: Gate not open (refused:native-draft): "),
        "{main}"
    );
    assert!(main.ends_with("it is draft."), "{main}");
    assert!(
        !main.contains("[cut]") && !main.contains("Native next action"),
        "{main}"
    );
    assert!(
        shown.contains(&format!("  Native next action: {native_next}")),
        "{shown}"
    );
    assert!(
        shown.contains("  Run: specsync change status add-thing"),
        "{shown}"
    );
    let status = fixture.ok(&["status", "--json"]);
    let detail = status["checkpoint"]["gate"]["detail"].as_str().unwrap();
    assert!(detail.contains(&native_next), "{detail}");
}

#[cfg(unix)]
#[test]
fn delta_guard_refusals_name_the_agent_actor_and_the_human_authority_note() {
    // TC-cli-delta-guard-refusals-name-the-agent-actor in 0.6.0 (REQ-cli-022): a remedy
    // that needs a new definition approval names the scope gate to open with
    // `--drives approve` first (REQ-cli-030), then, for authority given in advance, the
    // approval with the acting agent as `<agent>-<terminal>` and the human's earlier
    // words quoted (REQ-cli-029); it never asks the human to run a command.
    let agent = "--actor \"<agent>-<terminal>\" --note \"Approved under <the human's words> (<source>); covered scope: <scope>; limits: <limits>. Recorded by <agent> (<model>, hc <terminal>).\"";
    let check_refusal = |refused: &str, id: &str| {
        let gate = refused
            .find("--drives approve")
            .unwrap_or_else(|| panic!("no scope gate with --drives approve: {refused}"));
        let approve = format!("specsync change approve {id} {agent}");
        let approval = refused
            .find(&approve)
            .unwrap_or_else(|| panic!("missing {approve:?}: {refused}"));
        assert!(gate < approval, "the scope gate comes first: {refused}");
        assert!(
            refused.contains(&format!("specsync change check {id}")),
            "{refused}"
        );
        for stale in ["ask the human", "<their name>", "<agent-account>"] {
            assert!(!refused.contains(stale), "stale {stale:?}: {refused}");
        }
    };

    let id = "guarded-change";
    let fixture = approved_delta(id);
    fixture.write(
        &format!(".specsync/changes/{id}/deltas/cli.md"),
        &format!("{DELTA}\n### REQUIREMENT REQ-cli-091\n\nMore.\n"),
    );
    let before = records(&fixture);
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(
        refused.contains("refused:native-delta-changed"),
        "{refused}"
    );
    check_refusal(&refused, id);
    assert_eq!(records(&fixture), before);

    let id = "unverifiable-change";
    let fixture = approved_delta(id);
    fixture.write(
        &format!(".specsync/changes/{id}/approvals.json"),
        &json!({"approvals": [{"gate": "definition"}]}).to_string(),
    );
    let before = records(&fixture);
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(
        refused.contains("refused:native-delta-unverifiable"),
        "{refused}"
    );
    check_refusal(&refused, id);
    assert_eq!(records(&fixture), before);
}

const NEW_CHANGE: &str = "specsync change new \"<outcome>\" --spec <module> --path <path>";

/// Write a SYNTHETIC archive record for `id` finalized at `timestamp`.
#[cfg(unix)]
fn finalized(fixture: &Fixture, id: &str, timestamp: u64) {
    fixture.write(
        &format!(".specsync/archive/changes/2026-11-03-{id}/finalization.json"),
        &json!({"change_id": id, "timestamp": timestamp}).to_string(),
    );
}

/// Issue 76 item 2: at plan with SpecSync configured and no native change recorded, the
/// refused gate names `specsync change new`, not a bare status read.
#[cfg(unix)]
#[test]
fn plan_without_a_native_change_names_change_new() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fixture.ok(&[
        "stage",
        "reset",
        "--to",
        "plan",
        "--reason",
        "Plan the change",
    ]);
    fixture.ok(&["checkpoint", "--next-action", "Write the plan"]);
    fixture.ok(&["stage", "iterate", "--note", "Plan drafted"]);
    let next = fixture.ok(&["next", "--json"]);
    let action = &next["next_valid_action"];
    assert_eq!(action["command"], NEW_CHANGE, "{next}");
    let text = action["action"].as_str().unwrap();
    assert!(text.contains("refused:no-native-change"), "{text}");
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["next_valid_action"], action["action"]);
}

/// Issue 76 item 5: after a reset from done, the archived change from before the reset
/// is the previous cycle's, so `next` keeps the reset's guidance and names
/// `specsync change new`, at plan directly and after advancing from intent, and never
/// the archived refusal or another reset.
#[cfg(unix)]
#[test]
fn reset_after_an_archived_change_guides_to_a_new_change() {
    // Reset to plan: the reset's next action shows, with the new-change command.
    let fixture = done_fixture("old-change");
    finalized(&fixture, "old-change", 1_700_000_000);
    fixture.ok(&["stage", "reset", "--to", "plan", "--reason", "Next feature"]);
    let next = fixture.ok(&["next", "--json"]);
    let action = &next["next_valid_action"];
    let text = action["action"].as_str().unwrap();
    assert!(
        text.starts_with("Continue plan: New cycle from done after reset: Next feature."),
        "{text}"
    );
    assert_eq!(action["command"], NEW_CHANGE, "{action}");
    assert!(!text.contains("refused:native-archived"), "{text}");
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["next_valid_action"], action["action"]);

    // Reset to intent, then advance: no loop back to another reset.
    let fixture = done_fixture("old-change");
    finalized(&fixture, "old-change", 1_700_000_000);
    fixture.ok(&[
        "stage",
        "reset",
        "--to",
        "intent",
        "--reason",
        "Next feature",
    ]);
    let mut stages = vec![];
    while stage(&fixture) != "plan" {
        fixture.ok(&["stage", "advance"]);
        let next = fixture.ok(&["next", "--json"]);
        let action = &next["next_valid_action"];
        let text = action["action"].as_str().unwrap().to_string();
        assert!(!text.contains("stage reset"), "{}: {text}", stage(&fixture));
        assert_ne!(
            action["command"],
            RESET_COMMAND,
            "{}: {action}",
            stage(&fixture)
        );
        stages.push((stage(&fixture), action.clone()));
    }
    let (_, at_plan) = stages.last().unwrap();
    assert_eq!(at_plan["command"], NEW_CHANGE, "{at_plan}");
    assert!(
        at_plan["action"]
            .as_str()
            .unwrap()
            .contains("New cycle from done after reset: Next feature."),
        "{at_plan}"
    );

    // A change finalized after the latest reset is this cycle's: the archived-change
    // guidance stays.
    let fixture = done_fixture("new-change");
    fixture.ok(&[
        "stage",
        "reset",
        "--to",
        "intent",
        "--reason",
        "Next feature",
    ]);
    fixture.ok(&["stage", "advance"]);
    finalized(&fixture, "new-change", 4_000_000_000);
    let next = fixture.ok(&["next", "--json"]);
    let text = next["next_valid_action"]["action"].as_str().unwrap();
    assert!(
        text.starts_with("The recorded native change is archived while the checkpoint is at"),
        "{text}"
    );
    assert_eq!(next["next_valid_action"]["command"], RESET_COMMAND);
}

/// Review of #78: the previous-cycle rule uses the newest reset by recorded time, not
/// the last one appended; merged transition logs hold resets out of order.
#[cfg(unix)]
#[test]
fn reset_rule_uses_the_newest_reset_by_time_in_an_out_of_order_log() {
    let fixture = done_fixture("old-change");
    fixture.ok(&[
        "stage",
        "reset",
        "--to",
        "intent",
        "--reason",
        "Next feature",
    ]);
    let file = fixture.session_file();
    let record = fixture.read(&file);
    let reset = transitions(&fixture).pop().unwrap();
    assert_eq!(reset["kind"], "reset", "{reset}");
    let at = unix_of(reset["time"].as_str().unwrap());
    finalized(&fixture, "old-change", at);
    // An older reset entry after the newest one, as a hand edit or a merge can leave it.
    let line = record
        .lines()
        .rfind(|line| line.contains(" · reset · "))
        .unwrap();
    let older = line
        .replacen(reset["time"].as_str().unwrap(), &iso_of(at - 100), 1)
        .replace(reset["id"].as_str().unwrap(), "t-0ff5e7");
    fixture.write(
        &file,
        &record.replacen(line, &format!("{line}\n{older}"), 1),
    );
    assert_eq!(
        transitions(&fixture).len(),
        timeline_entries(&record).len() + 1
    );
    while stage(&fixture) != "plan" {
        fixture.ok(&["stage", "advance"]);
    }
    let next = fixture.ok(&["next", "--json"]);
    let action = &next["next_valid_action"];
    let text = action["action"].as_str().unwrap();
    assert!(!text.contains("is archived while"), "{text}");
    assert_eq!(action["command"], NEW_CHANGE, "{action}");
}

/// Leif's review thread on #78: on a project with event history (the cursor `chough
/// init` writes), the reset that began this cycle is found even after more than the
/// 128 recent events `next` shows have been recorded since it, so the archived change
/// stays the previous cycle's and `next` keeps naming a new native change.
/// A project with event history (the cursor `chough init` writes) whose archived
/// `old-change` closed before a reset to plan, followed by `iterations` iterations.
#[cfg(unix)]
fn reset_then_iterated(iterations: usize) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("archived");
    fixture.write(".stub-validity", "exact\n");
    fixture.write(".stub-next", "merge the PR on GitHub if it is still open\n");
    fixture.ok(&[
        "init",
        "--objective",
        "Ship a focused change",
        "--intent-scope",
        "LIFE",
        "--module",
        "cli",
    ]);
    fixture.ok(&["checkpoint", "--native-change", "old-change"]);
    fixture.ok(&["stage", "reset", "--to", "done", "--reason", "Closeout"]);
    finalized(&fixture, "old-change", 1_700_000_000);
    fixture.ok(&["stage", "reset", "--to", "plan", "--reason", "Next feature"]);
    for count in 0..iterations {
        fixture.ok(&["stage", "iterate", "--note", &format!("iteration {count}")]);
    }
    fixture
}

/// Leif's review thread on #78: on a project with event history, the reset that began
/// this cycle is found even after more than the 128 recent events `next` shows have
/// been recorded since it, so the archived change stays the previous cycle's and
/// `next` keeps naming a new native change.
#[cfg(unix)]
#[test]
fn a_reset_beyond_the_recent_history_window_still_starts_this_cycle() {
    let fixture = reset_then_iterated(130);
    let next = fixture.ok(&["next", "--json"]);
    let action = &next["next_valid_action"];
    let text = action["action"].as_str().unwrap();
    assert!(!text.contains("is archived while"), "{text}");
    assert_eq!(action["command"], NEW_CHANGE, "{action}");
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["next_valid_action"], text);
}

/// The same thread: when the current cycle's history cannot be read whole, `next` says
/// the cycle is unknown instead of inferring that the archived change closed this cycle
/// (a reset) or the previous one (a new change).
#[cfg(unix)]
#[test]
fn an_unreadable_cycle_start_is_reported_as_unknown() {
    let fixture = reset_then_iterated(3);
    // The reset entry that began this cycle cannot be read: its separators and entry ID
    // are gone, so the timeline cannot be read whole (REQ-history-002).
    let file = fixture.session_file();
    let record = fixture.read(&file);
    let line = record
        .lines()
        .rfind(|line| line.contains(" · reset · "))
        .unwrap()
        .to_string();
    fixture.write(
        &file,
        &record.replacen(&line, "- reset to plan, by hand", 1),
    );
    let next = fixture.ok(&["next", "--json"]);
    let action = &next["next_valid_action"];
    let text = action["action"].as_str().unwrap();
    assert!(
        text.starts_with("Unknown whether native change old-change belongs to this cycle"),
        "{text}"
    );
    assert_eq!(action["command"], "chough history show", "{action}");
    assert_ne!(action["command"], RESET_COMMAND, "{action}");
    assert_ne!(action["command"], NEW_CHANGE, "{action}");
}

#[cfg(unix)]
const RESET_COMMAND: &str = "chough stage reset --to intent --reason \"<why>\"";

/// Issue 63 (Kimi lab round 2): with SpecSync configured, `checkpoint --native-change`
/// asks SpecSync for the change before saving and refuses an ID it does not know,
/// leaving the checkpoint byte-identical, even when other updates ride along.
#[cfg(unix)]
#[test]
fn checkpoint_refuses_a_native_change_specsync_does_not_know() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    // SpecSync 6.1.2 exits non-zero for an unknown change; the stub knows every other ID.
    fs::rename(
        fixture.root().join("bin/specsync"),
        fixture.root().join("bin/specsync-known"),
    )
    .unwrap();
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nif [ \"$3\" = does-not-exist ]; then\n  echo 'change does-not-exist was not found' >&2; exit 1\nfi\nexec \"$(dirname \"$0\")/specsync-known\" \"$@\"\n",
    );
    fixture.executable("bin/specsync");
    let before = fixture.read(&fixture.session_file());

    let refused = fixture.fails(&["checkpoint", "--native-change", "does-not-exist"]);
    assert!(
        refused.contains("refused:native-change-unknown"),
        "{refused}"
    );
    assert!(refused.contains("does-not-exist"), "{refused}");
    assert!(
        refused.contains("specsync change status does-not-exist"),
        "{refused}"
    );
    assert_eq!(fixture.read(&fixture.session_file()), before);

    let refused = fixture.fails(&[
        "checkpoint",
        "--native-change",
        "does-not-exist",
        "--decision",
        "Rides along with a bad change",
    ]);
    assert!(
        refused.contains("refused:native-change-unknown"),
        "{refused}"
    );
    assert_eq!(fixture.read(&fixture.session_file()), before);

    // A SpecSync failure is not proof the change exists: refused, nothing saved.
    fixture.write(".stub-fail", "");
    let refused = fixture.fails(&["checkpoint", "--native-change", "add-thing"]);
    assert!(
        refused.contains("refused:native-change-unknown"),
        "{refused}"
    );
    assert_eq!(fixture.read(&fixture.session_file()), before);
    fs::remove_file(fixture.root().join(".stub-fail")).unwrap();

    let saved = fixture.ok(&["checkpoint", "--native-change", "add-thing"]);
    assert_eq!(saved["saved"], true);
    assert_eq!(fixture.session()["native_change"], "add-thing");
}

/// A reply about a different change is not proof the requested change exists: refused
/// with the checkpoint unchanged (#71 review). A reply naming no change is left to the
/// stage gates, as before.
#[cfg(unix)]
#[test]
fn checkpoint_refuses_a_native_reply_that_names_a_different_change() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fs::rename(
        fixture.root().join("bin/specsync"),
        fixture.root().join("bin/specsync-known"),
    )
    .unwrap();
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\ncase \"$3\" in\n  alias-of-add-thing) printf '%s\\n' '{\"change\":{\"id\":\"add-thing\"},\"summary\":{\"id\":\"add-thing\",\"state\":\"draft\"}}' ;;\n  split-reply) printf '%s\\n' '{\"change\":{\"id\":\"add-thing\"},\"summary\":{\"id\":\"split-reply\",\"state\":\"draft\"}}' ;;\n  nameless) printf '%s\\n' '{\"summary\":{\"state\":\"draft\"}}' ;;\n  *) exec \"$(dirname \"$0\")/specsync-known\" \"$@\" ;;\nesac\n",
    );
    fixture.executable("bin/specsync");
    let before = fixture.read(&fixture.session_file());
    for (id, named) in [
        ("alias-of-add-thing", "add-thing"),
        ("split-reply", "add-thing"),
    ] {
        let refused = fixture.fails(&["checkpoint", "--native-change", id]);
        assert!(
            refused.contains("refused:native-change-mismatch"),
            "{id}: {refused}"
        );
        assert!(refused.contains(id), "{id}: {refused}");
        assert!(refused.contains(named), "{id}: {refused}");
        assert_eq!(fixture.read(&fixture.session_file()), before, "{id}");
    }
    let saved = fixture.ok(&["checkpoint", "--native-change", "add-thing"]);
    assert_eq!(saved["saved"], true);
}

/// Review thread on #71: a JSON reply that names no change ID does not identify the
/// requested change, so it refuses like an unknown one and nothing is saved.
#[cfg(unix)]
#[test]
fn checkpoint_refuses_a_native_reply_that_names_no_change() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fs::rename(
        fixture.root().join("bin/specsync"),
        fixture.root().join("bin/specsync-known"),
    )
    .unwrap();
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\ncase \"$3\" in\n  empty-object) printf '%s\\n' '{}' ;;\n  empty-array) printf '%s\\n' '[]' ;;\n  nameless) printf '%s\\n' '{\"summary\":{\"state\":\"draft\"}}' ;;\n  *) exec \"$(dirname \"$0\")/specsync-known\" \"$@\" ;;\nesac\n",
    );
    fixture.executable("bin/specsync");
    let before = fixture.read(&fixture.session_file());
    for id in ["empty-object", "empty-array", "nameless"] {
        for extra in [vec![], vec!["--decision", "decided: ride along"]] {
            let mut args = vec!["checkpoint", "--native-change", id];
            args.extend(extra);
            let refused = fixture.fails(&args);
            assert!(
                refused.contains("refused:native-change-unknown"),
                "{id}: {refused}"
            );
            assert!(refused.contains(id), "{id}: {refused}");
            assert_eq!(fixture.read(&fixture.session_file()), before, "{id}");
        }
    }
    let saved = fixture.ok(&["checkpoint", "--native-change", "add-thing"]);
    assert_eq!(saved["saved"], true);
}

/// Without a SpecSync configuration there is nothing to ask, so the format check alone
/// still decides (issue 63 leaves non-native projects unchanged).
#[test]
fn checkpoint_native_change_without_specsync_keeps_the_format_check() {
    let fixture = Fixture::new();
    fixture.init();
    assert_eq!(
        fixture.ok(&["checkpoint", "--native-change", "any-change"])["saved"],
        true
    );
    assert!(
        fixture
            .fails(&["checkpoint", "--native-change", "bad id"])
            .contains("invalid native change ID")
    );
}

// ---------------------------------------------------------------------------
// Backfilled moves (REQ-cli-036, REQ-history-005).

/// Seconds since the epoch of a timeline time, `YYYY-MM-DDTHH:MM:SSZ` (UTC).
fn unix_of(time: &str) -> u64 {
    let number = |range: std::ops::Range<usize>| time[range].parse::<i64>().unwrap();
    let (year, month, day) = (number(0..4), number(5..7), number(8..10));
    let (year, month) = if month <= 2 {
        (year - 1, month + 9)
    } else {
        (year, month - 3)
    };
    let era = year.div_euclid(400);
    let of_era = year - era * 400;
    let of_year = (153 * month + 2) / 5 + day - 1;
    let of_era_days = of_era * 365 + of_era / 4 - of_era / 100 + of_year;
    let days = era * 146_097 + of_era_days - 719_468;
    (days * 86_400 + number(11..13) * 3_600 + number(14..16) * 60 + number(17..19)) as u64
}

/// The timeline time of `seconds` since the epoch, `YYYY-MM-DDTHH:MM:SSZ` (UTC).
fn iso_of(seconds: u64) -> String {
    let days = (seconds / 86_400) as i64 + 719_468;
    let era = days.div_euclid(146_097);
    let of_era = days - era * 146_097;
    let year_of_era = (of_era - of_era / 1_460 + of_era / 36_524 - of_era / 146_096) / 365;
    let of_year = of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * of_year + 2) / 153;
    let day = of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    let rest = seconds % 86_400;
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rest / 3_600,
        rest / 60 % 60,
        rest % 60
    )
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn rev_parse(fixture: &Fixture, rev: &str) -> String {
    let output = Command::new("git")
        .args(["rev-parse", rev])
        .current_dir(fixture.root())
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

/// A legacy project at plan with an approved native change whose earlier transitions
/// happened about `shift` seconds ago, 100 seconds apart, inside a Git repository.
#[cfg(unix)]
fn backfill_fixture(state: &str, shift: u64) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native(state);
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.ok(&["checkpoint", "--native-change", "add-thing"]);
    fixture.no_spec_approval("add-thing");
    // Move each timeline entry's time `shift` seconds back, 100 seconds apart.
    let file = fixture.session_file();
    let mut record = fixture.read(&file);
    for (index, entry) in transitions(&fixture).iter().enumerate() {
        let time = entry["time"].as_str().unwrap();
        let line = format!("- {time} · {} · ", entry["kind"].as_str().unwrap());
        let moved = iso_of(unix_of(time) - shift + 100 * index as u64);
        let at = record.find(&line).unwrap();
        record.replace_range(at + 2..at + 2 + time.len(), &moved);
    }
    fixture.write(&file, &record);
    fixture.commit("base");
    fixture
}

#[cfg(unix)]
#[test]
fn a_backfilled_move_keeps_its_original_time_and_quoted_evidence() {
    let fixture = backfill_fixture("approved", 10_000);
    let before = transitions(&fixture);
    let at = unix_now() - 5_000;
    let short = rev_parse(&fixture, "HEAD")[..12].to_string();
    fixture.write(".stub-count", "");
    fixture.native_calls();
    let quote = "Leif, 4:10 PM: yes, build it in a subagent";
    let out = fixture.ok(&[
        "stage",
        "backfill",
        "--at",
        &at.to_string(),
        "--commit",
        &short,
        "--quote",
        quote,
        "--note",
        "Built in a subagent",
    ]);
    assert!(fixture.native_calls() > 0, "the native gate is read now");
    fs::remove_file(fixture.root().join(".stub-count")).unwrap();
    assert_eq!(out["applied"]["kind"], "advance");
    assert_eq!(out["applied"]["to"], "build");
    assert_eq!(out["applied"]["gate"], "native:approved");
    let evidence = out["applied"]["backfill"]["evidence"].as_str().unwrap();
    assert!(evidence.contains("evidence only"), "{evidence}");
    assert!(
        evidence.contains("no human approval or review"),
        "{evidence}"
    );
    assert_eq!(stage(&fixture), "build");

    // Earlier entries are unchanged; the new one carries the past time and evidence
    // (REQ-history-005), which `stage show` reports.
    let after = transitions(&fixture);
    assert_eq!(after[..before.len()], before[..]);
    let entry = after.last().unwrap();
    assert_eq!(entry["kind"], "advance");
    assert_eq!(unix_of(entry["time"].as_str().unwrap()), at);
    assert_eq!(entry["text"], "Built in a subagent");
    let text = fixture.read(&fixture.session_file());
    let timeline = &text[text.find("\n## Timeline\n").unwrap()..];
    assert!(timeline.contains(quote), "{timeline}");
    assert!(
        timeline.contains(&rev_parse(&fixture, "HEAD")),
        "{timeline}"
    );
    let record = fixture.ok(&["stage", "show"])["recent_transitions"]
        .as_array()
        .unwrap()
        .last()
        .unwrap()
        .clone();
    assert_eq!(record["at_unix_seconds"], at);
    assert_eq!(record["reason"], "Built in a subagent");
    assert_eq!(record["backfill"]["commit"], rev_parse(&fixture, "HEAD"));
    assert_eq!(record["backfill"]["quote"], quote);
    let recorded = record["backfill"]["recorded_at_unix_seconds"]
        .as_u64()
        .unwrap();
    assert!(recorded >= at + 5_000 - 5 && recorded <= unix_now());
    for earlier in &before {
        for field in ["backfill", "recorded_at", "commit", "quote"] {
            assert!(earlier.get(field).is_none(), "{earlier}");
        }
    }

    // Nothing human is recorded.
    let session = fixture.session();
    assert_eq!(session["awaiting_human"], json!([]));
    assert_eq!(session["run_consents"], json!([]));
    assert!(!fixture.exists(".chough/preferences.json"));

    // Timing lists it separately and marks the segments it bounds.
    let report = fixture.ok(&["timing", "--json"]);
    let listed = report["backfilled"].as_array().unwrap();
    assert_eq!(listed.len(), 1, "{listed:?}");
    assert_eq!(listed[0]["from"], "plan");
    assert_eq!(listed[0]["to"], "build");
    assert_eq!(listed[0]["at"]["unix_seconds"], at);
    assert_eq!(listed[0]["recorded_at"]["unix_seconds"], recorded);
    assert_eq!(listed[0]["commit"], record["backfill"]["commit"]);
    assert_eq!(listed[0]["quote"], quote);
    assert!(report["labels"]["backfilled"].as_str().is_some());
    let segments = report["cycles"][0]["segments"].as_array().unwrap();
    let marked: Vec<&str> = segments
        .iter()
        .filter(|segment| segment["backfilled"] == true)
        .map(|segment| segment["stage"].as_str().unwrap())
        .collect();
    assert_eq!(marked, ["plan", "build"]);
    assert!(
        segments
            .iter()
            .filter(|segment| segment["stage"] == "design")
            .all(|segment| segment["backfilled"] == false)
    );
    let plan = report["cycles"][0]["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|total| total["stage"] == "plan")
        .unwrap()
        .clone();
    assert_eq!(plan["backfilled"]["seconds"], plan["stage_time"]["seconds"]);
    let text = String::from_utf8(fixture.run(&["timing"]).stdout).unwrap();
    assert!(text.contains("Backfilled"), "{text}");

    // Stages a backfilled move bounds are not estimate samples once the cycle ends.
    fixture.write(".stub-state", "verifying\n");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.write(".stub-state", "archived\n");
    fixture.ok(&["stage", "advance"]);
    // The move into done archives the session (REQ-history-006); timing reads the
    // archived story's timeline (REQ-reporting-001).
    assert_eq!(
        fixture.session_json("Ship a focused change")["stage"]["current"],
        "done"
    );
    let report = fixture.ok(&["timing", "--json"]);
    assert_eq!(report["cycles"][0]["completed"], true);
    let sampled: Vec<&str> = report["estimates"]["stages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|estimate| estimate["stage"].as_str().unwrap())
        .collect();
    assert!(sampled.contains(&"design"), "{sampled:?}");
    assert!(!sampled.contains(&"plan"), "{sampled:?}");
    assert!(!sampled.contains(&"build"), "{sampled:?}");
}

#[cfg(unix)]
#[test]
fn backfill_refuses_what_advance_or_its_evidence_does_not_support() {
    let fixture = backfill_fixture("draft", 10_000);
    let head = rev_parse(&fixture, "HEAD");
    let at = (unix_now() - 5_000).to_string();
    let refused = |args: &[&str], expected: &str| {
        // The session record holds the checkpoint and the timeline (REQ-history-002).
        let session = fixture.read(&fixture.session_file());
        let mut route = vec!["stage", "backfill"];
        route.extend_from_slice(args);
        let error = fixture.fails(&route);
        assert!(error.contains(expected), "{args:?}: {error}");
        assert_eq!(fixture.read(&fixture.session_file()), session, "{args:?}");
    };
    let ok = ["--commit", head.as_str(), "--quote", "Leif: yes"];
    let with = |at: &str, rest: &[&str]| -> Vec<String> {
        let mut args = vec!["--at".to_string(), at.to_string()];
        args.extend(rest.iter().map(|arg| arg.to_string()));
        args
    };
    let call = |args: Vec<String>, expected: &str| {
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        refused(&args, expected);
    };

    // Native state does not support build.
    call(with(&at, &ok), "refused:native-draft");
    fixture.write(".stub-state", "approved\n");

    // Times: future, before the latest transition, unparseable.
    call(
        with(&(unix_now() + 3_600).to_string(), &ok),
        "is in the future",
    );
    call(
        with("2020-01-01T00:00:00Z", &ok),
        "before the latest recorded transition",
    );
    call(with("yesterday", &ok), "--at");

    // Evidence: unknown commit, unreachable commit, blank quote.
    call(
        with(&at, &["--commit", "deadbeef", "--quote", "Leif: yes"]),
        "does not resolve",
    );
    fixture.git(&["checkout", "-q", "-b", "elsewhere"]);
    fixture.git(&["commit", "-q", "--allow-empty", "-m", "elsewhere"]);
    let elsewhere = rev_parse(&fixture, "HEAD");
    fixture.git(&["checkout", "-q", "-"]);
    call(
        with(&at, &["--commit", &elsewhere, "--quote", "Leif: yes"]),
        "not an ancestor of HEAD",
    );
    call(
        with(&at, &["--commit", &head, "--quote", "   "]),
        "quote must not be blank",
    );

    // Every advance refusal still applies: an open human gate and a side stage.
    // The gate is recorded now, so these backfill a move at the current time.
    fixture.ok(&["checkpoint", "--awaiting-human", "Scope approval"]);
    call(with(&unix_now().to_string(), &ok), "refused:awaiting-human");
    fixture.ok(&["checkpoint", "--clear-awaiting-human"]);
    fixture.ok(&["stage", "enter", "bugs", "--reason", "Capture"]);
    call(with(&unix_now().to_string(), &ok), "return");
    fixture.ok(&["stage", "return"]);

    // Still nothing human was recorded, and the gate refused as advance would.
    assert!(!fixture.exists(".chough/preferences.json"));
    assert_eq!(fixture.session()["run_consents"], json!([]));
    assert!(
        transitions(&fixture)
            .iter()
            .all(|record| record.get("backfill").is_none())
    );
}

#[test]
fn a_backfilled_move_is_carried_by_history_events() {
    let fixture = Fixture::new();
    fixture.ok(&[
        "init",
        "--objective",
        "Backfill with history",
        "--module",
        "cli",
    ]);
    fixture.commit("base");
    let latest = fixture.ok(&["stage", "show"])["recent_transitions"]
        .as_array()
        .unwrap()
        .last()
        .map(|record| record["at_unix_seconds"].as_u64().unwrap())
        .unwrap_or(0);
    let at = latest.max(unix_now() - 60);
    fixture.ok(&[
        "stage",
        "backfill",
        "--at",
        &at.to_string(),
        "--commit",
        "HEAD",
        "--quote",
        "Leif: intent approved",
    ]);
    let shown = fixture.ok(&["stage", "show"]);
    let record = shown["recent_transitions"]
        .as_array()
        .unwrap()
        .last()
        .unwrap()
        .clone();
    assert_eq!(record["at_unix_seconds"], at);
    assert_eq!(record["backfill"]["quote"], "Leif: intent approved");
    assert_eq!(record["backfill"]["commit"], rev_parse(&fixture, "HEAD"));
    // The session's timeline carries it (REQ-history-002, REQ-history-005).
    let entry = transitions(&fixture).pop().unwrap();
    assert_eq!(unix_of(entry["time"].as_str().unwrap()), at);
    assert!(
        fixture
            .read(&fixture.session_file())
            .contains("Leif: intent approved")
    );
    let report = fixture.ok(&["timing", "--json"]);
    assert_eq!(report["backfilled"].as_array().unwrap().len(), 1);
}

/// Release to done is never backfilled: it is advanced when it happens (review of #73).
#[test]
fn backfill_refuses_release_to_done() {
    let fixture = Fixture::new();
    fixture.init();
    // Release is authorized before the move into done, which otherwise archives the
    // session (REQ-history-006).
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "Human: please publish 0.2.0",
    ]);
    for _ in 0..6 {
        fixture.ok(&["stage", "advance"]);
    }
    fixture.ok(&["stage", "enter", "release", "--reason", "Requested"]);
    assert_eq!(stage(&fixture), "release");
    fixture.commit("base");
    let session = fixture.read(&fixture.session_file());
    let refused = fixture.fails(&[
        "stage",
        "backfill",
        "--at",
        &unix_now().to_string(),
        "--commit",
        "HEAD",
        "--quote",
        "Leif: it shipped",
    ]);
    assert!(
        refused.contains("release to done cannot be backfilled"),
        "{refused}"
    );
    assert_eq!(fixture.read(&fixture.session_file()), session);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(
        fixture.session_json("Ship a focused change")["stage"]["current"],
        "done"
    );
}

/// Every checkpoint store in the project, as relative paths: session records (a
/// `kind: session` front matter) and any 0.5.x-shaped JSON checkpoint (schema version 2
/// with the checkpoint's objective, stage and native change fields).
#[cfg(unix)]
fn checkpoint_files(fixture: &Fixture) -> Vec<String> {
    let mut found = vec![];
    let mut pending = vec![fixture.root().to_path_buf()];
    while let Some(path) = pending.pop() {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if path.is_dir() {
            if name != ".git" && name != "bin" {
                pending.extend(
                    fs::read_dir(&path)
                        .unwrap()
                        .map(|entry| entry.unwrap().path()),
                );
            }
            continue;
        }
        let bytes = fs::read(&path).unwrap();
        let session_record = bytes.starts_with(b"---\nkind: session\n");
        let json_checkpoint = serde_json::from_slice::<Value>(&bytes).is_ok_and(|value| {
            value["schema_version"] == 2
                && ["objective", "stage", "native_change"]
                    .iter()
                    .all(|field| value.get(field).is_some())
        });
        if session_record || json_checkpoint {
            let relative = path.strip_prefix(fixture.root()).unwrap();
            found.push(relative.to_string_lossy().into_owned());
        }
    }
    found.sort();
    found
}

/// REQ-cli-007, fourth criterion (#138, audit assertion review), in the 0.6.0 layout: the
/// one checkpoint store is the active session's `session.md` (REQ-cli-037) and no adapter
/// such as APS is required. An `aps` stand-in that records any call is never run while
/// checkpoints are saved, stages move and reports read; the only checkpoint stores are the
/// session records, and a write changes only the active session.
#[cfg(unix)]
#[test]
fn one_checkpoint_store_and_no_aps_adapter_in_either_mode() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write("bin/aps", "#!/bin/sh\necho \"$*\" >>.aps-calls\nexit 1\n");
    fixture.executable("bin/aps");
    fixture.init();
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "decided: one store because REQ-cli-007 says so",
    ]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "no visuals change"]);
    assert!(fixture.run(&["next"]).status.success());
    fixture.ok(&["status"]);
    let first_file = fixture.session_file();
    assert_eq!(
        checkpoint_files(&fixture),
        std::slice::from_ref(&first_file)
    );
    assert!(!fixture.exists(".chough/session.json"));

    // A second session gets its own record; a checkpoint write changes only the active one.
    fixture.ok(&[
        "session",
        "new",
        "second",
        "--objective",
        "Second objective",
    ]);
    let first = fixture.read(&first_file);
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "decided: only the active session changes because it is the one store",
    ]);
    assert_eq!(fixture.read(&first_file), first);
    assert!(
        fixture
            .read(&fixture.session_file_of("second"))
            .contains("only the active session changes")
    );
    fixture.ok(&["session", "use", "ship-a-focused-change"]);
    fixture.ok(&["stage", "advance"]);
    assert!(fixture.run(&["next"]).status.success());
    let mut expected = vec![first_file.clone(), fixture.session_file_of("second")];
    expected.sort();
    assert_eq!(
        checkpoint_files(&fixture),
        expected,
        "a checkpoint store other than the session records"
    );
    assert!(
        !fixture.exists(".aps-calls"),
        "an APS adapter was run: {}",
        fixture.read(".aps-calls")
    );
}

/// BUG-0020 (REQ-cli-022): with three definition approvals from a SpecSync workflow
/// version below 2, where the second matches the current definition digest and the
/// third is the latest, the guard compares the deltas with the second, as SpecSync
/// treats it as effective.
#[cfg(unix)]
#[test]
fn the_delta_guard_compares_with_the_effective_approval() {
    let id = "effective-change";
    let fixture = approved_delta(id);
    fixture.write(
        &format!(".specsync/changes/{id}/state.json"),
        &json!({"id": id, "state": "approved", "workflow_version": 1}).to_string(),
    );
    let good = delta_digest("cli", DELTA.as_bytes());
    let stale = "0".repeat(64);
    let ledger = |second: &str, third: &str| {
        json!({
            "approvals": [
                {"gate": "definition", "actor": "SYNTHETIC", "digest": "d-first", "approved_delta_digests": {"cli": stale}},
                {"gate": "definition", "actor": "SYNTHETIC", "digest": "d-current", "approved_delta_digests": {"cli": second}},
                {"gate": "definition", "actor": "SYNTHETIC", "digest": "d-later", "approved_delta_digests": {"cli": third}},
            ],
            "reopenings": [],
        })
        .to_string()
    };
    let approvals = format!(".specsync/changes/{id}/approvals.json");
    fixture.write(".stub-definition-digest", "d-current");
    // The effective (second) approval matches the delta and the latest does not.
    fixture.write(&approvals, &ledger(&good, &stale));
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "build");
    // The other way round: the latest matches, the effective one does not.
    fixture.ok(&["stage", "back", "--to", "plan", "--reason", "Again"]);
    fixture.write(&approvals, &ledger(&stale, &good));
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(
        refused.contains("refused:native-delta-changed"),
        "{refused}"
    );
    assert_eq!(stage(&fixture), "plan");
}
