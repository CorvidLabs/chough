//! The focused `chough next` view, its bounds and the one native read it shares with
//! `chough status` (REQ-cli-021, REQ-cli-003).

mod common;

use common::Fixture;
use serde_json::{Value, json};

/// The byte cap of each form of the next view.
const NEXT_LIMIT: usize = 16_384;

fn text(fixture: &Fixture, args: &[&str]) -> String {
    let output = fixture.run(args);
    assert!(
        output.status.success(),
        "{args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// Every string anywhere in a JSON value.
fn strings(value: &Value, found: &mut Vec<String>) {
    match value {
        Value::String(text) => found.push(text.clone()),
        Value::Array(items) => items.iter().for_each(|item| strings(item, found)),
        Value::Object(map) => map.values().for_each(|item| strings(item, found)),
        _ => {}
    }
}

#[cfg(unix)]
fn approved_at_plan(id: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(".stub-count", "");
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", id]);
    fixture.no_spec_approval(id);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture
}

#[cfg(unix)]
#[test]
fn next_prints_a_compact_action_view_as_text_by_default_and_json_with_a_flag() {
    let fixture = approved_at_plan("view-change");
    fixture.ok(&[
        "checkpoint",
        "--blocker",
        "The staging database is down",
        "--next-action",
        "Advance to build",
    ]);
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["schema_version"], 1);
    assert_eq!(next["view"], "next");
    assert_eq!(next["objective"], "Ship a focused change");
    assert_eq!(
        next["stage"],
        json!({"current": "plan", "status": "active", "iteration": 1, "callers": []})
    );
    let native = &next["native"];
    assert_eq!(native["change"], "view-change");
    assert_eq!(native["read"], "observed");
    assert_eq!(native["state"], "approved");
    assert_eq!(native["approval_valid"], true);
    assert_eq!(native["correction_valid"], true);
    assert_eq!(native["terminal_evidence"], Value::Null);
    assert_eq!(native["next_action"], "stub next action for approved");
    assert!(
        native["approved_deltas"]
            .as_str()
            .unwrap()
            .starts_with("no semantic deltas"),
        "{native}"
    );
    assert_eq!(next["gate"]["to"], "build");
    assert_eq!(next["gate"]["allowed"], true);
    assert_eq!(next["gate"]["label"], "native:approved");
    assert_eq!(
        next["next_valid_action"]["action"],
        "Continue plan: Advance to build"
    );
    assert!(next["next_valid_action"]["reason"].is_string());
    assert_eq!(next["awaiting_human"], json!({"total": 0, "items": []}));
    assert_eq!(
        next["blockers"],
        json!({"total": 1, "items": ["The staging database is down"]})
    );
    assert_eq!(next["blocking_bugs"], json!({"total": 0, "items": []}));
    assert_eq!(next["other_open_bugs"], 0);
    assert_eq!(next["full_view"], "chough status");
    assert_eq!(next["truncated"], false);
    for full_only in [
        "native_specsync",
        "native_summary",
        "snapshot",
        "observation_references",
        "checkpoint",
        "open_bugs",
    ] {
        assert!(next.get(full_only).is_none(), "{full_only} in {next}");
    }

    // Text is the default and carries the same decision.
    let shown = text(&fixture, &["next"]);
    assert!(serde_json::from_str::<Value>(&shown).is_err(), "{shown}");
    for line in [
        "Objective: Ship a focused change",
        "Stage: plan (active, iteration 1)",
        "Native change: view-change",
        "Next: Continue plan: Advance to build",
        "Blockers: 1",
        "The staging database is down",
        "Full view: chough status",
    ] {
        assert!(shown.contains(line), "missing {line:?} in:\n{shown}");
    }

    // Status stays the full JSON view with its fields, and says so in its help.
    let status = fixture.ok(&["status"]);
    for field in [
        "checkpoint",
        "native_specsync",
        "native_summary",
        "snapshot",
        "observations",
        "observation_references",
        "open_bugs",
        "blocking_bugs",
        "session_next_action",
    ] {
        assert!(status.get(field).is_some(), "{field} missing from status");
    }
    assert_eq!(
        status["next_valid_action"],
        next["next_valid_action"]["action"]
    );
    let help = text(&fixture, &["status", "--help"]);
    assert!(help.contains("always prints JSON"), "{help}");
}

#[cfg(unix)]
#[test]
fn next_and_status_each_read_native_status_once_and_agree_on_the_action() {
    let fixture = approved_at_plan("one-read");
    let mut seen = vec![];
    for stage in ["plan", "build", "test", "accept"] {
        if stage != "plan" {
            fixture.ok(&["stage", "advance"]);
        }
        fixture.native_calls();
        let next = fixture.ok(&["next", "--json"]);
        assert_eq!(fixture.native_calls(), 1, "next at {stage}");
        text(&fixture, &["next"]);
        assert_eq!(fixture.native_calls(), 1, "text next at {stage}");
        let status = fixture.ok(&["status", "--json"]);
        assert_eq!(fixture.native_calls(), 1, "status at {stage}");
        assert_eq!(next["stage"]["current"], stage);
        assert_eq!(
            status["next_valid_action"], next["next_valid_action"]["action"],
            "{stage}"
        );
        seen.push(next["gate"]["label"].clone());
    }
    // The gate for the next main stage is evaluated on that one response.
    assert_eq!(
        seen,
        vec![
            json!("native:approved"),
            json!("native:approved"),
            json!("native:approved"),
            json!("refused:native-approved"),
        ]
    );
    // Agreement holds for a refused gate and for a human gate too.
    fixture.write(".stub-approval", "false\n");
    fixture.ok(&["checkpoint", "--awaiting-human", "Implementation review"]);
    let next = fixture.ok(&["next", "--json"]);
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(
        status["next_valid_action"],
        next["next_valid_action"]["action"]
    );
    assert!(
        next["next_valid_action"]["action"]
            .as_str()
            .unwrap()
            .contains("Implementation review")
    );
}

#[cfg(unix)]
#[test]
fn both_forms_stay_within_the_cap_with_totals_for_large_records() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "large-records"]);
    fixture.no_spec_approval("large-records");
    // A native response near the 4 MiB parse bound with an 8 KiB next action.
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\necho \"$*\" >>.stub-calls\nnext=$(head -c 8192 /dev/zero | tr '\\0' n)\npad=$(head -c 4100000 /dev/zero | tr '\\0' x)\nprintf '{\"summary\":{\"state\":\"approved\",\"next_action\":\"%s\",\"approval_valid\":true,\"correction_valid\":true},\"change\":{\"affected_specs\":[],\"no_spec_change\":true},\"acceptance_entries\":\"%s\"}\\n' \"$next\" \"$pad\"\n",
    );
    fixture.executable("bin/specsync");
    fixture.write(".gitignore", ".stub-*\n");
    fixture.commit("fixture");

    // 500 observations that are present but stale (historical), each referenced by four
    // bugs, 7 unreferenced ones and 1 current referenced one.
    let stale = |id: &str| {
        json!({
            "schema_version": 2,
            "id": id,
            "criteria": ["LIFE-1"],
            "requirements": ["REQ-cli-001"],
            "contracts": ["specs/cli/cli.spec.md"],
            "kind": "product-test",
            "outcome": "passed",
            "environment": "local",
            "summary": format!("fixture observation {id}"),
            "artifacts": [],
            "artifact_fingerprints": {},
            "observed_at_unix_seconds": 1_700_000_000,
            "snapshot": {"revision": "0".repeat(40), "dirty": false, "fingerprint": "0".repeat(64)},
            "provenance": "fixture",
        })
    };
    let ids: Vec<String> = (0..500).map(|index| format!("stale-{index:03}")).collect();
    let loose: Vec<String> = (0..7).map(|index| format!("loose-{index}")).collect();
    for id in ids.iter().chain(&loose) {
        fixture.write(
            &format!(".chough/evidence/{id}.json"),
            &serde_json::to_string(&stale(id)).unwrap(),
        );
    }
    for (index, scope) in ["blocking", "blocking", "in-scope", "out-of-scope"]
        .iter()
        .enumerate()
    {
        let id = format!("BUG-{:04}", index + 1);
        let bug = json!({
            "schema_version": 1,
            "id": id,
            "title": format!("Fixture bug {id}"),
            "symptom": "It breaks",
            "trigger": "Running the probe",
            "expected": "It works",
            "actual": "It broke",
            "environment": "local",
            "observations": ids,
            "impact": "Not yet assessed.",
            "classification": "unclassified",
            "scope": scope,
            "status": "open",
            "regression_baseline": null,
            "source": "fixture",
            "created_at_unix_seconds": 1,
            "updated_at_unix_seconds": 1,
        });
        fixture.write(&format!(".chough/bugs/{id}.json"), &bug.to_string());
    }
    let added = fixture.run(&[
        "evidence",
        "add",
        "current-one",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "structural",
        "--outcome",
        "passed",
        "--environment",
        "local",
        "--summary",
        "Current observation",
    ]);
    assert!(
        added.status.success(),
        "{}",
        String::from_utf8_lossy(&added.stderr)
    );

    // 40 human gates and 40 blockers. The session record is bounded at 64 KiB, so six
    // of each are 4 KiB long, the most that fits beside the rest, and they come first.
    let mut session = fixture.session();
    let long = |prefix: &str, index: usize| {
        let mut text = format!("{prefix} {index:02} ");
        text.push_str(&"w".repeat(4_096 - text.len()));
        text
    };
    let list = |prefix: &str| -> Vec<String> {
        (0..40)
            .map(|index| {
                if index < 6 {
                    long(prefix, index)
                } else {
                    format!("{prefix} {index:02}")
                }
            })
            .collect()
    };
    session["awaiting_human"] = json!(list("Gate"));
    session["blockers"] = json!(list("Blocker"));
    session["observations"] = json!(["current-one", "stale-000"]);
    session["next_action"] = json!("n".repeat(4_096));
    session["objective"] = json!("o".repeat(4_000));
    let session_text = serde_json::to_string(&session).unwrap();
    assert!(session_text.len() < 65_536, "{}", session_text.len());
    fixture.write(".chough/session.json", &session_text);

    fixture.native_calls();
    let raw = text(&fixture, &["next", "--json"]);
    assert_eq!(fixture.native_calls(), 1);
    assert!(raw.len() <= NEXT_LIMIT, "JSON form is {} bytes", raw.len());
    let next: Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(next["awaiting_human"]["total"], 40);
    assert_eq!(next["blockers"]["total"], 40);
    assert_eq!(next["blocking_bugs"]["total"], 2);
    assert_eq!(next["other_open_bugs"], 2);
    let observations = &next["observations"];
    assert_eq!(observations["needs_attention_total"], 500);
    assert_eq!(observations["referenced_current"], 1);
    assert_eq!(observations["unreferenced"], 7);
    for list in [
        &next["awaiting_human"]["items"],
        &next["blockers"]["items"],
        &next["blocking_bugs"]["items"],
        &observations["items"],
    ] {
        assert!(list.as_array().unwrap().len() <= 10, "{list}");
    }
    for item in observations["items"].as_array().unwrap() {
        assert_eq!(item["label"], "historical", "{item}");
        assert!(item["referenced_by"].as_array().unwrap().len() <= 3);
    }
    let mut found = vec![];
    strings(&next, &mut found);
    for value in &found {
        assert!(value.len() <= 512, "{} bytes: {value:.80}", value.len());
    }
    assert!(next["objective"].as_str().unwrap().ends_with(" [cut]"));
    assert!(
        next["native"]["next_action"]
            .as_str()
            .unwrap()
            .ends_with(" [cut]")
    );
    assert!(
        next["awaiting_human"]["items"][0]
            .as_str()
            .unwrap()
            .ends_with(" [cut]")
    );
    let shown_items = next["awaiting_human"]["items"].as_array().unwrap().len()
        + next["blockers"]["items"].as_array().unwrap().len()
        + observations["items"].as_array().unwrap().len();
    assert_eq!(next["truncated"], shown_items < 30, "{next}");

    let shown = text(&fixture, &["next"]);
    assert_eq!(fixture.native_calls(), 1);
    assert!(
        shown.len() <= NEXT_LIMIT,
        "text form is {} bytes",
        shown.len()
    );
    for total in [
        "Human gates: 40",
        "Blockers: 40",
        "Blocking bugs: 2 (BUG-0001, BUG-0002; 2 other open",
        "Observations needing attention: 500",
        "1 referenced current, 7 unreferenced",
        "Full view: chough status",
    ] {
        assert!(shown.contains(total), "missing {total:?} in:\n{shown}");
    }
    assert!(!shown.contains(&"x".repeat(100)), "raw native output shown");
    assert!(!shown.contains(&"w".repeat(600)), "an uncut string shown");

    // Status keeps everything, as the full view.
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(fixture.native_calls(), 1);
    assert_eq!(
        status["checkpoint"]["awaiting_human"]
            .as_array()
            .unwrap()
            .len(),
        40
    );
    assert_eq!(status["observations"].as_array().unwrap().len(), 508);
    // The view cuts the shared action; status shows it whole.
    let action = next["next_valid_action"]["action"].as_str().unwrap();
    assert!(action.ends_with(" [cut]"), "{action}");
    assert!(
        status["next_valid_action"]
            .as_str()
            .unwrap()
            .starts_with(action.trim_end_matches(" [cut]"))
    );
}
