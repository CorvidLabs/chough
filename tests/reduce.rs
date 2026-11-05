//! Reducing old observation proof (REQ-cli-033, REQ-cli-045; KNOW-16, KNOW-17, KNOW-20):
//! closed-cycle checkpoint references drop at done, superseded routine runs of a cold
//! change are trimmed, proof no archive lists is attributed to a change by cycle time, and
//! the native validity reads run in parallel.
#![cfg(unix)]

mod common;

use common::Fixture;
use serde_json::{Value, json};
use std::time::{SystemTime, UNIX_EPOCH};

const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const DAY: u64 = 86_400;

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// A routine automated run of `case` in `environment`, observed `age_days` ago.
fn obs(fixture: &Fixture, id: &str, age_days: u64, case: &str, outcome: &str) {
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
        "observed_at_unix_seconds": now() - age_days * DAY,
        "snapshot": {"revision": "0".repeat(40), "dirty": false, "fingerprint": ZERO},
        "provenance": "fixture",
        "cases": [{"id": case, "digest": ZERO}],
        "method": "automated",
    });
    fixture.write(
        &format!(".chough/evidence/{id}.json"),
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
}

/// Archived changes a1 to a5 finalized 60, 50, 40, 30 and 20 days ago (a1 and a2 cold,
/// a3 to a5 hot). a1's manifest lists r1 to r4; no manifest lists u1 to u3. A stub
/// `specsync` reports every change archived and authenticated, sleeping `delay` seconds per
/// status read. The checkpoint is at accept on a5, so `stage advance` moves into done.
fn project(delay: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(
        "bin/specsync",
        &format!(
            "#!/bin/sh\nmkdir -p .stub-running && mkdir .stub-running/$$ && ls .stub-running | wc -l >> .stub-concurrency\nsleep {delay}\nrmdir .stub-running/$$\nprintf '{{\"summary\":{{\"id\":\"%s\",\"state\":\"archived\",\"approval_valid\":true,\"correction_valid\":true,\"terminal_evidence\":{{\"validity\":\"authenticated-history\"}}}},\"acceptance_entries\":[]}}\\n' \"$3\"\n"
        ),
    );
    fixture.executable("bin/specsync");
    fixture.hi_stand_in();
    for (index, (id, age)) in [("a1", 60), ("a2", 50), ("a3", 40), ("a4", 30), ("a5", 20)]
        .iter()
        .enumerate()
    {
        let dir = format!(".specsync/archive/changes/2026-0{}-01-{id}", index + 1);
        fixture.write(
            &format!("{dir}/finalization.json"),
            &format!(
                "{{\"schema_version\": 2, \"change_id\": \"{id}\", \"timestamp\": {}, \"implementation_commit\": \"{}\"}}\n",
                now() - age * DAY,
                "1".repeat(40)
            ),
        );
        let mut entries =
            vec![json!({"path": "specs/cli/cli.spec.md", "kind": "file", "payload_digest": ZERO})];
        let listed: &[&str] = match *id {
            "a1" => &["r1", "r2", "r3", "r4"],
            "a2" => &["s1", "s2"],
            _ => &[],
        };
        {
            for run in listed {
                entries.push(json!({"path": format!(".chough/evidence/{run}.json"), "kind": "file", "payload_digest": ZERO}));
            }
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
                "---\nid: {id}\nstate: archived\nbase_commit: {}\n---\n\n# Title of {id}\n\n## Acceptance Criteria\n\n- {id} works\n",
                "3".repeat(40)
            ),
        );
    }
    fixture.init();
    // Runs of TC-a in local listed by cold a1: r4 is the newest, r3 the newest failure.
    obs(&fixture, "r1", 70, "TC-a", "passed");
    obs(&fixture, "r2", 65, "TC-a", "passed");
    obs(&fixture, "r3", 63, "TC-a", "failed");
    obs(&fixture, "r4", 61, "TC-a", "passed");
    // Runs of TC-s listed by cold a2: s2 is the newest, so s1 is superseded.
    obs(&fixture, "s1", 53, "TC-s", "passed");
    obs(&fixture, "s2", 52, "TC-s", "passed");
    // Unlisted proof: u1 in cold a2's cycle, superseded by u1b in hot a3's cycle; u2 in
    // hot a4's cycle; u3 after a5 (current work).
    obs(&fixture, "u1", 55, "TC-u1", "passed");
    obs(&fixture, "u1b", 45, "TC-u1", "passed");
    obs(&fixture, "u2", 35, "TC-u2", "passed");
    obs(&fixture, "u3", 10, "TC-u3", "passed");
    fixture.commit("fixture");
    set_checkpoint(&fixture, &[]);
    fixture
}

fn set_checkpoint(fixture: &Fixture, observations: &[&str]) {
    let path = fixture.root().join(".chough/session.json");
    let mut session: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    session["stage"]["current"] = json!("accept");
    session["native_change"] = json!("a5");
    session["observations"] = json!(observations);
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&session).unwrap() + "\n",
    )
    .unwrap();
}

fn pruned(trim: &Value) -> Vec<String> {
    trim["summary"]["pruned"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .iter()
        .filter_map(|id| id.as_str().map(str::to_string))
        .collect()
}

#[test]
fn superseded_runs_of_a_cold_change_are_trimmed_keeping_the_newest_and_the_newest_failure() {
    let fixture = project("0");
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{moved:#}");
    let gone = pruned(trim);
    for id in ["r1", "r2"] {
        assert!(gone.contains(&id.to_string()), "{id} not trimmed: {trim:#}");
        assert!(!fixture.exists(&format!(".chough/evidence/{id}.json")));
    }
    for id in ["r3", "r4"] {
        assert!(
            fixture.exists(&format!(".chough/evidence/{id}.json")),
            "{id} must stay (newest failure, newest run): {trim:#}"
        );
    }
}

#[test]
fn unlisted_proof_is_attributed_to_its_change_by_cycle_time() {
    let fixture = project("0");
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert!(pruned(trim).contains(&"u1".to_string()), "{trim:#}");
    assert!(!fixture.exists(".chough/evidence/u1.json"));
    assert!(
        fixture.exists(".chough/evidence/u1b.json"),
        "the newest run stays"
    );
    assert!(
        fixture.exists(".chough/evidence/u2.json"),
        "hot a4's cycle stays"
    );
    assert!(
        fixture.exists(".chough/evidence/u3.json"),
        "current work stays"
    );
    let ledger = fixture.read(".chough/pruned.jsonl");
    let entry: Value = ledger
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .find(|entry| entry["id"] == "u1")
        .expect("u1 ledgered");
    assert_eq!(entry["archives"], json!(["a2"]), "{entry:#}");
    assert_eq!(entry["knowledge"], "knowledge/archives/a2.md", "{entry:#}");
    let note = fixture.read("knowledge/archives/a2.md");
    assert!(note.contains("u1"), "a2's note must summarize u1:\n{note}");
}

#[test]
fn done_drops_checkpoint_references_from_closed_cycles() {
    let fixture = project("0");
    // The current cycle began 15 days ago; r2 is from an earlier cycle, u3 from this one.
    let reset = json!({
        "schema_version": 1, "at_unix_seconds": now() - 15 * DAY, "kind": "reset",
        "from": "done", "to": "plan", "reason": "fixture cycle",
    });
    let path = fixture.root().join(".chough/transitions.jsonl");
    let mut log = std::fs::read_to_string(&path).unwrap_or_default();
    log.push_str(&(reset.to_string() + "\n"));
    std::fs::write(&path, log).unwrap();
    set_checkpoint(&fixture, &["r2", "u3"]);
    let moved = fixture.ok(&["stage", "advance"]);
    let dropped = &moved["applied"]["checkpoint_references"];
    assert_eq!(dropped["status"], "dropped", "{moved:#}");
    assert_eq!(dropped["dropped"], json!(["r2"]), "{moved:#}");
    assert_eq!(fixture.session()["observations"], json!(["u3"]));
    // With its reference gone, r2 is no longer protected by the checkpoint.
    assert!(pruned(&moved["applied"]["librarian_trim"]).contains(&"r2".to_string()));
    assert!(fixture.exists(".chough/evidence/u3.json"));
}

#[test]
fn native_validity_reads_run_in_parallel() {
    // Each status read takes 1 s and the stub records how many run at once. The trim
    // reads both cold archives' validity before and after removal; read one at a time,
    // no two reads would ever overlap.
    let fixture = project("1");
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(
        moved["applied"]["librarian_trim"]["status"], "trimmed",
        "{moved:#}"
    );
    let most = fixture
        .read(".stub-concurrency")
        .lines()
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    assert!(
        most >= 2,
        "validity reads ran one at a time (at most {most} at once)"
    );
}

/// The trim on Chough's own repository (#108 closeout): two eligible sets shared one log,
/// so removing the first left the log to the second, and the per-file re-check refused
/// the whole trim. A set that gains a file is still safe to trim: only planned files go.
#[test]
fn a_shared_artifact_released_by_an_earlier_removal_does_not_stop_the_trim() {
    let fixture = project("0");
    for (id, own) in [("r1", "r1.log"), ("r2", "r2.log")] {
        let path = format!(".chough/evidence/{id}.json");
        let mut record: Value = serde_json::from_str(&fixture.read(&path)).unwrap();
        let mut fingerprints = serde_json::Map::new();
        for log in ["shared.log", own] {
            let artifact = format!(".chough/artifacts/{log}");
            fixture.write(&artifact, &format!("{log}\n"));
            fingerprints.insert(artifact, json!(ZERO));
        }
        record["artifacts"] = json!(fingerprints.keys().collect::<Vec<_>>());
        record["artifact_fingerprints"] = Value::Object(fingerprints);
        fixture.write(
            &path,
            &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
        );
    }
    fixture.commit("shared log");
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{trim:#}");
    for id in ["r1", "r2"] {
        assert!(pruned(trim).contains(&id.to_string()), "{id}: {trim:#}");
        assert!(!fixture.exists(&format!(".chough/artifacts/{id}.log")));
    }
    assert!(
        fixture.exists(".chough/artifacts/shared.log"),
        "a shared log is never planned"
    );
}

<<<<<<< HEAD
/// Adds `paths` to cold a1's verification manifest.
fn list_in_a1(fixture: &Fixture, paths: &[&str]) {
    let path = ".specsync/archive/changes/2026-01-01-a1/verification.json";
    let mut verification: Value = serde_json::from_str(&fixture.read(path)).unwrap();
    let entries = verification["acceptance_manifest"]["entries"]
        .as_array_mut()
        .unwrap();
    for listed in paths {
        entries.push(json!({"path": listed, "kind": "file", "payload_digest": ZERO}));
    }
    fixture.write(path, &serde_json::to_string_pretty(&verification).unwrap());
}

/// Commits everything with author and committer dates `age_days` ago.
fn commit_aged(fixture: &Fixture, message: &str, age_days: u64) {
    let date = format!("@{} +0000", now() - age_days * DAY);
    for args in [
        vec!["add", "--all"],
        vec!["commit", "--quiet", "-m", message],
    ] {
        let status = std::process::Command::new("git")
            .args(&args)
            .current_dir(fixture.root())
            .env("GIT_AUTHOR_DATE", &date)
            .env("GIT_COMMITTER_DATE", &date)
            .status()
            .unwrap();
        assert!(status.success());
    }
}

fn bug(fixture: &Fixture, id: &str, status: &str, fixture_path: &str) {
    let record = json!({
        "schema_version": 1, "id": id, "title": "Fixture bug", "symptom": "s", "trigger": "t",
        "expected": "e", "actual": "a", "criteria": ["LIFE-1"], "requirements": ["REQ-cli-001"],
        "cases": [], "environment": "local", "revision": null, "steps": [], "fixture": fixture_path,
        "observations": [], "impact": "low", "classification": "product-defect", "scope": "in-scope",
        "status": status, "regression_baseline": null, "duplicate_of": null, "related": [],
        "source": "fixture", "notes": [], "created_at_unix_seconds": now() - 70 * DAY,
        "updated_at_unix_seconds": now() - 70 * DAY,
    });
    fixture.write(
        &format!(".chough/bugs/{id}.json"),
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
}

/// KNOW-18: loose artifacts that no record references are summarized and trimmed once the
/// change that made them is cold: listed only by a cold archive, or committed in a cold
/// change's cycle. Loose artifacts of current work, or named by an open bug, stay.
#[test]
fn loose_artifacts_of_a_cold_change_are_trimmed_and_current_or_bug_held_ones_stay() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/listed-pr/out.log", "listed\n");
    fixture.write(".chough/artifacts/bug-repro.log", "repro\n");
    list_in_a1(
        &fixture,
        &[
            ".chough/artifacts/listed-pr/out.log",
            ".chough/artifacts/bug-repro.log",
        ],
    );
    bug(
        &fixture,
        "BUG-0001",
        "open",
        ".chough/artifacts/bug-repro.log",
    );
    fixture.commit("listed loose artifacts");
    fixture.write(".chough/artifacts/old-pr/notes.log", "old\n");
    commit_aged(&fixture, "an old pull request folder", 55);
    fixture.write(".chough/artifacts/new-pr/notes.log", "new\n");
    fixture.commit("current work");
    set_checkpoint(&fixture, &[]);
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{trim:#}");
    assert!(
        !fixture.exists(".chough/artifacts/listed-pr/out.log"),
        "{trim:#}"
    );
    assert!(
        !fixture.exists(".chough/artifacts/old-pr/notes.log"),
        "{trim:#}"
    );
    assert!(
        fixture.exists(".chough/artifacts/new-pr/notes.log"),
        "current work stays"
    );
    assert!(
        fixture.exists(".chough/artifacts/bug-repro.log"),
        "an open bug keeps its fixture"
    );
    let ledger: Vec<Value> = fixture
        .read(".chough/pruned.jsonl")
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let old = ledger
        .iter()
        .find(|entry| entry["id"] == "artifact:old-pr")
        .expect("old-pr ledgered");
    assert_eq!(old["kind"], "loose-artifact", "{old:#}");
    assert_eq!(old["archives"], json!(["a2"]), "{old:#}");
    assert!(
        ledger
            .iter()
            .any(|entry| entry["id"] == "artifact:listed-pr" && entry["archives"] == json!(["a1"]))
    );
    let note = fixture.read("knowledge/archives/a2.md");
    assert!(
        note.contains(".chough/artifacts/old-pr/notes.log"),
        "{note}"
    );
}

/// KNOW-18: a fixture named by a closed bug is no longer held.
#[test]
fn a_closed_bug_releases_its_loose_fixture() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/bug-repro.log", "repro\n");
    list_in_a1(&fixture, &[".chough/artifacts/bug-repro.log"]);
    bug(
        &fixture,
        "BUG-0001",
        "verified",
        ".chough/artifacts/bug-repro.log",
    );
    fixture.commit("closed bug fixture");
    set_checkpoint(&fixture, &[]);
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(
        moved["applied"]["librarian_trim"]["status"], "trimmed",
        "{moved:#}"
    );
    assert!(!fixture.exists(".chough/artifacts/bug-repro.log"));
}

/// KNOW-19: key items move into the vault: a manual observation with its screenshot, and
/// the failure log of an older failed run, under the cold change's attachments.
#[test]
fn key_items_of_a_cold_change_move_into_the_vault() {
    let fixture = project("0");
    // m1: a manual observation with a screenshot, listed by cold a1.
    obs(&fixture, "m1", 64, "TC-m", "passed");
    let path = ".chough/evidence/m1.json";
    let mut record: Value = serde_json::from_str(&fixture.read(path)).unwrap();
    record["method"] = json!("manual");
    record["artifacts"] = json!([".chough/artifacts/m1-screen.png"]);
    record["artifact_fingerprints"] = json!({".chough/artifacts/m1-screen.png": ZERO});
    fixture.write(
        path,
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
    fixture.write(".chough/artifacts/m1-screen.png", "png bytes\n");
    // r0: an older failed run of TC-a (r3 is the newest failure) with its log.
    obs(&fixture, "r0", 75, "TC-a", "failed");
    let path = ".chough/evidence/r0.json";
    let mut record: Value = serde_json::from_str(&fixture.read(path)).unwrap();
    record["artifacts"] = json!([".chough/artifacts/r0.log"]);
    record["artifact_fingerprints"] = json!({".chough/artifacts/r0.log": ZERO});
    fixture.write(
        path,
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
    fixture.write(".chough/artifacts/r0.log", "failure first\n");
    list_in_a1(
        &fixture,
        &[".chough/evidence/m1.json", ".chough/evidence/r0.json"],
    );
    fixture.commit("key items");
    set_checkpoint(&fixture, &[]);
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{trim:#}");
    for (gone, kept) in [
        (
            ".chough/evidence/m1.json",
            "knowledge/attachments/a1/evidence/m1.json",
        ),
        (
            ".chough/artifacts/m1-screen.png",
            "knowledge/attachments/a1/artifacts/m1-screen.png",
        ),
        (
            ".chough/artifacts/r0.log",
            "knowledge/attachments/a1/artifacts/r0.log",
        ),
    ] {
        assert!(!fixture.exists(gone), "{gone} left: {trim:#}");
        assert!(fixture.exists(kept), "{kept} missing: {trim:#}");
    }
    assert!(
        !fixture.exists(".chough/evidence/r0.json"),
        "a routine record is not a key item"
    );
    assert!(!fixture.exists("knowledge/attachments/a1/evidence/r0.json"));
    let note = fixture.read("knowledge/archives/a1.md");
    assert!(
        note.contains("[artifacts/m1-screen.png](../attachments/a1/artifacts/m1-screen.png)"),
        "{note}"
    );
    let ledger = fixture.read(".chough/pruned.jsonl");
    assert!(
        ledger.contains("knowledge/attachments/a1/evidence/m1.json"),
        "{ledger}"
    );
=======
/// Review of #108: a current-cycle reference whose record exists but cannot be read is
/// kept, and the step reports `refused` instead of dropping it; done still completes.
#[test]
fn an_unreadable_record_keeps_its_reference_and_refuses_the_drop() {
    let fixture = project("0");
    let reset = json!({
        "schema_version": 1, "at_unix_seconds": now() - 15 * DAY, "kind": "reset",
        "from": "done", "to": "plan", "reason": "fixture cycle",
    });
    let path = fixture.root().join(".chough/transitions.jsonl");
    let mut log = std::fs::read_to_string(&path).unwrap_or_default();
    log.push_str(&(reset.to_string() + "\n"));
    std::fs::write(&path, log).unwrap();
    fixture.write(".chough/evidence/u3.json", "{ not json\n");
    set_checkpoint(&fixture, &["r2", "u3"]);
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["stage"]["current"], "done", "{moved:#}");
    let references = &moved["applied"]["checkpoint_references"];
    assert_eq!(references["status"], "refused", "{moved:#}");
    assert!(
        references["reason"]
            .as_str()
            .unwrap_or_default()
            .contains("u3"),
        "{moved:#}"
    );
    assert_eq!(fixture.session()["observations"], json!(["r2", "u3"]));
>>>>>>> origin/claude/know/reduce-proof
}
