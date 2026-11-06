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

/// Twelve cold archives c01 to c12 (finalized 150 to 40 days ago) and three hot ones
/// (c13 to c15). Each cold archive lists two passing runs of its own case, the older of
/// which is superseded, so the trim at done reads twelve archives' validity before and
/// twelve after removal. The stub sleeps 1 s per status read and records how many run
/// at once. The checkpoint is at accept on c15.
fn many_cold_archives() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nmkdir -p .stub-running && mkdir .stub-running/$$ && ls .stub-running | wc -l >> .stub-concurrency\nsleep 1\nrmdir .stub-running/$$\nprintf '{\"summary\":{\"id\":\"%s\",\"state\":\"archived\",\"approval_valid\":true,\"correction_valid\":true,\"terminal_evidence\":{\"validity\":\"authenticated-history\"}},\"acceptance_entries\":[]}\\n' \"$3\"\n",
    );
    fixture.executable("bin/specsync");
    fixture.hi_stand_in();
    for n in 1..=15u64 {
        let id = format!("c{n:02}");
        let dir = format!(".specsync/archive/changes/2026-01-{n:02}-{id}");
        let age = 160 - n * 10;
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
        if n <= 12 {
            for run in [format!("old{n:02}"), format!("new{n:02}")] {
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
    for n in 1..=12u64 {
        let age = 160 - n * 10;
        obs(
            &fixture,
            &format!("old{n:02}"),
            age + 3,
            &format!("TC-{n:02}"),
            "passed",
        );
        obs(
            &fixture,
            &format!("new{n:02}"),
            age + 2,
            &format!("TC-{n:02}"),
            "passed",
        );
    }
    fixture.commit("fixture");
    set_checkpoint(&fixture, &[]);
    // set_checkpoint names a5; this fixture's newest change is c15.
    let path = fixture.root().join(".chough/session.json");
    let mut session: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    session["native_change"] = json!("c15");
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&session).unwrap() + "\n",
    )
    .unwrap();
    fixture
}

/// CH-M11 (audit): REQ-cli-033 reads archive validity at most 8 at once. With twelve
/// affected archives, more than 8 reads would overlap if the ceiling were raised, on any
/// host with more than eight cores; `validity_readers` in src/evidence.rs holds the
/// ceiling on every host.
#[test]
fn at_most_eight_validity_reads_run_at_once() {
    let fixture = many_cold_archives();
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{moved:#}");
    let pruned = pruned(trim);
    for n in 1..=12 {
        assert!(
            pruned.contains(&format!("old{n:02}")),
            "old{n:02}: {trim:#}"
        );
        assert!(
            !pruned.contains(&format!("new{n:02}")),
            "new{n:02}: {trim:#}"
        );
    }
    let most = fixture
        .read(".stub-concurrency")
        .lines()
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    assert!(
        (2..=8).contains(&most),
        "{most} validity reads ran at once; REQ-cli-033 allows at most 8, and at least 2 overlap"
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

/// Adds `paths` to cold a1's verification manifest.
fn list_in_a1(fixture: &Fixture, paths: &[&str]) {
    list_in(fixture, ".specsync/archive/changes/2026-01-01-a1", paths);
}

/// Adds `paths` to the verification manifest of the archive in `dir`.
fn list_in(fixture: &Fixture, dir: &str, paths: &[&str]) {
    let path = &format!("{dir}/verification.json");
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
            .args([
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.invalid",
                "-c",
                "commit.gpgsign=false",
            ])
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

/// KNOW-18, KNOW-22: a fixture named by a closed bug is no longer held in `.chough/`; it
/// is a key item, so it moves into the vault under the bug.
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
    assert_eq!(
        fixture.read("knowledge/attachments/BUG-0001/artifacts/bug-repro.log"),
        "repro\n"
    );
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
}

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
}

/// #109 closeout on this repository: removing the runs that shared a log inside a loose
/// folder left that log to the folder's loose group, which then listed a file before its
/// planned ones, and the re-check refused the whole trim. A loose group that gains a file
/// is still safe to trim: only its planned files go.
#[test]
fn a_loose_folder_that_gains_a_released_file_is_still_trimmed() {
    let fixture = project("0");
    for id in ["r1", "r2"] {
        let path = format!(".chough/evidence/{id}.json");
        let mut record: Value = serde_json::from_str(&fixture.read(&path)).unwrap();
        record["artifacts"] = json!([".chough/artifacts/x/a-shared.log"]);
        record["artifact_fingerprints"] = json!({".chough/artifacts/x/a-shared.log": ZERO});
        fixture.write(
            &path,
            &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
        );
    }
    fixture.write(".chough/artifacts/x/a-shared.log", "shared\n");
    fixture.write(".chough/artifacts/x/z-loose.log", "loose\n");
    list_in_a1(&fixture, &[".chough/artifacts/x/z-loose.log"]);
    fixture.commit("a loose folder with a shared log");
    set_checkpoint(&fixture, &[]);
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{trim:#}");
    assert!(pruned(trim).contains(&"artifact:x".to_string()), "{trim:#}");
    assert!(!fixture.exists(".chough/artifacts/x/z-loose.log"));
    assert!(
        fixture.exists(".chough/artifacts/x/a-shared.log"),
        "an unplanned file stays"
    );
}

/// The other side of the case above: a planned loose file that an open bug names partway
/// through the apply still refuses the trim, and everything planned is restored.
#[test]
fn a_loose_file_protected_during_the_apply_refuses_the_trim() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/x/a-first.log", "first\n");
    fixture.write(".chough/artifacts/x/z-second.log", "second\n");
    list_in_a1(
        &fixture,
        &[
            ".chough/artifacts/x/a-first.log",
            ".chough/artifacts/x/z-second.log",
        ],
    );
    fixture.commit("a loose folder");
    set_checkpoint(&fixture, &[]);
    // An open bug naming z-second.log, held back until the apply has planned.
    bug(
        &fixture,
        "BUG-0009",
        "open",
        ".chough/artifacts/x/z-second.log",
    );
    fixture.write(".stub-bug", &fixture.read(".chough/bugs/BUG-0009.json"));
    std::fs::remove_file(fixture.root().join(".chough/bugs/BUG-0009.json")).unwrap();
    // A Git stand-in that, on the first unfiltered stdin hash (the HEAD check of the bytes
    // held for removal, after planning), files that bug: another agent's report.
    let real_git = String::from_utf8(
        std::process::Command::new("sh")
            .args(["-c", "command -v git"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    fixture.write(
        "bin/git",
        &format!(
            "#!/bin/sh\nif [ \"$1 $2 $3\" = \"hash-object --no-filters --stdin\" ] && [ -f .stub-bug ]; then\n  cp .stub-bug .chough/bugs/BUG-0009.json && rm .stub-bug\nfi\nexec '{}' \"$@\"\n",
            real_git.trim()
        ),
    );
    fixture.executable("bin/git");
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert!(
        !fixture.exists(".stub-bug"),
        "the hook did not run: {trim:#}"
    );
    assert_eq!(trim["status"], "refused", "{trim:#}");
    let reason = trim["reason"].as_str().unwrap_or_default();
    assert!(
        reason.contains("protections changed during removal") && reason.contains("artifact:x"),
        "{trim:#}"
    );
    assert_eq!(fixture.read(".chough/artifacts/x/a-first.log"), "first\n");
    assert_eq!(fixture.read(".chough/artifacts/x/z-second.log"), "second\n");
    assert!(!fixture.exists(".chough/removing"));
}

/// Renames cold a1 to `id`, keeping its date, manifest and finalization; returns its
/// directory.
fn rename_a1(fixture: &Fixture, id: &str) -> String {
    let dir = format!(".specsync/archive/changes/2026-01-01-{id}");
    std::fs::rename(
        fixture
            .root()
            .join(".specsync/archive/changes/2026-01-01-a1"),
        fixture.root().join(&dir),
    )
    .unwrap();
    for name in ["finalization.json", "change.md"] {
        let path = format!("{dir}/{name}");
        let text = fixture
            .read(&path)
            .replace("\"change_id\": \"a1\"", &format!("\"change_id\": \"{id}\""))
            .replace("id: a1\n", &format!("id: {id}\n"));
        fixture.write(&path, &text);
    }
    dir
}

fn mode(fixture: &Fixture, path: &str) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(fixture.root().join(path))
        .unwrap()
        .permissions()
        .mode()
        & 0o777
}

/// #109 on Windows: attachments of a long archive ID go under a short, stable folder, so
/// the vault can be checked out there.
#[test]
fn key_items_of_a_long_archive_id_move_under_a_short_folder() {
    let fixture = project("0");
    let id = "a1-a-long-archived-change-id-that-windows-paths-cannot-repeat";
    let dir = rename_a1(&fixture, id);
    fixture.write(".chough/artifacts/x/screen.png", "png\n");
    list_in(&fixture, &dir, &[".chough/artifacts/x/screen.png"]);
    fixture.commit("a long archive id");
    set_checkpoint(&fixture, &[]);
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{trim:#}");
    let folder = std::fs::read_dir(fixture.root().join("knowledge/attachments"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .find(|name| name.starts_with("a1-a-long-"))
        .unwrap_or_else(|| panic!("no short folder: {trim:#}"));
    let digest = folder.strip_prefix("a1-a-long-").unwrap();
    assert!(
        digest.len() == 8 && digest.chars().all(|c| c.is_ascii_hexdigit()),
        "{folder}"
    );
    let attachment = format!("knowledge/attachments/{folder}/artifacts/x/screen.png");
    assert_eq!(fixture.read(&attachment), "png\n");
    assert!(!fixture.exists(".chough/artifacts/x/screen.png"));
    let note = fixture.read(&format!("knowledge/archives/{id}.md"));
    assert!(
        note.contains(&format!("../attachments/{folder}/artifacts/x/screen.png")),
        "{note}"
    );
}

/// A key item whose attachment path would pass 220 characters stays, protected with that
/// reason, rather than writing a path a Windows clone cannot hold.
#[test]
fn a_key_item_whose_attachment_path_is_too_long_stays() {
    let fixture = project("0");
    let file = format!(".chough/artifacts/x/{}-failure-first.log", "n".repeat(190));
    fixture.write(&file, "fail\n");
    list_in_a1(&fixture, &[file.as_str()]);
    fixture.commit("a long key item");
    set_checkpoint(&fixture, &[]);
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{trim:#}");
    assert!(fixture.exists(&file), "{trim:#}");
    let protected = trim["summary"]["protected"].as_array().unwrap();
    let entry = protected
        .iter()
        .find(|entry| entry["id"] == "artifact:x")
        .unwrap_or_else(|| panic!("artifact:x not protected: {trim:#}"));
    assert!(
        entry["reasons"]
            .to_string()
            .contains("longer than 220 characters"),
        "{entry:#}"
    );
}

/// corvid-agent on #109: a key item keeps its mode in the vault.
#[test]
fn a_moved_key_item_keeps_its_mode() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/x/failure-first.sh", "#!/bin/sh\n");
    fixture.executable(".chough/artifacts/x/failure-first.sh");
    list_in_a1(&fixture, &[".chough/artifacts/x/failure-first.sh"]);
    fixture.commit("an executable key item");
    set_checkpoint(&fixture, &[]);
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{trim:#}");
    let attachment = "knowledge/attachments/a1/artifacts/x/failure-first.sh";
    assert_eq!(fixture.read(attachment), "#!/bin/sh\n");
    assert_eq!(mode(&fixture, attachment), 0o755);
}

/// corvid-agent on #109: the refused first closeout trim restored two scripts as 0644.
/// A restore puts back the mode with the bytes.
#[test]
fn a_refused_apply_restores_files_with_their_modes() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/x/run.sh", "#!/bin/sh\n");
    fixture.executable(".chough/artifacts/x/run.sh");
    list_in_a1(&fixture, &[".chough/artifacts/x/run.sh"]);
    fixture.commit("an executable loose script");
    set_checkpoint(&fixture, &[]);
    // A SpecSync stand-in whose terminal evidence validity changes once the script is
    // gone, so the apply is refused after removing it.
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nv=authenticated-history\n[ -e .chough/artifacts/x/run.sh ] || v=unauthenticated\nprintf '{\"summary\":{\"id\":\"%s\",\"state\":\"archived\",\"approval_valid\":true,\"correction_valid\":true,\"terminal_evidence\":{\"validity\":\"%s\"}},\"acceptance_entries\":[]}\\n' \"$3\" \"$v\"\n",
    );
    fixture.executable("bin/specsync");
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "refused", "{trim:#}");
    assert!(
        trim["reason"]
            .as_str()
            .unwrap_or_default()
            .contains("validity"),
        "{trim:#}"
    );
    assert_eq!(fixture.read(".chough/artifacts/x/run.sh"), "#!/bin/sh\n");
    assert_eq!(mode(&fixture, ".chough/artifacts/x/run.sh"), 0o755);
}

/// A Git stand-in that runs `action` once, on the first unfiltered stdin hash: the HEAD
/// check of the bytes held for removal, after planning and before any removal.
fn after_planning(fixture: &Fixture, action: &str) {
    let real_git = String::from_utf8(
        std::process::Command::new("sh")
            .args(["-c", "command -v git"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    fixture.write(".stub-pending", "");
    fixture.write(
        "bin/git",
        &format!(
            "#!/bin/sh\nif [ \"$1 $2 $3\" = \"hash-object --no-filters --stdin\" ] && [ -f .stub-pending ]; then\n  rm .stub-pending\n  {action}\nfi\nexec '{}' \"$@\"\n",
            real_git.trim()
        ),
    );
    fixture.executable("bin/git");
}

/// A SpecSync stand-in whose terminal evidence validity changes once `gone` is removed,
/// so the apply is refused after removing it.
fn refuse_once_removed(fixture: &Fixture, gone: &str) {
    fixture.write(
        "bin/specsync",
        &format!(
            "#!/bin/sh\nv=authenticated-history\n[ -e {gone} ] || v=unauthenticated\nprintf '{{\"summary\":{{\"id\":\"%s\",\"state\":\"archived\",\"approval_valid\":true,\"correction_valid\":true,\"terminal_evidence\":{{\"validity\":\"%s\"}}}},\"acceptance_entries\":[]}}\\n' \"$3\" \"$v\"\n"
        ),
    );
    fixture.executable("bin/specsync");
}

fn protection<'a>(trim: &'a Value, id: &str) -> &'a Value {
    trim["summary"]["protected"]
        .as_array()
        .and_then(|entries| entries.iter().find(|entry| entry["id"] == id))
        .unwrap_or_else(|| panic!("{id} is not protected: {trim:#}"))
}

/// #109 review P1: a move never overwrites an attachment that holds other bytes.
#[test]
fn an_attachment_holding_other_bytes_keeps_both_files() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/x/screen.png", "png\n");
    fixture.write(
        "knowledge/attachments/a1/artifacts/x/screen.png",
        "another agent's proof\n",
    );
    list_in_a1(&fixture, &[".chough/artifacts/x/screen.png"]);
    fixture.commit("an occupied attachment");
    set_checkpoint(&fixture, &[]);
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{trim:#}");
    assert_eq!(fixture.read(".chough/artifacts/x/screen.png"), "png\n");
    assert_eq!(
        fixture.read("knowledge/attachments/a1/artifacts/x/screen.png"),
        "another agent's proof\n"
    );
    assert!(
        protection(trim, "artifact:x")["reasons"]
            .to_string()
            .contains("different bytes"),
        "{trim:#}"
    );
}

/// #109 review P1: an attachment another writer creates during the apply refuses it, and
/// both files are kept.
#[test]
fn an_attachment_written_during_the_apply_refuses_it() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/x/screen.png", "png\n");
    list_in_a1(&fixture, &[".chough/artifacts/x/screen.png"]);
    fixture.commit("a key item");
    set_checkpoint(&fixture, &[]);
    after_planning(
        &fixture,
        "mkdir -p knowledge/attachments/a1/artifacts/x && printf 'written meanwhile\\n' > knowledge/attachments/a1/artifacts/x/screen.png",
    );
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert!(
        !fixture.exists(".stub-pending"),
        "the hook did not run: {trim:#}"
    );
    assert_eq!(trim["status"], "refused", "{trim:#}");
    assert_eq!(fixture.read(".chough/artifacts/x/screen.png"), "png\n");
    assert_eq!(
        fixture.read("knowledge/attachments/a1/artifacts/x/screen.png"),
        "written meanwhile\n"
    );
}

/// #109 review P1: an identical attachment already in place is not this apply's copy, so
/// a refused apply leaves it.
#[test]
fn an_identical_attachment_in_place_survives_a_refused_apply() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/x/screen.png", "png\n");
    fixture.write("knowledge/attachments/a1/artifacts/x/screen.png", "png\n");
    list_in_a1(&fixture, &[".chough/artifacts/x/screen.png"]);
    fixture.commit("an attachment already in place");
    set_checkpoint(&fixture, &[]);
    refuse_once_removed(&fixture, ".chough/artifacts/x/screen.png");
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "refused", "{trim:#}");
    assert_eq!(fixture.read(".chough/artifacts/x/screen.png"), "png\n");
    assert_eq!(
        fixture.read("knowledge/attachments/a1/artifacts/x/screen.png"),
        "png\n"
    );
}

/// #109 review P1: an open bug whose fixture is a folder keeps the files inside it.
#[test]
fn an_open_bug_folder_fixture_protects_its_files() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/open-repro/out.log", "repro\n");
    list_in_a1(&fixture, &[".chough/artifacts/open-repro/out.log"]);
    bug(&fixture, "BUG-0001", "open", ".chough/artifacts/open-repro");
    fixture.commit("an open bug's folder fixture");
    set_checkpoint(&fixture, &[]);
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{trim:#}");
    assert!(
        fixture.exists(".chough/artifacts/open-repro/out.log"),
        "{trim:#}"
    );
    assert!(
        protection(trim, "artifact:open-repro")["reasons"]
            .to_string()
            .contains("BUG-0001"),
        "{trim:#}"
    );
}

/// #109 review P1: a checkpoint reference to a folder keeps the loose files and the
/// observation sets inside it, and a sibling folder sharing its prefix is not covered.
/// Since KNOW-21 the move into done drops the checkpoint's file references first, so
/// the reference is exercised by a trim before done (`library archive --apply`).
#[test]
fn a_checkpoint_folder_reference_protects_loose_files_and_observation_sets() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/kept/loose.log", "loose\n");
    fixture.write(".chough/artifacts/kept-not/loose.log", "other\n");
    // Superseded run r1, listed by cold a1, keeps its log in the folder.
    let path = ".chough/evidence/r1.json";
    let mut record: Value = serde_json::from_str(&fixture.read(path)).unwrap();
    record["artifacts"] = json!([".chough/artifacts/kept/r1.log"]);
    record["artifact_fingerprints"] = json!({".chough/artifacts/kept/r1.log": ZERO});
    fixture.write(
        path,
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
    fixture.write(".chough/artifacts/kept/r1.log", "r1\n");
    list_in_a1(
        &fixture,
        &[
            ".chough/artifacts/kept/loose.log",
            ".chough/artifacts/kept-not/loose.log",
        ],
    );
    fixture.commit("a checkpoint-named folder");
    fixture.ok(&["checkpoint", "--file", ".chough/artifacts/kept"]);
    set_checkpoint(&fixture, &[]);
    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: summarize old proof",
    ]);
    let trim = &fixture.ok(&["library", "archive", "--apply"]);
    assert!(
        fixture.exists(".chough/artifacts/kept/loose.log"),
        "{trim:#}"
    );
    assert!(fixture.exists(".chough/evidence/r1.json"), "{trim:#}");
    assert!(fixture.exists(".chough/artifacts/kept/r1.log"), "{trim:#}");
    assert!(!pruned(trim).contains(&"r1".to_string()), "{trim:#}");
    assert!(
        !fixture.exists(".chough/artifacts/kept-not/loose.log"),
        "a sibling sharing the prefix is not inside the folder: {trim:#}"
    );
}

/// #109 review P1: one cold-listed file does not make a folder's current files cold.
#[test]
fn a_loose_folder_with_a_current_file_stays_whole() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/reused/a-old.log", "old\n");
    list_in_a1(&fixture, &[".chough/artifacts/reused/a-old.log"]);
    fixture.commit("an old listed log");
    fixture.write(".chough/artifacts/reused/current.log", "current\n");
    fixture.commit("current work in the same folder");
    set_checkpoint(&fixture, &[]);
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{trim:#}");
    assert!(
        fixture.exists(".chough/artifacts/reused/current.log"),
        "{trim:#}"
    );
    assert!(
        fixture.exists(".chough/artifacts/reused/a-old.log"),
        "{trim:#}"
    );
    assert!(
        protection(trim, "artifact:reused")["reasons"]
            .to_string()
            .contains("current.log"),
        "{trim:#}"
    );
}

/// Sets the legacy checkpoint's file and design references.
fn set_references(fixture: &Fixture, files: &[&str], design: &[&str]) {
    let path = fixture.root().join(".chough/session.json");
    let mut session: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    session["references"]["files"] = json!(files);
    session["references"]["design"] = json!(design);
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&session).unwrap() + "\n",
    )
    .unwrap();
}

/// KNOW-21: when a change reaches done its cycle closes, so done drops every file and
/// design reference from the checkpoint. A cold loose folder a reference held is then
/// trimmed. No reset is recorded here, so the observation part has no cycle start, but
/// the file and design part does not need one.
#[test]
fn done_drops_every_file_and_design_reference() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/old-pr/notes.log", "old\n");
    commit_aged(&fixture, "an old pull request folder", 55);
    fixture.write("src/main.rs", "fn main() {}\n");
    fixture.write("docs/design.md", "# Design\n");
    fixture.commit("sources");
    set_checkpoint(&fixture, &["u3"]);
    set_references(
        &fixture,
        &[".chough/artifacts/old-pr/notes.log", "src/main.rs"],
        &["docs/design.md"],
    );
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["stage"]["current"], "done", "{moved:#}");
    let references = &moved["applied"]["checkpoint_references"];
    assert_eq!(references["status"], "dropped", "{moved:#}");
    assert_eq!(
        references["dropped_files"],
        json!([".chough/artifacts/old-pr/notes.log", "src/main.rs"]),
        "{moved:#}"
    );
    assert_eq!(
        references["dropped_design"],
        json!(["docs/design.md"]),
        "{moved:#}"
    );
    let session = fixture.session();
    assert_eq!(session["references"]["files"], json!([]), "{session:#}");
    assert_eq!(session["references"]["design"], json!([]), "{session:#}");
    assert_eq!(
        session["observations"],
        json!(["u3"]),
        "observations keep their own rule"
    );
    assert!(
        !fixture.exists(".chough/artifacts/old-pr/notes.log"),
        "{:#}",
        moved["applied"]["librarian_trim"]
    );
}

/// KNOW-21: an archived session is done, so retention ignores its file and design
/// references; a cold loose folder only it names is trimmed.
#[test]
fn an_archived_session_file_reference_protects_nothing() {
    let fixture = project("0");
    fixture.write(".chough/artifacts/old-pr/notes.log", "old\n");
    commit_aged(&fixture, "an old pull request folder", 55);
    fixture.ok(&["history", "adopt", "--apply"]);
    fixture.ok(&["session", "adopt", "legacy"]);
    let mut archived: Value =
        serde_json::from_str(&fixture.read(".chough/sessions/legacy.json")).unwrap();
    archived["stage"]["current"] = json!("done");
    archived["native_change"] = json!("a2");
    archived["observations"] = json!([]);
    archived["references"]["files"] = json!([".chough/artifacts/old-pr/notes.log"]);
    archived["references"]["design"] = json!([".chough/artifacts/old-pr"]);
    fixture.write(
        ".chough/sessions/archive/old.json",
        &(serde_json::to_string_pretty(&archived).unwrap() + "\n"),
    );
    fixture.commit("an archived session naming the old folder");
    let moved = fixture.ok(&["stage", "advance"]);
    let trim = &moved["applied"]["librarian_trim"];
    assert_eq!(trim["status"], "trimmed", "{moved:#}");
    assert!(
        !fixture.exists(".chough/artifacts/old-pr/notes.log"),
        "{trim:#}"
    );
    assert_eq!(
        json!(
            serde_json::from_str::<Value>(&fixture.read(".chough/sessions/archive/old.json"))
                .unwrap()["references"]["files"]
        ),
        json!([".chough/artifacts/old-pr/notes.log"]),
        "the archived session itself is not rewritten"
    );
}

/// Runs of TC-b in a2's cold cycle: f1 failed, p1 passed after it, x1 another pass; the
/// unlinked f2 (a later failure) and n1 (the newest run) keep KNOW-17's protections off
/// the linked runs. f1 keeps its failure log. The bug links x1 first, so "earliest" is
/// read from observation times, not link order.
fn bug_with_runs(fixture: &Fixture, id: &str, status: &str) {
    obs(fixture, "f1", 58, "TC-b", "failed");
    obs(fixture, "p1", 57, "TC-b", "passed");
    obs(fixture, "x1", 56, "TC-b", "passed");
    obs(fixture, "f2", 55, "TC-b", "failed");
    obs(fixture, "n1", 54, "TC-b", "passed");
    let path = ".chough/evidence/f1.json";
    let mut record: Value = serde_json::from_str(&fixture.read(path)).unwrap();
    record["artifacts"] = json!([".chough/artifacts/f1.log"]);
    record["artifact_fingerprints"] = json!({".chough/artifacts/f1.log": ZERO});
    fixture.write(
        path,
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
    fixture.write(".chough/artifacts/f1.log", "failure first\n");
    bug(fixture, id, status, "a description, not a path");
    let path = format!(".chough/bugs/{id}.json");
    let mut record: Value = serde_json::from_str(&fixture.read(&path)).unwrap();
    record["observations"] = json!(["x1", "f1", "p1"]);
    fixture.write(
        &path,
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
    fixture.commit("a bug with linked runs");
    set_checkpoint(fixture, &[]);
}

/// KNOW-22: a closed bug stops holding its observations. Its key items, the earliest
/// linked failure with its artifacts and, for a verified bug, the first linked pass after
/// it, move into the vault under the bug; the rest is summarized and trimmed by the
/// normal cold rule. The bug record keeps its status and IDs.
#[test]
fn a_closed_bug_moves_its_key_items_into_the_vault_and_releases_the_rest() {
    for (status, fixing_run) in [
        ("verified", true),
        ("duplicate", false),
        ("wont-fix", false),
    ] {
        let fixture = project("0");
        bug_with_runs(&fixture, "BUG-0002", status);
        let bug_before = fixture.read(".chough/bugs/BUG-0002.json");
        let moved = fixture.ok(&["stage", "advance"]);
        let trim = &moved["applied"]["librarian_trim"];
        assert_eq!(trim["status"], "trimmed", "{status}: {trim:#}");
        let vault = "knowledge/attachments/BUG-0002";
        assert!(
            fixture.exists(&format!("{vault}/evidence/f1.json")),
            "{status}: {trim:#}"
        );
        assert!(
            fixture.exists(&format!("{vault}/artifacts/f1.log")),
            "{status}: {trim:#}"
        );
        assert!(
            !fixture.exists(".chough/evidence/f1.json"),
            "{status}: {trim:#}"
        );
        assert_eq!(
            fixture.exists(&format!("{vault}/evidence/p1.json")),
            fixing_run,
            "{status}: the fixing run moves only for a verified bug: {trim:#}"
        );
        assert!(
            !fixture.exists(".chough/evidence/p1.json"),
            "{status}: {trim:#}"
        );
        assert!(
            !fixture.exists(".chough/evidence/x1.json"),
            "{status}: {trim:#}"
        );
        assert!(
            !fixture.exists(&format!("{vault}/evidence/x1.json")),
            "{status}: {trim:#}"
        );
        for kept in ["f2", "n1"] {
            assert!(
                fixture.exists(&format!(".chough/evidence/{kept}.json")),
                "{status}: {trim:#}"
            );
        }
        assert_eq!(
            fixture.read(".chough/bugs/BUG-0002.json"),
            bug_before,
            "{status}"
        );
        let note = fixture.read("knowledge/bugs/BUG-0002.md");
        assert!(note.contains("id: \"BUG-0002\""), "{status}: {note}");
        assert!(
            note.contains("../attachments/BUG-0002/evidence/f1.json"),
            "{status}: {note}"
        );
        assert_eq!(
            note.contains("../attachments/BUG-0002/evidence/p1.json"),
            fixing_run,
            "{status}: {note}"
        );
        let ledger = fixture.read(".chough/pruned.jsonl");
        assert!(
            ledger.contains("knowledge/attachments/BUG-0002/evidence/f1.json"),
            "{ledger}"
        );
        assert!(ledger.contains("\"x1\""), "x1 is ledgered: {ledger}");
    }
}

/// KNOW-22: an open bug keeps every observation it links.
#[test]
fn an_open_bug_keeps_every_linked_observation() {
    let fixture = project("0");
    bug_with_runs(&fixture, "BUG-0002", "open");
    let moved = fixture.ok(&["stage", "advance"]);
    for kept in ["f1", "p1", "x1"] {
        assert!(
            fixture.exists(&format!(".chough/evidence/{kept}.json")),
            "{kept}: {:#}",
            moved["applied"]["librarian_trim"]
        );
    }
    assert!(!fixture.exists("knowledge/attachments/BUG-0002"));
}

/// A ledger line for a removed routine run of `archive`, pruned `age_minutes` ago.
fn ledger_line(id: &str, archive: &str, age_minutes: u64) -> String {
    json!({
        "schema_version": 1, "id": id, "kind": "product-test", "archives": [archive],
        "outcome": "passed", "environment": "local", "cases": ["TC-old"],
        "observed_at_unix_seconds": now() - 90 * DAY,
        "pruned_at_unix_seconds": now() - 30 * DAY - age_minutes * 60,
        "policy": "chough-retention/1", "reason": "fixture", "removed": [],
        "artifacts": {}, "record_digest": ZERO, "revision": "0".repeat(40), "head": "0".repeat(40),
        "knowledge": format!("knowledge/archives/{archive}.md"),
        // As a cycle-time trim records it, so the archive's note lists it (KNOW-20).
        "attributed": "cycle time",
    })
    .to_string()
}

/// 105 old entries of cold a1, oldest first, with a restore of the oldest right after it
/// and an old entry of an archive without a note; then the done move.
fn rotated(fixture: &Fixture) -> Value {
    let mut lines = vec![];
    for index in 0..105u64 {
        let id = format!("old-{index:03}");
        lines.push(ledger_line(&id, "a1", 1_000 - index));
        if index == 0 {
            lines.push(
                json!({"schema_version": 1, "event": "restored", "id": "old-000",
                       "reason": "fixture restore", "restored_at_unix_seconds": now() - 29 * DAY})
                .to_string(),
            );
        }
    }
    lines.insert(0, ledger_line("no-note", "zz-no-note", 2_000));
    fixture.write(".chough/pruned.jsonl", &(lines.join("\n") + "\n"));
    fixture.commit("an old ledger");
    set_checkpoint(fixture, &[]);
    fixture.ok(&["stage", "advance"])
}

fn ledger_ids(text: &str) -> Vec<(String, bool)> {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let entry: Value = serde_json::from_str(line).unwrap();
            (
                entry["id"].as_str().unwrap().to_string(),
                entry["event"] == "restored",
            )
        })
        .collect()
}

/// KNOW-23: at done the ledger keeps its newest 100 entries; older entries whose archives
/// all have a knowledge note move into the vault with their first archive, a restore with
/// its entry, and nothing is lost or duplicated. An entry whose archive has no note stays.
#[test]
fn done_keeps_the_newest_ledger_entries_and_moves_the_rest_into_the_vault() {
    let fixture = project("0");
    let moved = rotated(&fixture);
    let rotation = &moved["applied"]["ledger_rotation"];
    assert_eq!(rotation["status"], "rotated", "{moved:#}");
    let hot = ledger_ids(&fixture.read(".chough/pruned.jsonl"));
    let shard = ledger_ids(&fixture.read("knowledge/attachments/a1/pruned.jsonl"));
    let hot_entries: Vec<&String> = hot
        .iter()
        .filter(|(_, restored)| !restored)
        .map(|(id, _)| id)
        .collect();
    assert!(hot_entries.contains(&&"no-note".to_string()), "{hot:?}");
    assert_eq!(
        hot_entries.len(),
        101,
        "the newest 100 plus the entry without a note: {hot:?}"
    );
    assert!(shard.contains(&("old-000".to_string(), false)), "{shard:?}");
    assert!(
        shard.contains(&("old-000".to_string(), true)),
        "a restore moves with its entry: {shard:?}"
    );
    assert!(!hot.iter().any(|(id, _)| id == "old-000"), "{hot:?}");
    assert!(
        shard.iter().all(|(id, _)| !hot_entries.contains(&id)),
        "no entry is in both"
    );
    let mut all: Vec<String> = hot
        .iter()
        .chain(&shard)
        .map(|(id, restored)| format!("{id}{restored}"))
        .collect();
    let total = all.len();
    all.sort();
    all.dedup();
    assert_eq!(all.len(), total, "no line is duplicated");
    assert!(all.len() >= 107, "nothing lost: {}", all.len());
    assert_eq!(
        rotation["moved"].as_u64().unwrap() as usize,
        shard.iter().filter(|(_, r)| !r).count()
    );
}

/// KNOW-23: readers see moved entries: a moved ID referenced by an open bug is still
/// labeled pruned, its archive note still lists it, and the vault check passes.
#[test]
fn moved_ledger_entries_are_still_read() {
    let fixture = project("0");
    bug(&fixture, "BUG-0009", "open", "a description, not a path");
    let path = ".chough/bugs/BUG-0009.json";
    let mut record: Value = serde_json::from_str(&fixture.read(path)).unwrap();
    record["observations"] = json!(["old-001"]);
    fixture.write(
        path,
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
    let moved = rotated(&fixture);
    assert_eq!(
        moved["applied"]["ledger_rotation"]["status"], "rotated",
        "{moved:#}"
    );
    assert!(
        fixture
            .read("knowledge/attachments/a1/pruned.jsonl")
            .contains("\"old-001\"")
    );
    let shown = fixture.ok(&["bug", "show", "BUG-0009"]);
    assert!(shown.to_string().contains("\"pruned\""), "{shown:#}");
    assert!(fixture.read("knowledge/archives/a1.md").contains("old-001"));
    fixture.ok(&["library", "index", "--check"]);
}

/// KNOW-23: an interrupted rotation (shard written, ledger not yet replaced) leaves a
/// line in both; it is read once. Here the leftover copy of old-000's entry after its
/// restore must not make old-000 pruned again.
#[test]
fn an_interrupted_rotation_leftover_is_read_once() {
    let fixture = project("0");
    bug(&fixture, "BUG-0009", "open", "a description, not a path");
    let path = ".chough/bugs/BUG-0009.json";
    let mut record: Value = serde_json::from_str(&fixture.read(path)).unwrap();
    record["observations"] = json!(["old-000"]);
    fixture.write(
        path,
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
    let moved = rotated(&fixture);
    assert_eq!(
        moved["applied"]["ledger_rotation"]["status"], "rotated",
        "{moved:#}"
    );
    let shard = fixture.read("knowledge/attachments/a1/pruned.jsonl");
    let leftover = shard
        .lines()
        .find(|line| line.contains("\"old-000\"") && !line.contains("restored"))
        .unwrap()
        .to_string();
    let hot = fixture.read(".chough/pruned.jsonl");
    fixture.write(".chough/pruned.jsonl", &format!("{hot}{leftover}\n"));
    let shown = fixture.ok(&["bug", "show", "BUG-0009"]).to_string();
    assert!(
        !shown.contains("\"pruned\""),
        "a restored ID must not read as pruned: {shown}"
    );
}

/// #127 closeout: a real ledger is larger than the 64 KiB record bound, and so is a
/// shard. Rotation writes both within the 16 MiB ledger bound instead (REQ-cli-049).
#[test]
fn a_rotation_larger_than_the_record_bound_still_rotates() {
    let fixture = project("0");
    let padding = "p".repeat(600);
    let lines: Vec<String> = (0..300u64)
        .map(|index| {
            let mut entry: Value = serde_json::from_str(&ledger_line(
                &format!("big-{index:03}"),
                "a1",
                1_000 - index,
            ))
            .unwrap();
            entry["reason"] = json!(padding);
            entry.to_string()
        })
        .collect();
    fixture.write(".chough/pruned.jsonl", &(lines.join("\n") + "\n"));
    fixture.commit("a ledger over the record bound");
    set_checkpoint(&fixture, &[]);
    let moved = fixture.ok(&["stage", "advance"]);
    let rotation = &moved["applied"]["ledger_rotation"];
    assert_eq!(rotation["status"], "rotated", "{rotation:#}");
    let shard = fixture.read("knowledge/attachments/a1/pruned.jsonl");
    let hot = fixture.read(".chough/pruned.jsonl");
    assert!(
        shard.len() > 65_536 && hot.len() > 65_536,
        "{} {}",
        shard.len(),
        hot.len()
    );
    assert_eq!(
        shard.lines().count() + hot.lines().count(),
        300 + pruned(&moved["applied"]["librarian_trim"]).len()
    );
}

/// Archived sessions for KNOW-29: s1 to s5 belong to archives a1 to a5 (finalized 60 to 20
/// days ago), s0 ties with s1 on a1 and sorts before it by key, and s6 has no native
/// change, so it is the oldest. The active session is adopted from the legacy checkpoint.
fn seven_archived_sessions(fixture: &Fixture) -> Vec<(String, String)> {
    fixture.ok(&["history", "adopt", "--apply"]);
    fixture.ok(&["session", "adopt", "legacy"]);
    let template: Value =
        serde_json::from_str(&fixture.read(".chough/sessions/legacy.json")).unwrap();
    let mut written = vec![];
    for (key, change) in [
        ("s0", Some("a1")),
        ("s1", Some("a1")),
        ("s2", Some("a2")),
        ("s3", Some("a3")),
        ("s4", Some("a4")),
        ("s5", Some("a5")),
        ("s6", None),
    ] {
        let mut session = template.clone();
        session["stage"]["current"] = json!("done");
        session["objective"] = json!(format!("Archived objective {key}"));
        session["native_change"] = json!(change);
        session["observations"] = json!([]);
        // One session holds the legacy anchor: the adopting one, still open (#143).
        session["history_cursor"]["legacy"] = Value::Null;
        let text = serde_json::to_string_pretty(&session).unwrap() + "\n";
        let path = format!(".chough/sessions/archive/{key}.json");
        fixture.write(&path, &text);
        written.push((key.to_string(), text));
    }
    fixture.commit("seven archived sessions");
    written
}

/// KNOW-29: at done only the five most recent archived sessions stay in
/// `.chough/sessions/archive/`, newest by their native change's finalization; older ones
/// move byte for byte into the vault with a summary note, and nothing else changes.
#[test]
fn done_keeps_the_five_newest_archived_sessions_and_rolls_the_rest_into_the_vault() {
    let fixture = project("0");
    let written = seven_archived_sessions(&fixture);
    let moved = fixture.ok(&["stage", "advance"]);
    let rollup = &moved["applied"]["session_rollup"];
    assert_eq!(rollup["status"], "rolled", "{moved:#}");
    assert_eq!(rollup["moved"], json!(["s0", "s6"]), "{rollup:#}");
    for key in ["s1", "s2", "s3", "s4", "s5"] {
        assert!(
            fixture.exists(&format!(".chough/sessions/archive/{key}.json")),
            "{key}"
        );
    }
    for (key, text) in written.iter().filter(|(key, _)| key == "s0" || key == "s6") {
        assert!(
            !fixture.exists(&format!(".chough/sessions/archive/{key}.json")),
            "{key}"
        );
        let attachment = format!("knowledge/attachments/{key}/sessions/{key}.json");
        assert_eq!(&fixture.read(&attachment), text, "{key}");
        let note = fixture.read(&format!("knowledge/sessions/{key}.md"));
        assert!(note.contains(&format!("id: \"session/{key}\"")), "{note}");
        assert!(
            note.contains(&format!("../attachments/{key}/sessions/{key}.json")),
            "{note}"
        );
        assert!(
            note.contains(&format!("Archived objective {key}")),
            "{note}"
        );
    }
    fixture.ok(&["library", "index", "--check"]);
    fixture.ok(&["session", "list"]);
}

/// KNOW-29: with the librarian off, no session moves.
#[test]
fn with_the_librarian_off_archived_sessions_stay() {
    let fixture = project("0");
    seven_archived_sessions(&fixture);
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "off",
        "--source",
        "Human: keep everything",
    ]);
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(
        moved["applied"]["session_rollup"]["status"], "skipped",
        "{moved:#}"
    );
    for key in ["s0", "s1", "s6"] {
        assert!(
            fixture.exists(&format!(".chough/sessions/archive/{key}.json")),
            "{key}"
        );
    }
}

/// Record a reset `days` ago in the legacy transition log, so the current cycle began then.
fn reset_days_ago(fixture: &Fixture, days: u64) {
    let reset = json!({
        "schema_version": 1, "at_unix_seconds": now() - days * DAY, "kind": "reset",
        "from": "done", "to": "plan", "reason": "fixture cycle",
    });
    let path = fixture.root().join(".chough/transitions.jsonl");
    let mut log = std::fs::read_to_string(&path).unwrap_or_default();
    log.push_str(&(reset.to_string() + "\n"));
    std::fs::write(&path, log).unwrap();
}

/// REQ-cli-045 (#138, audit assertion review): with the librarian off by a recorded
/// preference, the move into done reports the checkpoint references `skipped` and leaves
/// every observation, file and design reference as it was, even one from a closed cycle.
#[test]
fn with_the_librarian_off_done_leaves_the_references_and_reports_skipped() {
    let fixture = project("0");
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "off",
        "--source",
        "Fixture: the human turned the librarian off",
    ]);
    reset_days_ago(&fixture, 15);
    set_checkpoint(&fixture, &["r2", "u3"]);
    set_references(&fixture, &["src/main.rs"], &["docs/design.md"]);
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["stage"]["current"], "done", "{moved:#}");
    let references = &moved["applied"]["checkpoint_references"];
    assert_eq!(
        references,
        &json!({"status": "skipped", "reason": "the librarian is off"}),
        "{moved:#}"
    );
    let session = fixture.session();
    assert_eq!(session["observations"], json!(["r2", "u3"]), "{session:#}");
    assert_eq!(session["references"]["files"], json!(["src/main.rs"]));
    assert_eq!(session["references"]["design"], json!(["docs/design.md"]));
}

/// REQ-cli-045 (#138): with no reset recorded, no reset time can be read, so no observation
/// reference is dropped, however old its record; with nothing else to drop the step reports
/// `skipped` with that reason.
#[test]
fn with_no_reset_time_done_drops_no_observation_reference() {
    let fixture = project("0");
    // r2 was observed 65 days ago and u3 10 days ago; with no reset neither is known to be
    // from a closed cycle.
    set_checkpoint(&fixture, &["r2", "u3"]);
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["stage"]["current"], "done", "{moved:#}");
    assert_eq!(
        moved["applied"]["checkpoint_references"],
        json!({"status": "skipped", "reason": "no reset time can be read, so no cycle is known to be closed"}),
        "{moved:#}"
    );
    assert_eq!(fixture.session()["observations"], json!(["r2", "u3"]));
}

/// #132: a script a test installs must run even while an old write handle to its path
/// is still open, as a child forked by another test thread holds one until it execs
/// (ETXTBSY, "Text file busy"). Both the `stand_in` helper and `Fixture::executable`
/// must give the script a fresh file.
#[cfg(unix)]
#[test]
fn a_script_installed_while_an_old_write_handle_is_open_still_runs() {
    let fixture = Fixture::new();
    let bin = fixture.root().join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    for (name, install) in [
        (
            "via-stand-in",
            Box::new(|path: &std::path::Path| {
                common::stand_in(path.parent().unwrap(), "via-stand-in", "ran");
            }) as Box<dyn Fn(&std::path::Path)>,
        ),
        (
            "via-executable",
            Box::new(|_: &std::path::Path| {
                fixture.write("bin/via-executable", "#!/bin/sh\nprintf 'ran\\n'\n");
                fixture.executable("bin/via-executable");
            }),
        ),
    ] {
        let path = bin.join(name);
        // The leaked handle: open for writing, never closed while the script runs.
        let leaked = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&path)
            .unwrap();
        install(&path);
        let output = std::process::Command::new(&path)
            .output()
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(String::from_utf8_lossy(&output.stdout), "ran\n", "{name}");
        drop(leaked);
    }
}

/// Adopts the legacy checkpoint as session `legacy`, the active session at accept.
fn adopted_session(fixture: &Fixture) {
    fixture.ok(&["history", "adopt", "--apply"]);
    fixture.ok(&["session", "adopt", "legacy"]);
    fixture.commit("adopted session");
}

/// #155 (REQ-history-006, DRIVE-13, DRIVE-3): the move into done archives the session
/// itself, in its own change, and clears the active pointer, so no later `session new`
/// in another worktree moves it.
#[test]
fn the_move_into_done_archives_the_session_and_clears_the_pointer() {
    let fixture = project("0");
    adopted_session(&fixture);
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["applied"]["to"], "done", "{moved:#}");
    let archive = &moved["applied"]["session_archive"];
    assert_eq!(archive["status"], "archived", "{moved:#}");
    assert_eq!(archive["key"], "legacy", "{moved:#}");
    // REQ-cli-049: the ledger rotation ran in the same move, before the archive.
    assert!(
        moved["applied"]["ledger_rotation"]["status"].is_string(),
        "{moved:#}"
    );
    assert_eq!(moved["stage"]["current"], "done", "{moved:#}");
    assert!(!fixture.exists(".chough/sessions/legacy.json"));
    let archived: Value =
        serde_json::from_str(&fixture.read(".chough/sessions/archive/legacy.json")).unwrap();
    assert_eq!(archived["stage"]["current"], "done", "{archived:#}");
    assert!(!fixture.exists(".chough/local/active-session"));
    let next = fixture.fails(&["next"]);
    assert!(next.contains("no open session"), "{next}");
    assert!(next.contains("chough session new <key>"), "{next}");
    let started = fixture.ok(&["session", "new", "beta", "--objective", "Next work"]);
    assert_eq!(started["archived"], json!([]), "{started:#}");
    let list = fixture.ok(&["session", "list"]);
    assert!(list.to_string().contains("\"archived\""), "{list:#}");
}

/// #155: a session with unfolded inbox entries stays open at done, as on a switch.
#[test]
fn the_move_into_done_keeps_a_session_with_unfolded_inbox_entries_open() {
    let fixture = project("0");
    adopted_session(&fixture);
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        "legacy",
        "--from",
        "worker",
        "--kind",
        "decision",
        "--text",
        "Fold me first",
    ]);
    let moved = fixture.ok(&["stage", "advance"]);
    let archive = &moved["applied"]["session_archive"];
    assert_eq!(archive["status"], "kept_open", "{moved:#}");
    assert_eq!(archive["inbox_unfolded"], 1, "{moved:#}");
    assert!(fixture.exists(".chough/sessions/legacy.json"));
    assert!(!fixture.exists(".chough/sessions/archive/legacy.json"));
    assert_eq!(fixture.read(".chough/local/active-session"), "legacy\n");
    fixture.ok(&["next", "--json"]);
}

/// #155: a project still on the single legacy checkpoint has no session to archive.
#[test]
fn the_move_into_done_skips_the_archive_without_sessions() {
    let fixture = project("0");
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(
        moved["applied"]["session_archive"]["status"], "skipped",
        "{moved:#}"
    );
    assert!(fixture.exists(".chough/session.json"));
}

/// #155: when release is authorized at the move into done, the session stays open for
/// it, and archives itself at the move from release into done.
#[test]
fn a_session_with_release_authorized_archives_itself_after_release() {
    let fixture = project("0");
    adopted_session(&fixture);
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "Human: release it",
    ]);
    let done = fixture.ok(&["stage", "advance"]);
    let archive = &done["applied"]["session_archive"];
    assert_eq!(archive["status"], "kept_open", "{done:#}");
    assert!(fixture.exists(".chough/sessions/legacy.json"));
    fixture.ok(&["stage", "enter", "release", "--reason", "Requested"]);
    let released = fixture.ok(&["stage", "advance"]);
    assert_eq!(released["applied"]["to"], "done", "{released:#}");
    assert_eq!(
        released["applied"]["session_archive"]["status"], "archived",
        "{released:#}"
    );
    assert!(!fixture.exists(".chough/sessions/legacy.json"));
    assert!(fixture.exists(".chough/sessions/archive/legacy.json"));
    assert!(!fixture.exists(".chough/local/active-session"));
}

/// #143 (REQ-history-011): the archived session that holds the legacy history anchor is
/// never rolled into the vault and does not count toward the five, so the anchor never
/// depends on the knowledge directory. Here s0, which would roll first, holds the
/// anchor; it stays, and s6 still rolls.
#[test]
fn the_roll_up_keeps_the_session_that_holds_the_legacy_anchor() {
    let fixture = project("0");
    // A legacy log, so adoption records a real legacy anchor in the adopting session.
    fixture.write(".chough/transitions.jsonl", "");
    seven_archived_sessions(&fixture);
    let adopting: Value =
        serde_json::from_str(&fixture.read(".chough/sessions/legacy.json")).unwrap();
    let anchor = adopting["history_cursor"]["legacy"].clone();
    assert!(anchor.is_object(), "{adopting:#}");
    let path = ".chough/sessions/archive/s0.json";
    let mut holder: Value = serde_json::from_str(&fixture.read(path)).unwrap();
    holder["history_cursor"]["legacy"] = anchor;
    fixture.write(
        path,
        &(serde_json::to_string_pretty(&holder).unwrap() + "\n"),
    );
    fixture.commit("s0 holds the legacy anchor");
    let moved = fixture.ok(&["stage", "advance"]);
    let rollup = &moved["applied"]["session_rollup"];
    assert_eq!(rollup["status"], "rolled", "{moved:#}");
    assert_eq!(rollup["moved"], json!(["s6"]), "{rollup:#}");
    for key in ["s0", "s1", "s2", "s3", "s4", "s5"] {
        assert!(
            fixture.exists(&format!(".chough/sessions/archive/{key}.json")),
            "{key}"
        );
    }
    assert!(
        !fixture.exists("knowledge/attachments/s0/sessions/s0.json"),
        "{rollup:#}"
    );
}
