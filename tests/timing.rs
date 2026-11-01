//! The read-only timing report and the gate records it reads (REQ-cli-020).
//!
//! Fixtures write transitions, observations, bugs and validation runs with fixed
//! timestamps, so every expected segment, cycle and duration is exact.

mod common;

use common::Fixture;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// A fixed time in the past, so ongoing segments end now, after it.
const T: u64 = 1_700_000_000;
const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn transition(at: Option<u64>, kind: &str, from: &str, to: &str, reason: &str) -> String {
    let mut record = json!({
        "schema_version": 1,
        "kind": kind,
        "from": from,
        "to": to,
        "reason": reason,
        "gate": null,
        "revision": null,
        "refresh": [],
    });
    if let Some(at) = at {
        record["at_unix_seconds"] = json!(at);
    }
    record.to_string()
}

fn write_transitions(fixture: &Fixture, lines: &[String]) {
    fixture.write(".chough/transitions.jsonl", &(lines.join("\n") + "\n"));
}

/// Every stage-moving kind, with one gate opened and closed during build.
fn full_history(with_gates: bool) -> Vec<String> {
    let mut lines = vec![
        transition(Some(T), "advance", "intent", "design", "intent approved"),
        transition(Some(T + 100), "skip", "design", "plan", "CLI change"),
        transition(Some(T + 300), "advance", "plan", "build", "scope approved"),
        transition(Some(T + 400), "iterate", "build", "build", "second pass"),
        transition(Some(T + 600), "pause", "build", "build", "lunch"),
        transition(Some(T + 900), "resume", "build", "build", ""),
        transition(Some(T + 1000), "enter", "build", "bugs", "a failure"),
        transition(Some(T + 1100), "return", "bugs", "build", "recorded"),
        transition(Some(T + 1200), "back", "build", "plan", "rework"),
        transition(Some(T + 1250), "advance", "plan", "build", "replanned"),
        transition(Some(T + 1300), "advance", "build", "test", "built"),
        transition(Some(T + 1400), "advance", "test", "accept", "tested"),
        transition(Some(T + 1500), "advance", "accept", "done", "archived"),
        transition(Some(T + 2500), "reset", "done", "plan", "next objective"),
        transition(Some(T + 2600), "reset", "plan", "intent", "start over"),
    ];
    if with_gates {
        lines.insert(
            3,
            transition(Some(T + 350), "gate-opened", "build", "build", "Review"),
        );
        lines.insert(
            5,
            transition(Some(T + 450), "gate-closed", "build", "build", "Review"),
        );
    }
    lines
}

fn fixture_with(lines: &[String]) -> Fixture {
    let fixture = Fixture::new();
    fixture.init();
    write_transitions(&fixture, lines);
    fixture
}

fn timing(fixture: &Fixture, extra: &[&str]) -> Value {
    let mut args = vec!["timing", "--json"];
    args.extend_from_slice(extra);
    fixture.ok(&args)
}

fn text(fixture: &Fixture, extra: &[&str]) -> String {
    let mut args = vec!["timing"];
    args.extend_from_slice(extra);
    let output = fixture.run(&args);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn seconds(duration: &Value) -> Option<u64> {
    duration["seconds"].as_u64()
}

fn at(time: &Value) -> Option<u64> {
    time["unix_seconds"].as_u64()
}

fn segments(report: &Value, cycle: usize) -> Vec<Value> {
    report["cycles"][cycle]["segments"]
        .as_array()
        .unwrap()
        .clone()
}

fn stage_total<'a>(report: &'a Value, cycle: usize, stage: &str) -> &'a Value {
    report["cycles"][cycle]["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["stage"] == stage)
        .unwrap_or_else(|| panic!("no {stage} in cycle {cycle}: {report:#}"))
}

/// Digests of every file under `.chough/` and `.specsync/`.
fn record_digests(root: &Path) -> BTreeMap<String, String> {
    fn walk(base: &Path, path: &Path, digests: &mut BTreeMap<String, String>) {
        if !path.exists() {
            return;
        }
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(base, &path, digests);
            } else {
                digests.insert(
                    path.strip_prefix(base).unwrap().display().to_string(),
                    format!("{:x}", Sha256::digest(fs::read(&path).unwrap())),
                );
            }
        }
    }
    let mut digests = BTreeMap::new();
    walk(root, &root.join(".chough"), &mut digests);
    walk(root, &root.join(".specsync"), &mut digests);
    digests
}

#[test]
fn every_transition_kind_yields_the_expected_segments_and_cycles() {
    let fixture = fixture_with(&full_history(false));
    let report = timing(&fixture, &[]);
    assert_eq!(report["schema_version"], 1);

    // init and migrate write no transition, so the first segment's start is unknown.
    let first = &report["before_first_record"];
    assert_eq!(first["stage"], "intent");
    assert_eq!(first["start"]["unix_seconds"], Value::Null);
    assert_eq!(
        first["start"]["unknown"],
        "no transition before the first record"
    );
    assert_eq!(at(&first["end"]), Some(T));
    assert_eq!(first["elapsed"]["seconds"], Value::Null);
    assert_eq!(first["elapsed"]["label"], "unknown");

    // Every reset starts a cycle; back does not.
    let cycles = report["cycles"].as_array().unwrap();
    assert_eq!(cycles.len(), 3, "{report:#}");
    assert_eq!(at(&cycles[0]["start"]), Some(T));
    assert_eq!(at(&cycles[1]["start"]), Some(T + 2500));
    assert_eq!(at(&cycles[2]["start"]), Some(T + 2600));
    assert_eq!(cycles[0]["completed"], true);
    assert_eq!(cycles[1]["completed"], true);
    assert_eq!(cycles[2]["completed"], false);

    let expected: [(&str, &str, u64, u64); 13] = [
        ("design", "active", 0, 100),
        ("plan", "active", 100, 300),
        ("build", "active", 300, 400),
        ("build", "active", 400, 600),
        ("build", "paused", 600, 900),
        ("build", "active", 900, 1000),
        ("bugs", "active", 1000, 1100),
        ("build", "active", 1100, 1200),
        ("plan", "active", 1200, 1250),
        ("build", "active", 1250, 1300),
        ("test", "active", 1300, 1400),
        ("accept", "active", 1400, 1500),
        ("done", "idle", 1500, 2500),
    ];
    let first_cycle = segments(&report, 0);
    assert_eq!(first_cycle.len(), expected.len(), "{report:#}");
    for (segment, (stage, state, start, end)) in first_cycle.iter().zip(expected) {
        assert_eq!(segment["stage"], stage, "{segment:#}");
        assert_eq!(segment["state"], state, "{segment:#}");
        assert_eq!(at(&segment["start"]), Some(T + start), "{segment:#}");
        assert_eq!(at(&segment["end"]), Some(T + end), "{segment:#}");
        assert_eq!(seconds(&segment["elapsed"]), Some(end - start));
        assert_eq!(segment["elapsed"]["label"], "elapsed wall-clock");
    }
    assert_eq!(first_cycle[3]["entered_by"], "iterate");
    assert_eq!(first_cycle[3]["iteration"], 2);
    assert_eq!(first_cycle[5]["iteration"], 2);
    assert_eq!(first_cycle[6]["side"], true);
    assert_eq!(first_cycle[8]["entered_by"], "back");

    // Paused time is shown on its own and excluded from active time; done is idle.
    let build = stage_total(&report, 0, "build");
    assert_eq!(build["visits"], 3, "{build:#}");
    assert_eq!(build["iterations"], 1);
    assert_eq!(seconds(&build["stage_time"]), Some(550));
    assert_eq!(seconds(&build["active"]), Some(550));
    assert_eq!(seconds(&build["paused"]), Some(300));
    assert_eq!(seconds(&build["waiting"]), Some(0));
    assert_eq!(stage_total(&report, 0, "plan")["visits"], 2);
    assert_eq!(
        seconds(&stage_total(&report, 0, "plan")["stage_time"]),
        Some(250)
    );
    assert_eq!(stage_total(&report, 0, "bugs")["side"], true);
    assert_eq!(cycles[0]["side_visits"], 1);
    assert_eq!(cycles[0]["iterations"], 1);
    assert_eq!(seconds(&cycles[0]["idle"]), Some(1000));
    assert!(
        cycles[0]["stages"]
            .as_array()
            .unwrap()
            .iter()
            .all(|entry| entry["stage"] != "done"),
        "idle done time is not stage time: {report:#}"
    );

    // The open segment ends now and is marked ongoing.
    let last = &segments(&report, 2)[0];
    assert_eq!(last["stage"], "intent");
    assert_eq!(last["ongoing"], true);
    assert_eq!(last["elapsed"]["label"], "ongoing");
    assert!(seconds(&last["elapsed"]).unwrap() > 0);

    // Filters narrow the report without changing derivation.
    let current = timing(&fixture, &["--cycle", "current"]);
    assert_eq!(current["cycles"].as_array().unwrap().len(), 1);
    assert_eq!(current["cycles"][0]["number"], 3);
    let plan = timing(&fixture, &["--stage", "plan"]);
    assert!(
        segments(&plan, 0)
            .iter()
            .all(|segment| segment["stage"] == "plan")
    );
    assert_eq!(segments(&plan, 0).len(), 2);

    let rendered = text(&fixture, &[]);
    assert!(
        rendered.contains("elapsed wall-clock time, not effort"),
        "{rendered}"
    );
    assert!(rendered.contains("idle between cycles"), "{rendered}");
    assert!(rendered.contains("unknown (no transition before the first record)"));
}

#[test]
fn gate_records_start_or_end_no_segment_and_give_waiting_time() {
    let plain = timing(&fixture_with(&full_history(false)), &[]);
    let gated = timing(&fixture_with(&full_history(true)), &[]);
    let boundaries = |report: &Value| -> Vec<(Value, Value, Value, Value)> {
        report["cycles"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|cycle| cycle["segments"].as_array().unwrap().clone())
            .filter(|segment| segment["ongoing"] != true)
            .map(|segment| {
                (
                    segment["stage"].clone(),
                    segment["start"].clone(),
                    segment["end"].clone(),
                    segment["iteration"].clone(),
                )
            })
            .collect()
    };
    assert_eq!(boundaries(&plain), boundaries(&gated));
    assert_eq!(
        stage_total(&plain, 0, "build")["visits"],
        stage_total(&gated, 0, "build")["visits"]
    );

    // The gate was open from T+350 to T+450, across two build segments.
    let gates = gated["gates"].as_array().unwrap();
    assert_eq!(gates.len(), 1, "{gated:#}");
    assert_eq!(gates[0]["gate"], "Review");
    assert_eq!(gates[0]["state"], "closed");
    assert_eq!(at(&gates[0]["opened"]), Some(T + 350));
    assert_eq!(at(&gates[0]["closed"]), Some(T + 450));
    assert_eq!(seconds(&gates[0]["waiting"]), Some(100));
    let first = segments(&gated, 0);
    assert_eq!(seconds(&first[2]["waiting"]), Some(50));
    assert_eq!(seconds(&first[2]["active"]), Some(50));
    assert_eq!(seconds(&first[3]["waiting"]), Some(50));
    assert_eq!(seconds(&first[3]["active"]), Some(150));
    let build = stage_total(&gated, 0, "build");
    assert_eq!(seconds(&build["stage_time"]), Some(550));
    assert_eq!(seconds(&build["waiting"]), Some(100));
    assert_eq!(seconds(&build["active"]), Some(450));
    assert_eq!(seconds(&build["paused"]), Some(300));
}

#[test]
fn checkpoint_gates_append_one_record_when_opened_and_one_when_cleared() {
    let fixture = Fixture::new();
    fixture.init();
    let records = |fixture: &Fixture| -> Vec<Value> {
        if !fixture.exists(".chough/transitions.jsonl") {
            return vec![];
        }
        fixture
            .read(".chough/transitions.jsonl")
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    };
    fixture.ok(&["checkpoint", "--awaiting-human", "Scope approval of x"]);
    let after_open = records(&fixture);
    assert_eq!(after_open.len(), 1, "{after_open:#?}");
    let opened = &after_open[0];
    assert_eq!(opened["kind"], "gate-opened");
    assert_eq!(opened["from"], "intent");
    assert_eq!(opened["to"], "intent");
    assert_eq!(opened["reason"], "Scope approval of x");
    assert_eq!(opened["gate"], Value::Null);
    assert!(opened["at_unix_seconds"].as_u64().is_some());
    assert!(opened.as_object().unwrap().contains_key("revision"));
    // The record says when, never who decided or what was decided.
    let keys: Vec<&String> = opened.as_object().unwrap().keys().collect();
    assert_eq!(
        keys,
        vec![
            "at_unix_seconds",
            "from",
            "gate",
            "kind",
            "reason",
            "refresh",
            "revision",
            "schema_version",
            "to"
        ]
    );

    // Reopening a gate that is already open appends nothing.
    fixture.ok(&["checkpoint", "--awaiting-human", "Scope approval of x"]);
    assert_eq!(records(&fixture).len(), 1);
    // Other checkpoint updates append nothing.
    fixture.ok(&["checkpoint", "--next-action", "Wait"]);
    assert_eq!(records(&fixture).len(), 1);

    fixture.ok(&["checkpoint", "--awaiting-human", "Second gate"]);
    fixture.ok(&["checkpoint", "--clear-awaiting-human"]);
    let kinds: Vec<(String, String)> = records(&fixture)
        .iter()
        .map(|record| {
            (
                record["kind"].as_str().unwrap().to_string(),
                record["reason"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(
        kinds,
        vec![
            ("gate-opened".into(), "Scope approval of x".into()),
            ("gate-opened".into(), "Second gate".into()),
            ("gate-closed".into(), "Scope approval of x".into()),
            ("gate-closed".into(), "Second gate".into()),
        ]
    );

    // Clearing with nothing open appends nothing; clear-and-reopen writes closed then opened.
    fixture.ok(&["checkpoint", "--clear-awaiting-human"]);
    assert_eq!(records(&fixture).len(), 4);
    fixture.ok(&["checkpoint", "--awaiting-human", "Review"]);
    fixture.ok(&[
        "checkpoint",
        "--clear-awaiting-human",
        "--awaiting-human",
        "Review",
    ]);
    let tail: Vec<String> = records(&fixture)[5..]
        .iter()
        .map(|record| record["kind"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(tail, vec!["gate-closed", "gate-opened"]);
    assert_eq!(fixture.session()["awaiting_human"], json!(["Review"]));

    // The report pairs them; the reopened gate is still open with elapsed-so-far.
    let report = timing(&fixture, &[]);
    let gates = report["gates"].as_array().unwrap();
    assert_eq!(gates.len(), 4, "{report:#}");
    assert!(gates[..3].iter().all(|gate| gate["state"] == "closed"));
    assert!(
        gates[..3]
            .iter()
            .all(|gate| gate["waiting"]["label"] != "unknown")
    );
    assert_eq!(gates[3]["gate"], "Review");
    assert_eq!(gates[3]["state"], "open");
    assert_eq!(gates[3]["waiting"]["label"], "ongoing");

    // Existing readers still parse a log that holds gate records.
    let context = fixture.ok(&["context", "--json"]);
    assert!(context["sections"].to_string().contains("gate-opened"));
    fixture.ok(&["next", "--json"]);
    let shown = fixture.ok(&["stage", "show"]);
    assert_eq!(shown["recent_transitions"].as_array().unwrap().len(), 7);
}

#[test]
fn gate_records_are_reconciled_with_the_awaiting_human_list() {
    let fixture = fixture_with(&[
        transition(Some(T), "advance", "intent", "design", "go"),
        transition(
            Some(T + 10),
            "gate-opened",
            "design",
            "design",
            "Edited away",
        ),
        transition(
            Some(T + 20),
            "gate-opened",
            "design",
            "design",
            "Still waiting",
        ),
        transition(
            Some(T + 30),
            "gate-closed",
            "design",
            "design",
            "Closed only",
        ),
        transition(Some(T + 100), "advance", "design", "plan", "go"),
    ]);
    // "Edited away" was removed from session.json by hand; "Legacy" predates gate records.
    let mut session = fixture.session();
    session["awaiting_human"] = json!(["Still waiting", "Legacy"]);
    fixture.write(
        ".chough/session.json",
        &serde_json::to_string_pretty(&session).unwrap(),
    );
    let report = timing(&fixture, &[]);
    let gate = |name: &str| -> Value {
        report["gates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|gate| gate["gate"] == name)
            .unwrap_or_else(|| panic!("no gate {name}: {report:#}"))
            .clone()
    };
    let edited = gate("Edited away");
    assert_eq!(edited["state"], "closed");
    assert_eq!(at(&edited["opened"]), Some(T + 10));
    assert_eq!(edited["closed"]["unix_seconds"], Value::Null);
    assert_eq!(edited["closed"]["unknown"], "cleared without a gate record");
    assert_eq!(edited["waiting"]["label"], "unknown");

    let waiting = gate("Still waiting");
    assert_eq!(waiting["state"], "open");
    assert_eq!(at(&waiting["opened"]), Some(T + 20));
    assert_eq!(waiting["waiting"]["label"], "ongoing");

    for name in ["Legacy", "Closed only"] {
        let unknown = gate(name);
        assert_eq!(
            unknown["opened"]["unix_seconds"],
            Value::Null,
            "{unknown:#}"
        );
        assert_eq!(
            unknown["opened"]["unknown"],
            "opened before gate records or record missing"
        );
        assert_eq!(unknown["waiting"]["label"], "unknown");
    }
    assert_eq!(gate("Legacy")["state"], "open");
    assert_eq!(gate("Closed only")["state"], "closed");
    assert_eq!(at(&gate("Closed only")["closed"]), Some(T + 30));

    // A gate with an unknown end leaves the affected active time unknown, not inflated.
    let design = &segments(&report, 0)[0];
    assert_eq!(design["stage"], "design");
    assert_eq!(seconds(&design["elapsed"]), Some(100));
    assert_eq!(design["active"]["label"], "unknown", "{design:#}");
    assert_eq!(design["waiting"]["label"], "unknown");
}

#[test]
fn missing_and_out_of_order_times_are_unknown_never_zero() {
    let fixture = fixture_with(&[
        transition(Some(T), "advance", "intent", "design", "go"),
        transition(None, "advance", "design", "plan", "no time"),
        transition(Some(T + 500), "advance", "plan", "build", "go"),
        transition(Some(T + 400), "advance", "build", "test", "clock went back"),
        transition(Some(T + 400), "advance", "test", "accept", "same second"),
        transition(Some(T + 900), "reset", "accept", "intent", "again"),
    ]);
    fixture.write(
        ".chough/transitions.jsonl",
        &(fixture.read(".chough/transitions.jsonl") + "not json at all\n"),
    );
    let report = timing(&fixture, &[]);
    let first = segments(&report, 0);
    // design ends at an unknown time, and plan starts at one.
    assert_eq!(first[0]["stage"], "design");
    assert_eq!(first[0]["end"]["unknown"], "missing timestamp");
    assert_eq!(first[0]["elapsed"]["seconds"], Value::Null);
    assert_eq!(first[0]["elapsed"]["label"], "unknown");
    assert_eq!(first[1]["stage"], "plan");
    assert_eq!(first[1]["start"]["unknown"], "missing timestamp");
    assert_eq!(first[1]["elapsed"]["seconds"], Value::Null);
    // build ends before it starts.
    assert_eq!(first[2]["stage"], "build");
    assert_eq!(first[2]["elapsed"]["seconds"], Value::Null);
    assert_eq!(first[2]["elapsed"]["reason"], "end before start");
    // test starts and ends in the same second: recorded together, not instant work.
    assert_eq!(first[3]["stage"], "test");
    assert_eq!(seconds(&first[3]["elapsed"]), Some(0));
    assert_eq!(first[3]["elapsed"]["label"], "recorded together");
    // Stage totals that include an unknown segment are unknown, never zero.
    assert_eq!(
        stage_total(&report, 0, "design")["stage_time"]["label"],
        "unknown"
    );
    assert_eq!(
        stage_total(&report, 0, "build")["stage_time"]["label"],
        "unknown"
    );
    // The unreadable line is skipped and reported.
    assert_eq!(report["transitions"]["unreadable_lines"], 1);
    assert!(report["partial"].to_string().contains("unreadable"));
    // One completed cycle is too few to estimate.
    let accept = report["estimates"]["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["stage"] == "accept")
        .unwrap()
        .clone();
    assert_eq!(accept["samples"], 1);
    assert_eq!(accept["median"], Value::Null);
    assert!(
        accept["label"]
            .as_str()
            .unwrap()
            .starts_with("too few to estimate")
    );
    let rendered = text(&fixture, &[]);
    assert!(
        rendered.contains("unknown (end before start)"),
        "{rendered}"
    );
    assert!(rendered.contains("recorded together"));
    assert!(rendered.contains("too few to estimate"));
}

#[test]
fn history_ranges_need_three_samples_and_exclude_idle_time() {
    let mut lines = vec![];
    let mut at = T;
    for plan in [100, 400, 200] {
        lines.push(transition(Some(at), "advance", "intent", "plan", "go"));
        at += plan;
        lines.push(transition(Some(at), "advance", "plan", "build", "go"));
        at += 50;
        lines.push(transition(Some(at), "advance", "build", "done", "go"));
        at += 10_000;
        lines.push(transition(Some(at), "reset", "done", "intent", "next"));
        at += 5;
    }
    let fixture = fixture_with(&lines);
    let report = timing(&fixture, &[]);
    let estimate = |stage: &str| -> Value {
        report["estimates"]["stages"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["stage"] == stage)
            .unwrap_or_else(|| panic!("no {stage} estimate: {report:#}"))
            .clone()
    };
    let plan = estimate("plan");
    assert_eq!(plan["samples"], 3, "{plan:#}");
    assert_eq!(plan["min"], 100);
    assert_eq!(plan["median"], 200);
    assert_eq!(plan["max"], 400);
    assert_eq!(
        plan["label"],
        "estimate from 3 recorded plan stage times; not a target"
    );
    assert!(
        report["estimates"]["stages"]
            .as_array()
            .unwrap()
            .iter()
            .all(|entry| entry["stage"] != "done"),
        "idle time is never an estimate sample"
    );
    // The segment before the first record and the ongoing cycle are not samples.
    assert_eq!(estimate("intent")["samples"], 2);
    let rendered = text(&fixture, &[]);
    assert!(rendered.contains("estimate from 3 recorded plan stage times; not a target"));
}

fn write_observation(
    fixture: &Fixture,
    id: &str,
    case: Option<&str>,
    environment: &str,
    outcome: &str,
    at: u64,
    duration: Option<u64>,
) {
    let mut record = json!({
        "schema_version": 2,
        "id": id,
        "criteria": ["LIFE-1"],
        "requirements": ["REQ-cli-001"],
        "contracts": ["specs/cli/cli.spec.md"],
        "kind": "product-test",
        "outcome": outcome,
        "environment": environment,
        "summary": format!("fixture observation {id}"),
        "artifacts": [],
        "artifact_fingerprints": {},
        "observed_at_unix_seconds": at,
        "snapshot": {"revision": "0".repeat(40), "dirty": false, "fingerprint": ZERO},
        "provenance": "fixture",
    });
    if let Some(case) = case {
        record["cases"] = json!([{"id": case, "digest": ZERO}]);
        record["method"] = json!("automated");
    }
    if let Some(duration) = duration {
        record["duration_seconds"] = json!(duration);
    }
    fixture.write(
        &format!(".chough/evidence/{id}.json"),
        &serde_json::to_string_pretty(&record).unwrap(),
    );
}

fn write_bug(fixture: &Fixture, id: &str, status: &str, observations: &[&str], notes: Value) {
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
        "source": "fixture",
        "notes": notes,
        "created_at_unix_seconds": T,
        "updated_at_unix_seconds": T + 5_000,
    });
    fixture.write(
        &format!(".chough/bugs/{id}.json"),
        &serde_json::to_string_pretty(&bug).unwrap(),
    );
}

#[test]
fn bug_fix_times_come_from_observations_notes_and_the_retention_ledger() {
    let fixture = Fixture::new();
    fixture.init();
    write_observation(
        &fixture,
        "fail-late",
        Some("TC-a"),
        "local",
        "failed",
        T + 300,
        None,
    );
    write_observation(
        &fixture,
        "fail-early",
        Some("TC-a"),
        "local",
        "failed",
        T + 200,
        None,
    );
    write_observation(
        &fixture,
        "pass",
        Some("TC-a"),
        "local",
        "passed",
        T + 900,
        None,
    );
    // A pruned failure, earlier than both present ones, is known from the ledger.
    fixture.write(
        ".chough/pruned.jsonl",
        &format!(
            "{}\n",
            json!({"schema_version": 1, "id": "fail-pruned", "outcome": "failed", "observed_at_unix_seconds": T + 100, "pruned_at_unix_seconds": T + 2_000})
        ),
    );
    let verified = |at: u64| {
        json!([
            {"at_unix_seconds": T + 50, "text": "status: open -> confirmed. looked at it"},
            {"at_unix_seconds": T + 600, "text": "free note mentioning verified"},
            {"at_unix_seconds": at, "text": "status: confirmed -> verified; scope: in-scope -> blocking. fixed", "observations": ["pass"]},
            {"at_unix_seconds": at + 50, "text": "status: reopened -> verified"},
        ])
    };
    write_bug(
        &fixture,
        "BUG-0001",
        "verified",
        &["fail-late", "fail-early", "pass"],
        verified(T + 1_000),
    );
    write_bug(
        &fixture,
        "BUG-0002",
        "verified",
        &["fail-late", "fail-pruned", "pass"],
        verified(T + 1_000),
    );
    write_bug(
        &fixture,
        "BUG-0003",
        "verified",
        &["fail-late", "gone", "pass"],
        verified(T + 1_000),
    );
    write_bug(&fixture, "BUG-0004", "open", &[], json!([]));
    let report = timing(&fixture, &[]);
    let bug = |id: &str| -> Value {
        report["bugs"]
            .as_array()
            .unwrap()
            .iter()
            .find(|bug| bug["id"] == id)
            .unwrap_or_else(|| panic!("no {id}: {report:#}"))
            .clone()
    };
    let one = bug("BUG-0001");
    assert_eq!(at(&one["created"]), Some(T));
    assert_eq!(at(&one["reproduced"]), Some(T + 200));
    assert_eq!(one["reproduced_source"], "observation fail-early");
    assert_eq!(at(&one["verified"]), Some(T + 1_000));
    assert_eq!(one["verified_source"], "from note text");
    assert_eq!(seconds(&one["fix_time"]), Some(800));
    assert_eq!(one["fix_time"]["label"], "elapsed wall-clock");

    let two = bug("BUG-0002");
    assert_eq!(at(&two["reproduced"]), Some(T + 100));
    assert_eq!(
        two["reproduced_source"],
        "retention ledger (pruned observation fail-pruned)"
    );
    assert_eq!(seconds(&two["fix_time"]), Some(900));

    // A linked observation that is neither present nor pruned could be the earlier failure.
    let three = bug("BUG-0003");
    assert_eq!(three["reproduced"]["unix_seconds"], Value::Null);
    assert_eq!(three["reproduced"]["unknown"], "unavailable");
    assert_eq!(three["fix_time"]["label"], "unknown");

    let four = bug("BUG-0004");
    assert_eq!(
        four["reproduced"]["unknown"],
        "no linked failed observation"
    );
    assert_eq!(
        four["verified"]["unknown"],
        "no note records status -> verified"
    );
    assert_eq!(four["fix_time"]["seconds"], Value::Null);

    // Two fix times are too few to estimate.
    assert_eq!(report["estimates"]["bug_fixes"]["samples"], 2);
    assert!(
        report["estimates"]["bug_fixes"]["label"]
            .as_str()
            .unwrap()
            .starts_with("too few to estimate")
    );
    let only = timing(&fixture, &["--bug", "BUG-0002"]);
    assert_eq!(only["bugs"].as_array().unwrap().len(), 1);
}

#[test]
fn observations_group_per_case_and_validation_elapsed_is_total_run_wall_clock() {
    let fixture = Fixture::new();
    fixture.init();
    write_observation(
        &fixture,
        "a-1",
        Some("TC-a"),
        "local",
        "failed",
        T + 10,
        None,
    );
    write_observation(
        &fixture,
        "a-2",
        Some("TC-a"),
        "local",
        "passed",
        T + 20,
        Some(7),
    );
    write_observation(&fixture, "a-ci", Some("TC-a"), "ci", "passed", T + 30, None);
    write_observation(
        &fixture,
        "b-1",
        Some("TC-b"),
        "local",
        "not-run",
        T + 40,
        None,
    );
    write_observation(&fixture, "loose", None, "local", "passed", T + 50, None);
    fixture.write(
        ".chough/validation/val-1.json",
        &serde_json::to_string_pretty(&json!({
            "schema_version": 1,
            "id": "val-1",
            "case": "TC-a",
            "times": 3,
            "interval_seconds": 60,
            "environment": "local",
            "authority": "run consent: fixture",
            "state": "complete",
            "completed": 3,
            "failures": 1,
            "not_run": 0,
            "observations": ["a-1", "a-2"],
            "bugs": [],
            "elapsed_seconds": 125,
            "started_at_unix_seconds": T,
        }))
        .unwrap(),
    );
    let report = timing(&fixture, &[]);
    let group = |case: Value, environment: &str| -> Value {
        report["tests"]
            .as_array()
            .unwrap()
            .iter()
            .find(|group| group["case"] == case && group["environment"] == environment)
            .unwrap_or_else(|| panic!("no group {case} {environment}: {report:#}"))
            .clone()
    };
    let local = group(json!("TC-a"), "local");
    assert_eq!(local["count"], 2);
    let observations = local["observations"].as_array().unwrap();
    assert_eq!(observations[0]["id"], "a-1");
    assert_eq!(observations[0]["outcome"], "failed");
    assert_eq!(at(&observations[0]["observed"]), Some(T + 10));
    // Old records hold no run time: unknown, never zero.
    assert_eq!(observations[0]["run_time"]["seconds"], Value::Null);
    assert_eq!(
        observations[0]["run_time"]["reason"],
        "no run time recorded"
    );
    assert_eq!(seconds(&observations[1]["run_time"]), Some(7));
    assert_eq!(observations[1]["run_time"]["label"], "elapsed wall-clock");
    assert_eq!(group(json!("TC-a"), "ci")["count"], 1);
    assert_eq!(
        group(json!("TC-b"), "local")["observations"][0]["outcome"],
        "not-run"
    );
    assert_eq!(group(Value::Null, "local")["count"], 1);

    let run = &report["validation_runs"][0];
    assert_eq!(run["id"], "val-1");
    assert_eq!(run["case"], "TC-a");
    assert_eq!(at(&run["started"]), Some(T));
    assert_eq!(run["completed"], 3);
    assert_eq!(run["failures"], 1);
    assert_eq!(run["attempts"], 2);
    assert_eq!(seconds(&run["run_wall_clock"]), Some(125));
    assert_eq!(
        run["run_wall_clock"]["label"],
        "total run wall-clock, including intervals and resumes"
    );

    let only = timing(&fixture, &["--case", "TC-b"]);
    assert_eq!(only["tests"].as_array().unwrap().len(), 1);
    assert!(only["validation_runs"].as_array().unwrap().is_empty());
    let rendered = text(&fixture, &[]);
    assert!(rendered.contains("total run wall-clock, including intervals and resumes"));
    assert!(!rendered.to_lowercase().contains("effort:"));
}

#[test]
fn transitions_beyond_the_bound_are_partial_and_every_record_stays_byte_identical() {
    let fixture = Fixture::new();
    fixture.init();
    let padding = "x".repeat(4_000);
    let line = transition(Some(T), "reset", "intent", "intent", &padding);
    let count = 16 * 1_048_576 / line.len() + 200;
    let mut content = String::with_capacity(count * (line.len() + 1));
    for index in 0..count {
        content.push_str(&transition(
            Some(T + index as u64),
            "reset",
            "intent",
            "intent",
            &padding,
        ));
        content.push('\n');
    }
    fixture.write(".chough/transitions.jsonl", &content);
    write_observation(&fixture, "a-1", Some("TC-a"), "local", "failed", T, None);
    write_bug(&fixture, "BUG-0001", "open", &["a-1"], json!([]));
    fixture.write(".specsync/changes/some-change/state.json", "{}\n");
    let before = record_digests(fixture.root());
    let report = timing(&fixture, &["--cycle", "current"]);
    assert!(
        report["transitions"]["skipped_bytes"].as_u64().unwrap() > 0,
        "{}",
        report["transitions"]
    );
    assert!(
        report["partial"]
            .to_string()
            .contains(".chough/transitions.jsonl"),
        "{}",
        report["partial"]
    );
    text(&fixture, &["--cycle", "current"]);
    assert_eq!(before, record_digests(fixture.root()));
    assert!(!fixture.exists(".chough/artifacts"));
}

/// A SpecSync stand-in: status per change from `.stub-status-<id>`, a default otherwise.
#[cfg(unix)]
fn native_fixture() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(
        "bin/specsync",
        r#"#!/bin/sh
id="$3"
if [ -f ".stub-fail-$id" ]; then echo 'status failed' >&2; exit 2; fi
if [ -f ".stub-sleep-$id" ]; then exec sleep 30; fi
if [ -f ".stub-status-$id" ]; then cat ".stub-status-$id"; exit 0; fi
printf '{"change":{"id":"%s","created_at":100,"updated_at":200},"summary":{"state":"archived","approval_valid":true}}\n' "$id"
"#,
    );
    fixture.executable("bin/specsync");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "cur-change"]);
    fixture.write(
        ".stub-status-cur-change",
        &json!({"change": {"id": "cur-change", "created_at": T, "updated_at": T + 900}, "summary": {"state": "approved", "approval_valid": false}}).to_string(),
    );
    fixture.write(
        ".specsync/changes/cur-change/approvals.json",
        &json!({
            "approvals": [
                {"gate": "definition", "actor": "Secret Actor Name", "timestamp": T + 100, "note": "Secret note text", "digest": "abc"},
                {"gate": "definition", "actor": "Secret Actor Name", "timestamp": T + 200, "note": "Secret note text", "digest": "def"},
            ],
            "reopenings": [],
        })
        .to_string(),
    );
    fixture.write(
        ".specsync/changes/cur-change/verification.json",
        "{\"timestamp\": \"yesterday\"}\n",
    );
    for index in 0..12 {
        let id = format!("arch-{index:02}");
        let directory = format!(".specsync/archive/changes/2026-01-{:02}-{id}", index + 1);
        fixture.write(
            &format!("{directory}/approvals.json"),
            &json!({
                "approvals": [
                    {"gate": "definition", "actor": "Secret Actor Name", "timestamp": 1_000, "note": "Secret note text"},
                    {"gate": "finalization", "actor": "Secret Actor Name", "timestamp": 3_000, "note": "Secret note text"},
                ],
                "reopenings": if index == 11 { json!([{"timestamp": 3_500, "actor": "Secret Actor Name", "reason": "Secret reason"}]) } else { json!([]) },
            })
            .to_string(),
        );
        fixture.write(
            &format!("{directory}/verification.json"),
            "{\"timestamp\": 2000, \"commit\": \"abc\"}\n",
        );
        fixture.write(
            &format!("{directory}/review.json"),
            "{\"timestamp\": 2500, \"reviewer\": \"Secret Actor Name\"}\n",
        );
        fixture.write(
            &format!("{directory}/finalization.json"),
            "{\"timestamp\": 3000}\n",
        );
    }
    fixture.write(".stub-fail-arch-10", "");
    fixture
}

#[cfg(unix)]
#[test]
fn native_times_come_only_from_status_and_native_records_read_read_only() {
    let fixture = native_fixture();
    let before = record_digests(fixture.root());
    let report = timing(&fixture, &[]);
    let rendered = text(&fixture, &[]);
    assert_eq!(before, record_digests(fixture.root()));
    for output in [report.to_string(), rendered.clone()] {
        assert!(
            !output.contains("Secret"),
            "actor, note or reason leaked: {output}"
        );
    }
    let native = &report["native"];
    assert_eq!(native["state"], "observed", "{native:#}");
    let current = &native["current"];
    assert_eq!(current["id"], "cur-change");
    assert_eq!(at(&current["created"]), Some(T));
    assert_eq!(at(&current["updated"]), Some(T + 900));
    // Status reports the approval invalid: neither approval is current.
    let approvals = current["approvals"].as_array().unwrap();
    assert_eq!(approvals.len(), 2);
    assert_eq!(at(&approvals[1]["at"]), Some(T + 200));
    assert_eq!(approvals[1]["standing"], "not current");
    assert_eq!(approvals[0]["standing"], "not current");
    assert!(
        approvals
            .iter()
            .all(|approval| approval["standing"] != "current")
    );
    // An unrecognised record shape is unknown with its reason.
    assert_eq!(current["verification"]["unix_seconds"], Value::Null);
    assert_eq!(
        current["verification"]["unknown"],
        "unrecognised record shape"
    );
    assert_eq!(current["review"]["unknown"], "no native record");

    // History reads the 10 most recent archived changes and reports the rest skipped.
    let history = native["history"].as_array().unwrap();
    assert_eq!(history.len(), 10, "{native:#}");
    assert_eq!(history[0]["id"], "arch-11");
    assert_eq!(native["skipped_archived"], 2);
    assert!(report["partial"].to_string().contains("2 older archived"));
    let newest = &history[0];
    assert_eq!(at(&newest["created"]), Some(100));
    assert_eq!(at(&newest["verification"]), Some(2_000));
    assert_eq!(at(&newest["review"]), Some(2_500));
    assert_eq!(at(&newest["finalization"]), Some(3_000));
    // A later reopening supersedes the finalization approval.
    let gates: Vec<(Value, Value)> = newest["approvals"]
        .as_array()
        .unwrap()
        .iter()
        .map(|approval| (approval["gate"].clone(), approval["standing"].clone()))
        .collect();
    assert_eq!(
        gates,
        vec![
            (json!("definition"), json!("reopened")),
            (json!("finalization"), json!("reopened"))
        ]
    );
    assert_eq!(history[2]["approvals"][0]["standing"], "current");
    // A failed status read leaves that change's times unknown with the reason.
    let failed = &history[1];
    assert_eq!(failed["id"], "arch-10");
    assert_eq!(failed["created"]["unix_seconds"], Value::Null);
    assert!(
        failed["created"]["unknown"]
            .as_str()
            .unwrap()
            .contains("failed"),
        "{failed:#}"
    );
    assert_eq!(failed["approvals"][0]["standing"], "unknown");
}

#[cfg(unix)]
#[test]
fn a_timed_out_or_unconfigured_native_read_leaves_native_times_unknown() {
    let fixture = native_fixture();
    fixture.write(".stub-sleep-cur-change", "");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(fixture.root())
        .args(["timing", "--json", "--cycle", "current"])
        .env("CHOUGH_NATIVE_TIMEOUT_SECONDS", "1")
        .env(
            "PATH",
            std::env::join_paths(std::iter::once(fixture.root().join("bin")).chain(
                std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()),
            ))
            .unwrap(),
        )
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let current = &report["native"]["current"];
    assert_eq!(current["created"]["unix_seconds"], Value::Null);
    assert!(
        current["created"]["unknown"]
            .as_str()
            .unwrap()
            .contains("timed-out"),
        "{current:#}"
    );
    assert_eq!(current["approvals"][1]["standing"], "unknown");

    let plain = Fixture::new();
    plain.init();
    let report = timing(&plain, &[]);
    assert_eq!(report["native"]["state"], "not-configured");
    assert_eq!(report["native"]["current"], Value::Null);
}

#[cfg(unix)]
#[test]
fn a_case_run_records_its_run_time_and_the_report_shows_it() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Automated\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\npaths = [\"src/\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"It prints ok\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"sleep 1; echo ok\"]\nexpect_output = [\"ok\"]\n",
    );
    fixture.init();
    fixture.commit("fixture");
    let run = fixture.ok(&["cases", "run", "TC-cli-0001", "--environment", "local"]);
    let id = run["observation"].as_str().unwrap();
    let record: Value =
        serde_json::from_str(&fixture.read(&format!(".chough/evidence/{id}.json"))).unwrap();
    let duration = record["duration_seconds"].as_u64().unwrap();
    assert!((1..30).contains(&duration), "{record:#}");
    let report = timing(&fixture, &["--case", "TC-cli-0001"]);
    let observation = &report["tests"][0]["observations"][0];
    assert_eq!(observation["id"], id);
    assert_eq!(seconds(&observation["run_time"]), Some(duration));
    assert_eq!(observation["run_time"]["label"], "elapsed wall-clock");
}
