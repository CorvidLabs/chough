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

    // Strings that JSON escapes to six bytes per character stay within the cap too:
    // each string is bounded by its escaped length, not its raw length.
    let control = "\u{1}".repeat(600);
    let escaped = "\\u0001".repeat(600);
    fixture.write(
        "bin/specsync",
        &format!(
            "#!/bin/sh\necho \"$*\" >>.stub-calls\nprintf '%s\\n' '{{\"summary\":{{\"state\":\"{escaped}\",\"next_action\":\"{escaped}\",\"approval_valid\":true,\"correction_valid\":true,\"terminal_evidence\":{{\"validity\":\"{escaped}\"}}}},\"change\":{{\"affected_specs\":[],\"no_spec_change\":true}}}}'\n"
        ),
    );
    session["objective"] = json!(control);
    session["next_action"] = json!(control);
    // Within the 64 KiB session bound: four escaped strings in each list.
    let list = |prefix: &str| -> Vec<String> {
        (0..40)
            .map(|index| {
                if index < 4 {
                    control.clone()
                } else {
                    format!("{prefix} {index:02}")
                }
            })
            .collect()
    };
    session["awaiting_human"] = json!(list("Gate"));
    session["blockers"] = json!(list("Blocker"));
    let session_text = serde_json::to_string(&session).unwrap();
    assert!(session_text.len() < 65_536, "{}", session_text.len());
    fixture.write(".chough/session.json", &session_text);
    let raw = text(&fixture, &["next", "--json"]);
    assert!(raw.len() <= NEXT_LIMIT, "JSON form is {} bytes", raw.len());
    let next: Value = serde_json::from_str(&raw).unwrap();
    assert!(
        next["native"]["state"]
            .as_str()
            .unwrap()
            .ends_with(" [cut]")
    );
    let mut found = vec![];
    strings(&next, &mut found);
    for value in &found {
        let width = serde_json::to_string(value).unwrap().len() - 2;
        assert!(width <= 512, "{width} escaped bytes: {value:?}");
    }
    assert!(next["objective"].as_str().unwrap().ends_with(" [cut]"));
    assert_eq!(next["awaiting_human"]["total"], 40);
    let shown = text(&fixture, &["next"]);
    assert!(
        shown.len() <= NEXT_LIMIT,
        "text form is {} bytes",
        shown.len()
    );
}

// ---------------------------------------------------------------------------
// Fresh-agent handoff: archived-not-done guidance, recent decisions, the last failure
// and the lesson pointer (REQ-cli-021, REQ-cli-002).

/// done_action's new-cycle steps, shared by the archived-not-done action.
const NEW_CYCLE: &str = "Start a new cycle: chough stage reset --to intent --reason \"<why>\" (or --to plan when the approved intent already covers the new work), then specsync change new \"<outcome>\" --spec <module> --path <path>, then chough checkpoint --native-change <id> --next-action \"<next step>\".";
const RESET: &str = "chough stage reset --to intent --reason \"<why>\"";

/// Split a suggested command into arguments, honoring double quotes.
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

/// Run a suggested `chough ...` command with its placeholders filled.
fn run_suggested(fixture: &Fixture, command: &str) -> Value {
    let command = command.replace("<why>", "New work after the archive");
    let arguments = words(&command);
    assert_eq!(arguments[0], "chough", "{command}");
    let arguments: Vec<&str> = arguments[1..].iter().map(String::as_str).collect();
    fixture.ok(&arguments)
}

/// A checkpoint at `stage` whose recorded native change is archived with authenticated
/// terminal evidence.
#[cfg(unix)]
fn archived_at(stage: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("archived");
    fixture.write(".stub-next", "merge the PR on GitHub if it is still open\n");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "old-change"]);
    if stage != "intent" {
        fixture.ok(&[
            "stage",
            "reset",
            "--to",
            stage,
            "--reason",
            "Left behind an archive",
        ]);
    }
    fixture.ok(&[
        "checkpoint",
        "--next-action",
        "specsync change check old-change --commit, then push",
    ]);
    fixture
}

fn bug(id: &str, status: &str, scope: &str) -> Value {
    json!({
        "schema_version": 1,
        "id": id,
        "title": format!("Fixture bug {id}"),
        "symptom": "It breaks",
        "trigger": "Running the probe",
        "expected": "It works",
        "actual": "It broke",
        "environment": "local",
        "observations": [],
        "impact": "Not yet assessed.",
        "classification": "unclassified",
        "scope": scope,
        "status": status,
        "regression_baseline": null,
        "source": "fixture",
        "created_at_unix_seconds": 1,
        "updated_at_unix_seconds": 1,
    })
}

fn observation(id: &str, outcome: &str, observed: u64) -> Value {
    json!({
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
    })
}

fn write_record(fixture: &Fixture, directory: &str, value: &Value) {
    fixture.write(
        &format!(".chough/{directory}/{}.json", value["id"].as_str().unwrap()),
        &serde_json::to_string(value).unwrap(),
    );
}

/// Set a file's modification time to `seconds` after the Unix epoch.
fn set_mtime(path: &std::path::Path, seconds: u64) {
    let file = std::fs::File::options().write(true).open(path).unwrap();
    file.set_modified(std::time::UNIX_EPOCH + std::time::Duration::from_secs(seconds))
        .unwrap();
}

#[cfg(unix)]
#[test]
fn an_archived_change_before_done_names_a_new_cycle_not_continue() {
    for stage in ["plan", "test", "accept"] {
        let fixture = archived_at(stage);
        fixture.write(".stub-count", "");
        let next = fixture.ok(&["next", "--json"]);
        assert_eq!(fixture.native_calls(), 1, "next at {stage}");
        let status = fixture.ok(&["status", "--json"]);
        assert_eq!(fixture.native_calls(), 1, "status at {stage}");
        let action = &next["next_valid_action"];
        let full = status["next_valid_action"].as_str().unwrap();
        let opening = format!(
            "Native change old-change is archived while the checkpoint is at {stage}, so this cycle's work is closed. "
        );
        let expected = if stage == "accept" {
            format!(
                "{opening}Record done first: chough stage advance (accept to done is allowed). Then: {NEW_CYCLE}"
            )
        } else {
            format!("{opening}{NEW_CYCLE}")
        };
        assert_eq!(full, expected, "status at {stage}");
        assert_eq!(action["action"], expected, "next at {stage}");
        assert!(!full.contains("Continue"), "{full}");
        assert!(!full.contains("Gate not open"), "{full}");
        assert!(!full.contains("specsync change check old-change"), "{full}");
        let reason = action["reason"].as_str().unwrap();
        assert!(
            reason.contains(&format!(
                "The archive's terminal evidence is authenticated-history; continuing {stage} would work on a closed change."
            )),
            "{reason}"
        );
        assert!(
            reason.contains(
                "Native next action, check it first: merge the PR on GitHub if it is still open."
            ),
            "{reason}"
        );
        let command = action["command"].as_str().unwrap();
        assert_eq!(
            command,
            if stage == "accept" {
                "chough stage advance"
            } else {
                RESET
            },
            "{stage}"
        );
        // Text carries the same decision.
        let shown = text(&fixture, &["next"]);
        assert!(shown.contains(&format!("Next: {expected}")), "{shown}");
        assert!(shown.contains(&format!("  Run: {command}")), "{shown}");
        assert!(!shown.contains("Next: Continue"), "{shown}");

        // The suggested command, placeholders filled, succeeds.
        if stage == "accept" {
            let advanced = run_suggested(&fixture, command);
            assert_eq!(advanced["applied"]["to"], "done", "{advanced}");
            let done = fixture.ok(&["next", "--json"]);
            assert_eq!(done["next_valid_action"]["action"], NEW_CYCLE);
            assert_eq!(done["next_valid_action"]["command"], RESET);
            let reset = run_suggested(&fixture, RESET);
            assert_eq!(reset["applied"]["to"], "intent");
        } else {
            let reset = run_suggested(&fixture, command);
            assert_eq!(reset["applied"]["to"], "intent", "{reset}");
            assert_eq!(fixture.session()["stage"]["current"], "intent", "{stage}");
        }
    }
}

#[cfg(unix)]
#[test]
fn an_archived_change_keeps_todays_action_when_something_else_applies() {
    let archived = "is archived while the checkpoint";
    // Paused.
    let fixture = archived_at("plan");
    fixture.ok(&["stage", "pause", "--reason", "Lunch"]);
    let action = fixture.ok(&["next", "--json"])["next_valid_action"]["action"].clone();
    assert!(action.as_str().unwrap().contains("is paused"), "{action}");
    // Awaiting a human.
    fixture.ok(&["stage", "resume"]);
    fixture.ok(&["checkpoint", "--awaiting-human", "Scope decision"]);
    let action = fixture.ok(&["next", "--json"])["next_valid_action"]["action"].clone();
    assert!(
        action
            .as_str()
            .unwrap()
            .starts_with("Waiting for a human decision"),
        "{action}"
    );
    // A blocking bug at test.
    let fixture = archived_at("test");
    write_record(&fixture, "bugs", &bug("BUG-0001", "open", "blocking"));
    let action = fixture.ok(&["next", "--json"])["next_valid_action"]["action"].clone();
    assert!(
        action
            .as_str()
            .unwrap()
            .starts_with("Acceptance-blocking bugs are open (BUG-0001)"),
        "{action}"
    );
    // An archive whose terminal evidence does not authenticate.
    for stage in ["plan", "test"] {
        let fixture = archived_at(stage);
        fixture.write(".stub-validity", "corrupt-history\n");
        let next = fixture.ok(&["next", "--json"]);
        let action = next["next_valid_action"]["action"].as_str().unwrap();
        assert!(!action.contains(archived), "{stage}: {action}");
        let status = fixture.ok(&["status", "--json"]);
        assert_eq!(status["next_valid_action"], action, "{stage}");
    }
    // A side stage returns to its caller first.
    let fixture = archived_at("plan");
    fixture.ok(&["stage", "enter", "bugs", "--reason", "Triage"]);
    let action = fixture.ok(&["next", "--json"])["next_valid_action"]["action"].clone();
    assert!(
        action.as_str().unwrap().starts_with("Continue bugs"),
        "{action}"
    );
}

#[test]
fn next_shows_the_five_newest_decisions_with_kind_and_reason() {
    let fixture = Fixture::new();
    fixture.init();
    let none = fixture.ok(&["next", "--json"]);
    assert_eq!(none["decisions"], json!({"total": 0, "items": []}));
    assert!(text(&fixture, &["next"]).contains("Decisions: none"));

    let long = format!("failed: {} because the cache was cold", "x".repeat(4_000));
    let decisions = [
        "decided: an old decision because it is old".to_string(),
        "rejected: another old one".to_string(),
        "decided: lessons live in specs/<module>/context.md because SpecSync already briefs it"
            .to_string(),
        "2026-10-31: a dated entry because it has a date".to_string(),
        "an unprefixed entry".to_string(),
        long.clone(),
        "rejected: a structured decision schema because it needs a migration because PR #26 conflicts".to_string(),
    ];
    for decision in &decisions {
        fixture.ok(&["checkpoint", "--decision", decision]);
    }
    let stored = fixture.session()["decisions"].clone();
    assert_eq!(stored, json!(decisions), "stored strings are unchanged");

    let next = fixture.ok(&["next", "--json"]);
    let block = &next["decisions"];
    assert_eq!(block["total"], 7);
    let items = block["items"].as_array().unwrap();
    assert_eq!(items.len(), 5, "{block}");
    assert_eq!(
        items[0],
        json!({"kind": "rejected", "text": "a structured decision schema because it needs a migration", "reason": "PR #26 conflicts"})
    );
    assert_eq!(items[1]["kind"], "failed");
    let cut = items[1]["text"].as_str().unwrap();
    assert!(cut.ends_with(" [cut]"), "{cut:.80}");
    assert!(cut.len() <= 512 && cut.starts_with("xxxx"), "{}", cut.len());
    assert_eq!(items[1]["reason"], "the cache was cold");
    assert_eq!(
        items[2],
        json!({"kind": null, "text": "an unprefixed entry", "reason": null})
    );
    assert_eq!(
        items[3],
        json!({"kind": null, "text": "2026-10-31: a dated entry because it has a date", "reason": null})
    );
    assert_eq!(
        items[4],
        json!({"kind": "decided", "text": "lessons live in specs/<module>/context.md", "reason": "SpecSync already briefs it"})
    );

    let shown = text(&fixture, &["next"]);
    for line in [
        "Decisions (5 of 7, newest first):",
        "  rejected: a structured decision schema because it needs a migration (because PR #26 conflicts)",
        "  an unprefixed entry\n",
        "  2026-10-31: a dated entry because it has a date\n",
        "  decided: lessons live in specs/<module>/context.md (because SpecSync already briefs it)",
        " [cut] (because the cache was cold)",
    ] {
        assert!(shown.contains(line), "missing {line:?} in:\n{shown}");
    }
    assert!(!shown.contains("an old decision"), "{shown}");
    assert!(!shown.contains("another old one"), "{shown}");
}

#[test]
fn next_names_the_latest_failed_observation_by_recorded_time_with_its_bugs() {
    let fixture = Fixture::new();
    fixture.init();
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["last_failure"], Value::Null);
    assert!(text(&fixture, &["next"]).contains("Last failure: none"));

    // Passed and not-run records never count, however recent.
    write_record(
        &fixture,
        "evidence",
        &observation("passed-late", "passed", 9_000),
    );
    write_record(
        &fixture,
        "evidence",
        &observation("not-run-late", "not-run", 9_500),
    );
    assert_eq!(fixture.ok(&["next", "--json"])["last_failure"], Value::Null);

    // Recorded times disagree with file names and modification times: the name that
    // sorts last and the newest file are the oldest failures.
    write_record(
        &fixture,
        "evidence",
        &observation("m-latest", "failed", 3_000),
    );
    write_record(
        &fixture,
        "evidence",
        &observation("a-middle", "failed", 2_000),
    );
    write_record(
        &fixture,
        "evidence",
        &observation("z-oldest", "failed", 1_000),
    );
    let evidence = fixture.root().join(".chough/evidence");
    set_mtime(&evidence.join("m-latest.json"), 1_000_000);
    set_mtime(&evidence.join("a-middle.json"), 2_000_000);
    set_mtime(&evidence.join("z-oldest.json"), 3_000_000);
    // Four bugs link the latest failure, by observations, a note and a regression
    // baseline; one links an older failure only.
    let mut linked = bug("BUG-0001", "fix-pending-verification", "in-scope");
    linked["observations"] = json!(["m-latest"]);
    write_record(&fixture, "bugs", &linked);
    let mut noted = bug("BUG-0002", "open", "in-scope");
    noted["notes"] =
        json!([{"at_unix_seconds": 5, "text": "Seen again", "observations": ["m-latest"]}]);
    write_record(&fixture, "bugs", &noted);
    let mut baseline = bug("BUG-0003", "verified", "in-scope");
    baseline["regression_baseline"] = json!("m-latest");
    write_record(&fixture, "bugs", &baseline);
    let mut fourth = bug("BUG-0004", "open", "out-of-scope");
    fourth["observations"] = json!(["a-middle", "m-latest"]);
    write_record(&fixture, "bugs", &fourth);
    let mut older = bug("BUG-0005", "open", "in-scope");
    older["observations"] = json!(["z-oldest"]);
    write_record(&fixture, "bugs", &older);

    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        next["last_failure"],
        json!({
            "id": "m-latest",
            "observed_at_unix_seconds": 3_000,
            "summary": "fixture observation m-latest",
            "bugs": {"total": 4, "items": [
                {"id": "BUG-0001", "status": "fix-pending-verification"},
                {"id": "BUG-0002", "status": "open"},
                {"id": "BUG-0003", "status": "verified"},
            ]},
        })
    );
    let shown = text(&fixture, &["next"]);
    for line in [
        "Last failure: m-latest at 3000: fixture observation m-latest",
        "  Linked bugs: BUG-0001 fix-pending-verification, BUG-0002 open, BUG-0003 verified (3 of 4)",
    ] {
        assert!(shown.contains(line), "missing {line:?} in:\n{shown}");
    }

    // A tie on recorded time goes to the greater observation ID; an unlinked failure
    // shows no bugs. A long summary is cut.
    let mut tie = observation("n-tie", "failed", 3_000);
    tie["summary"] = json!("s".repeat(2_000));
    write_record(&fixture, "evidence", &tie);
    set_mtime(&evidence.join("n-tie.json"), 10);
    let next = fixture.ok(&["next", "--json"]);
    let failure = &next["last_failure"];
    assert_eq!(failure["id"], "n-tie");
    assert_eq!(failure["bugs"], json!({"total": 0, "items": []}));
    let summary = failure["summary"].as_str().unwrap();
    assert!(
        summary.len() <= 512 && summary.ends_with(" [cut]"),
        "{}",
        summary.len()
    );
    let shown = text(&fixture, &["next"]);
    assert!(
        shown.contains("Last failure: n-tie at 3000: sss"),
        "{shown}"
    );
    assert!(shown.contains("  Linked bugs: none"), "{shown}");
}

#[test]
fn next_points_to_the_lesson_source_and_counts_archived_bundles() {
    let fixture = Fixture::new();
    fixture.init();
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        next["lessons"],
        json!({
            "sources": {"total": 1, "items": [
                {"module": "cli", "path": "specs/cli/context.md", "section_line": null, "note": "missing"},
            ]},
            "archived_bundles": {"total": 0, "items": []},
        })
    );

    fixture.write(
        "specs/cli/context.md",
        "# CLI context\n\n## Decisions\n\nKeep it small.\n\n## Lessons\n\nSECRET-LESSON-TEXT\n",
    );
    fixture.write(
        "specs/docs/context.md",
        "# Docs context\n\n## Lessons learned elsewhere\n\nSECRET-LESSON-TEXT\n",
    );
    fixture.write(
        "specs/native/context.md",
        "# Native\n\n## Lessons\n\nSECRET-LESSON-TEXT\n",
    );
    fixture.write("specs/native/native.3md", "SECRET-LESSON-TEXT\n");
    let mut session = fixture.session();
    session["modules"] = json!(["cli", "docs", "ghost", "native"]);
    fixture.write(".chough/session.json", &session.to_string());
    // Seven archived bundles written out of order, and an archive without one.
    for day in [3, 7, 1, 5, 2, 6, 4] {
        fixture.write(
            &format!(".specsync/archive/changes/2026-10-0{day}-change-{day}/lesson-bundle.md"),
            "SECRET-LESSON-TEXT\n",
        );
    }
    fixture.write(
        ".specsync/archive/changes/2026-10-09-no-bundle/change.md",
        "# No bundle\n",
    );

    let raw = text(&fixture, &["next", "--json"]);
    let next: Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(
        next["lessons"],
        json!({
            "sources": {"total": 4, "items": [
                {"module": "cli", "path": "specs/cli/context.md", "section_line": 7, "note": null},
                {"module": "docs", "path": "specs/docs/context.md", "section_line": null, "note": "no Lessons section"},
                {"module": "ghost", "path": "specs/ghost/context.md", "section_line": null, "note": "missing"},
                {"module": "native", "path": "specs/native/native.3md", "section_line": null, "note": null},
            ]},
            "archived_bundles": {"total": 7, "items": [
                ".specsync/archive/changes/2026-10-07-change-7/lesson-bundle.md",
                ".specsync/archive/changes/2026-10-06-change-6/lesson-bundle.md",
                ".specsync/archive/changes/2026-10-05-change-5/lesson-bundle.md",
                ".specsync/archive/changes/2026-10-04-change-4/lesson-bundle.md",
                ".specsync/archive/changes/2026-10-03-change-3/lesson-bundle.md",
            ]},
        })
    );
    let shown = text(&fixture, &["next"]);
    for line in [
        "Lesson sources (4 of 4 modules):",
        "  cli: specs/cli/context.md (## Lessons at line 7)",
        "  docs: specs/docs/context.md (no Lessons section)",
        "  ghost: specs/ghost/context.md (missing)",
        "  native: specs/native/native.3md\n",
        "Archived lesson bundles (5 of 7, newest first):",
        "  .specsync/archive/changes/2026-10-07-change-7/lesson-bundle.md",
    ] {
        assert!(shown.contains(line), "missing {line:?} in:\n{shown}");
    }
    assert!(!shown.contains("2026-10-02-change-2"), "{shown}");
    assert!(!raw.contains("SECRET-LESSON-TEXT"), "{raw}");
    assert!(!shown.contains("SECRET-LESSON-TEXT"), "{shown}");
    // Nothing is written: no fold state, ledger or index.
    assert!(!fixture.exists(".chough/lessons"));
}

#[cfg(unix)]
#[test]
fn handoff_blocks_stay_within_the_cap_and_are_byte_identical() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "large-handoff"]);
    fixture.no_spec_approval("large-handoff");
    fixture.write(".gitignore", ".stub-*\n");
    fixture.commit("fixture");

    // 500 referenced stale observations, each referenced by four bugs.
    let stale: Vec<String> = (0..500).map(|index| format!("stale-{index:03}")).collect();
    for id in &stale {
        write_record(
            &fixture,
            "evidence",
            &observation(id, "passed", 1_700_000_000),
        );
    }
    for index in 1..=4 {
        let mut record = bug(&format!("BUG-{index:04}"), "open", "in-scope");
        record["observations"] = json!(stale);
        write_record(&fixture, "bugs", &record);
    }
    // 200 more observations, 100 failed; the latest recorded failure is fail-037.
    for index in 0..100_u64 {
        write_record(
            &fixture,
            "evidence",
            &observation(&format!("pass-{index:03}"), "passed", 1_800_000_000 + index),
        );
        let observed = if index == 37 {
            1_900_000_000
        } else {
            1_800_000_000 + index
        };
        let mut failed = observation(&format!("fail-{index:03}"), "failed", observed);
        failed["summary"] = json!(format!("failure {index:03} {}", "f".repeat(4_000)));
        write_record(&fixture, "evidence", &failed);
    }
    // 40 bugs link the latest failure.
    for index in 5..45 {
        let mut record = bug(&format!("BUG-{index:04}"), "open", "in-scope");
        record["observations"] = json!(["fail-037"]);
        write_record(&fixture, "bugs", &record);
    }
    // 40 archived bundles and 16 modules.
    for index in 0..40 {
        fixture.write(
            &format!(
                ".specsync/archive/changes/2026-{:02}-{:02}-change-with-a-long-descriptive-name-{index:02}/lesson-bundle.md",
                index / 28 + 1,
                index % 28 + 1
            ),
            "bundle\n",
        );
    }
    let modules: Vec<String> = (0..16).map(|index| format!("module-{index:02}")).collect();
    for module in &modules {
        fixture.write(
            &format!("specs/{module}/context.md"),
            "# Context\n\n## Lessons\n\nA lesson.\n",
        );
    }
    // The session record is bounded at 64 KiB, so 50 decisions, 40 gates and 40
    // blockers of 4 KiB each cannot all be stored. The 5 decisions shown and the first
    // 2 gates and blockers are 4 KiB long; the other shown gates and blockers are 700
    // bytes, longer than the 512-byte cut, so every shown entry fills its cut.
    let sized = |prefix: &str, index: usize, size: usize| {
        let mut text = format!("{prefix} {index:02} ");
        text.push_str(&"w".repeat(size * 3 / 4));
        text.push_str(" because ");
        text.push_str(&"r".repeat(size - text.len()));
        text
    };
    let list = |prefix: &str, count: usize, long: std::ops::Range<usize>| -> Vec<String> {
        (0..count)
            .map(|index| {
                if long.contains(&index) {
                    sized(prefix, index, 4_096)
                } else if prefix != "decided:" && index < 10 {
                    sized(prefix, index, 700)
                } else {
                    format!("{prefix} {index:02}")
                }
            })
            .collect()
    };
    let mut session = fixture.session();
    session["modules"] = json!(modules);
    session["decisions"] = json!(list("decided:", 50, 45..50));
    session["awaiting_human"] = json!(list("Gate", 40, 0..2));
    session["blockers"] = json!(list("Blocker", 40, 0..2));
    session["observations"] = json!(["stale-000"]);
    let session_text = serde_json::to_string(&session).unwrap();
    assert!(session_text.len() < 65_536, "{}", session_text.len());
    fixture.write(".chough/session.json", &session_text);
    fixture.write(".stub-count", "");

    let raw = text(&fixture, &["next", "--json"]);
    assert_eq!(fixture.native_calls(), 1);
    let shown = text(&fixture, &["next"]);
    assert_eq!(fixture.native_calls(), 1);
    assert!(raw.len() <= NEXT_LIMIT, "JSON form is {} bytes", raw.len());
    assert!(
        shown.len() <= NEXT_LIMIT,
        "text form is {} bytes",
        shown.len()
    );
    let next: Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(next["schema_version"], 1);
    assert_eq!(next["truncated"], true);
    assert_eq!(next["decisions"]["total"], 50);
    assert_eq!(next["last_failure"]["id"], "fail-037");
    assert_eq!(
        next["last_failure"]["observed_at_unix_seconds"],
        1_900_000_000_u64
    );
    assert_eq!(next["last_failure"]["bugs"]["total"], 40);
    assert_eq!(next["lessons"]["sources"]["total"], 16);
    assert_eq!(next["lessons"]["archived_bundles"]["total"], 40);
    // The 500 stale ones and the latest failure, which 40 bugs reference.
    assert_eq!(next["observations"]["needs_attention_total"], 501);
    assert_eq!(next["awaiting_human"]["total"], 40);
    assert_eq!(next["blockers"]["total"], 40);
    for (list, bound) in [
        (&next["decisions"]["items"], 5),
        (&next["last_failure"]["bugs"]["items"], 3),
        (&next["lessons"]["sources"]["items"], 10),
        (&next["lessons"]["archived_bundles"]["items"], 5),
        (&next["awaiting_human"]["items"], 10),
        (&next["blockers"]["items"], 10),
        (&next["observations"]["items"], 10),
    ] {
        assert!(list.as_array().unwrap().len() <= bound, "{list}");
    }
    let mut found = vec![];
    strings(&next, &mut found);
    for value in &found {
        assert!(value.len() <= 512, "{} bytes: {value:.80}", value.len());
    }
    for total in [
        "Decisions (",
        " of 50, newest first):",
        "Last failure: fail-037 at 1900000000: failure 037",
        " of 40)",
        " of 16 modules):",
        "Archived lesson bundles (",
        "Output truncated at 16384 bytes",
    ] {
        assert!(shown.contains(total), "missing {total:?} in:\n{shown}");
    }

    // Byte-identical across runs, and after every record's modification time changes.
    assert_eq!(text(&fixture, &["next", "--json"]), raw);
    assert_eq!(text(&fixture, &["next"]), shown);
    let mut stamp = 4_000_000_u64;
    for directory in [".chough/evidence", ".chough/bugs", ".chough"] {
        let mut entries: Vec<_> = std::fs::read_dir(fixture.root().join(directory))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file())
            .collect();
        entries.sort();
        entries.reverse();
        for path in entries {
            set_mtime(&path, stamp);
            stamp += 7;
        }
    }
    for module in &modules {
        set_mtime(
            &fixture.root().join(format!("specs/{module}/context.md")),
            stamp,
        );
        stamp = stamp.saturating_sub(11);
    }
    assert_eq!(text(&fixture, &["next", "--json"]), raw);
    assert_eq!(text(&fixture, &["next"]), shown);
    assert_eq!(fixture.native_calls(), 4, "one native read per run");
}
