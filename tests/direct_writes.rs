//! Direct lifecycle writes noted (REQ-cli-060, REQ-history-013, DRIVE-21, #183): a
//! SpecSync lifecycle record of the active session's change that changes outside
//! Chough is recorded once, as a `direct-native-write` history event, with the next
//! successful stage move; Chough's own change steps are never flagged.
#![cfg(unix)]

mod common;

use common::Fixture;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const ID: &str = "d1";
const KEY: &str = "direct-d1";
const DIR: &str = ".specsync/changes/d1";

/// A stand-in `specsync`. `change status` reports the change with the state in
/// `state.json`; `change answer` rewrites `state.json`; `change check` writes
/// `verification.json` and moves the state to verifying; `change ship` records its
/// call and exits 0. Anything else exits 0 with `{}`.
const STUB: &str = r#"#!/bin/sh
dir=.specsync/changes/d1
state=$(sed -n 's/.*"state": *"\([a-z_]*\)".*/\1/p' "$dir/state.json" 2>/dev/null)
[ -n "$state" ] || state=approved
case "$2" in
status)
    printf '{"summary":{"id":"%s","state":"%s","approval_valid":true,"correction_valid":true,"next_action":"next","terminal_evidence":{"validity":"authenticated-history"}},"change":{"affected_specs":["cli"],"no_spec_change":true},"acceptance_entries":[{"path":"specs/cli/cli.spec.md","kind":"file"}]}\n' "$3" "$state"
    ;;
answer)
    n=$(cat .stub-answers 2>/dev/null || echo 0); n=$((n + 1)); echo "$n" >.stub-answers
    printf '{"state": "%s", "answers": %s}\n' "$state" "$n" >"$dir/state.json"
    echo '{}'
    ;;
check)
    printf '{"passed": true}\n' >"$dir/verification.json"
    printf '{"state": "verifying"}\n' >"$dir/state.json"
    echo 'verified'
    ;;
ship)
    echo "$*" >>.stub-ship-calls
    echo 'shipped'
    ;;
*)
    echo '{}'
    ;;
esac
"#;

/// A session for native change `d1` at intent, with its lifecycle records present.
fn project() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(".specsync/sdd.json", "{\"enabled\": true}\n");
    fixture.write("bin/specsync", STUB);
    fixture.executable("bin/specsync");
    fixture.hi_stand_in();
    fixture.write(&format!("{DIR}/state.json"), "{\"state\": \"approved\"}\n");
    fixture.no_spec_approval(ID);
    fixture.ok(&["session", "new", KEY, "--objective", "Watch d1"]);
    fixture.ok(&["checkpoint", "--native-change", ID]);
    fixture
}

/// Every Timeline entry of the active session, in order, as `chough history show`
/// reads session.md.
fn transitions(fixture: &Fixture) -> Vec<Value> {
    fixture.ok(&["history", "show"])["entries"]
        .as_array()
        .unwrap()
        .clone()
}

fn direct_writes(fixture: &Fixture) -> Vec<Value> {
    transitions(fixture)
        .into_iter()
        .filter(|transition| transition["kind"] == "direct-native-write")
        .collect()
}

fn iterate(fixture: &Fixture, note: &str) {
    fixture.ok(&["stage", "iterate", "--note", note]);
}

/// Every file under `.chough/` with its bytes.
fn dot_chough(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(base, &path, files);
            } else {
                let name = path.strip_prefix(base).unwrap().display().to_string();
                files.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    let mut files = BTreeMap::new();
    walk(root, &root.join(".chough"), &mut files);
    files
}

#[test]
fn a_direct_lifecycle_write_is_recorded_once_with_the_next_stage_move() {
    let fixture = project();
    iterate(&fixture, "first sight");
    assert!(direct_writes(&fixture).is_empty(), "first sight flagged");
    // A direct approval rewrites approvals.json, as `specsync change approve` does.
    fixture.write(
        &format!("{DIR}/approvals.json"),
        "{\"approvals\":[{\"gate\":\"definition\",\"actor\":\"SYNTHETIC-direct\",\"approved_delta_digests\":{}}],\"reopenings\":[]}\n",
    );
    iterate(&fixture, "after a direct approval");
    let found = direct_writes(&fixture);
    assert_eq!(found.len(), 1, "{found:#?}");
    let event = &found[0];
    assert_eq!(event["from"], event["to"]);
    assert_eq!(
        event["direct_write"],
        json!({
            "change": ID,
            "files": [{"path": "approvals.json", "change": "changed"}],
            "before_state": "approved",
            "after_state": "approved",
        })
    );
    iterate(&fixture, "nothing new");
    assert_eq!(direct_writes(&fixture).len(), 1, "recorded twice");
}

#[test]
fn chough_change_steps_are_not_flagged_but_an_earlier_direct_write_is() {
    let fixture = project();
    iterate(&fixture, "first sight");
    fixture.ok(&["change", "answer", ID, "constraints", "none"]);
    let checked = fixture.run(&["change", "check", ID]);
    assert!(checked.status.success());
    iterate(&fixture, "after chough's own steps");
    assert!(
        direct_writes(&fixture).is_empty(),
        "chough's own steps were flagged: {:#?}",
        direct_writes(&fixture)
    );
    // A direct review before another chough step is still recorded, and only it.
    fixture.write(&format!("{DIR}/review.json"), "{\"verdict\": \"pass\"}\n");
    fixture.ok(&["change", "answer", ID, "constraints", "still none"]);
    iterate(&fixture, "after a direct review");
    let found = direct_writes(&fixture);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(
        found[0]["direct_write"]["files"],
        json!([{"path": "review.json", "change": "added"}])
    );
    assert_eq!(found[0]["direct_write"]["before_state"], "verifying");
}

/// The fresh-eyes finding on plain `specsync change check`: the docs name it for
/// materializing an approved delta without committing, and the direct write it leaves
/// is recorded with text that says what changed, never as an empty Timeline entry.
#[test]
fn a_direct_write_entry_says_what_changed_outside_chough() {
    let fixture = project();
    iterate(&fixture, "first sight");
    // The plain check, run with `specsync` as the docs say, not through Chough.
    let checked = std::process::Command::new(fixture.root().join("bin/specsync"))
        .args(["change", "check", ID])
        .current_dir(fixture.root())
        .output()
        .unwrap();
    assert!(checked.status.success());
    iterate(&fixture, "after a plain check");
    let found = direct_writes(&fixture);
    assert_eq!(found.len(), 1, "{found:#?}");
    let text = "d1's lifecycle records changed outside Chough: state.json changed, verification.json added; state approved → verifying";
    assert_eq!(found[0]["reason"], text, "{found:#?}");
    let session = fixture.read(&fixture.session_file());
    assert!(
        session.contains(&format!(
            " · direct-native-write · intent → intent · {text} ^"
        )),
        "{session}"
    );
}

#[test]
fn first_sight_sets_a_baseline_and_flags_nothing() {
    let fixture = project();
    // Records that existed before Chough ever looked are not flagged.
    fixture.write(&format!("{DIR}/review.json"), "{\"verdict\": \"pass\"}\n");
    iterate(&fixture, "first sight");
    assert!(direct_writes(&fixture).is_empty());
    fs::remove_file(fixture.root().join(DIR).join("review.json")).unwrap();
    iterate(&fixture, "after a removal");
    let found = direct_writes(&fixture);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(
        found[0]["direct_write"]["files"],
        json!([{"path": "review.json", "change": "removed"}])
    );
}

#[test]
fn next_and_status_show_a_pending_write_and_change_nothing() {
    let fixture = project();
    iterate(&fixture, "first sight");
    fixture.write(&format!("{DIR}/review.json"), "{\"verdict\": \"pass\"}\n");
    let before = dot_chough(fixture.root());
    let next = fixture.run(&["next", "--environment", "local"]);
    assert!(next.status.success());
    let text = String::from_utf8_lossy(&next.stdout);
    let line = text
        .lines()
        .find(|line| line.starts_with("Direct lifecycle writes:"))
        .unwrap_or_else(|| panic!("next names no direct write:\n{text}"));
    assert!(line.contains("review.json added"), "{line}");
    let status = fixture.ok(&["status"]);
    assert_eq!(
        status["direct_writes"]["pending"][0]["files"],
        json!([{"path": "review.json", "change": "added"}]),
        "{:#}",
        status["direct_writes"]
    );
    assert_eq!(dot_chough(fixture.root()), before, ".chough/ changed");
    assert!(
        direct_writes(&fixture).is_empty(),
        "next or status recorded"
    );
}

#[test]
fn a_direct_write_before_ship_is_recorded_at_the_done_move() {
    let fixture = project();
    // The review gate drives the native review as the decided human (REQ-records-008).
    fixture.write(
        ".chough/local/identity.toml",
        "[[human]]\nname = \"SYNTHETIC Human\"\naliases = [\"SYNTHETIC-human\"]\nspecsync_actor = \"SYNTHETIC-reviewer\"\n",
    );
    // An archived change at accept with its review gate open, as for `change ship`.
    let archive = ".specsync/archive/changes/2026-11-01-d1";
    fixture.write(
        &format!("{archive}/finalization.json"),
        &format!(
            "{{\"schema_version\": 2, \"change_id\": \"d1\", \"timestamp\": 500, \"implementation_commit\": \"{}\"}}\n",
            "1".repeat(40)
        ),
    );
    fixture.write(
        &format!("{archive}/verification.json"),
        &json!({
            "commit": "2".repeat(40), "passed": true, "requirement_ids": ["REQ-cli-001"],
            "acceptance_manifest": {"schema_version": 1, "entries": [{"path": "specs/cli/cli.spec.md", "kind": "file"}]},
        })
        .to_string(),
    );
    fixture.write(
        &format!("{archive}/change.md"),
        "---\nid: d1\nstate: archived\n---\n\n# Title of d1\n\n## Intent\n\nx\n\n## Acceptance Criteria\n\n- d1 works\n",
    );
    fixture.write(&format!("{DIR}/state.json"), "{\"state\": \"archived\"}\n");
    fixture.ok(&[
        "stage",
        "reset",
        "--to",
        "accept",
        "--reason",
        "Ready to ship",
    ]);
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Implementation review of d1",
    ]);
    fixture.commit("fixture");
    // The review is recorded directly, as before slice 5.
    fixture.write(&format!("{DIR}/review.json"), "{\"verdict\": \"pass\"}\n");
    // Ship goes through the gate list (REQ-cli-061): list, then ship with it.
    let listed = fixture.ok(&["gate", "list", "--json"]);
    let list_id = listed["list"].as_str().unwrap().to_string();
    let shipped = fixture.run(&[
        "change",
        "ship",
        ID,
        "--list",
        &list_id,
        "--gate",
        "1",
        "--decided-by",
        "SYNTHETIC-human",
        "--words",
        "pass",
        "--where",
        "a test",
        "--recorded-by",
        "SYNTHETIC-agent",
    ]);
    assert!(
        shipped.status.success(),
        "ship failed: {}",
        String::from_utf8_lossy(&shipped.stderr)
    );
    let found = direct_writes(&fixture);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(
        found[0]["direct_write"]["files"],
        json!([{"path": "review.json", "change": "added"}])
    );
    let done = transitions(&fixture)
        .into_iter()
        .any(|transition| transition["to"] == "done");
    assert!(done, "no move into done");
}

#[test]
fn a_direct_write_field_on_another_kind_is_an_invalid_payload() {
    let fixture = Fixture::new();
    fixture.init();
    let write = json!({"change": ID, "files": [{"path": "approvals.json", "change": "changed"}], "before_state": "approved", "after_state": "approved"});
    let entry = |kind: &str| {
        format!(
            "- 2026-11-11T00:00:00Z · {kind} · intent → intent ·  ^t-aaaaaa\n  - direct_write: {}\n",
            serde_json::to_string(&write).unwrap()
        )
    };
    let file = fixture.session_file();
    let original = fixture.read(&file);
    fixture.write(
        &file,
        &format!("{original}\n{}", entry("direct-native-write")),
    );
    let shown = fixture.ok(&["history", "show"]);
    let entries = shown["entries"].as_array().unwrap();
    assert_eq!(entries.last().unwrap()["direct_write"], write);
    for kind in ["gate-closed", "iterate"] {
        fixture.write(&file, &format!("{original}\n{}", entry(kind)));
        let error = fixture.fails(&["history", "show"]);
        assert!(error.contains("invalid transition"), "{kind}: {error}");
    }
}

/// #189: a `direct-native-write` event is a known kind that starts or ends no stage
/// segment, so `chough timing` skips it without calling the report partial.
#[test]
fn timing_skips_direct_write_events_without_marking_the_report_partial() {
    let fixture = project();
    iterate(&fixture, "first sight");
    fixture.write(
        &format!("{DIR}/approvals.json"),
        "{\"approvals\":[{\"gate\":\"definition\",\"actor\":\"SYNTHETIC-direct\",\"approved_delta_digests\":{}}],\"reopenings\":[]}\n",
    );
    iterate(&fixture, "after a direct approval");
    assert_eq!(direct_writes(&fixture).len(), 1, "no direct write recorded");
    let report = fixture.ok(&["timing", "--json"]);
    let text = report.to_string();
    assert!(
        !text.contains("unrecognised kinds"),
        "timing called the report partial for a direct write: {report:#}"
    );
    fn find<'a>(value: &'a Value, key: &str, found: &mut Vec<&'a Value>) {
        match value {
            Value::Object(map) => {
                for (name, inner) in map {
                    if name == key {
                        found.push(inner);
                    }
                    find(inner, key, found);
                }
            }
            Value::Array(items) => items.iter().for_each(|item| find(item, key, found)),
            _ => {}
        }
    }
    let mut found = vec![];
    find(&report, "unrecognised_kinds", &mut found);
    assert!(
        !found.is_empty(),
        "timing reports no unrecognised_kinds count: {report:#}"
    );
    assert!(found.iter().all(|count| **count == json!(0)), "{found:?}");
}

/// REQ-history-002, TC-history-direct-write-fields-are-labelled-lines (t38j's fresh-eyes
/// report): a direct write's `direct_write` is written as labelled lines, its files one
/// level further as `<path>: <change>`, never a JSON object, and reads back whole.
#[test]
fn a_direct_write_is_written_as_labelled_lines() {
    let fixture = project();
    iterate(&fixture, "first sight");
    fixture.write(
        &format!("{DIR}/approvals.json"),
        "{\"approvals\":[{\"gate\":\"definition\",\"actor\":\"SYNTHETIC-direct\",\"approved_delta_digests\":{}}],\"reopenings\":[]}\n",
    );
    iterate(&fixture, "after a direct approval");
    let text = fixture.read(&fixture.session_file());
    assert!(!text.contains("direct_write: {"), "{text}");
    assert!(
        text.contains(&format!(
            "  - direct_write:\n    - change: {ID}\n    - files:\n      - approvals.json: changed\n    - before_state: approved\n    - after_state: approved\n"
        )),
        "{text}"
    );
    let found = direct_writes(&fixture);
    assert_eq!(
        found[0]["direct_write"],
        json!({
            "change": ID,
            "files": [{"path": "approvals.json", "change": "changed"}],
            "before_state": "approved",
            "after_state": "approved",
        })
    );
}
