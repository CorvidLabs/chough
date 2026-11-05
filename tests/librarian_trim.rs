//! Trimming old proof automatically when a change reaches done with the librarian on,
//! and `retention manual` opting out (REQ-cli-041, KNOW-13, #88).
#![cfg(unix)]

mod common;

use common::Fixture;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const DAY: u64 = 86_400;

fn digest(content: &str) -> String {
    format!("{:x}", Sha256::digest(content.as_bytes()))
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// A routine automated run of case TC-a, `age_days` old, with optional log artifacts.
fn write_obs(fixture: &Fixture, id: &str, age_days: u64, logs: &[(&str, &str)]) {
    let mut fingerprints = BTreeMap::new();
    for (path, content) in logs {
        fixture.write(path, content);
        fingerprints.insert(path.to_string(), digest(content));
    }
    let record = json!({
        "schema_version": 2,
        "id": id,
        "criteria": ["LIFE-1"],
        "requirements": ["REQ-cli-001"],
        "contracts": ["specs/cli/cli.spec.md"],
        "kind": "product-test",
        "outcome": "passed",
        "environment": "local",
        "summary": format!("fixture observation {id}"),
        "artifacts": logs.iter().map(|(path, _)| *path).collect::<Vec<_>>(),
        "artifact_fingerprints": fingerprints,
        "observed_at_unix_seconds": now() - age_days * DAY - 60,
        "snapshot": {"revision": "0".repeat(40), "dirty": false, "fingerprint": ZERO},
        "provenance": "fixture",
        "cases": [{"id": "TC-a", "digest": ZERO}],
        "method": "automated",
    });
    fixture.write(
        &format!(".chough/evidence/{id}.json"),
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
}

/// Five archived changes (a1 and a2 cold, a3 to a5 hot) whose manifests list proof, a
/// stub `specsync` reporting every change archived and authenticated, and the checkpoint
/// at accept on archived change a5, so `stage advance` moves into done.
fn at_accept() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nprintf '{\"summary\":{\"id\":\"%s\",\"state\":\"archived\",\"approval_valid\":true,\"correction_valid\":true,\"terminal_evidence\":{\"validity\":\"authenticated-history\"}},\"acceptance_entries\":[]}\\n' \"$3\"\n",
    );
    fixture.executable("bin/specsync");
    fixture.hi_stand_in();
    let listing: [(&str, &[&str]); 5] = [
        ("a1", &["cold-a"]),
        ("a2", &["cold-b"]),
        ("a3", &[]),
        ("a4", &[]),
        ("a5", &["n1"]),
    ];
    for (index, (id, listed)) in listing.iter().enumerate() {
        let dir = format!(".specsync/archive/changes/2026-0{}-01-{id}", index + 1);
        fixture.write(
            &format!("{dir}/finalization.json"),
            &format!(
                "{{\"schema_version\": 2, \"change_id\": \"{id}\", \"timestamp\": {}, \"implementation_commit\": \"{}\"}}\n",
                (index + 1) * 100,
                "1".repeat(40)
            ),
        );
        let mut entries: Vec<Value> =
            vec![json!({"path": "specs/cli/cli.spec.md", "kind": "file", "payload_digest": ZERO})];
        for obs in *listed {
            entries.push(json!({"path": format!(".chough/evidence/{obs}.json"), "kind": "file", "payload_digest": ZERO}));
        }
        if *id == "a1" {
            entries.push(json!({"path": ".chough/artifacts/cold-a.log", "kind": "file", "payload_digest": digest("cold log\n")}));
        }
        fixture.write(
            &format!("{dir}/verification.json"),
            &serde_json::to_string_pretty(&json!({
                "commit": "2".repeat(40),
                "passed": true,
                "requirement_ids": ["REQ-cli-001"],
                "acceptance_manifest": {"schema_version": 1, "entries": entries},
            }))
            .unwrap(),
        );
        fixture.write(
            &format!("{dir}/change.md"),
            &format!(
                "---\nid: {id}\nstate: archived\nbase_commit: {}\n---\n\n# Title of {id}\n\n## Intent\n\nx\n\n## Acceptance Criteria\n\n- {id} works\n",
                "3".repeat(40)
            ),
        );
    }
    fixture.init();
    for (id, age) in [("n1", 1), ("n2", 2), ("n3", 3)] {
        write_obs(&fixture, id, age, &[]);
    }
    write_obs(
        &fixture,
        "cold-a",
        5,
        &[(".chough/artifacts/cold-a.log", "cold log\n")],
    );
    write_obs(&fixture, "cold-b", 6, &[]);
    fixture.commit("fixture");
    let path = fixture.root().join(".chough/session.json");
    let mut session: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    session["stage"]["current"] = json!("accept");
    session["native_change"] = json!("a5");
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&session).unwrap() + "\n",
    )
    .unwrap();
    fixture
}

fn advance_to_done(fixture: &Fixture) -> Value {
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["applied"]["to"], "done", "{moved:#}");
    assert_eq!(moved["stage"]["current"], "done", "{moved:#}");
    moved["applied"]["librarian_trim"].clone()
}

#[test]
fn entering_done_trims_cold_archived_proof_by_default() {
    let fixture = at_accept();
    let trim = advance_to_done(&fixture);
    assert_eq!(trim["status"], "trimmed", "{trim:#}");
    let pruned = trim["summary"]["pruned"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    assert!(pruned.contains(&json!("cold-a")), "{trim:#}");
    assert!(!fixture.exists(".chough/evidence/cold-a.json"));
    assert!(
        fixture.exists(".chough/evidence/n1.json"),
        "hot proof stays"
    );
    assert!(
        trim["authority"]
            .as_str()
            .unwrap_or_default()
            .contains("KNOW-13"),
        "{trim:#}"
    );
    let ledger = fixture.read(".chough/pruned.jsonl");
    assert!(
        ledger
            .lines()
            .any(|line| line.contains("cold-a") && line.contains("KNOW-13")),
        "{ledger}"
    );
}

#[test]
fn retention_manual_opts_out_of_trimming_at_done() {
    let fixture = at_accept();
    fixture.ok(&[
        "pref",
        "set",
        "retention",
        "manual",
        "--source",
        "Human: trim old proof only when I ask",
    ]);
    let trim = advance_to_done(&fixture);
    assert_eq!(trim["status"], "skipped", "{trim:#}");
    assert!(fixture.exists(".chough/evidence/cold-a.json"));
    assert!(!fixture.exists(".chough/pruned.jsonl"));
}

#[test]
fn a_librarian_off_project_is_not_trimmed_at_done() {
    let fixture = at_accept();
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "off",
        "--source",
        "Human: no librarian here",
    ]);
    let trim = advance_to_done(&fixture);
    assert_eq!(trim["status"], "skipped", "{trim:#}");
    assert!(fixture.exists(".chough/evidence/cold-a.json"));
}

#[test]
fn an_open_native_change_keeps_proof_and_the_move_still_succeeds() {
    let fixture = at_accept();
    fixture.write(".specsync/changes/open-one/change.md", "open\n");
    let trim = advance_to_done(&fixture);
    assert_eq!(trim["status"], "refused", "{trim:#}");
    assert!(
        trim["reason"]
            .as_str()
            .unwrap_or_default()
            .contains("open-one"),
        "{trim:#}"
    );
    assert!(fixture.exists(".chough/evidence/cold-a.json"));
}

/// #97 review: a recorded `retention off` is a standing preference against removal, so
/// it stops trimming at done too; only the unset default lets the librarian trim.
#[test]
fn a_recorded_retention_off_is_not_trimmed_at_done() {
    let fixture = at_accept();
    fixture.ok(&[
        "pref",
        "set",
        "retention",
        "off",
        "--source",
        "Human: never remove proof without asking me",
    ]);
    let trim = advance_to_done(&fixture);
    assert_eq!(trim["status"], "skipped", "{trim:#}");
    assert!(fixture.exists(".chough/evidence/cold-a.json"));
    assert!(!fixture.exists(".chough/pruned.jsonl"));
}

/// REQ-cli-043: the move into done regenerates the knowledge directory after the trim,
/// so the trimmed proof's summary is in `knowledge/`, even in a project that never built it.
#[test]
fn entering_done_refreshes_the_knowledge_entries_after_the_trim() {
    let fixture = at_accept();
    let _ = std::fs::remove_dir_all(fixture.root().join("knowledge"));
    let moved = fixture.ok(&["stage", "advance"]);
    let refresh = &moved["applied"]["librarian_index"];
    assert_eq!(refresh["status"], "refreshed", "{moved:#}");
    assert_eq!(refresh["index"], "knowledge/INDEX.md", "{moved:#}");
    assert_eq!(
        moved["applied"]["librarian_trim"]["status"], "trimmed",
        "{moved:#}"
    );
    assert!(fixture.exists("knowledge/README.md") && fixture.exists("knowledge/INDEX.md"));
    let entry = fixture.read("knowledge/archives/a1.md");
    assert!(
        entry.contains("cold-a") && entry.contains("pruned"),
        "{entry}"
    );
}

/// REQ-cli-043: with nothing trimmed, the index is still refreshed at done.
#[test]
fn done_refreshes_the_knowledge_index_when_nothing_is_trimmed() {
    let fixture = at_accept();
    fixture.ok(&[
        "pref",
        "set",
        "retention",
        "manual",
        "--source",
        "Human: trim old proof only when I ask",
    ]);
    let _ = std::fs::remove_dir_all(fixture.root().join("knowledge"));
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(
        moved["applied"]["librarian_trim"]["status"], "skipped",
        "{moved:#}"
    );
    assert_eq!(
        moved["applied"]["librarian_index"]["status"], "refreshed",
        "{moved:#}"
    );
    assert!(fixture.exists("knowledge/archives/a5.md"));
    assert!(fixture.exists(".chough/evidence/cold-a.json"));
}

/// REQ-cli-043: with the librarian off nothing is built at done.
#[test]
fn a_librarian_off_project_gets_no_knowledge_directory_at_done() {
    let fixture = at_accept();
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "off",
        "--source",
        "Human: no librarian here",
    ]);
    let _ = std::fs::remove_dir_all(fixture.root().join("knowledge"));
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["stage"]["current"], "done", "{moved:#}");
    assert_eq!(
        moved["applied"]["librarian_index"]["status"], "skipped",
        "{moved:#}"
    );
    assert!(!fixture.exists("knowledge"));
}
