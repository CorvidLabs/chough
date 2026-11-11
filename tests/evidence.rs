//! Reference-aware evidence retention and observation labels (REQ-history-001).
//! the age rules are exercised without waiting.

mod common;

use common::Fixture;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
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

/// REQ-records-009, REQ-cli-012, LIFE-8: an observation the 0.6.0 migration did not
/// carry reads `removed`, naming the migration's receipt; an ID no record ever held reads
/// `unavailable`; neither ever counts as current, passed or verifying evidence. On a
/// 0.5.x project the open session references `n1`, which becomes its entry and reads
/// stale; nothing references `old-pass`, so the migration removes it; and a bug links
/// `old-fail`, which no file holds any more. After the migration the checkpoint
/// references `old-pass` and `never-recorded`.
#[test]
fn removed_and_missing_observations_are_labeled_and_never_count() {
    fn copy(from: &std::path::Path, to: &std::path::Path) {
        fs::create_dir_all(to).unwrap();
        for entry in fs::read_dir(from).unwrap().flatten() {
            let target = to.join(entry.file_name());
            if entry.path().is_dir() {
                copy(&entry.path(), &target);
            } else {
                fs::copy(entry.path(), target).unwrap();
            }
        }
    }
    let fixture = Fixture::new();
    let release = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/releases/v0.5.0/.chough");
    for folder in ["sessions", "history", "bugs"] {
        copy(
            &release.join(folder),
            &fixture.root().join(".chough").join(folder),
        );
    }
    write_obs(&fixture, &Obs::run("n1", "TC-a", "passed", 20));
    write_obs(&fixture, &Obs::run("old-pass", "TC-a", "passed", 41));
    let session = ".chough/sessions/fixture-first.json";
    let mut checkpoint: Value = serde_json::from_str(&fixture.read(session)).unwrap();
    checkpoint["observations"] = json!(["n1"]);
    fixture.write(session, &serde_json::to_string_pretty(&checkpoint).unwrap());
    let bug_file = fs::read_dir(fixture.root().join(".chough/bugs"))
        .unwrap()
        .flatten()
        .map(|entry| format!(".chough/bugs/{}", entry.file_name().to_string_lossy()))
        .find(|name| name.ends_with(".json"))
        .unwrap();
    let mut bug: Value = serde_json::from_str(&fixture.read(&bug_file)).unwrap();
    bug["observations"] = json!(["old-fail"]);
    fixture.write(&bug_file, &serde_json::to_string_pretty(&bug).unwrap());
    let old_bug = bug["id"].as_str().unwrap().to_string();
    fixture.commit("a 0.5.x project");
    let plan = fixture.ok(&["migrate"]);
    assert_eq!(plan["aliases"]["old-pass"], "removed", "{plan:#}");
    assert_eq!(plan["aliases"]["old-fail"], "removed", "{plan:#}");
    let digest = plan["plan_digest"].as_str().unwrap().to_string();
    let applied = fixture.ok(&[
        "migrate",
        "--apply",
        &digest,
        "--consent",
        "SYNTHETIC-human said migrate it (fixture)",
    ]);
    let receipt = applied["receipt"].as_str().unwrap().to_string();
    fixture.commit("the migrated records");

    // References written after the migration.
    fixture.ok(&[
        "checkpoint",
        "--observation",
        "old-pass",
        "--observation",
        "never-recorded",
    ]);

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
    assert_eq!(listed["old-pass"], "removed", "{next}");
    assert_eq!(listed["never-recorded"], "unavailable", "{next}");
    assert_eq!(listed["n1"], "stale", "{next}");
    assert!(listed.values().all(|label| label != "current"), "{next}");
    let report = fixture.ok(&["status", "--environment", "local"]);
    let references = report["observation_references"].as_array().unwrap();
    let label = |id: &str| {
        references
            .iter()
            .find(|entry| entry["id"] == id)
            .unwrap_or_else(|| panic!("{id} missing from {references:?}"))["label"]
            .clone()
    };
    assert_eq!(label("old-pass"), "removed");
    assert_eq!(label("never-recorded"), "unavailable");
    assert_eq!(label("n1"), "stale");
    for observation in report["observations"].as_array().unwrap() {
        assert!(
            !observation["record"]["aliases"]
                .to_string()
                .contains("old-pass"),
            "{observation:#}"
        );
        assert_ne!(observation["label"], "current", "{observation:#}");
    }

    // The bug shows the link it lost as removed, naming the receipt, and never verifies.
    let shown = fixture.ok(&["bug", "show", &old_bug]);
    let lost = shown["observation_labels"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == "old-fail")
        .unwrap_or_else(|| panic!("{shown:#}"))
        .clone();
    assert_eq!(lost["label"], "removed", "{shown:#}");
    assert_eq!(lost["receipt"], receipt.as_str(), "{shown:#}");
    assert_eq!(shown["verification"], "unverified", "{shown:#}");
    let refused = fixture.fails(&["bug", "update", &old_bug, "--status", "verified"]);
    assert!(refused.contains("passed observation"), "{refused}");
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
