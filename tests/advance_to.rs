//! Several forward moves on one native read, and one native read on the move into done
//! (REQ-cli-052, DRIVE-9, DRIVE-2).
#![cfg(unix)]

mod common;

use common::Fixture;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;

const ID: &str = "add-thing";
const KEY: &str = "alpha";

fn session(fixture: &Fixture) -> Value {
    let name = if fixture.exists(&format!(".chough/sessions/{KEY}.json")) {
        format!(".chough/sessions/{KEY}.json")
    } else {
        ".chough/session.json".to_string()
    };
    serde_json::from_str(&fixture.read(&name)).unwrap()
}

fn current(fixture: &Fixture) -> String {
    session(fixture)["stage"]["current"]
        .as_str()
        .unwrap()
        .to_string()
}

/// The transitions in the active session's history cycle, oldest first by time and ID.
fn events(fixture: &Fixture) -> Vec<Value> {
    let cycle = session(fixture)["history_cursor"]["cycle"]
        .as_str()
        .unwrap()
        .to_string();
    let dir = fixture
        .root()
        .join(format!(".chough/history/cycles/{cycle}"));
    let mut found: Vec<Value> = match fs::read_dir(dir) {
        Ok(entries) => entries
            .map(|entry| {
                serde_json::from_str::<Value>(&fs::read_to_string(entry.unwrap().path()).unwrap())
                    .unwrap()
            })
            .collect(),
        Err(_) => vec![],
    };
    found.sort_by_key(|event| {
        (
            event["transition"]["at_unix_seconds"].as_u64(),
            event["id"].as_str().map(str::to_string),
        )
    });
    found
}

/// Every file under `.chough/` with its bytes, so a refusal can be shown to write nothing.
fn records(fixture: &Fixture) -> BTreeMap<String, Vec<u8>> {
    fn walk(dir: &std::path::Path, root: &std::path::Path, out: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, root, out);
            } else if !path.ends_with("checkpoint.lock") {
                let name = path.strip_prefix(root).unwrap().display().to_string();
                out.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(
        &fixture.root().join(".chough"),
        fixture.root(),
        &mut out,
    );
    out
}

/// A per-change session at plan on an approved native change, with native calls counted.
fn approved_session_at_plan() -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.ok(&["session", "new", KEY, "--objective", "Ship alpha"]);
    fixture.ok(&["checkpoint", "--native-change", ID]);
    fixture.no_spec_approval(ID);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(current(&fixture), "plan");
    fixture.write(".stub-count", "");
    fixture
}

fn blocking_bug(fixture: &Fixture) {
    fixture.ok(&[
        "bug",
        "add",
        "--title",
        "Totals are wrong",
        "--symptom",
        "Total is off by one",
        "--trigger",
        "Any order",
        "--expected",
        "Exact total",
        "--actual",
        "Total plus one",
        "--environment",
        "test-local",
        "--requirement",
        "REQ-cli-001",
        "--scope",
        "blocking",
    ]);
}

#[test]
fn a_batched_advance_reads_native_status_once_in_a_session() {
    let fixture = approved_session_at_plan();
    let before = events(&fixture).len();
    let batch = fixture.ok(&["stage", "advance", "--to", "accept", "--note", "Batch"]);
    assert_eq!(fixture.native_calls(), 1, "{batch:#}");
    assert_eq!(current(&fixture), "accept");
    let applied = &batch["applied"];
    assert_eq!(applied["kind"], "advance");
    assert_eq!(applied["from"], "plan");
    assert_eq!(applied["to"], "accept");
    let steps: Vec<(&str, &str, &str)> = applied["steps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|step| {
            (
                step["from"].as_str().unwrap(),
                step["to"].as_str().unwrap(),
                step["gate"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        steps,
        [
            ("plan", "build", "native:approved"),
            ("build", "test", "native:approved"),
            ("test", "accept", "native:approved"),
        ],
        "{batch:#}"
    );
    assert_eq!(applied["gate"], "native:approved");
    let events = events(&fixture);
    assert_eq!(events.len(), before + 3);
    let recorded: Vec<(&str, &str, &str, &str, &str)> = events[before..]
        .iter()
        .map(|event| {
            let transition = &event["transition"];
            (
                transition["kind"].as_str().unwrap(),
                transition["from"].as_str().unwrap(),
                transition["to"].as_str().unwrap(),
                transition["gate"].as_str().unwrap(),
                transition["reason"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        recorded,
        [
            ("advance", "plan", "build", "native:approved", "Batch"),
            ("advance", "build", "test", "native:approved", "Batch"),
            ("advance", "test", "accept", "native:approved", "Batch"),
        ]
    );
}

#[test]
fn a_batched_advance_in_a_single_checkpoint_project() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", ID]);
    fixture.no_spec_approval(ID);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.write(".stub-count", "");
    let before = fixture.read(".chough/transitions.jsonl").lines().count();
    let batch = fixture.ok(&["stage", "advance", "--to", "accept"]);
    assert_eq!(fixture.native_calls(), 1, "{batch:#}");
    assert_eq!(current(&fixture), "accept");
    let log = fixture.read(".chough/transitions.jsonl");
    let added: Vec<(String, String)> = log
        .lines()
        .skip(before)
        .map(|line| {
            let transition: Value = serde_json::from_str(line).unwrap();
            (
                transition["from"].as_str().unwrap().to_string(),
                transition["to"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(
        added,
        [
            ("plan".to_string(), "build".to_string()),
            ("build".to_string(), "test".to_string()),
            ("test".to_string(), "accept".to_string()),
        ]
    );
}

#[test]
fn a_batch_with_no_native_prerequisite_reads_nothing() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.ok(&["session", "new", KEY, "--objective", "Ship alpha"]);
    fixture.ok(&["checkpoint", "--native-change", ID]);
    fixture.write(".stub-count", "");
    let before = events(&fixture).len();
    let batch = fixture.ok(&["stage", "advance", "--to", "plan"]);
    assert_eq!(fixture.native_calls(), 0, "{batch:#}");
    assert_eq!(current(&fixture), "plan");
    assert_eq!(events(&fixture).len(), before + 2);
    assert_eq!(batch["applied"]["steps"].as_array().unwrap().len(), 2);
}

#[test]
fn a_refused_step_refuses_the_whole_batch_with_nothing_recorded() {
    // A refusal on the last step, after the shared native read.
    let fixture = approved_session_at_plan();
    blocking_bug(&fixture);
    fixture.native_calls();
    let before = records(&fixture);
    let refused = fixture.fails(&["stage", "advance", "--to", "accept"]);
    assert!(refused.contains("refused:blocking-bugs"), "{refused}");
    assert!(refused.contains("test -> accept"), "{refused}");
    assert!(refused.contains("nothing was recorded"), "{refused}");
    assert_eq!(fixture.native_calls(), 1);
    assert!(records(&fixture) == before, "a refused batch wrote records");

    // A refused native gate on the first step.
    let fixture = approved_session_at_plan();
    fixture.write(".stub-state", "draft\n");
    let before = records(&fixture);
    let refused = fixture.fails(&["stage", "advance", "--to", "test"]);
    assert!(refused.contains("refused:native-draft"), "{refused}");
    assert!(refused.contains("plan -> build"), "{refused}");
    assert_eq!(fixture.native_calls(), 1);
    assert!(records(&fixture) == before, "a refused batch wrote records");

    // Targets that are not a main stage ahead, and an open human gate, read nothing.
    let fixture = approved_session_at_plan();
    for (target, says) in [
        ("plan", "ahead"),
        ("intent", "ahead"),
        ("bugs", "main stage"),
        ("release", "main stage"),
        ("nope", "main stage"),
    ] {
        let before = records(&fixture);
        let refused = fixture.fails(&["stage", "advance", "--to", target]);
        assert!(refused.contains(says), "{target}: {refused}");
        assert_eq!(fixture.native_calls(), 0, "{target}");
        assert!(records(&fixture) == before, "{target} wrote records");
    }
    fixture.ok(&["checkpoint", "--awaiting-human", "Scope decision"]);
    let before = records(&fixture);
    let refused = fixture.fails(&["stage", "advance", "--to", "build"]);
    assert!(refused.contains("refused:awaiting-human"), "{refused}");
    assert_eq!(fixture.native_calls(), 0);
    assert!(records(&fixture) == before, "a refused batch wrote records");
}

/// A per-change session on archived change `a5`, whose archive is the latest, with a
/// stub `specsync` that reports it archived and authenticated with an acceptance
/// manifest and counts its calls, at `stage`.
fn archived_session_at(stage: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(".specsync/sdd.json", "{\"enabled\": true}\n");
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nif [ -f .stub-count ]; then echo \"$*\" >>.stub-calls; fi\nprintf '{\"summary\":{\"id\":\"%s\",\"state\":\"archived\",\"approval_valid\":true,\"correction_valid\":true,\"terminal_evidence\":{\"validity\":\"authenticated-history\"}},\"acceptance_entries\":[{\"path\":\"specs/cli/cli.spec.md\",\"kind\":\"file\"}]}\\n' \"$3\"\n",
    );
    fixture.executable("bin/specsync");
    fixture.hi_stand_in();
    let dir = ".specsync/archive/changes/2026-11-01-a5";
    fixture.write(
        &format!("{dir}/finalization.json"),
        &format!(
            "{{\"schema_version\": 2, \"change_id\": \"a5\", \"timestamp\": 500, \"implementation_commit\": \"{}\"}}\n",
            "1".repeat(40)
        ),
    );
    fixture.write(
        &format!("{dir}/verification.json"),
        &serde_json::to_string_pretty(&json!({
            "commit": "2".repeat(40),
            "passed": true,
            "requirement_ids": ["REQ-cli-001"],
            "acceptance_manifest": {"schema_version": 1, "entries": [
                {"path": "specs/cli/cli.spec.md", "kind": "file"}
            ]},
        }))
        .unwrap(),
    );
    fixture.write(
        &format!("{dir}/change.md"),
        "---\nid: a5\nstate: archived\n---\n\n# Title of a5\n\n## Intent\n\nx\n\n## Acceptance Criteria\n\n- a5 works\n",
    );
    fixture.ok(&["session", "new", KEY, "--objective", "Ship a5"]);
    fixture.ok(&["checkpoint", "--native-change", "a5"]);
    fixture.ok(&[
        "stage",
        "reset",
        "--to",
        stage,
        "--reason",
        "Archived change",
    ]);
    fixture.commit("fixture");
    fixture.write(".stub-count", "");
    fixture
}

#[test]
fn advance_into_done_reads_native_status_once() {
    let fixture = archived_session_at("accept");
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["applied"]["to"], "done", "{moved:#}");
    assert_eq!(moved["applied"]["gate"], "native:archived", "{moved:#}");
    assert_eq!(
        moved["applied"]["librarian_trim"]["status"], "trimmed",
        "{moved:#}"
    );
    assert_eq!(fixture.native_calls(), 1, "{moved:#}");

    let fixture = archived_session_at("test");
    let moved = fixture.ok(&["stage", "advance", "--to", "done"]);
    assert_eq!(moved["applied"]["to"], "done", "{moved:#}");
    assert_eq!(moved["applied"]["steps"].as_array().unwrap().len(), 2);
    assert_eq!(
        moved["applied"]["librarian_trim"]["status"], "trimmed",
        "{moved:#}"
    );
    assert_eq!(fixture.native_calls(), 1, "{moved:#}");
}

#[test]
fn a_plain_advance_is_unchanged() {
    let fixture = approved_session_at_plan();
    let before = events(&fixture).len();
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(fixture.native_calls(), 1);
    assert_eq!(moved["applied"]["from"], "plan");
    assert_eq!(moved["applied"]["to"], "build");
    assert_eq!(moved["applied"]["gate"], "native:approved");
    assert!(moved["applied"].get("steps").is_none(), "{moved:#}");
    assert_eq!(events(&fixture).len(), before + 1);
}
