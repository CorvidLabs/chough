//! Reference-aware evidence retention and observation labels (REQ-history-001).
//!
//! Fixture observations are written with backdated `observed_at_unix_seconds` values so
//! the age rules are exercised without waiting.

mod common;

use common::Fixture;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const DAY: u64 = 86_400;
const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn digest(content: &str) -> String {
    format!("{:x}", Sha256::digest(content.as_bytes()))
}

/// One fixture observation; artifacts are written with their content.
struct Obs<'a> {
    id: &'a str,
    case: Option<&'a str>,
    environment: &'a str,
    outcome: &'a str,
    method: Option<&'a str>,
    kind: &'a str,
    age_days: u64,
    artifacts: Vec<(&'a str, &'a str)>,
}

impl<'a> Obs<'a> {
    /// A routine automated product-test run of a case.
    fn run(id: &'a str, case: &'a str, outcome: &'a str, age_days: u64) -> Self {
        Self {
            id,
            case: Some(case),
            environment: "local",
            outcome,
            method: Some("automated"),
            kind: "product-test",
            age_days,
            artifacts: vec![],
        }
    }

    fn with_log(mut self, path: &'a str, content: &'a str) -> Self {
        self.artifacts.push((path, content));
        self
    }
}

fn write_obs(fixture: &Fixture, obs: &Obs<'_>) {
    let mut fingerprints = BTreeMap::new();
    for (path, content) in &obs.artifacts {
        fixture.write(path, content);
        fingerprints.insert(path.to_string(), digest(content));
    }
    let mut record = json!({
        "schema_version": 2,
        "id": obs.id,
        "criteria": ["LIFE-1"],
        "requirements": ["REQ-cli-001"],
        "contracts": ["specs/cli/cli.spec.md"],
        "kind": obs.kind,
        "outcome": obs.outcome,
        "environment": obs.environment,
        "summary": format!("fixture observation {}", obs.id),
        "artifacts": obs.artifacts.iter().map(|(path, _)| *path).collect::<Vec<_>>(),
        "artifact_fingerprints": fingerprints,
        "observed_at_unix_seconds": now() - obs.age_days * DAY - 60,
        "snapshot": {"revision": "0".repeat(40), "dirty": false, "fingerprint": ZERO},
        "provenance": "fixture",
    });
    if let Some(case) = obs.case {
        record["cases"] = json!([{"id": case, "digest": ZERO}]);
    }
    if let Some(method) = obs.method {
        record["method"] = json!(method);
    }
    fixture.write(
        &format!(".chough/evidence/{}.json", obs.id),
        &serde_json::to_string_pretty(&record).unwrap(),
    );
}

fn write_bug(
    fixture: &Fixture,
    id: &str,
    status: &str,
    observations: &[&str],
    baseline: Option<&str>,
) {
    let bug = json!({
        "schema_version": 1,
        "id": id,
        "title": format!("Fixture bug {id}"),
        "symptom": "It breaks",
        "trigger": "Running the probe",
        "expected": "It works",
        "actual": "It broke",
        "environment": "local",
        "observations": observations,
        "impact": "Not yet assessed.",
        "classification": "unclassified",
        "scope": "in-scope",
        "status": status,
        "regression_baseline": baseline,
        "source": "fixture",
        "created_at_unix_seconds": 1,
        "updated_at_unix_seconds": 1,
    });
    fixture.write(
        &format!(".chough/bugs/{id}.json"),
        &serde_json::to_string_pretty(&bug).unwrap(),
    );
}

fn write_run(fixture: &Fixture, id: &str, observations: &[&str], completed: u32, failures: u32) {
    let run = json!({
        "schema_version": 1,
        "id": id,
        "case": "TC-val",
        "times": completed,
        "interval_seconds": 0,
        "environment": "local",
        "authority": "run consent: fixture",
        "state": "complete",
        "completed": completed,
        "failures": failures,
        "not_run": 0,
        "observations": observations,
        "bugs": [],
        "elapsed_seconds": 1,
        "started_at_unix_seconds": 1,
    });
    fixture.write(
        &format!(".chough/validation/{id}.json"),
        &serde_json::to_string_pretty(&run).unwrap(),
    );
}

/// Every file under the project (outside .git) with its bytes.
fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(root: &Path, dir: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.file_name().unwrap() == ".git" {
                continue;
            }
            if path.is_dir() {
                walk(root, &path, files);
            } else {
                let name = path
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                files.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    let mut files = BTreeMap::new();
    walk(root, root, &mut files);
    files
}

fn ids(list: &Value) -> BTreeSet<String> {
    list.as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["id"].as_str().unwrap().to_string())
        .collect()
}

fn reasons(preview: &Value, id: &str) -> Vec<String> {
    preview["protected"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == id)
        .unwrap_or_else(|| panic!("{id} is not protected: {preview:#}"))["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|reason| reason.as_str().unwrap().to_string())
        .collect()
}

fn set<const N: usize>(items: [&str; N]) -> BTreeSet<String> {
    items.into_iter().map(String::from).collect()
}

fn project() -> Fixture {
    let fixture = Fixture::new();
    fixture.init();
    fixture.commit("fixture");
    fixture
}

#[test]
fn preview_lists_sets_with_reasons_and_bytes_and_changes_nothing() {
    let fixture = project();
    for (id, age) in [
        ("r30", 30),
        ("r29", 29),
        ("r28", 28),
        ("r27", 27),
        ("r26", 26),
    ] {
        let log = format!(".chough/artifacts/{id}.log");
        write_obs(
            &fixture,
            &Obs::run(id, "TC-a", "passed", age).with_log(&log, "ok\n"),
        );
    }
    let before = tree(fixture.root());
    let preview = fixture.ok(&["evidence", "prune"]);
    assert_eq!(tree(fixture.root()), before, "the preview changed a file");
    assert!(!fixture.exists(".chough/pruned.jsonl"));
    assert_eq!(preview["applied"], false);
    assert_eq!(ids(&preview["eligible"]), set(["r29", "r30"]));
    for id in ["r26", "r27", "r28"] {
        assert_eq!(reasons(&preview, id), vec!["newest"]);
    }
    let eligible = &preview["eligible"].as_array().unwrap()[0];
    assert_eq!(eligible["cases"], json!(["TC-a"]));
    assert_eq!(eligible["environment"], "local");
    assert_eq!(eligible["outcome"], "passed");
    assert!(eligible["age_days"].as_u64().unwrap() >= 29);
    let record_bytes = fs::metadata(fixture.root().join(format!(
        ".chough/evidence/{}.json",
        eligible["id"].as_str().unwrap()
    )))
    .unwrap()
    .len();
    assert_eq!(eligible["bytes"].as_u64().unwrap(), record_bytes + 3);
    let reclaimable: u64 = preview["eligible"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["bytes"].as_u64().unwrap())
        .sum();
    assert_eq!(preview["totals"]["reclaimable_bytes"], reclaimable);
    assert_eq!(preview["totals"]["eligible"], 2);
    assert_eq!(preview["totals"]["protected"], 3);
    assert!(preview["totals"]["evidence_bytes"].as_u64().unwrap() >= reclaimable);
    assert!(preview["warning"].is_null());
    assert_eq!(preview["policy"]["keep_newest"], 3);
    assert_eq!(preview["policy"]["min_age_days"], 14);
}

#[test]
fn every_protection_reason_is_named() {
    let fixture = project();
    // Three newer routine passes fill the newest slots for TC-a.
    for (id, age) in [("fill1", 20), ("fill2", 21), ("fill3", 22)] {
        write_obs(&fixture, &Obs::run(id, "TC-a", "passed", age));
    }
    write_obs(&fixture, &Obs::run("cp", "TC-a", "passed", 50));
    write_obs(&fixture, &Obs::run("bug-obs", "TC-a", "passed", 51));
    write_obs(&fixture, &Obs::run("bug-base", "TC-a", "passed", 52));
    let mut manual = Obs::run("manual", "TC-a", "passed", 53);
    manual.method = Some("manual");
    write_obs(&fixture, &manual);
    let mut human = Obs::run("human", "TC-a", "passed", 54);
    human.kind = "human-observation";
    write_obs(&fixture, &human);
    let mut delivery = Obs::run("delivery", "TC-a", "passed", 54);
    delivery.kind = "delivery-observation";
    write_obs(&fixture, &delivery);
    write_obs(&fixture, &Obs::run("fail-new", "TC-a", "failed", 55));
    write_obs(&fixture, &Obs::run("fail-old", "TC-a", "failed", 56));
    for (id, outcome, age) in [
        ("val-first", "passed", 63),
        ("val-fail", "failed", 62),
        ("val-mid", "passed", 61),
        ("val-last", "passed", 60),
    ] {
        write_obs(&fixture, &Obs::run(id, "TC-val", outcome, age));
    }
    // TC-val's own newest three are other runs, so only the run protects its sets.
    for (id, age) in [
        ("vfill1", 30),
        ("vfill2", 31),
        ("vfill3", 32),
        ("vfail", 33),
    ] {
        let outcome = if id == "vfail" { "failed" } else { "passed" };
        write_obs(&fixture, &Obs::run(id, "TC-val", outcome, age));
    }
    write_run(
        &fixture,
        "tc-val-1",
        &["val-first", "val-fail", "val-mid", "val-last"],
        4,
        1,
    );
    // TC-b: three newest plus a fourth that is still younger than 14 days.
    for (id, age) in [("b1", 1), ("b2", 2), ("b3", 3), ("young", 13)] {
        write_obs(&fixture, &Obs::run(id, "TC-b", "passed", age));
    }
    fixture.ok(&["checkpoint", "--observation", "cp"]);
    write_bug(&fixture, "BUG-0001", "verified", &["bug-obs"], None);
    write_bug(&fixture, "BUG-0002", "open", &[], Some("bug-base"));

    let preview = fixture.ok(&["evidence", "prune"]);
    assert_eq!(
        ids(&preview["eligible"]),
        set(["fail-old", "val-mid"]),
        "{preview:#}"
    );
    assert_eq!(reasons(&preview, "cp"), vec!["checkpoint"]);
    assert_eq!(reasons(&preview, "bug-obs"), vec!["BUG-0001 verified"]);
    assert_eq!(reasons(&preview, "bug-base"), vec!["BUG-0002 open"]);
    assert_eq!(reasons(&preview, "manual"), vec!["manual"]);
    assert_eq!(reasons(&preview, "human"), vec!["human-observation"]);
    assert_eq!(reasons(&preview, "delivery"), vec!["delivery-observation"]);
    assert_eq!(reasons(&preview, "fail-new"), vec!["last failure"]);
    assert_eq!(reasons(&preview, "vfail"), vec!["last failure"]);
    for id in ["val-first", "val-fail", "val-last"] {
        assert_eq!(reasons(&preview, id), vec!["validation tc-val-1"]);
    }
    assert_eq!(reasons(&preview, "fill1"), vec!["newest"]);
    assert_eq!(reasons(&preview, "young"), vec!["recent"]);
    assert_eq!(reasons(&preview, "b1"), vec!["newest", "recent"]);
}

#[test]
fn newest_runs_count_per_case_and_environment() {
    let fixture = project();
    for (id, age) in [("l1", 20), ("l2", 21), ("l3", 22), ("l4", 23)] {
        write_obs(&fixture, &Obs::run(id, "TC-a", "passed", age));
    }
    // Another environment keeps its own newest three.
    for (id, age) in [("ci1", 40), ("ci2", 41), ("ci3", 42)] {
        let mut obs = Obs::run(id, "TC-a", "passed", age);
        obs.environment = "ci";
        write_obs(&fixture, &obs);
    }
    // Exactly 14 days old is not younger than 14 days.
    let mut edge = Obs::run("edge", "TC-a", "passed", 14);
    edge.environment = "edge-env";
    write_obs(&fixture, &edge);
    for (id, age) in [("e1", 1), ("e2", 2), ("e3", 3)] {
        let mut obs = Obs::run(id, "TC-a", "passed", age);
        obs.environment = "edge-env";
        write_obs(&fixture, &obs);
    }
    let preview = fixture.ok(&["evidence", "prune"]);
    assert_eq!(
        ids(&preview["eligible"]),
        set(["edge", "l4"]),
        "{preview:#}"
    );
    for id in ["ci1", "ci2", "ci3", "l1", "l2", "l3"] {
        assert_eq!(reasons(&preview, id), vec!["newest"]);
    }
}

#[test]
fn apply_needs_consent_or_the_standing_preference_and_appends_the_ledger() {
    let fixture = project();
    for (id, age) in [("n1", 20), ("n2", 21), ("n3", 22)] {
        write_obs(&fixture, &Obs::run(id, "TC-a", "passed", age));
    }
    fixture.write("docs/shot.txt", "user file\n");
    let mut old = Obs::run("old", "TC-a", "passed", 40)
        .with_log(".chough/artifacts/old.log", "old log\n")
        .with_log(".chough/artifacts/shared.log", "shared\n")
        .with_log("docs/shot.txt", "user file\n");
    old.outcome = "passed";
    write_obs(&fixture, &old);
    write_obs(
        &fixture,
        &Obs::run("keeper", "TC-a", "passed", 41)
            .with_log(".chough/artifacts/shared.log", "shared\n"),
    );
    fixture.ok(&["checkpoint", "--observation", "keeper"]);

    let before = tree(fixture.root());
    let refused = fixture.fails(&["evidence", "prune", "--apply"]);
    assert!(refused.contains("consent grant retention"), "{refused}");
    assert!(refused.contains("retention auto"), "{refused}");
    assert_eq!(tree(fixture.root()), before);

    let prefs = fixture.ok(&["pref", "show"]);
    assert_eq!(prefs["effective"]["retention"]["value"], "off");
    assert_eq!(prefs["effective"]["retention"]["origin"], "default");

    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: prune old routine logs",
    ]);
    let applied = fixture.ok(&["evidence", "prune", "--apply"]);
    assert_eq!(applied["applied"], true);
    assert_eq!(applied["pruned"], json!(["old"]));
    assert!(
        applied["authority"]
            .as_str()
            .unwrap()
            .starts_with("run consent")
    );
    assert!(!fixture.exists(".chough/evidence/old.json"));
    assert!(!fixture.exists(".chough/artifacts/old.log"));
    // Still referenced by another record, or outside Chough's artifacts: kept.
    assert!(fixture.exists(".chough/artifacts/shared.log"));
    assert_eq!(fixture.read("docs/shot.txt"), "user file\n");
    assert!(fixture.exists(".chough/evidence/keeper.json"));

    let ledger = fixture.read(".chough/pruned.jsonl");
    let lines: Vec<&str> = ledger.lines().collect();
    assert_eq!(lines.len(), 1);
    let entry: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(entry["id"], "old");
    assert_eq!(entry["cases"], json!(["TC-a"]));
    assert_eq!(entry["environment"], "local");
    assert_eq!(entry["outcome"], "passed");
    assert_eq!(entry["revision"], "0".repeat(40));
    assert!(entry["observed_at_unix_seconds"].as_u64().unwrap() < now() - 40 * DAY);
    assert_eq!(
        entry["artifacts"][".chough/artifacts/old.log"],
        digest("old log\n")
    );
    assert_eq!(
        entry["removed"],
        json!([".chough/evidence/old.json", ".chough/artifacts/old.log"])
    );
    assert!(entry["reason"].as_str().unwrap().contains("14 days"));
    assert_eq!(entry["policy"]["keep_newest"], 3);
    assert!(
        entry["policy"]["authority"]
            .as_str()
            .unwrap()
            .starts_with("run consent")
    );
    assert_eq!(entry["head"].as_str().unwrap().len(), 40);

    // A second apply has nothing left to prune and appends nothing.
    let again = fixture.ok(&["evidence", "prune", "--apply"]);
    assert_eq!(again["pruned"], json!([]));
    assert_eq!(fixture.read(".chough/pruned.jsonl"), ledger);

    // The standing preference alone also authorizes apply.
    let other = project();
    for (id, age) in [("n1", 20), ("n2", 21), ("n3", 22), ("old", 40)] {
        write_obs(&other, &Obs::run(id, "TC-a", "passed", age));
    }
    other.fails(&["pref", "set", "retention", "always", "--source", "Human"]);
    other.ok(&[
        "pref",
        "set",
        "retention",
        "auto",
        "--source",
        "Human: prune routine runs automatically",
    ]);
    let applied = other.ok(&["evidence", "prune", "--apply"]);
    assert_eq!(applied["pruned"], json!(["old"]));
    assert!(
        applied["authority"]
            .as_str()
            .unwrap()
            .starts_with("standing preference")
    );
}

#[test]
fn removed_and_missing_observations_are_labeled_and_never_count() {
    let fixture = project();
    for (id, age) in [
        ("n1", 20),
        ("n2", 21),
        ("n3", 22),
        ("old-fail", 40),
        ("old-pass", 41),
    ] {
        let outcome = if id == "old-fail" { "failed" } else { "passed" };
        write_obs(&fixture, &Obs::run(id, "TC-a", outcome, age));
    }
    // A newer failure keeps old-fail out of the last-failure slot.
    write_obs(&fixture, &Obs::run("newer-fail", "TC-a", "failed", 30));
    fixture.ok(&["consent", "grant", "retention", "--source", "Human: prune"]);
    let applied = fixture.ok(&["evidence", "prune", "--apply"]);
    assert_eq!(
        applied["pruned"].as_array().unwrap().len(),
        2,
        "{applied:#}"
    );

    // References written after pruning: the checkpoint, a bug and a validation run.
    fixture.ok(&[
        "checkpoint",
        "--observation",
        "old-pass",
        "--observation",
        "never-recorded",
        "--observation",
        "n1",
    ]);
    write_bug(
        &fixture,
        "BUG-0001",
        "open",
        &["old-fail", "old-pass", "never-recorded"],
        None,
    );
    write_run(
        &fixture,
        "tc-a-1",
        &["old-fail", "old-pass", "never-recorded"],
        3,
        1,
    );

    // The focused next view lists only the referenced observations that need attention.
    let next = fixture.ok(&["next", "--environment", "local", "--json"]);
    let listed: BTreeMap<String, String> = next["observations"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            (
                entry["id"].as_str().unwrap().to_string(),
                entry["label"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(listed["old-pass"], "pruned", "{next}");
    assert_eq!(listed["old-fail"], "pruned", "{next}");
    assert_eq!(listed["never-recorded"], "unavailable", "{next}");
    assert_eq!(listed["n1"], "historical", "{next}");
    assert!(listed.values().all(|label| label != "current"), "{next}");
    for command in ["status"] {
        let report = fixture.ok(&[command, "--environment", "local"]);
        let references = report["observation_references"].as_array().unwrap();
        let label = |id: &str| {
            references
                .iter()
                .find(|entry| entry["id"] == id)
                .unwrap_or_else(|| panic!("{id} missing from {references:?}"))["label"]
                .clone()
        };
        assert_eq!(label("old-pass"), "pruned");
        assert_eq!(label("old-fail"), "pruned");
        assert_eq!(label("never-recorded"), "unavailable");
        assert_eq!(label("n1"), "historical");
        for observation in report["observations"].as_array().unwrap() {
            assert_ne!(observation["record"]["id"], "old-pass");
            assert!(observation["label"] == "historical" || observation["label"] == "current");
        }
    }

    let bug = fixture.ok(&["bug", "show", "BUG-0001"]);
    assert_eq!(bug["verification"], "unverified");
    let labels: BTreeMap<String, String> = bug["observation_labels"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            (
                entry["id"].as_str().unwrap().to_string(),
                entry["label"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(labels["old-pass"], "pruned");
    assert_eq!(labels["old-fail"], "pruned");
    assert_eq!(labels["never-recorded"], "unavailable");
    let refused = fixture.fails(&["bug", "update", "BUG-0001", "--status", "verified"]);
    assert!(refused.contains("passed observation"), "{refused}");
    assert_eq!(
        fixture.ok(&["bug", "list"])["bugs"][0]["verification"],
        "unverified"
    );

    let validation = fixture.ok(&["validate", "status"]);
    let run = &validation["runs"][0];
    assert_eq!(run["completed"], 3);
    assert_eq!(run["failures"], 1);
    let labels: Vec<&str> = run["observation_labels"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["label"].as_str().unwrap())
        .collect();
    assert_eq!(labels, vec!["pruned", "pruned", "unavailable"]);
}

#[test]
fn a_validation_run_keeps_its_counts_after_its_middle_passes_are_pruned() {
    let fixture = project();
    let run: Vec<String> = (0..6).map(|index| format!("v{index}")).collect();
    for (index, id) in run.iter().enumerate() {
        let outcome = if index == 2 { "failed" } else { "passed" };
        write_obs(
            &fixture,
            &Obs::run(id, "TC-val", outcome, 60 - index as u64),
        );
    }
    for (id, age) in [("f1", 20), ("f2", 21), ("f3", 22), ("ff", 23)] {
        let outcome = if id == "ff" { "failed" } else { "passed" };
        write_obs(&fixture, &Obs::run(id, "TC-val", outcome, age));
    }
    let refs: Vec<&str> = run.iter().map(String::as_str).collect();
    write_run(&fixture, "tc-val-1", &refs, 6, 1);
    fixture.ok(&["consent", "grant", "retention", "--source", "Human: prune"]);
    let applied = fixture.ok(&["evidence", "prune", "--apply"]);
    let pruned: BTreeSet<String> = applied["pruned"]
        .as_array()
        .unwrap()
        .iter()
        .map(|id| id.as_str().unwrap().to_string())
        .collect();
    assert_eq!(pruned, set(["v1", "v3", "v4"]));
    let status = fixture.ok(&["validate", "status"]);
    let stored = &status["runs"][0];
    assert_eq!(stored["completed"], 6);
    assert_eq!(stored["failures"], 1);
    assert_eq!(stored["observations"].as_array().unwrap().len(), 6);
    let labels: Vec<&str> = stored["observation_labels"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["label"].as_str().unwrap())
        .collect();
    assert_eq!(
        labels,
        vec![
            "historical",
            "pruned",
            "historical",
            "pruned",
            "pruned",
            "historical"
        ]
    );
}

/// A SpecSync stand-in whose acceptance entries come from `.stub-manifest-<id>` files.
#[cfg(unix)]
fn native_fixture() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(
        "bin/specsync",
        r#"#!/bin/sh
if [ -f .stub-fail ]; then echo 'status failed' >&2; exit 2; fi
id="$3"
entries=$(cat ".stub-manifest-$id" 2>/dev/null || echo '')
printf '{"summary":{"id":"%s","state":"stub"},"acceptance_entries":[%s]}\n' "$id" "$entries"
"#,
    );
    fixture.executable("bin/specsync");
    fixture.write(".specsync/changes/active-one/change.md", "active\n");
    for (dir, id, timestamp) in [
        ("2026-01-01-old-one", "old-one", 100),
        ("2026-02-01-new-one", "new-one", 200),
    ] {
        fixture.write(
            &format!(".specsync/archive/changes/{dir}/finalization.json"),
            &format!(
                "{{\"schema_version\": 2, \"change_id\": \"{id}\", \"timestamp\": {timestamp}}}\n"
            ),
        );
    }
    let entry = |path: &str| format!("{{\"kind\":\"file\",\"path\":\"{path}\"}}");
    fixture.write(
        ".stub-manifest-active-one",
        &entry(".chough/artifacts/listed-active.log"),
    );
    fixture.write(
        ".stub-manifest-new-one",
        &entry(".chough/evidence/listed-archive.json"),
    );
    fixture.write(
        ".stub-manifest-old-one",
        &entry(".chough/evidence/listed-old.json"),
    );
    fixture.init();
    fixture.commit("fixture");
    for (id, age) in [("n1", 20), ("n2", 21), ("n3", 22)] {
        write_obs(&fixture, &Obs::run(id, "TC-a", "passed", age));
    }
    write_obs(
        &fixture,
        &Obs::run("listed-active", "TC-a", "passed", 40)
            .with_log(".chough/artifacts/listed-active.log", "log\n"),
    );
    write_obs(&fixture, &Obs::run("listed-archive", "TC-a", "passed", 41));
    write_obs(&fixture, &Obs::run("listed-old", "TC-a", "passed", 42));
    write_obs(&fixture, &Obs::run("plain", "TC-a", "passed", 43));
    fixture
}

#[cfg(unix)]
#[test]
fn native_manifests_protect_their_sets_and_a_failed_read_protects_everything() {
    let fixture = native_fixture();
    let preview = fixture.ok(&["evidence", "prune"]);
    assert_eq!(preview["native"]["state"], "observed", "{preview:#}");
    assert_eq!(
        reasons(&preview, "listed-active"),
        vec!["native active active-one"]
    );
    assert_eq!(
        reasons(&preview, "listed-archive"),
        vec!["native archive new-one"]
    );
    // Only the most recent archive protects; older archives authenticate from history.
    assert_eq!(ids(&preview["eligible"]), set(["listed-old", "plain"]));

    fixture.write(".stub-fail", "");
    let before = tree(fixture.root());
    let failed = fixture.ok(&["evidence", "prune"]);
    assert_eq!(failed["native"]["state"], "unavailable");
    assert_eq!(failed["eligible"], json!([]));
    assert!(reasons(&failed, "plain").contains(&"native unavailable".to_string()));
    fixture.ok(&["consent", "grant", "retention", "--source", "Human: prune"]);
    let applied = fixture.ok(&["evidence", "prune", "--apply"]);
    assert_eq!(applied["pruned"], json!([]));
    let mut after = tree(fixture.root());
    after.remove(".chough/session.json");
    // The consent grant is a checkpoint writer: it may create the project's lock (#83).
    after.remove(".chough/local/.gitignore");
    after.remove(".chough/local/checkpoint.lock");
    let mut before = before;
    before.remove(".chough/session.json");
    before.remove(".chough/local/.gitignore");
    before.remove(".chough/local/checkpoint.lock");
    assert_eq!(after, before);

    // Output that is not a manifest is a failed read too.
    fs::remove_file(fixture.root().join(".stub-fail")).unwrap();
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nprintf '{\"summary\":{\"state\":\"stub\"}}\\n'\n",
    );
    fixture.executable("bin/specsync");
    let malformed = fixture.ok(&["evidence", "prune"]);
    assert_eq!(malformed["native"]["state"], "unavailable");
    assert_eq!(malformed["eligible"], json!([]));
}

#[cfg(unix)]
/// #136: rebuilding the knowledge index lists the observation in INDEX.md, and
/// that generated write must not make the observation stale (REQ-cli-004 AC2).
#[test]
fn rebuilding_the_knowledge_index_keeps_the_observation_current() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"Probe reports healthy\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo healthy: yes\"]\nexpect_output = [\"healthy: yes\"]\n",
    );
    fixture.init();
    fixture.commit("fixture");
    let run = fixture.ok(&["cases", "run", "TC-cli-0001", "--environment", "test-local"]);
    let observation = run["observation"].as_str().unwrap().to_string();
    fixture.ok(&["checkpoint", "--observation", &observation]);
    let label = |fixture: &Fixture| {
        fixture.ok(&["status", "--environment", "test-local"])["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["record"]["id"] == observation)
            .map(|entry| entry["label"].as_str().unwrap().to_string())
    };
    assert_eq!(label(&fixture).as_deref(), Some("current"));

    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/INDEX.md");
    assert!(index.contains(&observation), "{index}");
    assert_eq!(label(&fixture).as_deref(), Some("current"));
}

#[test]
fn automatic_pruning_follows_case_runs_only_when_retention_is_auto() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"Probe reports healthy\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo healthy: yes\"]\nexpect_output = [\"healthy: yes\"]\n",
    );
    fixture.init();
    fixture.commit("fixture");
    for (id, age) in [("old1", 40), ("old2", 41), ("old3", 42), ("old4", 43)] {
        let mut obs = Obs::run(id, "TC-cli-0001", "passed", age);
        obs.environment = "test-local";
        write_obs(&fixture, &obs);
    }
    let run = |fixture: &Fixture| {
        fixture.ok(&["cases", "run", "TC-cli-0001", "--environment", "test-local"])
    };
    let first = run(&fixture);
    assert!(first["retention"].is_null(), "{first:#}");
    assert!(fixture.exists(".chough/evidence/old4.json"));

    fixture.ok(&[
        "pref",
        "set",
        "retention",
        "auto",
        "--source",
        "Human: prune routine runs automatically",
    ]);
    let second = run(&fixture);
    // The two fresh runs and old1 are the newest three; old2 to old4 are pruned.
    assert_eq!(
        second["retention"]["pruned"],
        json!(["old2", "old3", "old4"]),
        "{second:#}"
    );
    assert!(!fixture.exists(".chough/evidence/old4.json"));
    assert_eq!(fixture.read(".chough/pruned.jsonl").lines().count(), 3);

    // The fresh run is current; a pruned one is labeled, not passed.
    fixture.ok(&["checkpoint", "--observation", "old4"]);
    let status = fixture.ok(&["status", "--environment", "test-local"]);
    let current = status["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|observation| observation["label"] == "current")
        .count();
    assert_eq!(current, 2, "{status:#}");
    assert_eq!(status["observation_references"][0]["label"], "pruned");

    // validate run prunes after recording as well.
    fixture.ok(&[
        "consent",
        "grant",
        "repeat-validation",
        "--source",
        "Human: repeat",
    ]);
    let validated = fixture.ok(&[
        "validate",
        "run",
        "TC-cli-0001",
        "--times",
        "1",
        "--environment",
        "test-local",
    ]);
    assert_eq!(validated["run"]["completed"], 1);
    assert!(validated["retention"].is_object(), "{validated:#}");
}

/// Rewrites a fixture record so it binds every listed case.
fn bind_cases(fixture: &Fixture, id: &str, cases: &[&str]) {
    let name = format!(".chough/evidence/{id}.json");
    let mut record: Value = serde_json::from_str(&fixture.read(&name)).unwrap();
    record["cases"] = cases
        .iter()
        .map(|case| json!({"id": case, "digest": ZERO}))
        .collect();
    fixture.write(&name, &serde_json::to_string_pretty(&record).unwrap());
}

#[test]
fn a_multi_case_record_counts_in_the_group_of_every_case() {
    let fixture = project();
    for (id, age) in [("x0", 15), ("x1", 16), ("x2", 17)] {
        write_obs(&fixture, &Obs::run(id, "TC-x", "passed", age));
    }
    // The newest TC-y run, and TC-y's newest failure, but listed second.
    write_obs(&fixture, &Obs::run("m1", "TC-x", "failed", 30));
    bind_cases(&fixture, "m1", &["TC-x", "TC-y"]);
    write_obs(&fixture, &Obs::run("xf", "TC-x", "failed", 20));
    for (id, age) in [("y0", 40), ("y1", 41), ("y2", 42)] {
        write_obs(&fixture, &Obs::run(id, "TC-y", "passed", age));
    }
    let preview = fixture.ok(&["evidence", "prune"]);
    assert_eq!(ids(&preview["eligible"]), set(["y2"]), "{preview:#}");
    assert_eq!(reasons(&preview, "m1"), vec!["newest", "last failure"]);
    let entry = preview["protected"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == "m1")
        .unwrap();
    assert_eq!(entry["cases"], json!(["TC-x", "TC-y"]));
}

#[test]
fn one_artifact_spelled_two_ways_is_shared_and_kept() {
    let fixture = project();
    write_obs(
        &fixture,
        &Obs::run("a1", "TC-a", "passed", 1).with_log(".chough/artifacts/shared.log", "shared\n"),
    );
    for (id, age) in [("b0", 20), ("b1", 21), ("b2", 22)] {
        write_obs(&fixture, &Obs::run(id, "TC-b", "passed", age));
    }
    let mut old = Obs::run("old", "TC-b", "passed", 40)
        .with_log(".chough/artifacts//shared.log", "shared\n")
        .with_log(".chough/artifacts/./old.log", "old\n");
    old.artifacts
        .push((".CHOUGH/ARTIFACTS/SHARED.LOG", "shared\n"));
    // On a case-sensitive filesystem the upper-case spelling is its own file.
    let insensitive = {
        fixture.write(".chough/artifacts/probe.txt", "x");
        let found = fixture.exists(".chough/ARTIFACTS/PROBE.TXT");
        fs::remove_file(fixture.root().join(".chough/artifacts/probe.txt")).unwrap();
        found
    };
    if !insensitive {
        old.artifacts.pop();
    }
    write_obs(&fixture, &old);

    let preview = fixture.ok(&["evidence", "prune"]);
    assert_eq!(ids(&preview["eligible"]), set(["old"]), "{preview:#}");
    assert_eq!(
        preview["eligible"][0]["files"],
        json!([".chough/evidence/old.json", ".chough/artifacts/./old.log"])
    );
    fixture.ok(&["consent", "grant", "retention", "--source", "Human: prune"]);
    let applied = fixture.ok(&["evidence", "prune", "--apply"]);
    assert_eq!(applied["pruned"], json!(["old"]));
    assert_eq!(fixture.read(".chough/artifacts/shared.log"), "shared\n");
    assert!(!fixture.exists(".chough/artifacts/old.log"));

    for spelling in [
        ".chough/artifacts//shared.log",
        ".chough/artifacts/shared.log/",
    ] {
        let refused = fixture.fails(&[
            "evidence",
            "add",
            "OBS-product-test-000000000000000000000006",
            "--criterion",
            "LIFE-1",
            "--requirement",
            "REQ-cli-001",
            "--contract",
            "specs/cli/cli.spec.md",
            "--kind",
            "product-test",
            "--outcome",
            "passed",
            "--environment",
            "local",
            "--summary",
            "Spelled oddly",
            "--artifact",
            spelling,
        ]);
        assert!(refused.contains("spelled canonically"), "{refused}");
    }
}

#[test]
fn bug_fixtures_and_checkpoint_paths_protect_the_sets_they_name() {
    let fixture = project();
    for (id, age) in [("c0", 20), ("c1", 21), ("c2", 22)] {
        write_obs(&fixture, &Obs::run(id, "TC-c", "passed", age));
    }
    for id in ["c3", "c4", "c5"] {
        let log = format!(".chough/artifacts/{id}.log");
        write_obs(
            &fixture,
            &Obs::run(id, "TC-c", "passed", 43).with_log(&log, "log\n"),
        );
    }
    write_bug(&fixture, "BUG-0001", "verified", &[], None);
    let name = ".chough/bugs/BUG-0001.json";
    let mut bug: Value = serde_json::from_str(&fixture.read(name)).unwrap();
    bug["fixture"] = json!(".chough/artifacts/c3.log");
    fixture.write(name, &serde_json::to_string_pretty(&bug).unwrap());
    fixture.ok(&["checkpoint", "--file", ".chough/artifacts/c4.log"]);

    let preview = fixture.ok(&["evidence", "prune"]);
    assert_eq!(ids(&preview["eligible"]), set(["c5"]), "{preview:#}");
    assert_eq!(reasons(&preview, "c3"), vec!["BUG-0001 verified"]);
    assert_eq!(reasons(&preview, "c4"), vec!["checkpoint"]);
    fixture.ok(&["consent", "grant", "retention", "--source", "Human: prune"]);
    let applied = fixture.ok(&["evidence", "prune", "--apply"]);
    assert_eq!(applied["pruned"], json!(["c5"]));
    assert!(fixture.exists(".chough/artifacts/c3.log"));
    assert!(fixture.exists(".chough/artifacts/c4.log"));
    assert!(!fixture.exists(".chough/artifacts/c5.log"));
}

#[test]
fn bug_show_labels_note_observations_once() {
    let fixture = project();
    write_obs(&fixture, &Obs::run("seen", "TC-a", "failed", 1));
    write_bug(&fixture, "BUG-0001", "open", &["seen"], None);
    let name = ".chough/bugs/BUG-0001.json";
    let mut bug: Value = serde_json::from_str(&fixture.read(name)).unwrap();
    bug["notes"] = json!([
        {"at_unix_seconds": 2, "text": "again", "observations": ["seen", "gone"]}
    ]);
    fixture.write(name, &serde_json::to_string_pretty(&bug).unwrap());
    let shown = fixture.ok(&["bug", "show", "BUG-0001"]);
    assert_eq!(
        shown["observation_labels"],
        json!([
            {"id": "seen", "role": "observation", "label": "historical"},
            {"id": "gone", "role": "note", "label": "unavailable"},
        ]),
        "{shown:#}"
    );
}

#[test]
fn retention_revalidation_keeps_changed_missing_and_newly_referenced_proof() {
    fn setup() -> Fixture {
        let fixture = Fixture::new();
        fixture.init();
        for index in 0..4 {
            let id = format!("revalidate-{index}");
            let path = format!(".chough/artifacts/revalidate-{index}.log");
            write_obs(
                &fixture,
                &Obs::run(&id, "TC-cli-0001", "passed", 40).with_log(&path, "original proof"),
            );
        }
        fixture.ok(&[
            "consent",
            "grant",
            "retention",
            "--source",
            "fixture authorization",
        ]);
        assert_eq!(fixture.ok(&["evidence", "prune"])["totals"]["eligible"], 1);
        fixture
    }
    let f = setup();
    let original = f.read(".chough/evidence/revalidate-0.json");
    f.write(
        ".chough/artifacts/revalidate-0.log",
        "changed proof retained",
    );
    let report = f.ok(&["evidence", "prune", "--apply"]);
    assert!(report["pruned"].as_array().unwrap().is_empty());
    assert_eq!(report["skipped"][0]["id"], "revalidate-0");
    assert_eq!(f.read(".chough/evidence/revalidate-0.json"), original);
    assert_eq!(
        f.read(".chough/artifacts/revalidate-0.log"),
        "changed proof retained"
    );

    let f = setup();
    f.ok(&["checkpoint", "--observation", "revalidate-0"]);
    assert!(
        f.ok(&["evidence", "prune", "--apply"])["pruned"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(f.exists(".chough/artifacts/revalidate-0.log"));

    let f = setup();
    fs::remove_file(f.root().join(".chough/artifacts/revalidate-0.log")).unwrap();
    let report = f.ok(&["evidence", "prune", "--apply"]);
    assert!(report["pruned"].as_array().unwrap().is_empty());
    assert_eq!(report["skipped"][0]["id"], "revalidate-0");
    assert!(f.exists(".chough/evidence/revalidate-0.json"));
}

/// Five archived changes (a5..a3 hot, a2 and a1 cold) whose verification manifests list
/// observation records, with a SpecSync stand-in that reports terminal evidence validity.
/// `.stub-break` names a file whose absence makes every archive read `unauthenticated`.
/// `.stub-hook` is a script the status call after the first N (`.stub-hook-after`) runs
/// once, as an editor or another agent could act while the native check runs.
#[cfg(unix)]
fn knowledge_fixture() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(
        "bin/specsync",
        r#"#!/bin/sh
id="$3"
validity=authenticated-history
if [ -f .stub-break ] && [ ! -f "$(cat .stub-break)" ]; then validity=unauthenticated; fi
if [ -f .stub-fail ] && [ "$(cat .stub-fail)" = "$id" ]; then echo "status unavailable" >&2; exit 1; fi
if [ -f .stub-hook ]; then
  # Status reads may run in parallel (REQ-cli-033), so the call count is kept under a lock.
  until mkdir .stub-lock 2>/dev/null; do sleep 0.01; done
  if [ -f .stub-hook ]; then
    calls=$(($(cat .stub-calls 2>/dev/null || echo 0) + 1)); echo "$calls" > .stub-calls
    if [ "$calls" -gt "$(cat .stub-hook-after)" ]; then sh .stub-hook; rm .stub-hook .stub-hook-after .stub-calls; fi
  fi
  rmdir .stub-lock
fi
printf '{"summary":{"id":"%s","state":"archived","terminal_evidence":{"validity":"%s"}},"acceptance_entries":[]}\n' "$id" "$validity"
"#,
    );
    fixture.executable("bin/specsync");
    fixture.write(
        "specs/cli/context.md",
        "# Context\n\nIntro.\n\n## Lessons\n\n- 2026-01-01: a lesson.\n",
    );
    let listing: [(&str, &[&str]); 5] = [
        ("a1", &["cold-a", "shared", "bugref", "n1"]),
        ("a2", &["cold-b", "manual", "cold-c"]),
        ("a3", &[]),
        ("a4", &["shared"]),
        ("a5", &["hot-x"]),
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
        if *id == "a5" {
            // Real cumulative manifests exceed the 64 KiB structured-record bound.
            for index in 0..800 {
                entries.push(json!({"path": format!("docs/padding/{index:04}.md"), "kind": "file", "payload_digest": ZERO}));
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
                "---\nid: {id}\nstate: archived\nbase_commit: {}\n---\n\n# Title of {id}\n\n## Intent\n\nx\n\n## Acceptance Criteria\n\n- {id} works\n- {id} is documented\n\n## Other\n\n- not a criterion\n",
                "3".repeat(40)
            ),
        );
        fixture.write(
            &format!("{dir}/lesson-bundle.md"),
            &format!("# Lesson bundle: {id}\n"),
        );
    }
    fixture.init();
    for (id, outcome, age) in [
        ("n1", "failed", 1),
        ("n2", "passed", 2),
        ("n3", "passed", 3),
    ] {
        write_obs(&fixture, &Obs::run(id, "TC-a", outcome, age));
    }
    write_obs(
        &fixture,
        &Obs::run("cold-a", "TC-a", "passed", 5)
            .with_log(".chough/artifacts/cold-a.log", "cold log\n"),
    );
    for (id, age) in [
        ("cold-b", 6),
        ("hot-x", 7),
        ("shared", 8),
        ("bugref", 9),
        ("plain", 10),
    ] {
        write_obs(&fixture, &Obs::run(id, "TC-a", "passed", age));
    }
    let mut manual = Obs::run("manual", "TC-a", "passed", 11);
    manual.method = Some("manual");
    write_obs(&fixture, &manual);
    // An open bug: since KNOW-22 a closed one no longer holds proof in the trim.
    write_bug(&fixture, "BUG-0001", "open", &["bugref"], None);
    fixture.commit("fixture");
    // Written after the commit, so its bytes are not at HEAD.
    write_obs(&fixture, &Obs::run("cold-c", "TC-a", "passed", 12));
    fixture
}

#[cfg(unix)]
#[test]
fn archive_summary_previews_cold_archive_proof_and_keeps_protections() {
    let fixture = knowledge_fixture();
    let before = tree(fixture.root());
    let preview = fixture.ok(&["library", "archive"]);
    assert_eq!(tree(fixture.root()), before, "the preview wrote");
    assert_eq!(preview["applied"], false);
    assert_eq!(preview["hot"], json!(["a5", "a4", "a3"]));
    assert_eq!(preview["cold"], json!(["a2", "a1"]));
    // The manual observation of cold a2 is a key item: it moves into the vault (KNOW-19).
    assert_eq!(
        ids(&preview["eligible"]),
        set(["cold-a", "cold-b", "manual"]),
        "{preview:#}"
    );
    let manual = preview["eligible"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == "manual")
        .unwrap();
    assert_eq!(
        manual["moves"][".chough/evidence/manual.json"],
        "knowledge/attachments/a2/evidence/manual.json",
        "{manual:#}"
    );
    let cold_a = preview["eligible"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == "cold-a")
        .unwrap();
    assert_eq!(
        cold_a["files"],
        json!([
            ".chough/evidence/cold-a.json",
            ".chough/artifacts/cold-a.log"
        ])
    );
    assert_eq!(cold_a["archives"], json!(["a1"]));
    assert!(cold_a["bytes"].as_u64().unwrap() > 0);
    assert!(reasons(&preview, "shared").contains(&"hot archive a4".to_string()));
    assert!(reasons(&preview, "hot-x").contains(&"hot archive a5".to_string()));
    assert!(
        reasons(&preview, "bugref")
            .iter()
            .any(|reason| reason.starts_with("BUG-0001")),
        "{:?}",
        reasons(&preview, "bugref")
    );
    // The manual observation is no longer protected: it moves (asserted above, KNOW-19).
    assert!(reasons(&preview, "n1").contains(&"newest".to_string()));
    assert_eq!(ids(&preview["uncommitted"]), set(["cold-c"]));
    // An observation no archive lists is left to `evidence prune`.
    assert!(!ids(&preview["protected"]).contains("plain"));
    assert!(!ids(&preview["eligible"]).contains("plain"));
    assert_eq!(preview["totals"]["eligible"], 3);
    // An archive manifest that cannot be read fails closed instead of reading as empty.
    fixture.write(
        ".specsync/archive/changes/2026-05-01-a5/verification.json",
        "{not json",
    );
    let refused = fixture.fails(&["library", "archive"]);
    assert!(
        refused.contains("a5") && refused.contains("verification"),
        "{refused}"
    );
}

#[cfg(unix)]
#[test]
fn archive_summary_apply_is_authorized_ledgered_and_restores_on_validity_change() {
    let fixture = knowledge_fixture();
    let before = tree(fixture.root());
    assert!(
        fixture
            .fails(&["library", "archive", "--apply"])
            .contains("not authorized")
    );
    assert_eq!(tree(fixture.root()), before);
    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: summarize old proof",
    ]);
    fixture.write(".specsync/changes/open-one/change.md", "open\n");
    let refused = fixture.fails(&["library", "archive", "--apply"]);
    assert!(refused.contains("open-one"), "{refused}");
    assert!(fixture.exists(".chough/evidence/cold-a.json"));
    fs::remove_dir_all(fixture.root().join(".specsync/changes")).unwrap();

    // A removal that changes an affected archive's validity is restored and refused.
    fixture.write(".stub-break", ".chough/evidence/cold-a.json");
    let record = fixture.read(".chough/evidence/cold-a.json");
    let log = fixture.read(".chough/artifacts/cold-a.log");
    let broken = fixture.fails(&["library", "archive", "--apply"]);
    assert!(broken.contains("validity"), "{broken}");
    assert_eq!(fixture.read(".chough/evidence/cold-a.json"), record);
    assert_eq!(fixture.read(".chough/artifacts/cold-a.log"), log);
    assert!(fixture.exists(".chough/evidence/cold-b.json"));
    let ledger = fixture.read(".chough/pruned.jsonl");
    assert!(
        ledger.lines().any(|line| line.contains("\"restored\"")),
        "{ledger}"
    );
    fs::remove_file(fixture.root().join(".stub-break")).unwrap();

    let applied = fixture.ok(&["library", "archive", "--apply"]);
    assert_eq!(
        applied["pruned"],
        json!(["cold-a", "cold-b", "manual"]),
        "{applied:#}"
    );
    assert!(!fixture.exists(".chough/evidence/cold-a.json"));
    assert!(!fixture.exists(".chough/artifacts/cold-a.log"));
    assert!(!fixture.exists(".chough/evidence/cold-b.json"));
    // The manual observation moved into the vault instead of being removed (KNOW-19).
    assert!(!fixture.exists(".chough/evidence/manual.json"));
    assert!(fixture.exists("knowledge/attachments/a2/evidence/manual.json"));
    for kept in ["shared", "hot-x", "bugref", "n1", "cold-c", "plain"] {
        assert!(
            fixture.exists(&format!(".chough/evidence/{kept}.json")),
            "{kept}"
        );
    }
    let lines: Vec<Value> = fixture
        .read(".chough/pruned.jsonl")
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .filter(|entry: &Value| entry["event"].is_null())
        .collect();
    let archived: Vec<&Value> = lines
        .iter()
        .filter(|entry| {
            entry["reason"]
                .as_str()
                .unwrap()
                .starts_with("archived-summary")
        })
        .collect();
    assert!(archived.iter().any(|entry| entry["id"] == "cold-a"
        && entry["knowledge"] == "knowledge/archives/a1.md"
        && entry["artifacts"][".chough/artifacts/cold-a.log"] == digest("cold log\n")));
    let entry = fixture.read("knowledge/archives/a1.md");
    assert!(entry.contains(".chough/evidence/cold-a.json"), "{entry}");
    assert!(entry.contains("Git "), "{entry}");
    // A removed observation referenced later reads as pruned, never current or passed.
    fixture.ok(&["checkpoint", "--observation", "cold-a"]);
    let next = fixture.ok(&["next", "--json"]);
    let item = next["observations"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["id"] == "cold-a")
        .cloned()
        .unwrap_or_else(|| panic!("{next:#}"));
    assert_eq!(item["label"], "pruned");
}

#[cfg(unix)]
#[test]
fn knowledge_entries_summarize_each_archive_deterministically() {
    let fixture = knowledge_fixture();
    fixture.write("library/briefs/old.md", "# Old brief\n");
    fixture.write(
        "library/snapshots/2024-01-01-1/manifest.json",
        "{\"schema_version\":1,\"reason\":\"old\",\"revision\":null,\"created_at_unix_seconds\":1,\"entries\":[]}\n",
    );
    let earlier = fixture.read("library/briefs/old.md");
    fixture.ok(&["library", "index"]);
    let first = tree(&fixture.root().join("knowledge"));
    fixture.ok(&["library", "index"]);
    assert_eq!(
        tree(&fixture.root().join("knowledge")),
        first,
        "regeneration differs"
    );
    assert_eq!(fixture.read("library/briefs/old.md"), earlier);
    let index = fixture.read("knowledge/INDEX.md");
    assert!(
        index.contains("[a5](archives/a5.md) Title of a5 (hot)"),
        "{index}"
    );
    assert!(
        index.contains("[a1](archives/a1.md) Title of a1 (cold)"),
        "{index}"
    );
    assert!(index.contains("## Lessons index"), "{index}");
    assert!(
        index.contains("specs/cli/context.md](../specs/cli/context.md) (## Lessons at line 5)"),
        "{index}"
    );
    assert!(index.contains("2026-05-01-a5/lesson-bundle.md"), "{index}");
    assert!(!index.contains("a lesson."), "lesson text copied: {index}");
    assert!(index.contains("## Earlier library folder"), "{index}");
    assert!(index.contains("../library/briefs/old.md"), "{index}");
    assert!(
        index.contains("../library/snapshots/2024-01-01-1/manifest.json"),
        "{index}"
    );
    let entry = fixture.read("knowledge/archives/a1.md");
    for expected in [
        "# a1: Title of a1",
        "- a1 works",
        "- a1 is documented",
        "REQ-cli-001",
        &"1".repeat(40),
        &"2".repeat(40),
        &"3".repeat(40),
        &format!(
            "`.chough/artifacts/cold-a.log` | {} | working tree",
            digest("cold log\n")
        ),
        "| cold-a | passed | local | TC-a |",
        "BUG-0001",
        "2026-01-01-a1/lesson-bundle.md",
        "not a ledger",
    ] {
        assert!(entry.contains(expected), "missing {expected:?} in {entry}");
    }
    assert!(!entry.contains("not a criterion"), "{entry}");
    // Listed with a digest that differs from the file on disk.
    assert!(
        entry.contains("working tree (changed since archive)"),
        "{entry}"
    );
    // No gate reads the knowledge directory.
    let mut with = fixture.ok(&["next", "--json"]);
    fs::remove_dir_all(fixture.root().join("knowledge")).unwrap();
    let mut without = fixture.ok(&["next", "--json"]);
    with.as_object_mut().unwrap().remove("knowledge");
    without.as_object_mut().unwrap().remove("knowledge");
    assert_eq!(with, without);
}

#[cfg(unix)]
#[test]
fn knowledge_entries_stay_bounded_with_cumulative_manifests_and_refusals_write_nothing() {
    let fixture = knowledge_fixture();
    // A cumulative manifest relists earlier proof and adds far more than 64 KiB of paths.
    let path = ".specsync/archive/changes/2026-05-01-a5/verification.json";
    let mut manifest: Value = serde_json::from_str(&fixture.read(path)).unwrap();
    let entries = manifest["acceptance_manifest"]["entries"]
        .as_array_mut()
        .unwrap();
    for shared in ["cold-a", "shared"] {
        entries.push(json!({"path": format!(".chough/evidence/{shared}.json"), "kind": "file", "payload_digest": ZERO}));
    }
    for index in 0..1500 {
        entries.push(json!({"path": format!(".chough/evidence/OBS-padding-cumulative-proof-{index:04}-0123456789abcdef.json"), "kind": "file", "payload_digest": ZERO}));
    }
    fixture.write(path, &serde_json::to_string_pretty(&manifest).unwrap());
    fixture.ok(&["library", "init"]);
    let entry = fixture.read("knowledge/archives/a5.md");
    assert!(entry.len() <= 65_536, "{} bytes", entry.len());
    assert!(
        entry.contains("1501 first listed by this archive"),
        "{entry}"
    );
    assert!(
        entry.contains("not shown; read them from the manifest"),
        "{entry}"
    );
    // Proof an earlier archive first listed is detailed there, not repeated here.
    assert!(!entry.contains("`.chough/evidence/cold-a.json`"), "{entry}");
    assert!(
        fixture
            .read("knowledge/archives/a1.md")
            .contains("`.chough/evidence/cold-a.json`")
    );
    assert!(
        !fixture
            .read("knowledge/archives/a4.md")
            .contains("`.chough/evidence/shared.json`")
    );
    fixture.ok(&[
        "library",
        "snapshot",
        "specs/cli/context.md",
        "--reason",
        "bounded",
    ]);

    // An index that cannot be bounded refuses before anything is written.
    let fresh = knowledge_fixture();
    // `chough init` in the fixture built the folder (REQ-cli-043); start without it.
    fs::remove_dir_all(fresh.root().join("knowledge")).unwrap();
    for (index, id) in ["a1", "a2", "a3", "a4", "a5"].iter().enumerate() {
        let change = format!(
            ".specsync/archive/changes/2026-0{}-01-{id}/change.md",
            index + 1
        );
        let text = fresh.read(&change).replace(
            &format!("# Title of {id}"),
            &format!("# Title of {id} {}", "x".repeat(20_000)),
        );
        fresh.write(&change, &text);
    }
    let refused = fresh.fails(&["library", "init"]);
    assert!(refused.contains("65536"), "{refused}");
    assert!(
        !fresh.exists("knowledge"),
        "a refused init left a partial knowledge folder"
    );
}

#[cfg(unix)]
#[test]
fn archive_summary_apply_fails_closed_without_native_validity_and_rolls_back_partial_removal() {
    use std::os::unix::fs::PermissionsExt;
    let fixture = knowledge_fixture();
    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: summarize old proof",
    ]);
    // SpecSync cannot report a cold archive's validity: refuse, remove and ledger nothing.
    fixture.write(".stub-fail", "a1");
    let record = fixture.read(".chough/evidence/cold-a.json");
    let refused = fixture.fails(&["library", "archive", "--apply"]);
    assert!(
        refused.contains("a1") && refused.contains("validity"),
        "{refused}"
    );
    assert_eq!(fixture.read(".chough/evidence/cold-a.json"), record);
    assert!(fixture.exists(".chough/evidence/cold-b.json"));
    assert!(!fixture.exists(".chough/pruned.jsonl"));
    fs::remove_file(fixture.root().join(".stub-fail")).unwrap();

    // A removal that errors partway restores what it removed and records the restore.
    let artifacts = fixture.root().join(".chough/artifacts");
    fs::set_permissions(&artifacts, fs::Permissions::from_mode(0o555)).unwrap();
    let log = fixture.read(".chough/artifacts/cold-a.log");
    let failed = fixture.fails(&["library", "archive", "--apply"]);
    fs::set_permissions(&artifacts, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(failed.contains("removal failed partway"), "{failed}");
    assert_eq!(fixture.read(".chough/evidence/cold-a.json"), record);
    assert_eq!(fixture.read(".chough/artifacts/cold-a.log"), log);
    assert!(fixture.exists(".chough/evidence/cold-b.json"));
    let ledger = fixture.read(".chough/pruned.jsonl");
    assert!(
        ledger
            .lines()
            .any(|line| line.contains("\"restored\"") && line.contains("cold-a")),
        "{ledger}"
    );
    let preview = fixture.ok(&["library", "archive"]);
    assert_eq!(
        ids(&preview["eligible"]),
        set(["cold-a", "cold-b", "manual"]),
        "{preview:#}"
    );
}

#[cfg(unix)]
#[test]
fn archive_summary_apply_fails_closed_when_a_file_changes_during_the_native_status_check() {
    let fixture = knowledge_fixture();
    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: summarize old proof",
    ]);
    let record = fixture.read(".chough/evidence/cold-a.json");
    let log = fixture.read(".chough/artifacts/cold-a.log");
    // Planning reads the latest archive's manifest with one status call; the next one is
    // the validity check that runs after the plan found every file identical at HEAD.
    fixture.write(
        ".stub-hook",
        "printf 'new uncommitted bytes\\n' >> .chough/artifacts/cold-a.log\n",
    );
    fixture.write(".stub-hook-after", "1\n");
    let refused = fixture.fails(&["library", "archive", "--apply"]);
    assert!(fixture.exists(".chough/artifacts/cold-a.log"), "{refused}");
    assert!(refused.contains("changed"), "{refused}");
    assert!(
        refused.contains(".chough/artifacts/cold-a.log"),
        "{refused}"
    );
    // The bytes written during the check were never committed; they are kept, and
    // nothing else was removed or ledgered.
    assert_eq!(
        fixture.read(".chough/artifacts/cold-a.log"),
        format!("{log}new uncommitted bytes\n")
    );
    assert_eq!(fixture.read(".chough/evidence/cold-a.json"), record);
    assert!(fixture.exists(".chough/evidence/cold-b.json"));
    assert!(!fixture.exists(".chough/pruned.jsonl"));
}

#[cfg(unix)]
#[test]
fn archive_summary_lists_a_crlf_file_differing_from_head_as_uncommitted_under_autocrlf() {
    let fixture = knowledge_fixture();
    let status = std::process::Command::new("git")
        .args(["config", "core.autocrlf", "true"])
        .current_dir(fixture.root())
        .status()
        .unwrap();
    assert!(status.success());
    // Same text with CRLF endings: a clean filter would hash it to HEAD's LF blob.
    let record = fixture.read(".chough/evidence/cold-b.json");
    assert!(!record.contains('\r'));
    fixture.write(
        ".chough/evidence/cold-b.json",
        &record.replace('\n', "\r\n"),
    );
    let preview = fixture.ok(&["library", "archive"]);
    assert_eq!(
        ids(&preview["eligible"]),
        set(["cold-a", "manual"]),
        "{preview:#}"
    );
    assert!(
        preview["uncommitted"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["id"] == "cold-b"),
        "{preview:#}"
    );
}

#[cfg(unix)]
#[test]
fn archive_summary_apply_fails_closed_when_a_protection_appears_during_the_native_status_check() {
    let fixture = knowledge_fixture();
    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: summarize old proof",
    ]);
    // The checkpoint as it would read once it references cold-a.
    let session = fixture.read(".chough/session.json");
    fixture.ok(&["checkpoint", "--observation", "cold-a"]);
    fixture.write(".stub-session", &fixture.read(".chough/session.json"));
    fixture.write(".chough/session.json", &session);
    assert_eq!(
        ids(&fixture.ok(&["library", "archive"])["eligible"]),
        set(["cold-a", "cold-b", "manual"])
    );
    // It starts referencing cold-a during the first validity call, after planning.
    fixture.write(".stub-hook", "cp .stub-session .chough/session.json\n");
    fixture.write(".stub-hook-after", "1\n");
    let record = fixture.read(".chough/evidence/cold-a.json");
    let refused = fixture.fails(&["library", "archive", "--apply"]);
    assert!(
        refused.contains("protections or committed bytes changed"),
        "{refused}"
    );
    assert!(refused.contains("cold-a"), "{refused}");
    assert_eq!(fixture.read(".chough/evidence/cold-a.json"), record);
    assert!(fixture.exists(".chough/artifacts/cold-a.log"));
    assert!(fixture.exists(".chough/evidence/cold-b.json"));
    assert!(!fixture.exists(".chough/pruned.jsonl"));
    assert!(!fixture.exists(".chough/removing"));
}

#[cfg(unix)]
#[test]
fn archive_summary_apply_fails_closed_when_a_protection_appears_after_the_validity_check() {
    let fixture = knowledge_fixture();
    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: summarize old proof",
    ]);
    // The checkpoint as it would read once it references cold-a.
    let session = fixture.read(".chough/session.json");
    fixture.ok(&["checkpoint", "--observation", "cold-a"]);
    fixture.write(".stub-session", &fixture.read(".chough/session.json"));
    fixture.write(".chough/session.json", &session);
    // A Git stand-in that forwards to Git and, on the first unfiltered stdin hash (the
    // HEAD check of the bytes held for removal, after the validity check and its
    // re-plan), makes the checkpoint reference cold-a: another agent's update that
    // changes neither archive validity nor the candidate's bytes.
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
            "#!/bin/sh\nif [ \"$1 $2 $3\" = \"hash-object --no-filters --stdin\" ] && [ -f .stub-session ]; then\n  cp .stub-session .chough/session.json && rm .stub-session\nfi\nexec '{}' \"$@\"\n",
            real_git.trim()
        ),
    );
    fixture.executable("bin/git");
    let record = fixture.read(".chough/evidence/cold-a.json");
    let log = fixture.read(".chough/artifacts/cold-a.log");
    let other = fixture.read(".chough/evidence/cold-b.json");
    let refused = fixture.fails(&["library", "archive", "--apply"]);
    assert!(
        !fixture.exists(".stub-session"),
        "the hook did not run: {refused}"
    );
    assert!(
        refused.contains("protections changed during removal") && refused.contains("cold-a"),
        "{refused}"
    );
    // The checkpoint references cold-a, and every file of every set is still there.
    assert!(fixture.read(".chough/session.json").contains("cold-a"));
    assert_eq!(fixture.read(".chough/evidence/cold-a.json"), record);
    assert_eq!(fixture.read(".chough/artifacts/cold-a.log"), log);
    assert_eq!(fixture.read(".chough/evidence/cold-b.json"), other);
    assert!(!fixture.exists(".chough/removing"));
}

#[cfg(unix)]
#[test]
fn archive_summary_restore_keeps_a_file_recreated_during_the_apply() {
    let fixture = knowledge_fixture();
    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: summarize old proof",
    ]);
    // Removing cold-a.json changes validity, so the apply restores; before it does, a
    // writer recreates cold-a.json with new bytes during the re-read of validity.
    fixture.write(".stub-break", ".chough/evidence/cold-a.json");
    let record = fixture.read(".chough/evidence/cold-a.json");
    let recreated = format!("{record}\n  \n");
    fixture.write(".stub-recreated", &recreated);
    fixture.write(
        ".stub-hook",
        "cp .stub-recreated .chough/evidence/cold-a.json\n",
    );
    // Planning reads one manifest and the first validity check reads a1 and a2.
    fixture.write(".stub-hook-after", "3\n");
    let log = fixture.read(".chough/artifacts/cold-a.log");
    let failed = fixture.fails(&["library", "archive", "--apply"]);
    assert!(failed.contains("validity"), "{failed}");
    assert!(
        failed.contains("not restored because a file was written there meanwhile")
            && failed.contains(".chough/evidence/cold-a.json"),
        "{failed}"
    );
    assert_eq!(fixture.read(".chough/evidence/cold-a.json"), recreated);
    assert_eq!(fixture.read(".chough/artifacts/cold-a.log"), log);
    assert!(fixture.exists(".chough/evidence/cold-b.json"));
    assert!(!fixture.exists(".chough/removing"));
    let ledger = fixture.read(".chough/pruned.jsonl");
    assert!(
        ledger
            .lines()
            .any(|line| line.contains("\"restored\"") && line.contains("written there meanwhile")),
        "{ledger}"
    );
}

#[cfg(unix)]
#[test]
fn a_stale_removal_hold_never_breaks_status_and_blocks_the_next_apply() {
    let fixture = knowledge_fixture();
    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: summarize old proof",
    ]);
    // A hold an interrupted apply left behind sits outside every record directory.
    fixture.write(
        ".chough/removing/0123456789abcdef01234567.hold",
        "held bytes\n",
    );
    let status = fixture.ok(&["status"]);
    assert_eq!(
        status["removal_holds"],
        json!([".chough/removing/0123456789abcdef01234567.hold"]),
        "{status:#}"
    );
    // Every other reader of the record directories still works with the hold present.
    fixture.ok(&["next", "--json"]);
    fixture.ok(&["context", "--json"]);
    fixture.ok(&["timing", "--json"]);
    fixture.ok(&["bug", "list"]);
    fixture.ok(&["evidence", "prune"]);
    let preview = fixture.ok(&["library", "archive"]);
    assert_eq!(preview["stale_holds"], status["removal_holds"]);
    let refused = fixture.fails(&["library", "archive", "--apply"]);
    assert!(refused.contains(".chough/removing"), "{refused}");
    assert!(fixture.exists(".chough/evidence/cold-a.json"));
    assert_eq!(
        fixture.read(".chough/removing/0123456789abcdef01234567.hold"),
        "held bytes\n"
    );
    assert!(!fixture.exists(".chough/pruned.jsonl"));
    // Once the holds are resolved, the apply runs and leaves no hold behind.
    fs::remove_dir_all(fixture.root().join(".chough/removing")).unwrap();
    let applied = fixture.ok(&["library", "archive", "--apply"]);
    assert_eq!(
        applied["pruned"],
        json!(["cold-a", "cold-b", "manual"]),
        "{applied:#}"
    );
    assert!(!fixture.exists(".chough/removing"));
    assert_eq!(fixture.ok(&["status"])["removal_holds"], json!([]));
}

/// Retention protects what every open and archived session references, not only the
/// active one, and fails closed when a session cannot be read.
#[test]
fn retention_protects_what_inactive_and_archived_sessions_reference() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha"]);
    fixture.commit("fixture");
    for (id, age) in [
        ("n1", 20),
        ("n2", 21),
        ("n3", 22),
        ("alpha-old", 40),
        ("beta-old", 41),
        ("gamma-old", 42),
        ("loose", 43),
    ] {
        write_obs(&fixture, &Obs::run(id, "TC-a", "passed", age));
    }
    fixture.ok(&["checkpoint", "--observation", "alpha-old"]);
    fixture.ok(&["session", "new", "gamma"]);
    fixture.ok(&["checkpoint", "--observation", "gamma-old"]);
    // gamma's bytes move to the archive, as a switch archives a done session.
    fs::create_dir_all(fixture.root().join(".chough/sessions/archive")).unwrap();
    fs::rename(
        fixture.root().join(".chough/sessions/gamma.json"),
        fixture.root().join(".chough/sessions/archive/gamma.json"),
    )
    .unwrap();
    fixture.ok(&["session", "new", "beta"]);
    fixture.ok(&["checkpoint", "--observation", "beta-old"]);

    let preview = fixture.ok(&["evidence", "prune"]);
    assert_eq!(ids(&preview["eligible"]), set(["loose"]), "{preview:#}");
    assert_eq!(reasons(&preview, "beta-old"), vec!["checkpoint"]);
    assert_eq!(reasons(&preview, "alpha-old"), vec!["session alpha"]);
    assert_eq!(
        reasons(&preview, "gamma-old"),
        vec!["archived session gamma"]
    );

    fixture.ok(&["consent", "grant", "retention", "--source", "Human: prune"]);
    let applied = fixture.ok(&["evidence", "prune", "--apply"]);
    assert_eq!(applied["pruned"], json!(["loose"]));
    for id in ["alpha-old", "beta-old", "gamma-old"] {
        assert!(
            fixture.exists(&format!(".chough/evidence/{id}.json")),
            "{id}"
        );
    }

    // A session that cannot be read protects everything: nothing is removed.
    write_obs(&fixture, &Obs::run("later", "TC-a", "passed", 44));
    fixture.write(".chough/sessions/delta.json", "not a session\n");
    let before = tree(fixture.root());
    let refused = fixture.fails(&["evidence", "prune", "--apply"]);
    assert!(refused.contains("delta"), "{refused}");
    assert_eq!(tree(fixture.root()), before);
}

/// Issue #98 (REQ-history-009): an observation a subagent reported through an unfolded inbox entry is protected
/// from retention like one the session already holds, so a later fold never adds an
/// observation that retention removed meanwhile.
#[test]
fn retention_protects_observations_named_by_unfolded_inbox_entries() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha"]);
    fixture.commit("fixture");
    for (id, age) in [("n1", 20), ("n2", 21), ("n3", 22), ("reported", 40)] {
        write_obs(&fixture, &Obs::run(id, "TC-a", "passed", age));
    }
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        "alpha",
        "--from",
        "subagent-1",
        "--kind",
        "observation",
        "--text",
        "reported",
    ]);

    let preview = fixture.ok(&["evidence", "prune"]);
    assert!(ids(&preview["eligible"]).is_empty(), "{preview:#}");
    assert_eq!(
        reasons(&preview, "reported"),
        vec!["inbox of session alpha"]
    );

    fixture.ok(&["consent", "grant", "retention", "--source", "Human: prune"]);
    fixture.ok(&["evidence", "prune", "--apply"]);
    assert!(fixture.exists(".chough/evidence/reported.json"));
    fixture.ok(&["inbox", "fold"]);
    assert!(fixture.exists(".chough/evidence/reported.json"));
}

/// #99 review (REQ-history-009): retention refuses while an unfolded inbox entry cannot be
/// read, since it may name an observation, and removes nothing; the same as an unreadable
/// session.
#[test]
fn retention_refuses_while_an_unfolded_inbox_entry_cannot_be_read() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha"]);
    fixture.commit("fixture");
    for (id, age) in [("n1", 20), ("n2", 21), ("n3", 22), ("old", 40)] {
        write_obs(&fixture, &Obs::run(id, "TC-a", "passed", age));
    }
    fixture.ok(&["consent", "grant", "retention", "--source", "Human: prune"]);
    assert_eq!(
        ids(&fixture.ok(&["evidence", "prune"])["eligible"]),
        set(["old"])
    );
    let entry = ".chough/sessions/alpha.inbox/INB-000000000000000000000000.json";
    fixture.write(entry, "{bad");
    let before = tree(fixture.root());
    let refused = fixture.fails(&["evidence", "prune", "--apply"]);
    assert!(
        refused.contains("INB-000000000000000000000000"),
        "{refused}"
    );
    assert_eq!(tree(fixture.root()), before);
}

/// #110: a refused apply removes only the vault copies it made that are still as it wrote
/// them. A copy another writer changed (bytes or mode) after the apply created it is kept
/// as written, and the refusal says so; the original source is restored either way.
#[cfg(unix)]
#[test]
fn a_refused_apply_keeps_vault_copies_another_writer_changed() {
    use std::os::unix::fs::PermissionsExt;
    let attachment = "knowledge/attachments/a2/evidence/manual.json";
    for (hook, expect_kept) in [
        (
            format!("printf 'another writer updated this attachment\\n' > {attachment}"),
            Some("another writer updated this attachment\n"),
        ),
        (format!("chmod 600 {attachment}"), None),
        (String::new(), None),
    ] {
        let fixture = knowledge_fixture();
        fixture.ok(&[
            "consent",
            "grant",
            "retention",
            "--source",
            "Human: summarize old proof",
        ]);
        let source = fixture.read(".chough/evidence/manual.json");
        fixture.write(".stub-break", ".chough/evidence/manual.json");
        if !hook.is_empty() {
            fixture.write(".stub-hook", &format!("{hook}\n"));
            fixture.write(".stub-hook-after", "3\n");
        }
        let refused = fixture.fails(&["library", "archive", "--apply"]);
        assert!(refused.contains("validity"), "{hook}: {refused}");
        assert!(
            !fixture.exists(".stub-hook"),
            "{hook}: the hook did not run: {refused}"
        );
        assert_eq!(
            fixture.read(".chough/evidence/manual.json"),
            source,
            "{hook}"
        );
        match (hook.as_str(), expect_kept) {
            ("", _) => assert!(!fixture.exists(attachment), "an unchanged copy is removed"),
            (_, Some(bytes)) => {
                assert_eq!(fixture.read(attachment), bytes, "{refused}");
                assert!(refused.contains(attachment), "{refused}");
            }
            (_, None) => {
                let mode = fs::metadata(fixture.root().join(attachment))
                    .unwrap_or_else(|_| panic!("a mode change was not kept: {refused}"))
                    .permissions()
                    .mode();
                assert_eq!(mode & 0o777, 0o600);
                assert!(refused.contains(attachment), "{refused}");
            }
        }
    }
}

/// #148: a directory under `.specsync/changes/` with no change files (left empty by a
/// branch switch, which removes tracked files but not directories) is not an open
/// change: the knowledge index does not list it as active, and the trim does not refuse
/// on it. A directory with a change file still counts as open.
#[cfg(unix)]
#[test]
fn an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change() {
    let fixture = knowledge_fixture();
    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: summarize old proof",
    ]);
    fs::create_dir_all(fixture.root().join(".specsync/changes/left-behind/deltas")).unwrap();
    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/INDEX.md");
    assert!(!index.contains("left-behind"), "{index}");
    let applied = fixture.ok(&["library", "archive", "--apply"]);
    assert!(!applied.to_string().contains("left-behind"), "{applied:#}");
    fixture.write(".specsync/changes/really-open/change.md", "open\n");
    let refused = fixture.fails(&["library", "archive", "--apply"]);
    assert!(refused.contains("really-open"), "{refused}");
    assert!(!refused.contains("left-behind"), "{refused}");
}

#[cfg(unix)]
#[test]
fn evidence_prune_restores_partial_removal() {
    use std::os::unix::fs::PermissionsExt;

    let fixture = project();
    for (id, age) in [("keep-a", 20), ("keep-b", 21), ("keep-c", 22)] {
        write_obs(&fixture, &Obs::run(id, "TC-a", "passed", age));
    }
    let log = ".chough/artifacts/partial.log";
    write_obs(
        &fixture,
        &Obs::run("partial", "TC-a", "passed", 40).with_log(log, "uncommitted original\n"),
    );
    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: preserve partial prune evidence",
    ]);
    let record = ".chough/evidence/partial.json";
    let original_record = fixture.read(record);
    fs::set_permissions(fixture.root().join(log), fs::Permissions::from_mode(0o640)).unwrap();
    let evidence = fixture.root().join(".chough/evidence");
    fs::set_permissions(&evidence, fs::Permissions::from_mode(0o555)).unwrap();
    let refused = fixture.fails(&["evidence", "prune", "--apply"]);
    fs::set_permissions(&evidence, fs::Permissions::from_mode(0o755)).unwrap();

    assert!(refused.contains("restored"), "{refused}");
    assert_eq!(fixture.read(record), original_record);
    assert_eq!(fixture.read(log), "uncommitted original\n");
    let mode = fs::metadata(fixture.root().join(log))
        .unwrap()
        .permissions()
        .mode();
    assert_eq!(mode & 0o777, 0o640);
    let ledger = fixture.read(".chough/pruned.jsonl");
    assert!(ledger.contains("\"event\":\"restored\""), "{ledger}");
    assert!(ledger.contains("partial"), "{ledger}");
    assert!(!fixture.exists(".chough/removing"));
}

/// #14 item 3: an absolute artifact path is fingerprinted in place within the
/// bound and never copied into the project; missing and symlinked external
/// paths are refused.
#[test]
fn an_external_artifact_is_fingerprinted_in_place() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let external =
        std::env::temp_dir().join(format!("chough-14-external-{}.png", std::process::id()));
    std::fs::write(&external, b"png-bytes").unwrap();
    let added = fixture.ok(&[
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "human-observation",
        "--outcome",
        "passed",
        "--environment",
        "test-local",
        "--summary",
        "screenshot checked",
        "--artifact",
        external.to_str().unwrap(),
    ]);
    let record = added["recorded"].as_str().unwrap().to_string();
    let observation: Value = serde_json::from_str(&fixture.read(&record)).unwrap();
    // The committed record holds a neutral ID and the fingerprint, never the path,
    // which can name a person or a machine; the path stays machine-local (#14).
    let key = observation["artifacts"][0].as_str().unwrap().to_string();
    assert!(
        key.starts_with("external:") && key.len() == 25,
        "{observation:#}"
    );
    let digest = observation["artifact_fingerprints"][&key].as_str().unwrap();
    assert_eq!(digest.len(), 64, "{observation:#}");
    assert!(
        digest.starts_with(&key["external:".len()..]),
        "{observation:#}"
    );
    let bytes = fixture.read(&record);
    let home = std::env::var("HOME").unwrap_or_default();
    for leak in [
        external.to_str().unwrap(),
        "chough-14-external",
        home.as_str(),
    ] {
        assert!(
            leak.is_empty() || !bytes.contains(leak),
            "{leak} in {bytes}"
        );
    }
    let map: Value =
        serde_json::from_str(&fixture.read(".chough/local/external-artifacts.json")).unwrap();
    assert_eq!(map[&key], serde_json::json!(external.to_str().unwrap()));
    assert_eq!(fixture.read(".chough/local/.gitignore").trim(), "*");
    // Nothing was copied into the project.
    assert!(!fixture.exists(".chough/artifacts"));

    // A missing external path is refused with its name.
    let missing = external.with_file_name("chough-14-missing.png");
    let refused = fixture.fails(&[
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "human-observation",
        "--outcome",
        "passed",
        "--environment",
        "test-local",
        "--summary",
        "screenshot checked",
        "--artifact",
        missing.to_str().unwrap(),
    ]);
    assert!(refused.contains("missing artifact"), "{refused}");

    // A symlink, a directory and a file over the 64 MiB bound are refused by name.
    let add_external = |path: &std::path::Path| {
        fixture.fails(&[
            "evidence",
            "add",
            "--criterion",
            "LIFE-1",
            "--requirement",
            "REQ-cli-001",
            "--contract",
            "specs/cli/cli.spec.md",
            "--kind",
            "human-observation",
            "--outcome",
            "passed",
            "--environment",
            "test-local",
            "--summary",
            "screenshot checked",
            "--artifact",
            path.to_str().unwrap(),
        ])
    };
    let directory = external.with_file_name(format!("chough-14-dir-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let refused = add_external(&directory);
    assert!(refused.contains("not a plain file"), "{refused}");
    #[cfg(unix)]
    {
        let link = external.with_file_name(format!("chough-14-link-{}.png", std::process::id()));
        let _ = std::fs::remove_file(&link);
        std::os::unix::fs::symlink(&external, &link).unwrap();
        let refused = add_external(&link);
        assert!(refused.contains("not a plain file"), "{refused}");
        std::fs::remove_file(&link).unwrap();
    }
    let oversized = external.with_file_name(format!("chough-14-big-{}.bin", std::process::id()));
    std::fs::File::create(&oversized)
        .unwrap()
        .set_len(64 * 1_048_576 + 1)
        .unwrap();
    let refused = add_external(&oversized);
    assert!(refused.contains("64 MiB"), "{refused}");
    std::fs::remove_file(&oversized).unwrap();
    std::fs::remove_dir(&directory).unwrap();

    // Freshness re-reads the external file in place: changing it makes the
    // observation stop reading current.
    let id = observation["id"].as_str().unwrap().to_string();
    fixture.ok(&["checkpoint", "--observation", &id]);
    let label = |fixture: &Fixture| {
        fixture.ok(&["status", "--environment", "test-local"])["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["record"]["id"] == id)
            .map(|entry| entry["label"].as_str().unwrap().to_string())
    };
    assert_eq!(label(&fixture).as_deref(), Some("current"));
    std::fs::write(&external, b"different-bytes").unwrap();
    assert_eq!(label(&fixture).as_deref(), Some("historical"));
    // On a machine without the mapping (or without the file), the observation is
    // external and not checkable here, neither current nor stale.
    std::fs::write(&external, b"png-bytes").unwrap();
    assert_eq!(label(&fixture).as_deref(), Some("current"));
    std::fs::remove_file(fixture.root().join(".chough/local/external-artifacts.json")).unwrap();
    assert_eq!(
        label(&fixture).as_deref(),
        Some("external-not-checkable-here")
    );
    std::fs::remove_file(&external).unwrap();
}

/// #14: pruning an eligible observation whose artifacts include an external one removes
/// its record and its `.chough/artifacts` log, and never touches the external file; the
/// pre-removal recheck does not treat the neutral external ID as a missing project file.
#[test]
fn prune_apply_never_removes_an_external_artifact() {
    let fixture = project();
    for (id, age) in [("n1", 20), ("n2", 21), ("n3", 22)] {
        write_obs(&fixture, &Obs::run(id, "TC-a", "passed", age));
    }
    write_obs(
        &fixture,
        &Obs::run("old", "TC-a", "passed", 40).with_log(".chough/artifacts/old.log", "old log\n"),
    );
    let external = std::env::temp_dir().join(format!("chough-14-prune-{}.png", std::process::id()));
    std::fs::write(&external, "screenshot").unwrap();
    let id = format!("external:{}", &digest("screenshot")[..16]);
    let path = ".chough/evidence/old.json";
    let mut record: Value = serde_json::from_str(&fixture.read(path)).unwrap();
    record["artifacts"].as_array_mut().unwrap().push(json!(id));
    record["artifact_fingerprints"][&id] = json!(digest("screenshot"));
    fixture.write(
        path,
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
    fixture.write(
        ".chough/local/external-artifacts.json",
        &json!({ id.clone(): external.to_str().unwrap() }).to_string(),
    );
    fixture.commit("an old run with an external artifact");

    let preview = fixture.ok(&["evidence", "prune"]);
    assert_eq!(ids(&preview["eligible"]), set(["old"]), "{preview:#}");
    fixture.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "Human: prune old routine logs",
    ]);
    let applied = fixture.ok(&["evidence", "prune", "--apply"]);
    assert_eq!(applied["pruned"], json!(["old"]), "{applied:#}");
    assert!(!fixture.exists(path));
    assert!(!fixture.exists(".chough/artifacts/old.log"));
    assert_eq!(std::fs::read_to_string(&external).unwrap(), "screenshot");
    std::fs::remove_file(&external).unwrap();
}
