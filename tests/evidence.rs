//! Reference-aware evidence retention and observation labels (REQ-cli-018).
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
    let mut before = before;
    before.remove(".chough/session.json");
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
            "spelled",
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
