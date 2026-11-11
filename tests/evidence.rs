//! Reference-aware evidence retention and observation labels (REQ-history-001).
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
    fixture.bug_md(id, "in-scope", status, observations);
    if let Some(baseline) = baseline {
        let name = format!(".chough/bugs/{id}.md");
        fixture.write(
            &name,
            &fixture.read(&name).replace(
                "receipts: []\n",
                &format!("receipts: []\nregression_baseline: {baseline}\n"),
            ),
        );
    }
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


fn project() -> Fixture {
    let fixture = Fixture::new();
    fixture.init();
    fixture.commit("fixture");
    fixture
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
    // An observation that is no longer present reads missing (REQ-cli-012).
    assert_eq!(labels["old-pass"], "missing");
    assert_eq!(labels["old-fail"], "missing");
    assert_eq!(labels["never-recorded"], "missing");
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
    assert_eq!(label(&fixture).as_deref(), Some("current"));
}


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

#[test]
fn an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change() {
    let fixture = knowledge_fixture();
    fixture.ok(&["library", "index"]);
    let before = fixture.read("knowledge/index.md");
    fs::create_dir_all(fixture.root().join(".specsync/changes/left-behind/deltas")).unwrap();
    fixture.ok(&["library", "index", "--check"]);
    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/index.md");
    assert_eq!(index, before);
    assert!(!index.contains("left-behind"), "{index}");
    let next = fixture.ok(&["next", "--json"]);
    assert!(!next.to_string().contains("left-behind"), "{next:#}");
    fixture.ok(&["gate", "list", "--json"]);
}
