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

/// Every history event's transition, in no particular order.
fn transitions(fixture: &Fixture) -> Vec<Value> {
    let mut found = vec![];
    let Ok(cycles) = fs::read_dir(fixture.root().join(".chough/history/cycles")) else {
        return found;
    };
    for cycle in cycles {
        for event in fs::read_dir(cycle.unwrap().path()).unwrap() {
            let value: Value =
                serde_json::from_str(&fs::read_to_string(event.unwrap().path()).unwrap()).unwrap();
            found.push(value["transition"].clone());
        }
    }
    found
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
    let shipped = fixture.run(&[
        "change",
        "ship",
        ID,
        "--close-gate",
        "Implementation review of d1",
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
    let line = |kind: &str| {
        json!({"schema_version": 1, "at_unix_seconds": 1, "kind": kind, "from": "intent", "to": "intent", "reason": null, "gate": null, "revision": null, "refresh": [], "direct_write": write}).to_string()
    };
    fixture.write(
        ".chough/transitions.jsonl",
        &format!("{}\n", line("direct-native-write")),
    );
    let shown = fixture.ok(&["stage", "show"]);
    assert_eq!(shown["recent_transitions"][0]["direct_write"], write);
    for kind in ["gate-closed", "iterate"] {
        fixture.write(".chough/transitions.jsonl", &format!("{}\n", line(kind)));
        let error = fixture.fails(&["stage", "show"]);
        assert!(error.contains("invalid transition"), "{kind}: {error}");
    }
}

#[test]
fn the_change_log_names_the_oldest_chough_that_reads_direct_write_events() {
    let log =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("CHANGELOG.md")).unwrap();
    let entry = log
        .split("\n- ")
        .find(|entry| entry.contains("direct-native-write"))
        .expect("no change-log entry names direct-native-write");
    assert!(
        entry.contains("oldest Chough that reads"),
        "the entry names no oldest reader: {entry}"
    );
    assert!(
        entry.contains("refuse"),
        "the entry does not say older builds refuse it: {entry}"
    );
}
