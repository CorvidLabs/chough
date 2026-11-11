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

    let added: Value = serde_json::from_slice(&added.stdout).unwrap();
    let current: Value =
        serde_json::from_str(&fixture.read(added["recorded"].as_str().unwrap())).unwrap();
    let current_id = current["id"].as_str().unwrap();

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
    session["observations"] = json!([current_id, "stale-000"]);
    session["next_action"] = json!("n".repeat(4_096));
    session["objective"] = json!("o".repeat(4_000));
    let session_text = write_session(&fixture, &session);
    assert!(
        session_text.len() <= common::CHECKPOINT_CAP,
        "{}",
        session_text.len()
    );

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
    let session_text = write_session(&fixture, &session);
    assert!(
        session_text.len() <= common::CHECKPOINT_CAP,
        "{}",
        session_text.len()
    );
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
    archived_change_at(stage, "old-change")
}

/// The same with a named change, reached by advancing (not by a reset, which starts the
/// new cycle) while the change was approved, then archived.
#[cfg(unix)]
fn archived_change_at(stage: &str, change: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.write(".stub-next", "merge the PR on GitHub if it is still open\n");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", change]);
    fixture.no_spec_approval(change);
    while fixture.session()["stage"]["current"] != stage {
        fixture.ok(&[
            "stage",
            "advance",
            "--note",
            "Reached with the change approved",
        ]);
    }
    fixture.write(".stub-state", "archived\n");
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

/// After the suggested reset the archived rule does not suggest another reset, although
/// the checkpoint still names the archived change until a new one is recorded: the
/// reset's new-cycle next action shows at intent, and at plan too, with the command
/// `specsync change new` rather than the archived gate refusal (issue 76 item 5).
#[cfg(unix)]
fn assert_after_reset(fixture: &Fixture, from: &str) {
    let next = fixture.ok(&["next", "--json"]);
    let action = next["next_valid_action"]["action"].as_str().unwrap();
    let current = fixture.session()["stage"]["current"].clone();
    let current = current.as_str().unwrap();
    assert!(
        action.starts_with(&format!(
            "Continue {current}: New cycle from {from} after reset: "
        )),
        "{action}"
    );
    if current != "intent" {
        assert_eq!(
            next["next_valid_action"]["command"],
            "specsync change new \"<outcome>\" --spec <module> --path <path>",
            "{next}"
        );
        assert!(!action.contains("refused:native-archived"), "{action}");
        // The gate itself stays refused: only the guidance changes after the reset.
        assert_eq!(next["gate"]["label"], "refused:native-archived", "{next}");
        assert_eq!(next["gate"]["allowed"], false, "{next}");
    }
    assert!(!action.contains("is archived while"), "{action}");
    assert_ne!(next["next_valid_action"]["command"], RESET, "{next}");
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["next_valid_action"], action);
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
            "The recorded native change is archived while the checkpoint is at {stage}, so this cycle's work is closed. "
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
                "Native change old-change is archived while the checkpoint is at {stage}, with terminal evidence authenticated-history; continuing {stage} would work on a closed change."
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
            // In 0.6.0 a session archives itself at done (REQ-history-006); a release
            // consent keeps it open so the done-stage assertions still apply.
            fixture.ok(&[
                "consent",
                "grant",
                "release",
                "--source",
                "Human: release it",
            ]);
            let advanced = run_suggested(&fixture, command);
            assert_eq!(advanced["applied"]["to"], "done", "{advanced}");
            let done = fixture.ok(&["next", "--json"]);
            assert_eq!(done["next_valid_action"]["action"], NEW_CYCLE);
            assert_eq!(done["next_valid_action"]["command"], RESET);
            let reset = run_suggested(&fixture, RESET);
            assert_eq!(reset["applied"]["to"], "intent");
            assert_after_reset(&fixture, "done");
        } else {
            let reset = run_suggested(&fixture, command);
            assert_eq!(reset["applied"]["to"], "intent", "{reset}");
            assert_eq!(fixture.session()["stage"]["current"], "intent", "{stage}");
            assert_after_reset(&fixture, stage);
        }
        // The `--to plan` route starts the new cycle the same way.
        let fixture = archived_at(stage);
        let reset = run_suggested(&fixture, &RESET.replace("--to intent", "--to plan"));
        assert_eq!(reset["applied"]["to"], "plan", "{reset}");
        assert_after_reset(&fixture, stage);
    }

    // A real change ID of 150 characters: the action is not cut, so `next` keeps every
    // new-cycle step and agrees with `status`; the reason names the change.
    let long = format!("fresh-agent-handoff-{}", "x".repeat(130));
    assert_eq!(long.len(), 150);
    for stage in ["plan", "accept"] {
        let fixture = archived_change_at(stage, &long);
        let next = fixture.ok(&["next", "--json"]);
        let action = next["next_valid_action"]["action"].as_str().unwrap();
        assert!(!action.contains(" [cut]"), "{stage}: {action}");
        assert!(
            action.ends_with("--next-action \"<next step>\"."),
            "{stage}: {action}"
        );
        let status = fixture.ok(&["status", "--json"]);
        assert_eq!(status["next_valid_action"], action, "{stage}");
        let reason = next["next_valid_action"]["reason"].as_str().unwrap();
        assert!(
            reason.starts_with(&format!(
                "Native change {long} is archived while the checkpoint is at {stage}"
            )),
            "{reason}"
        );
        let shown = text(&fixture, &["next"]);
        assert!(shown.contains(&format!("Next: {action}")), "{shown}");
    }

    keeps_todays_action_when_something_else_applies();
}

/// Paused stages, awaiting-human gates, blocking bugs at test, an archive whose terminal
/// evidence does not authenticate and side stages keep today's action (part of
/// TC-cli-0126).
#[cfg(unix)]
fn keeps_todays_action_when_something_else_applies() {
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
        "Last failure: m-latest at 1970-01-01T00:50:00Z: fixture observation m-latest",
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
        shown.contains("Last failure: n-tie at 1970-01-01T00:50:00Z: sss"),
        "{shown}"
    );
    assert!(shown.contains("  Linked bugs: none"), "{shown}");
}

#[test]
fn next_searches_a_context_md_larger_than_4_mib() {
    let fixture = Fixture::new();
    fixture.init();
    // Native spec files have no size bound (REQ-cli-034): past 4 MiB the Lessons
    // heading is still found, CRLF included, and a file without one says so.
    let line = "Filler line for a large native context file.\r\n";
    let filler = line.repeat(4 * 1_048_576 / line.len() + 2);
    assert!(filler.len() > 4 * 1_048_576);
    let filler_lines = filler.matches('\n').count();
    fixture.write(
        "specs/cli/context.md",
        &format!("{filler}## Lessons\r\n\r\nSECRET-LESSON-TEXT\r\n"),
    );
    fixture.write("specs/docs/context.md", &filler);
    let mut session = fixture.session();
    session["modules"] = json!(["cli", "docs"]);
    write_session(&fixture, &session);
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        next["lessons"]["sources"],
        json!({"total": 2, "items": [
            {"module": "cli", "path": "specs/cli/context.md", "section_line": filler_lines + 1, "note": null},
            {"module": "docs", "path": "specs/docs/context.md", "section_line": null, "note": "no Lessons section"},
        ]})
    );
    assert!(!next.to_string().contains("SECRET-LESSON-TEXT"));
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
    write_session(&fixture, &session);
    // Seven archived bundles written out of order, and an archive without one.
    for day in [3, 7, 1, 5, 2, 6, 4] {
        let dir = format!(".specsync/archive/changes/2026-10-0{day}-change-{day}");
        fixture.write(&format!("{dir}/lesson-bundle.md"), "SECRET-LESSON-TEXT\n");
        fixture.write(
            &format!("{dir}/finalization.json"),
            &json!({"change_id": format!("change-{day}"), "timestamp": 1_790_000_000 + day})
                .to_string(),
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
        "Archived lesson bundles (3 of 7, newest first):",
        "  .specsync/archive/changes/2026-10-07-change-7/lesson-bundle.md",
    ] {
        assert!(shown.contains(line), "missing {line:?} in:\n{shown}");
    }
    assert!(!shown.contains("2026-10-04-change-4"), "{shown}");
    assert!(!raw.contains("SECRET-LESSON-TEXT"), "{raw}");
    assert!(!shown.contains("SECRET-LESSON-TEXT"), "{shown}");
    // Nothing is written: no fold state, ledger or index.
    assert!(!fixture.exists(".chough/lessons"));

    // A symlinked module directory or archive is not a source: the module gets a note
    // and the archive no bundles, and `next` still answers.
    #[cfg(unix)]
    {
        let elsewhere = tempfile::tempdir().unwrap();
        std::fs::write(elsewhere.path().join("context.md"), "## Lessons\n").unwrap();
        std::os::unix::fs::symlink(elsewhere.path(), fixture.root().join("specs/ghost")).unwrap();
        std::fs::rename(
            fixture.root().join(".specsync/archive/changes"),
            fixture.root().join(".specsync/archive/real"),
        )
        .unwrap();
        std::os::unix::fs::symlink(
            fixture.root().join(".specsync/archive/real"),
            fixture.root().join(".specsync/archive/changes"),
        )
        .unwrap();
        let next = fixture.ok(&["next", "--json"]);
        let ghost = &next["lessons"]["sources"]["items"][2];
        assert_eq!(ghost["path"], "specs/ghost/context.md", "{ghost}");
        assert_eq!(ghost["section_line"], Value::Null, "{ghost}");
        let note = ghost["note"].as_str().unwrap();
        assert!(note.starts_with("cannot be read: symlink"), "{note}");
        let bundles = &next["lessons"]["archived_bundles"];
        assert_eq!(bundles["total"], 0, "{bundles}");
        assert_eq!(bundles["items"], json!([]), "{bundles}");
        let note = bundles["note"].as_str().unwrap();
        assert!(note.starts_with("cannot be read: "), "{note}");
        let shown = text(&fixture, &["next"]);
        assert!(shown.contains("  ghost: specs/ghost/context.md (cannot be read: symlink"));
        assert!(
            shown.contains(&format!("Archived lesson bundles: {note}")),
            "{shown}"
        );
    }
}

/// Review of #78 (non-blocking): one symlinked archive directory makes `library index`
/// refuse, and `next` surfaces that same error for the archived bundles instead of
/// silently listing none. SYNTHETIC archives.
#[cfg(unix)]
#[test]
fn next_surfaces_the_error_library_index_refuses_a_symlinked_archive_with() {
    let fixture = Fixture::new();
    fixture.init();
    let base = ".specsync/archive/changes/2026-11-03";
    fixture.write(
        &format!("{base}-a/finalization.json"),
        &json!({"change_id": "a", "timestamp": 100}).to_string(),
    );
    fixture.write(&format!("{base}-a/lesson-bundle.md"), "SYNTHETIC lesson\n");
    let elsewhere = tempfile::tempdir().unwrap();
    std::fs::write(
        elsewhere.path().join("finalization.json"),
        json!({"change_id": "b", "timestamp": 200}).to_string(),
    )
    .unwrap();
    std::os::unix::fs::symlink(elsewhere.path(), fixture.root().join(format!("{base}-b"))).unwrap();
    let refused = fixture.fails(&["library", "index"]);
    let next = fixture.ok(&["next", "--json"]);
    let bundles = &next["lessons"]["archived_bundles"];
    assert_eq!(bundles["total"], 0, "{bundles}");
    assert_eq!(bundles["items"], json!([]), "{bundles}");
    let note = bundles["note"].as_str().unwrap();
    let error = note.strip_prefix("cannot be read: ").unwrap();
    assert!(error.contains("symlink"), "{note}");
    assert!(
        refused.contains(error),
        "next: {note}\nlibrary index: {refused}"
    );
}

/// Issue 74: `next` names the newest 3 lesson bundles by finalization timestamp, then
/// directory, the same archives `chough library index` marks hot (KNOW-8), not the
/// newest by directory name.
#[test]
fn next_lists_the_newest_three_bundles_by_finalization_like_the_librarian() {
    let fixture = Fixture::new();
    fixture.init();
    // Same-day SYNTHETIC archives finalized in cycle order 1 to 6, whose slug order
    // differs from that order; "zzz-unfinalized" has no finalization.json.
    let cycles = [
        "enforce-rules",
        "preserve-history",
        "protect-secrets",
        "add-bounded-pagination",
        "search-messages",
        "export-chats",
    ];
    for (index, slug) in cycles.iter().enumerate() {
        let dir = format!(".specsync/archive/changes/2026-11-03-{slug}");
        fixture.write(
            &format!("{dir}/finalization.json"),
            &json!({"change_id": slug, "timestamp": 1_793_700_000 + 600 * index as u64})
                .to_string(),
        );
        fixture.write(
            &format!("{dir}/verification.json"),
            "{\"acceptance_manifest\":{\"entries\":[]}}\n",
        );
        fixture.write(&format!("{dir}/lesson-bundle.md"), "SYNTHETIC lesson\n");
    }
    fixture.write(
        ".specsync/archive/changes/2026-11-03-zzz-unfinalized/lesson-bundle.md",
        "SYNTHETIC lesson\n",
    );

    let next = fixture.ok(&["next", "--json"]);
    let path = |slug: &str| format!(".specsync/archive/changes/2026-11-03-{slug}/lesson-bundle.md");
    assert_eq!(
        next["lessons"]["archived_bundles"],
        json!({"total": 6, "items": [
            path("export-chats"),
            path("search-messages"),
            path("add-bounded-pagination"),
        ]})
    );
    let shown = text(&fixture, &["next"]);
    assert!(
        shown.contains(&format!(
            "Archived lesson bundles (3 of 6, newest first):\n  {}\n  {}\n  {}\n",
            path("export-chats"),
            path("search-messages"),
            path("add-bounded-pagination")
        )),
        "{shown}"
    );

    // The librarian's hot archives are the same three.
    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/INDEX.md");
    for (slug, status) in [
        ("export-chats", "hot"),
        ("search-messages", "hot"),
        ("add-bounded-pagination", "hot"),
        ("protect-secrets", "cold"),
        ("preserve-history", "cold"),
        ("enforce-rules", "cold"),
    ] {
        assert!(
            index.contains(&format!(
                "- [{slug}](archives/{slug}.md) {slug} ({status})\n"
            )),
            "{slug} {status} in {index}"
        );
    }
}

/// Review of #78: `next` reads the hot archives with the librarian's own rule, so the
/// newest archive without a lesson bundle is shown as missing, and hidden directories,
/// archives without a readable finalization naming their change, and invalid records
/// are not counted. SYNTHETIC archives.
#[test]
fn next_and_the_librarian_pick_the_same_hot_archives_when_a_bundle_is_missing() {
    let fixture = Fixture::new();
    fixture.init();
    let base = ".specsync/archive/changes/2026-11-03";
    let archive = |dir: &str, finalization: Option<&str>, bundle: bool| {
        if let Some(finalization) = finalization {
            fixture.write(&format!("{base}-{dir}/finalization.json"), finalization);
        }
        fixture.write(
            &format!("{base}-{dir}/verification.json"),
            "{\"acceptance_manifest\":{\"entries\":[]}}\n",
        );
        if bundle {
            fixture.write(
                &format!("{base}-{dir}/lesson-bundle.md"),
                "SYNTHETIC lesson\n",
            );
        }
    };
    let record =
        |id: &str, timestamp: Value| json!({"change_id": id, "timestamp": timestamp}).to_string();
    archive("d", Some(&record("d", json!(300))), false);
    archive("b", Some(&record("b", json!(200))), true);
    archive("c", Some(&record("c", json!(200))), true);
    archive("a", Some(&record("a", json!(100))), true);
    archive("e", Some(&record("e", json!(50))), true);
    archive("f", Some(&record("f", json!("bad"))), true);
    archive("g", Some(&record("another", json!(400))), true);
    archive("h", None, true);
    archive("i", Some("{not json"), true);
    fixture.write(
        ".specsync/archive/changes/.hidden-z/finalization.json",
        &record("z", json!(5000)),
    );
    fixture.write(
        ".specsync/archive/changes/.hidden-z/lesson-bundle.md",
        "SYNTHETIC lesson\n",
    );

    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        next["lessons"]["archived_bundles"],
        json!({"total": 6, "items": [
            format!("{base}-d/lesson-bundle.md (missing)"),
            format!("{base}-c/lesson-bundle.md"),
            format!("{base}-b/lesson-bundle.md"),
        ]})
    );
    let shown = text(&fixture, &["next"]);
    assert!(
        shown.contains(&format!(
            "Archived lesson bundles (3 of 6, newest first):\n  {base}-d/lesson-bundle.md (missing)\n  {base}-c/lesson-bundle.md\n  {base}-b/lesson-bundle.md\n"
        )),
        "{shown}"
    );

    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/INDEX.md");
    for (slug, status) in [("d", "hot"), ("c", "hot"), ("b", "hot"), ("a", "cold")] {
        assert!(
            index.contains(&format!(
                "- [{slug}](archives/{slug}.md) {slug} ({status})\n"
            )),
            "{slug} {status} in {index}"
        );
    }
}

/// Render a 0.5.x-shaped checkpoint JSON into the active session's session.md
/// (front matter plus body sections), keeping its id and Timeline, and return the
/// bytes written.
fn write_session(fixture: &Fixture, session: &Value) -> String {
    let file = fixture.session_file();
    let existing = fixture.read(&file);
    let text = render_session(&existing, session);
    fixture.write(&file, &text);
    text
}

/// The session.md bytes for a 0.5.x-shaped checkpoint JSON, given the active session's
/// current file (its id and Timeline are kept).
fn render_session(existing: &str, session: &Value) -> String {
    let id = existing
        .lines()
        .find_map(|line| line.strip_prefix("id: "))
        .unwrap()
        .to_string();
    let timeline = existing
        .split("\n## Timeline\n")
        .nth(1)
        .unwrap_or("")
        .to_string();
    let scalar = |value: &Value| value.as_str().unwrap_or_default().to_string();
    let list = |value: &Value| -> String {
        let items: Vec<&str> = value
            .as_array()
            .map(|all| all.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        format!("[{}]", items.join(", "))
    };
    let mut front = format!("---\nkind: session\nid: {id}\naliases: []\n");
    front.push_str(&format!("objective: {}\n", scalar(&session["objective"])));
    front.push_str(&format!(
        "intent_scope: {}\n",
        scalar(&session["intent_scope"])
    ));
    front.push_str(&format!("modules: {}\n", list(&session["modules"])));
    let status = scalar(&session["stage"]["status"]);
    front.push_str(&format!(
        "stage: {}\n",
        scalar(&session["stage"]["current"])
    ));
    front.push_str(&format!(
        "stage_status: {}\n",
        if status.is_empty() { "active" } else { &status }
    ));
    front.push_str(&format!(
        "iteration: {}\n",
        session["stage"]["iteration"].as_u64().unwrap_or(1)
    ));
    front.push_str("callers: []\npr: \"\"\n");
    front.push_str(&format!(
        "native_change: \"{}\"\n",
        scalar(&session["native_change"])
    ));
    for key in ["criteria", "requirements", "cases", "design", "files"] {
        front.push_str(&format!("{key}: {}\n", list(&session["references"][key])));
    }
    for key in ["observations", "bugs", "blockers"] {
        front.push_str(&format!("{key}: {}\n", list(&session[key])));
    }
    let gates = session["awaiting_human"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    if gates.is_empty() {
        front.push_str("waiting_on_human: []\n");
    } else {
        front.push_str("waiting_on_human:\n");
        for gate in &gates {
            front.push_str(&format!(
                "  - gate: {}\n    holds: intent\n    opened: 2026-01-01T00:00:00Z\n",
                gate.as_str().unwrap()
            ));
        }
    }
    front.push_str("run_consents: []\n");
    front.push_str(&format!("next: {}\n", scalar(&session["next_action"])));
    let items = |key: &str| -> String {
        session[key]
            .as_array()
            .map(|all| {
                all.iter()
                    .filter_map(Value::as_str)
                    .map(|item| format!("- {item}\n"))
                    .collect()
            })
            .unwrap_or_default()
    };
    format!(
        "{}---\n# {}\n\n## Goal\n\n{}\n\n## Where it stands\n\n{}\n## Decisions and why\n\n{}\n## What was verified\n\n## Open threads\n\n{}\n## Timeline\n{}",
        front,
        scalar(&session["objective"]),
        scalar(&session["objective"]),
        items("completed"),
        items("decisions"),
        items("open_questions"),
        timeline
    )
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
        let dir = format!(
            ".specsync/archive/changes/2026-{:02}-{:02}-change-with-a-long-descriptive-name-{index:02}",
            index / 28 + 1,
            index % 28 + 1
        );
        fixture.write(&format!("{dir}/lesson-bundle.md"), "bundle\n");
        fixture.write(
            &format!("{dir}/finalization.json"),
            &json!({
                "change_id": format!("change-with-a-long-descriptive-name-{index:02}"),
                "timestamp": 1_790_000_000 + index,
            })
            .to_string(),
        );
    }
    let modules: Vec<String> = (0..16).map(|index| format!("module-{index:02}")).collect();
    for module in &modules {
        fixture.write(
            &format!("specs/{module}/context.md"),
            "# Context\n\n## Lessons\n\nA lesson.\n",
        );
    }
    // REQ-cli-021 AC1 at its full shape (#125, D9; DRIVE-14): 50 decisions, 40 gates
    // and 40 blockers of 4,096 bytes each, which the 1 MiB checkpoint cap holds.
    let sized = |prefix: &str, index: usize| {
        let mut text = format!("{prefix} {index:02} ");
        text.push_str(&"w".repeat(4_096 * 3 / 4));
        text.push_str(" because ");
        text.push_str(&"r".repeat(4_096 - text.len()));
        text
    };
    let list = |prefix: &str, count: usize| -> Vec<String> {
        (0..count).map(|index| sized(prefix, index)).collect()
    };
    let mut session = fixture.session();
    session["modules"] = json!(modules);
    session["decisions"] = json!(list("decided:", 50));
    session["awaiting_human"] = json!(list("Gate", 40));
    session["blockers"] = json!(list("Blocker", 40));
    session["observations"] = json!(["stale-000"]);
    let session_text = write_session(&fixture, &session);
    assert!(
        session_text.len() > 130 * 4_096 && session_text.len() <= common::CHECKPOINT_CAP,
        "{}",
        session_text.len()
    );
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
        (&next["lessons"]["archived_bundles"]["items"], 3),
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
        "Last failure: fail-037 at 2030-03-17T17:46:40Z: failure 037",
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

/// A portable report fixture with both legacy and slug case definitions. The
/// observations are synthetic records, so no product test or native approval is
/// implied by their passed outcome.
fn read_budget_fixture(with_cases: bool) -> Fixture {
    use sha2::{Digest, Sha256};
    let fixture = Fixture::new();
    fixture.init();
    if with_cases {
        let mut definitions = "profile = \"chough-testing/1\"\nmodule = \"cli\"\n".to_string();
        for id in ["TC-cli-0001", "TC-cli-report-slug"] {
            definitions.push_str(&format!(
                r#"
[[case]]
id = "{id}"
title = "A case"
status = "active"
track = "functional"
method = "automated"
criteria = ["LIFE-3"]
requirements = ["REQ-cli-001"]
environments = ["local"]
artifacts = ["test-log"]
expected = ["reported"]
[case.bind]
adapter = "command"
argv = ["unused-test-fixture"]
expect_output = ["reported"]
"#
            ));
        }
        fixture.write(".chough/cases/cli.cases.toml", &definitions);
    }
    fixture.write(".chough/artifacts/report-input.log", "artifact\n");
    fixture.write(
        ".specsync/changes/observed/plan.md",
        "A native source artifact, without configured native tools.\n",
    );
    fixture.commit("fixed report inputs");
    let cases = if with_cases {
        fixture.ok(&["cases", "list"])["cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|case| json!({"id": case["id"], "digest": case["digest"]}))
            .collect::<Vec<_>>()
    } else {
        vec![]
    };
    let snapshot = fixture.ok(&["status"])["snapshot"].clone();
    let fingerprint = format!("{:x}", Sha256::digest(b"artifact\n"));
    let mut refs = vec!["checkpoint".to_string()];
    for index in 0..12 {
        let id = format!("report-observation-{index}");
        let record = json!({
            "schema_version": 2,
            "id": id,
            "criteria": ["LIFE-3"],
            "requirements": ["REQ-cli-001"],
            "contracts": ["specs/cli/cli.spec.md"],
            "kind": "dry-run",
            "outcome": "passed",
            "environment": "local",
            "summary": "Synthetic report input, not an executed product check",
            "artifacts": [".chough/artifacts/report-input.log"],
            "artifact_fingerprints": {".chough/artifacts/report-input.log": fingerprint},
            "observed_at_unix_seconds": 1,
            "snapshot": snapshot,
            "provenance": "synthetic test record",
            "cases": cases,
        });
        fixture.write(&format!(".chough/evidence/{id}.json"), &record.to_string());
        refs.extend(["--observation".to_string(), id]);
    }
    fixture.ok(&refs.iter().map(String::as_str).collect::<Vec<_>>());
    fixture
}

fn traced_report(fixture: &Fixture, args: &[&str], trace: bool) -> std::process::Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(fixture.root())
        .args(args)
        .env("CHOUGH_TRACE", if trace { "1" } else { "0" })
        .output()
        .unwrap()
}

fn report_records(fixture: &Fixture) -> std::collections::BTreeMap<String, Vec<u8>> {
    fn collect(
        root: &std::path::Path,
        path: &std::path::Path,
        records: &mut std::collections::BTreeMap<String, Vec<u8>>,
    ) {
        if path.is_dir() {
            for entry in std::fs::read_dir(path).unwrap() {
                collect(root, &entry.unwrap().path(), records);
            }
        } else if path.is_file() {
            records.insert(
                path.strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
                std::fs::read(path).unwrap(),
            );
        }
    }
    let mut records = std::collections::BTreeMap::new();
    for name in [".chough", ".specsync"] {
        collect(fixture.root(), &fixture.root().join(name), &mut records);
    }
    records
}

#[test]
fn report_case_reads_are_shared_across_observations_and_references() {
    let fixture = read_budget_fixture(true);
    for (args, lookups) in [
        (vec!["status", "--environment", "local"], 48),
        (vec!["next", "--environment", "local", "--json"], 24),
    ] {
        let before = report_records(&fixture);
        let output = traced_report(&fixture, &args, true);
        assert!(output.status.success());
        let trace = String::from_utf8(output.stderr).unwrap();
        assert_eq!(trace.matches("case-suite loads=").count(), 1, "{trace}");
        assert!(
            trace.contains(&format!(
                "case-suite loads=1 lookups={lookups} unique_ids=2"
            )),
            "{trace}"
        );
        assert_eq!(trace.matches("phase case-suite ").count(), 1, "{trace}");
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        if args[0] == "status" {
            assert!(
                report["observations"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|row| { row["freshness"] == "current-observation" })
            );
            assert!(
                report["observation_references"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|row| { row["label"] == "current" })
            );
        } else {
            assert_eq!(report["observations"]["referenced_current"], 12);
        }
        assert_eq!(before, report_records(&fixture));
    }
}

#[test]
fn report_case_reads_are_lazy_when_observations_have_no_case_binding() {
    let fixture = read_budget_fixture(false);
    // A malformed unrelated definition must remain unconsulted without a binding.
    fixture.write(".chough/cases/cli.cases.toml", "not TOML");
    for command in ["status", "next"] {
        let output = traced_report(&fixture, &[command, "--environment", "local"], true);
        assert!(output.status.success());
        let trace = String::from_utf8(output.stderr).unwrap();
        assert!(
            trace.contains("case-suite loads=0 lookups=0 unique_ids=0"),
            "{trace}"
        );
        assert!(!trace.contains("phase case-suite "), "{trace}");
    }
}

fn report_freshness(fixture: &Fixture, environment: &str) -> String {
    let report = fixture.ok(&["status", "--environment", environment]);
    let records = report["observations"].as_array().unwrap();
    let state = records[0]["freshness"].as_str().unwrap();
    assert!(records.iter().all(|row| row["freshness"] == state));
    state.to_string()
}

#[test]
fn report_read_context_does_not_survive_the_next_request() {
    let fixture = read_budget_fixture(true);
    let original = fixture.read(".chough/cases/cli.cases.toml");
    assert_eq!(report_freshness(&fixture, "local"), "current-observation");
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &original.replace("reported", "changed"),
    );
    assert_eq!(report_freshness(&fixture, "local"), "stale-case-definition");
    fixture.write(".chough/cases/cli.cases.toml", &original);
    assert_eq!(report_freshness(&fixture, "local"), "current-observation");
    fixture.write(".chough/artifacts/report-input.log", "changed\n");
    assert_eq!(report_freshness(&fixture, "local"), "stale-artifact");
    fixture.write(".chough/artifacts/report-input.log", "artifact\n");
    assert_eq!(report_freshness(&fixture, "another"), "stale-environment");
    fixture.write("src/input.txt", "changed\n");
    assert_eq!(
        report_freshness(&fixture, "local"),
        "stale-revision-or-inputs"
    );
    fixture.write("src/input.txt", "first version\n");
    assert_eq!(report_freshness(&fixture, "local"), "current-observation");
    fixture.write(".specsync/changes/observed/plan.md", "changed\n");
    assert_eq!(
        report_freshness(&fixture, "local"),
        "stale-revision-or-inputs"
    );
}

#[test]
fn report_traces_preserve_stdout_exit_results_and_managed_records() {
    let fixture = read_budget_fixture(true);
    for args in [
        vec!["status", "--environment", "local"],
        vec!["next", "--environment", "local"],
        vec!["next", "--environment", "local", "--json"],
    ] {
        let before = report_records(&fixture);
        let quiet = traced_report(&fixture, &args, false);
        let traced = traced_report(&fixture, &args, true);
        assert!(quiet.status.success());
        assert_eq!(quiet.status.code(), traced.status.code());
        assert_eq!(quiet.stdout, traced.stdout);
        assert_eq!(quiet.stderr, b"");
        let trace = String::from_utf8(traced.stderr).unwrap();
        for phase in [
            "snapshot.git-enumeration",
            "snapshot.hash",
            "snapshot.git-status",
            "freshness.artifacts",
        ] {
            assert!(trace.contains(&format!("phase {phase} ")), "{trace}");
        }
        assert!(trace.lines().all(|line| line.len() <= 1024));
        assert_eq!(before, report_records(&fixture));
    }
    fixture.write(".chough/cases/cli.cases.toml", "broken case file");
    let loader_error = traced_report(&fixture, &["cases", "list"], false);
    for command in ["status", "next"] {
        let before = report_records(&fixture);
        let quiet = traced_report(&fixture, &[command], false);
        let traced = traced_report(&fixture, &[command], true);
        assert!(!quiet.status.success());
        assert_eq!(
            quiet.stderr, loader_error.stderr,
            "preserve the original loader error chain"
        );
        assert_eq!(quiet.status.code(), traced.status.code());
        assert_eq!(quiet.stdout, traced.stdout);
        let traced_error = String::from_utf8(traced.stderr)
            .unwrap()
            .split_inclusive('\n')
            .filter(|line| !line.starts_with("chough-trace: "))
            .collect::<String>();
        assert_eq!(traced_error, String::from_utf8(quiet.stderr).unwrap());
        assert_eq!(before, report_records(&fixture));
    }
}

#[test]
fn report_read_context_preserves_missing_retired_and_duplicate_case_outcomes() {
    let fixture = read_budget_fixture(true);
    let path = ".chough/cases/cli.cases.toml";
    let original = fixture.read(path);
    fixture.write(
        path,
        &original.replace("status = \"active\"", "status = \"retired\""),
    );
    assert_eq!(report_freshness(&fixture, "local"), "stale-case-definition");
    std::fs::remove_file(fixture.root().join(path)).unwrap();
    assert_eq!(report_freshness(&fixture, "local"), "stale-case-definition");
    fixture.write(path, &original.replace("TC-cli-report-slug", "TC-cli-0001"));
    let before = report_records(&fixture);
    for command in ["next", "status"] {
        let output = traced_report(&fixture, &[command], true);
        assert!(!output.status.success());
        assert!(
            String::from_utf8(output.stderr)
                .unwrap()
                .contains("duplicate case ID TC-cli-0001")
        );
        assert_eq!(before, report_records(&fixture));
    }
    fixture.write(path, &original);
    assert_eq!(report_freshness(&fixture, "local"), "current-observation");
}

/// Open questions are part of resuming (LIFE-3): `next` shows them bounded like
/// blockers, `status` reports every one, and resolving one removes it (#84).
#[cfg(unix)]
#[test]
fn open_questions_appear_in_next_and_status() {
    let fixture = approved_at_plan("question-change");
    let questions: Vec<String> = (1..=12)
        .map(|n| format!("Should SAVE{n} stack with SAVE10?"))
        .collect();
    let mut args = vec!["checkpoint"];
    for question in &questions {
        args.extend(["--question", question.as_str()]);
    }
    fixture.ok(&args);

    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["open_questions"]["total"], 12, "{next:#}");
    let items = next["open_questions"]["items"].as_array().unwrap();
    assert_eq!(items.len(), 10, "{next:#}");
    assert_eq!(items[0], questions[0].as_str());
    let shown = text(&fixture, &["next"]);
    assert!(shown.contains("Open questions: 12 open"), "{shown}");
    assert!(shown.contains(&format!("  - {}", questions[0])), "{shown}");
    assert!(shown.contains("2 more; chough status"), "{shown}");
    let status = fixture.ok(&["status"]);
    assert_eq!(
        status["checkpoint"]["open_questions"],
        Value::from(questions.clone()),
        "{status:#}"
    );

    fixture.ok(&["checkpoint", "--resolve-question", &questions[0]]);
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["open_questions"]["total"], 11);
    assert_ne!(next["open_questions"]["items"][0], questions[0].as_str());
    let status = fixture.ok(&["status"]);
    assert!(
        !status["checkpoint"]["open_questions"]
            .as_array()
            .unwrap()
            .contains(&Value::from(questions[0].as_str()))
    );
}

/// DRIVE-14 (REQ-cli-032): a checkpoint at exactly the 1 MiB cap is read, and `next`,
/// `status` and `context` answer within the debug-build budget of 2 seconds, with the
/// focused view still within 16,384 bytes; one byte more is refused naming the cap.
#[test]
fn a_checkpoint_at_the_cap_stays_fast_and_one_byte_more_is_refused() {
    let fixture = Fixture::new();
    fixture.init();
    let mut session = fixture.session();
    let lists = [
        "decisions",
        "completed",
        "open_questions",
        "blockers",
        "awaiting_human",
    ];
    for list in lists {
        session[list] = json!([]);
    }
    let file = fixture.session_file();
    let existing = fixture.read(&file);
    let size = |session: &Value| render_session(&existing, session).len();
    'fill: for list in lists {
        for index in 0..199 {
            if size(&session) + 4_200 > common::CHECKPOINT_CAP {
                break 'fill;
            }
            session[list]
                .as_array_mut()
                .unwrap()
                .push(json!(format!("{list} {index:03} {}", "x".repeat(4_000))));
        }
    }
    let compact = render_session(&existing, &session);
    let at_cap = format!(
        "{compact}{}",
        " ".repeat(common::CHECKPOINT_CAP - compact.len())
    );
    assert_eq!(at_cap.len(), common::CHECKPOINT_CAP);
    fixture.write(&file, &at_cap);
    for args in [
        &["next"][..],
        &["next", "--json"],
        &["status", "--json"],
        &["context"],
    ] {
        let started = std::time::Instant::now();
        let output = fixture.run(args);
        let elapsed = started.elapsed();
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            elapsed.as_secs_f64() < 2.0,
            "{args:?} took {elapsed:?} at the cap"
        );
        if args[0] == "next" {
            assert!(
                output.stdout.len() <= 16_384,
                "{args:?}: {}",
                output.stdout.len()
            );
        }
    }
    fixture.write(&file, &format!("{at_cap} "));
    let output = fixture.run(&["next"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("1048576"));
}
