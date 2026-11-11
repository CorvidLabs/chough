//! The read-only timing report and the gate records it reads (REQ-reporting-001).
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

/// `YYYY-MM-DDTHH:MM:SSZ` for a Unix time, matching the Timeline's format.
fn iso_of(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    let (hour, minute, second) = (rem / 3_600, rem % 3_600 / 60, rem % 60);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// The active session's Timeline entries, as `chough history show` reads session.md.
fn timeline_entries(fixture: &Fixture) -> Vec<Value> {
    fixture.ok(&["history", "show"])["entries"]
        .as_array()
        .unwrap()
        .clone()
}

/// Append the 0.5.x-shaped transition JSON lines to the active session's Timeline.
fn write_transitions(fixture: &Fixture, lines: &[String]) {
    let file = fixture.session_file();
    let mut text = fixture.read(&file);
    if !text.ends_with('\n') {
        text.push('\n');
    }
    for (index, line) in lines.iter().enumerate() {
        let transition: Value = serde_json::from_str(line).unwrap();
        let reason = transition["reason"].as_str().unwrap_or_default();
        let gate = transition["gate"].as_str();
        let text_of = if reason.is_empty() {
            gate.unwrap_or("")
        } else {
            reason
        };
        let at = transition["at_unix_seconds"]
            .as_u64()
            .map(iso_of)
            .unwrap_or_else(|| "sometime".to_string());
        let mut entry = format!(
            "- {} · {} · {} → {} · {} ^t-{:06x}\n",
            at,
            transition["kind"].as_str().unwrap(),
            transition["from"].as_str().unwrap(),
            transition["to"].as_str().unwrap(),
            text_of.replace('\n', " "),
            index
        );
        if let Some(gate) = gate {
            entry.push_str(&format!("  - gate: {}\n", gate.replace('\n', " ")));
        }
        if let Some(revision) = transition["revision"].as_str() {
            entry.push_str(&format!("  - revision: {revision}\n"));
        }
        if let Some(refresh) = transition["refresh"].as_array() {
            let items: Vec<&str> = refresh.iter().filter_map(Value::as_str).collect();
            if !items.is_empty() {
                entry.push_str(&format!("  - refresh: {}\n", items.join(", ")));
            }
        }
        for key in ["backfill", "relayed_by", "direct_write"] {
            if !transition[key].is_null() {
                entry.push_str(&format!(
                    "  - {key}: {}\n",
                    serde_json::to_string(&transition[key]).unwrap()
                ));
            }
        }
        text.push_str(&entry);
    }
    fixture.write(&file, &text);
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

/// Variables that change native reads or tracing; tests set them explicitly so the
/// caller's environment cannot leak in.
const NATIVE_VARIABLES: [&str; 3] = [
    "CHOUGH_NATIVE_TIMEOUT_SECONDS",
    "CHOUGH_TIMING_NATIVE_BUDGET_SECONDS",
    "CHOUGH_TRACE",
];

/// Run chough with the fixture's stand-ins first on PATH and exactly `env` set among
/// the native variables.
fn run_env(fixture: &Fixture, args: &[&str], env: &[(&str, &str)]) -> std::process::Output {
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_chough"));
    command.arg("--root").arg(fixture.root()).args(args);
    for name in NATIVE_VARIABLES {
        command.env_remove(name);
    }
    command.envs(env.iter().copied());
    command.env(
        "PATH",
        std::env::join_paths(std::iter::once(fixture.root().join("bin")).chain(
            std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()),
        ))
        .unwrap(),
    );
    command.output().unwrap()
}

fn json_ok(output: &std::process::Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

/// The change IDs the stand-in was asked for since the last call, in order.
fn stub_calls(fixture: &Fixture) -> Vec<String> {
    let path = fixture.root().join(".stub-calls");
    let calls = fs::read_to_string(&path)
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .collect();
    let _ = fs::remove_file(path);
    calls
}

/// PIDs of stand-in descendants that are still running.
fn surviving_children(fixture: &Fixture) -> Vec<String> {
    fs::read_to_string(fixture.root().join(".stub-children"))
        .unwrap_or_default()
        .lines()
        .filter(|pid| common::alive(pid))
        .map(str::to_string)
        .collect()
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
    let records = |fixture: &Fixture| -> Vec<Value> { timeline_entries(fixture) };
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
    // The record says when, and what the gate holds and drives (REQ-cli-030), never
    // who decided or what was decided.
    let keys: Vec<&String> = opened.as_object().unwrap().keys().collect();
    assert_eq!(
        keys,
        vec![
            "at_unix_seconds",
            "drives",
            "from",
            "gate",
            "holds",
            "id",
            "kind",
            "reason",
            "refresh",
            "revision",
            "schema_version",
            "to"
        ]
    );
    assert_eq!(
        (&opened["holds"], &opened["drives"]),
        (&Value::from("intent"), &Value::from("none"))
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
    // "Edited away" was removed from the session record by hand; "Legacy" predates
    // gate records.
    let file = fixture.session_file();
    let text = fixture.read(&file);
    assert!(text.contains("waiting_on_human: []"), "{text}");
    let edited = text.replace(
        "waiting_on_human: []",
        "waiting_on_human:\n  - gate: Still waiting\n    holds: design\n    opened: 2026-01-01T00:00:00Z\n  - gate: Legacy\n    holds: design\n    opened: 2026-01-01T00:00:00Z",
    );
    fixture.write(&file, &edited);
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

    // A second gate-opened for a gate already open means it was cleared in between
    // without a record (a hand edit or a failed append): the earlier opening is closed
    // at an unknown time and the later one starts a new gate.
    let reopened = fixture_with(&[
        transition(Some(T), "advance", "intent", "design", "go"),
        transition(Some(T + 100), "gate-opened", "design", "design", "Review"),
        transition(Some(T + 500), "gate-opened", "design", "design", "Review"),
        transition(Some(T + 1_000), "advance", "design", "plan", "go"),
    ]);
    let reopened_file = reopened.session_file();
    let reopened_text = reopened.read(&reopened_file);
    assert!(
        reopened_text.contains("waiting_on_human: []"),
        "{reopened_text}"
    );
    reopened.write(
        &reopened_file,
        &reopened_text.replace(
            "waiting_on_human: []",
            "waiting_on_human:\n  - gate: Review\n    holds: design\n    opened: 2026-01-01T00:00:00Z",
        ),
    );
    let report = timing(&reopened, &[]);
    let gates = report["gates"].as_array().unwrap();
    assert_eq!(gates.len(), 2, "{report:#}");
    assert_eq!(at(&gates[0]["opened"]), Some(T + 100));
    assert_eq!(gates[0]["state"], "closed");
    assert_eq!(gates[0]["closed"]["unix_seconds"], Value::Null);
    assert_eq!(
        gates[0]["closed"]["unknown"],
        "cleared without a gate record"
    );
    assert_eq!(gates[0]["waiting"]["label"], "unknown");
    assert_eq!(at(&gates[1]["opened"]), Some(T + 500));
    assert_eq!(gates[1]["state"], "open");
    let design = &segments(&report, 0)[0];
    assert_eq!(design["stage"], "design");
    assert_eq!(design["waiting"]["label"], "unknown", "{design:#}");
    assert_eq!(design["active"]["label"], "unknown");
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
    let file = fixture.session_file();
    fixture.write(
        &file,
        &(fixture.read(&file) + "- this line is not an entry\n"),
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

    // A gate that closes before it opens: one of its times is wrong, so every segment
    // it could overlap under either reading has unknown waiting and active time.
    let reversed = fixture_with(&[
        transition(Some(T), "advance", "intent", "design", "go"),
        transition(Some(T + 500), "gate-opened", "design", "design", "Skewed"),
        transition(Some(T + 450), "gate-closed", "design", "design", "Skewed"),
        transition(Some(T + 1_000), "advance", "design", "plan", "go"),
    ]);
    let report = timing(&reversed, &[]);
    let gate = &report["gates"][0];
    assert_eq!(gate["waiting"]["seconds"], Value::Null);
    assert_eq!(gate["waiting"]["reason"], "end before start");
    for segment in segments(&report, 0) {
        assert_eq!(segment["waiting"]["label"], "unknown", "{segment:#}");
        assert_eq!(segment["waiting"]["seconds"], Value::Null);
        assert_eq!(segment["active"]["label"], "unknown", "{segment:#}");
    }

    // An open gate whose recorded open time is after now has an unknown open time.
    let future = fixture_with(&[
        transition(Some(T), "advance", "intent", "design", "go"),
        transition(
            Some(T + 9_000_000_000),
            "gate-opened",
            "design",
            "design",
            "Ahead",
        ),
    ]);
    let future_file = future.session_file();
    let future_text = future.read(&future_file);
    assert!(
        future_text.contains("waiting_on_human: []"),
        "{future_text}"
    );
    future.write(
        &future_file,
        &future_text.replace(
            "waiting_on_human: []",
            "waiting_on_human:\n  - gate: Ahead\n    holds: intent\n    opened: 2026-01-01T00:00:00Z",
        ),
    );
    let report = timing(&future, &[]);
    let design = &segments(&report, 0)[0];
    assert_eq!(design["stage"], "design");
    assert_eq!(design["waiting"]["label"], "unknown", "{design:#}");
    assert_eq!(design["active"]["label"], "unknown");
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

    // A current cycle that has reached done is complete, so it is a sample.
    let mut lines = vec![];
    let mut at = T;
    for (index, plan) in [100, 200, 300].into_iter().enumerate() {
        if index > 0 {
            lines.push(transition(Some(at), "reset", "done", "intent", "next"));
            at += 5;
        }
        lines.push(transition(Some(at), "advance", "intent", "plan", "go"));
        at += plan;
        lines.push(transition(Some(at), "advance", "plan", "done", "go"));
        at += 1_000;
    }
    let report = timing(&fixture_with(&lines), &[]);
    let plan = report["estimates"]["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["stage"] == "plan")
        .unwrap()
        .clone();
    assert_eq!(plan["samples"], 3, "{plan:#}");
    assert_eq!(plan["max"], 300);
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

/// A Markdown bug created at `T` whose Log holds each note as a dated entry.
fn write_bug(fixture: &Fixture, id: &str, status: &str, evidence: &[&str], notes: Value) {
    fixture.bug_md(id, "in-scope", status, evidence);
    let name = format!(".chough/bugs/{id}.md");
    let mut text = fixture
        .read(&name)
        .replace(
            "created: 2026-11-10T00:00:01Z",
            &format!("created: {}", iso_of(T)),
        )
        .replace("- 2026-11-10T00:00:01Z · Reported (fixture)\n", "");
    for note in notes.as_array().unwrap() {
        text.push_str(&format!(
            "- {} · {}\n",
            iso_of(note["at_unix_seconds"].as_u64().unwrap()),
            note["text"].as_str().unwrap()
        ));
    }
    fixture.write(&name, &text);
}

/// An evidence entry of the active session with the fields timing reads
/// (REQ-cli-017): case, result, environment, observed time and run time.
fn write_entry(fixture: &Fixture, id: &str, case: &str, result: &str, observed: u64) {
    fixture.write(
        &format!(".chough/sessions/{}/evidence/{id}.md", fixture.active_id()),
        &format!(
            "---\nkind: evidence\nid: {id}\naliases: []\nclaim: SYNTHETIC claim {id}\nresult: {result}\ncase: {case}\nenvironment: local\nobserved: {}\nduration_seconds: 3\n---\n**SYNTHETIC claim {id}.**\n",
            iso_of(observed)
        ),
    );
}

/// One receipt map in front matter, as an archived story or a bug holds it
/// (REQ-history-006); `observed` is left out when None.
fn receipt(id: &str, case: &str, result: &str, observed: Option<u64>) -> String {
    let mut text = format!(
        "  - id: {id}\n    aliases: OBS-{id}-000000000000000000000001\n    claim: SYNTHETIC claim {id}\n    result: {result}\n    case: {case}\n    environment: local\n"
    );
    if let Some(observed) = observed {
        text.push_str(&format!("    observed: {}\n", iso_of(observed)));
    }
    text
}

/// Close session `key` (REQ-history-006) and give its story these receipts.
fn archive_with_receipts(fixture: &Fixture, key: &str, receipts: &[String]) -> String {
    let id = fixture.id_of(key);
    fixture.ok(&["session", "close", &id, "--reason", "SYNTHETIC closed"]);
    let story = fixture.story_of(key);
    let text = fixture.read(&story);
    assert!(text.contains("\nreceipts: []\n"), "{text}");
    fixture.write(
        &story,
        &text.replace(
            "\nreceipts: []\n",
            &format!("\nreceipts:\n{}", receipts.concat()),
        ),
    );
    id
}

/// TC-reporting-0086, rewritten for 0.6.0 (REQ-reporting-001): a bug's reproduction is
/// its earliest linked failure, read from an open session's entry, an archived story's
/// receipt or the bug's own receipt; its verification is the first Log entry recording
/// `status: <old> -> verified`; no retention ledger is read. A linked ID that is missing,
/// or a linked failure without an observed time, leaves the reproduction unknown.
#[test]
fn bug_fix_times_come_from_entries_receipts_and_the_bug_log() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["session", "new", "old-work", "--objective", "Old work"]);
    archive_with_receipts(
        &fixture,
        "old-work",
        &[receipt("fail-archived", "TC-a", "failed", Some(T + 100))],
    );
    fixture.ok(&["session", "new", "now-work", "--objective", "Now work"]);
    write_entry(&fixture, "fail-late", "TC-a", "failed", T + 300);
    write_entry(&fixture, "fail-early", "TC-a", "failed", T + 200);
    write_entry(&fixture, "pass", "TC-a", "passed", T + 900);
    let verified = |at: u64| {
        json!([
            {"at_unix_seconds": T + 50, "text": "status: open -> investigating. looked at it"},
            {"at_unix_seconds": T + 600, "text": "free note mentioning verified · by SYNTHETIC-agent"},
            {"at_unix_seconds": at, "text": "status: investigating -> verified; scope: in-scope -> blocking; evidence: pass · by SYNTHETIC-agent"},
            {"at_unix_seconds": at + 50, "text": "status: deferred -> verified"},
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
        &[
            "fail-late",
            "OBS-fail-archived-000000000000000000000001",
            "pass",
        ],
        verified(T + 1_000),
    );
    write_bug(
        &fixture,
        "BUG-0003",
        "verified",
        &["fail-late", "gone", "pass", "fail-pruned"],
        verified(T + 1_000),
    );
    write_bug(&fixture, "BUG-0004", "open", &[], json!([]));
    // The bug's own receipt: a failure with no observed time could be the earlier one.
    write_bug(
        &fixture,
        "BUG-0005",
        "verified",
        &["fail-late", "fail-untimed", "pass"],
        verified(T + 1_000),
    );
    let name = ".chough/bugs/BUG-0005.md";
    fixture.write(
        name,
        &fixture.read(name).replace(
            "\nreceipts: []\n",
            &format!(
                "\nreceipts:\n{}",
                receipt("fail-untimed", "TC-a", "failed", None)
            ),
        ),
    );
    // A bug with an alias answers to it in `--bug`.
    let name = ".chough/bugs/BUG-0002.md";
    fixture.write(
        name,
        &fixture
            .read(name)
            .replace("aliases: []", "aliases: [BUG-0099]"),
    );
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
    assert_eq!(one["reproduced_source"], "evidence fail-early (entry)");
    assert_eq!(at(&one["verified"]), Some(T + 1_000));
    assert_eq!(one["verified_source"], "from the bug Log");
    assert_eq!(seconds(&one["fix_time"]), Some(800));
    assert_eq!(one["fix_time"]["label"], "elapsed wall-clock");

    // The earlier failure is a receipt in an archived story, linked by its alias.
    let two = bug("BUG-0002");
    assert_eq!(at(&two["reproduced"]), Some(T + 100));
    assert_eq!(
        two["reproduced_source"],
        "evidence OBS-fail-archived-000000000000000000000001 (story-receipt)"
    );
    assert_eq!(seconds(&two["fix_time"]), Some(900));

    // A linked ID that is neither an entry nor a receipt, such as a pruned 0.5.x
    // observation, could be the earlier failure; no retention ledger is read.
    let three = bug("BUG-0003");
    assert_eq!(three["reproduced"]["unix_seconds"], Value::Null);
    assert_eq!(three["reproduced"]["unknown"], "unavailable");
    assert_eq!(three["fix_time"]["label"], "unknown");

    // A receipt failure without an observed time could be the earlier failure.
    let five = bug("BUG-0005");
    assert_eq!(five["reproduced"]["unix_seconds"], Value::Null);
    assert_eq!(five["reproduced"]["unknown"], "unavailable");
    assert_eq!(five["fix_time"]["label"], "unknown");

    let four = bug("BUG-0004");
    assert_eq!(
        four["reproduced"]["unknown"],
        "no linked failed observation"
    );
    assert_eq!(
        four["verified"]["unknown"],
        "no Log entry records status -> verified"
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
    for only in ["BUG-0002", "BUG-0099"] {
        let report = timing(&fixture, &["--bug", only]);
        assert_eq!(report["bugs"].as_array().unwrap().len(), 1, "{only}");
        assert_eq!(report["bugs"][0]["id"], "BUG-0002", "{only}");
    }
}

/// TC-reporting-timing-from-timelines (REQ-reporting-001): with one open session and
/// three archived stories, one of them unreadable, the report gives each readable
/// session's stage segments and gate waits from its own timeline and each case's runs
/// from entries and receipts; the unreadable story is named and the report is partial;
/// every record stays byte-identical.
#[test]
fn timing_reads_timelines_and_receipts() {
    let fixture = Fixture::new();
    fixture.init();
    let first = fixture.active_id();
    let mut archived = vec![];
    for (index, key) in ["alpha", "beta", "gamma"].into_iter().enumerate() {
        let base = T + 10_000 * index as u64;
        fixture.ok(&["session", "new", key, "--objective", &format!("Ship {key}")]);
        write_transitions(
            &fixture,
            &[
                transition(Some(base), "advance", "intent", "design", "intent approved"),
                transition(
                    Some(base + 100),
                    "gate-opened",
                    "design",
                    "design",
                    "Design review",
                ),
                transition(
                    Some(base + 160),
                    "gate-closed",
                    "design",
                    "design",
                    "Design review",
                ),
                transition(Some(base + 200), "advance", "design", "plan", "designed"),
            ],
        );
        archived.push(archive_with_receipts(
            &fixture,
            key,
            &[receipt(
                &format!("{key}-run"),
                "TC-a",
                "passed",
                Some(base + 150),
            )],
        ));
    }
    // gamma's story no longer reads.
    let gamma = fixture.story_of("gamma");
    fixture.write(&gamma, "---\nkind: session\nbroken: [\n---\n# Gamma\n");
    fixture.ok(&["session", "new", "delta", "--objective", "Ship delta"]);
    write_transitions(
        &fixture,
        &[transition(
            Some(T + 50_000),
            "advance",
            "intent",
            "design",
            "intent approved",
        )],
    );
    write_entry(&fixture, "delta-run", "TC-a", "failed", T + 50_100);
    fixture.bug_md("BUG-0001", "in-scope", "open", &["bug-run"]);
    let name = ".chough/bugs/BUG-0001.md";
    fixture.write(
        name,
        &fixture.read(name).replace(
            "\nreceipts: []\n",
            &format!(
                "\nreceipts:\n{}",
                receipt("bug-run", "TC-a", "failed", Some(T + 40_000))
            ),
        ),
    );

    let before = record_digests(fixture.root());
    let report = timing(&fixture, &[]);
    assert_eq!(record_digests(fixture.root()), before);

    // Each readable session, open or archived, from its own timeline.
    let sessions = report["sessions"].as_array().unwrap();
    let find = |id: &str| -> &Value {
        sessions
            .iter()
            .find(|session| session["id"] == id)
            .unwrap_or_else(|| panic!("no session {id}: {report:#}"))
    };
    let delta = find(&fixture.id_of("delta"));
    assert_eq!(delta["state"], "open");
    assert_eq!(delta["selected"], true);
    // The stage is the checkpoint's own; the entries above were written by hand.
    assert_eq!(delta["stage"], "intent");
    assert_eq!(find(&first)["state"], "open");
    for (index, id) in archived.iter().take(2).enumerate() {
        let base = T + 10_000 * index as u64;
        let story = find(id);
        assert_eq!(story["state"], "archived", "{story:#}");
        assert_eq!(story["selected"], false, "{story:#}");
        let segments = story["cycles"][0]["segments"].as_array().unwrap();
        let design = segments
            .iter()
            .find(|segment| segment["stage"] == "design")
            .unwrap_or_else(|| panic!("{story:#}"));
        assert_eq!(at(&design["start"]), Some(base), "{story:#}");
        assert_eq!(at(&design["end"]), Some(base + 200), "{story:#}");
        assert_eq!(seconds(&design["waiting"]), Some(60), "{story:#}");
        assert_eq!(seconds(&design["active"]), Some(140), "{story:#}");
        assert_eq!(story["gates"][0]["gate"], "Design review", "{story:#}");
        assert_eq!(
            seconds(&story["gates"][0]["waiting"]),
            Some(60),
            "{story:#}"
        );
    }
    assert!(
        sessions.iter().all(|session| session["id"] != archived[2]),
        "{report:#}"
    );
    // The unreadable story is named and the report is partial.
    let partial = report["partial"].to_string();
    assert!(partial.contains(&gamma), "{partial}");
    // Two closed gates are too few to estimate, but each is a sample.
    assert_eq!(report["estimates"]["gates"]["samples"], 2, "{report:#}");

    // Each case's runs come from the open session's entry and the readable receipts.
    let group = report["tests"]
        .as_array()
        .unwrap()
        .iter()
        .find(|group| group["case"] == "TC-a" && group["environment"] == "local")
        .unwrap_or_else(|| panic!("{report:#}"));
    let runs: Vec<(String, String)> = group["observations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|run| {
            (
                run["id"].as_str().unwrap().to_string(),
                run["found_in"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(
        runs,
        vec![
            (
                "alpha-run".to_string(),
                format!("receipt in {}", archived[0])
            ),
            (
                "beta-run".to_string(),
                format!("receipt in {}", archived[1])
            ),
            ("bug-run".to_string(), "receipt in bug BUG-0001".to_string()),
            (
                "delta-run".to_string(),
                format!("entry in {}", fixture.id_of("delta"))
            ),
        ],
        "{report:#}"
    );
    assert_eq!(seconds(&group["observations"][3]["run_time"]), Some(3));
    assert_eq!(group["observations"][0]["run_time"]["seconds"], Value::Null);

    // `--cycle current` shows the selected session only.
    let current = timing(&fixture, &["--cycle", "current"]);
    let ids: Vec<&Value> = current["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|session| &session["id"])
        .collect();
    assert_eq!(ids, vec![&json!(fixture.id_of("delta"))]);
}

/// A correction (REQ-history-015) is a known Timeline kind: it starts and ends no stage
/// segment and is never counted as an unrecognised kind.
#[test]
fn a_correction_entry_starts_no_segment_and_is_not_unrecognised() {
    let lines = full_history(true);
    let plain = timing(&fixture_with(&lines), &[]);
    let mut corrected = lines.clone();
    corrected.insert(
        6,
        transition(
            Some(T + 500),
            "correction",
            "build",
            "build",
            "the second pass was a fix, not a rework",
        ),
    );
    let report = timing(&fixture_with(&corrected), &[]);
    assert_eq!(report["transitions"]["unrecognised_kinds"], 0, "{report:#}");
    assert_eq!(report["cycles"], plain["cycles"], "{report:#}");
    assert_eq!(
        report["transitions"]["records"],
        plain["transitions"]["records"]
    );
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
        "a-ci-quick",
        Some("TC-a"),
        "ci",
        "passed",
        T + 35,
        Some(0),
    );
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
    let ci = group(json!("TC-a"), "ci");
    assert_eq!(ci["count"], 2);
    // A run under one second is recorded together, not a measured zero.
    assert_eq!(seconds(&ci["observations"][1]["run_time"]), Some(0));
    assert_eq!(
        ci["observations"][1]["run_time"]["label"],
        "recorded together"
    );
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

    // Compatibility decision: `duration_seconds` is an optional schema 2 field, read
    // here, while observation fields stay closed. A reader refuses a field it does not
    // know, so builds before this field refuse records that carry it (docs/cli.md says
    // so), exactly as this build refuses a field it does not know.
    assert!(fixture.run(&["status"]).status.success());
    let mut record: Value =
        serde_json::from_str(&fixture.read(".chough/evidence/a-2.json")).unwrap();
    record["field_from_a_later_build"] = json!(1);
    fixture.write(
        ".chough/evidence/a-2.json",
        &serde_json::to_string_pretty(&record).unwrap(),
    );
    let error = fixture.fails(&["status"]);
    assert!(error.contains("unknown field"), "{error}");
}

#[test]
fn transitions_beyond_the_bound_are_partial_and_every_record_stays_byte_identical() {
    let fixture = Fixture::new();
    fixture.init();
    // The timeline lives in session.md, so the record itself must stay under the 1
    // MiB session bound; enough small entries pass the read bound's 4,096 entries.
    let count = 4_200;
    let lines: Vec<String> = (0..count)
        .map(|index| transition(Some(T + index as u64), "reset", "intent", "intent", "x"))
        .collect();
    write_transitions(&fixture, &lines);
    write_observation(&fixture, "a-1", Some("TC-a"), "local", "failed", T, None);
    write_bug(&fixture, "BUG-0001", "open", &["a-1"], json!([]));
    fixture.write(".specsync/changes/some-change/state.json", "{}\n");
    let before = record_digests(fixture.root());
    let report = timing(&fixture, &["--cycle", "current"]);
    assert!(
        report["partial"]
            .to_string()
            .contains("entries skipped beyond the 4096-entry timing bound"),
        "{}",
        report["partial"]
    );
    text(&fixture, &["--cycle", "current"]);
    assert_eq!(before, record_digests(fixture.root()));
    assert!(!fixture.exists(".chough/artifacts"));
}

/// A SpecSync stand-in: status per change from `.stub-status-<id>`, a default otherwise.
/// Every call appends its change ID to `.stub-calls`. `.stub-malformed-<id>` prints
/// output that is not JSON; `.stub-hang-<id>` waits on a background child that holds
/// the output pipes and records its PID in `.stub-children`.
#[cfg(unix)]
fn native_fixture() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(
        "bin/specsync",
        r#"#!/bin/sh
id="$3"
echo "$id" >>.stub-calls
if [ -f ".stub-fail-$id" ]; then echo 'status failed' >&2; exit 2; fi
if [ -f ".stub-malformed-$id" ]; then echo 'not json {'; exit 0; fi
if [ -f ".stub-hang-$id" ]; then sleep 60 & echo $! >>.stub-children; wait; fi
if [ -f ".stub-sleep-$id" ]; then exec sleep 30; fi
if [ -f ".stub-delay-$id" ]; then sleep "$(cat ".stub-delay-$id")"; fi
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

/// Issue #100 (LIFE-15): a native approval ledger is SpecSync's file, read within
/// SpecSync's bound and never Chough's 65,536-byte record bound, so a long ledger still
/// gives its approval times.
#[cfg(unix)]
#[test]
fn native_approval_ledgers_over_the_record_bound_still_give_their_times() {
    let fixture = native_fixture();
    let note = "x".repeat(70_000);
    let ledger = json!({
        "approvals": [
            {"gate": "definition", "actor": "agent", "timestamp": T + 100, "note": note, "digest": "abc"},
            {"gate": "definition", "actor": "agent", "timestamp": T + 200, "note": "short", "digest": "def"},
        ],
        "reopenings": [],
    })
    .to_string();
    assert!(ledger.len() > 65_536);
    fixture.write(".specsync/changes/cur-change/approvals.json", &ledger);
    let report = timing(&fixture, &[]);
    let current = &report["native"]["current"];
    assert_eq!(current["approvals_unknown"], Value::Null, "{current:#}");
    let approvals = current["approvals"].as_array().unwrap();
    assert_eq!(approvals.len(), 2, "{current:#}");
    assert_eq!(at(&approvals[1]["at"]), Some(T + 200));
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
    // A later reopening supersedes the terminal (finalization) approval only; the
    // definition approval keeps the validity native status reports.
    let gates: Vec<(Value, Value)> = newest["approvals"]
        .as_array()
        .unwrap()
        .iter()
        .map(|approval| (approval["gate"].clone(), approval["standing"].clone()))
        .collect();
    assert_eq!(
        gates,
        vec![
            (json!("definition"), json!("current")),
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

    // Changes archived on the same date are ordered by their finalization time, most
    // recent first, not by name.
    for (id, finalized) in [("aaa-later", 9_000), ("zzz-earlier", 8_000)] {
        fixture.write(
            &format!(".specsync/archive/changes/2026-02-01-{id}/finalization.json"),
            &format!("{{\"timestamp\": {finalized}}}\n"),
        );
    }
    let report = timing(&fixture, &[]);
    let history = report["native"]["history"].as_array().unwrap();
    assert_eq!(history[0]["id"], "aaa-later", "{:#}", report["native"]);
    assert_eq!(history[1]["id"], "zzz-earlier");
    assert_eq!(report["native"]["skipped_archived"], 4);

    // REQ-reporting-002: the checkpoint change, also archived, is read once and its one
    // response serves both places the report shows it.
    fixture.write(
        ".specsync/archive/changes/2026-03-01-cur-change/finalization.json",
        "{\"timestamp\": 9500}\n",
    );
    stub_calls(&fixture);
    let report = timing(&fixture, &[]);
    let calls = stub_calls(&fixture);
    assert_eq!(
        calls.iter().filter(|id| *id == "cur-change").count(),
        1,
        "{calls:?}"
    );
    assert_eq!(calls.len(), 10, "one read per distinct change: {calls:?}");
    let native = &report["native"];
    assert_eq!(native["history"][0]["id"], "cur-change", "{native:#}");
    assert_eq!(
        native["history"][0]["created"],
        native["current"]["created"]
    );
    assert_eq!(native["history"][0]["status_read"], "observed");
    let selection = &native["history_selection"];
    assert_eq!(selection["state"], "selected", "{selection:#}");
    assert_eq!(selection["filters"], json!([]));
    assert_eq!(selection["read"], 9);
    assert_eq!(selection["reused"], 1);
    assert_eq!(selection["skipped_beyond_bound"], 5);
    assert_eq!(selection["not_read_budget"], 0);
    assert_eq!(native["skipped_archived"], 5);
    let estimates = report["estimates"]["stages"].clone();
    assert!(
        text(&fixture, &[]).contains("Archived native history: read"),
        "unfiltered text names the selection"
    );
    stub_calls(&fixture);

    // A filtered report does not show archived history, so it reads only the
    // checkpoint change and says why history is absent: null, never an empty list.
    for filter in [
        ["--cycle", "current"],
        ["--stage", "test"],
        ["--case", "TC-a"],
        ["--bug", "BUG-0001"],
    ] {
        let report = timing(&fixture, &filter);
        assert_eq!(stub_calls(&fixture), vec!["cur-change"], "{filter:?}");
        let native = &report["native"];
        assert_eq!(native["history"], Value::Null, "{filter:?}: {native:#}");
        assert_eq!(native["skipped_archived"], Value::Null, "{filter:?}");
        assert_eq!(native["current"]["id"], "cur-change");
        let selection = &native["history_selection"];
        assert_eq!(selection["state"], "skipped-by-filter", "{filter:?}");
        assert!(
            selection["filters"]
                .as_array()
                .unwrap()
                .iter()
                .any(|named| named.as_str().unwrap().starts_with(filter[0])),
            "{filter:?}: {selection:#}"
        );
        assert!(
            report["partial"]
                .to_string()
                .contains("archived native history not read"),
            "{filter:?}: {}",
            report["partial"]
        );
        // Stage estimates come only from transitions.
        assert_eq!(report["estimates"]["stages"], estimates, "{filter:?}");
        let rendered = text(&fixture, &filter);
        stub_calls(&fixture);
        assert!(
            rendered.contains("Archived native history was not read"),
            "{filter:?}: {rendered}"
        );
    }
    before_filters_unchanged(&fixture);
}

/// Records stay byte-identical across the filtered and unfiltered reports above.
#[cfg(unix)]
fn before_filters_unchanged(fixture: &Fixture) {
    let before = record_digests(fixture.root());
    timing(fixture, &["--cycle", "current"]);
    timing(fixture, &[]);
    assert_eq!(before, record_digests(fixture.root()));
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

    // REQ-reporting-002: one whole-report budget. A hung read whose child holds the pipes
    // gets the remaining budget, not the 120 s per-read deadline, and every read the
    // budget leaves no time for is not-read with unknown times, never zero.
    let fixture = native_fixture();
    fixture.write(".stub-hang-cur-change", "");
    stub_calls(&fixture);
    let started = std::time::Instant::now();
    let output = run_env(
        &fixture,
        &["timing", "--json"],
        &[
            ("CHOUGH_TIMING_NATIVE_BUDGET_SECONDS", "2"),
            ("CHOUGH_NATIVE_TIMEOUT_SECONDS", "120"),
        ],
    );
    let elapsed = started.elapsed();
    let report = json_ok(&output);
    assert!(
        elapsed < std::time::Duration::from_secs(30),
        "bounded by the budget, not the per-read deadline: {elapsed:?}"
    );
    assert_eq!(stub_calls(&fixture), vec!["cur-change"]);
    assert!(
        surviving_children(&fixture).is_empty(),
        "a stand-in descendant outlived the report"
    );
    let native = &report["native"];
    assert_eq!(native["current"]["status_read"], "timed-out", "{native:#}");
    assert_eq!(native["current"]["created"]["unix_seconds"], Value::Null);
    let history = native["history"].as_array().unwrap();
    assert_eq!(history.len(), 10);
    for change in history {
        assert_eq!(change["status_read"], "not-read", "{change:#}");
        assert_eq!(change["not_read_reason"], "budget-exhausted");
        assert_eq!(change["created"]["unix_seconds"], Value::Null);
        assert!(
            change["created"]["unknown"]
                .as_str()
                .unwrap()
                .contains("budget"),
            "{change:#}"
        );
        // No status read and no local record read: every time is unknown.
        for time in ["updated", "verification", "review", "finalization"] {
            assert_eq!(
                change[time]["unix_seconds"],
                Value::Null,
                "{time}: {change:#}"
            );
            assert!(
                change[time]["unknown"].as_str().unwrap().contains("budget"),
                "{time}: {change:#}"
            );
        }
        assert_eq!(change["approvals"], json!([]));
        assert_eq!(change["reopenings"], json!([]));
        assert!(
            change["approvals_unknown"]
                .as_str()
                .unwrap()
                .contains("budget")
        );
    }
    assert_eq!(native["history_selection"]["not_read_budget"], 10);
    assert_eq!(native["history_selection"]["read"], 0);
    assert!(
        report["partial"].to_string().contains("native budget"),
        "{}",
        report["partial"]
    );

    // A per-read deadline shorter than the remaining budget still applies; hung,
    // failing and malformed responses keep their own explicit states.
    let fixture = native_fixture();
    fixture.write(".stub-hang-arch-11", "");
    fixture.write(".stub-malformed-arch-09", "");
    let report = json_ok(&run_env(
        &fixture,
        &["timing", "--json"],
        &[
            ("CHOUGH_TIMING_NATIVE_BUDGET_SECONDS", "60"),
            ("CHOUGH_NATIVE_TIMEOUT_SECONDS", "1"),
        ],
    ));
    assert!(surviving_children(&fixture).is_empty());
    let history = report["native"]["history"].as_array().unwrap();
    let state = |id: &str| {
        history
            .iter()
            .find(|change| change["id"] == id)
            .map(|change| change["status_read"].clone())
    };
    assert_eq!(state("arch-11"), Some(json!("timed-out")));
    assert_eq!(state("arch-10"), Some(json!("failed")));
    assert_eq!(state("arch-09"), Some(json!("invalid-output")));
    assert_eq!(state("arch-08"), Some(json!("observed")));
    assert_eq!(report["native"]["history_selection"]["not_read_budget"], 0);

    // Invalid budgets are refused before any native process starts; the bounds work.
    for invalid in ["0", "3601", "abc", "", "1.5", "-1"] {
        stub_calls(&fixture);
        let output = run_env(
            &fixture,
            &["timing", "--json"],
            &[("CHOUGH_TIMING_NATIVE_BUDGET_SECONDS", invalid)],
        );
        assert!(!output.status.success(), "{invalid:?} was accepted");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("CHOUGH_TIMING_NATIVE_BUDGET_SECONDS"),
            "{invalid:?}"
        );
        assert!(stub_calls(&fixture).is_empty(), "{invalid:?} ran a read");
    }
    fs::remove_file(fixture.root().join(".stub-hang-arch-11")).unwrap();
    for valid in ["1", "3600"] {
        let report = json_ok(&run_env(
            &fixture,
            &["timing", "--json"],
            &[("CHOUGH_TIMING_NATIVE_BUDGET_SECONDS", valid)],
        ));
        assert_eq!(report["native"]["history_selection"]["state"], "selected");
    }
}

/// Whether `traced` equals `plain` except that times (`unix_seconds`/`utc`) and spans
/// (`seconds`) may be later by exactly `shift` seconds, every other field equal; the
/// first other difference is returned as a JSON path.
fn differs_only_by_clock(
    path: &str,
    plain: &Value,
    traced: &Value,
    shift: u64,
) -> Result<(), String> {
    if plain == traced {
        return Ok(());
    }
    let later = |key: &str,
                 a: &serde_json::Map<String, Value>,
                 b: &serde_json::Map<String, Value>| {
        match (a[key].as_u64(), b[key].as_u64()) {
            (Some(before), Some(after)) => before + shift == after,
            _ => false,
        }
    };
    match (plain, traced) {
        (Value::Object(a), Value::Object(b)) => {
            let keys =
                |map: &serde_json::Map<String, Value>| map.keys().cloned().collect::<Vec<_>>();
            if keys(a) != keys(b) {
                return Err(format!("{path} (keys)"));
            }
            if a.contains_key("unix_seconds") && a.contains_key("utc") {
                return if later("unix_seconds", a, b) && a["unknown"] == b["unknown"] {
                    Ok(())
                } else {
                    Err(format!("{path}.unix_seconds"))
                };
            }
            if a.contains_key("seconds") && a.contains_key("label") {
                return if later("seconds", a, b)
                    && a["label"] == b["label"]
                    && a["reason"] == b["reason"]
                {
                    Ok(())
                } else {
                    Err(format!("{path}.seconds"))
                };
            }
            for (key, value) in a {
                differs_only_by_clock(&format!("{path}.{key}"), value, &b[key], shift)?;
            }
            Ok(())
        }
        (Value::Array(a), Value::Array(b)) if a.len() == b.len() => {
            for (index, (x, y)) in a.iter().zip(b).enumerate() {
                differs_only_by_clock(&format!("{path}[{index}]"), x, y, shift)?;
            }
            Ok(())
        }
        _ => Err(path.to_string()),
    }
}

#[cfg(unix)]
#[test]
fn timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit() {
    let fixture = native_fixture();
    // DRIVE-19 (#169): each run takes over a second, so the plain and traced runs always
    // straddle a second boundary; the comparison must not depend on when they ran.
    fixture.write(".stub-delay-cur-change", "1.1");
    stub_calls(&fixture);
    let plain = run_env(&fixture, &["timing", "--json"], &[]);
    let calls = stub_calls(&fixture).len();
    let traced = run_env(&fixture, &["timing", "--json"], &[("CHOUGH_TRACE", "1")]);
    stub_calls(&fixture);
    // Reports end ongoing segments at their own `generated_at`. The traced report must
    // equal the plain one except that every time and span that runs to now is later by
    // exactly the gap between the two runs; nothing else may differ (DRIVE-19, #169).
    let plain_report: Value = serde_json::from_slice(&plain.stdout).unwrap();
    let traced_report: Value = serde_json::from_slice(&traced.stdout).unwrap();
    let shift = traced_report["generated_at"]["unix_seconds"]
        .as_u64()
        .unwrap()
        - plain_report["generated_at"]["unix_seconds"]
            .as_u64()
            .unwrap();
    if let Err(path) = differs_only_by_clock("", &plain_report, &traced_report, shift) {
        panic!("traced and untraced reports differ at {path} beyond a {shift}s clock shift");
    }
    // The comparison is strict: a changed field, or a clock shift by any other amount,
    // is a difference.
    let mut changed = traced_report.clone();
    changed["filters"]["cycle"] = json!("changed");
    assert!(differs_only_by_clock("", &plain_report, &changed, shift).is_err());
    assert!(differs_only_by_clock("", &plain_report, &traced_report, shift + 1).is_err());
    assert_eq!(plain.status.code(), traced.status.code());
    assert!(plain.stderr.is_empty(), "untraced stderr is unchanged");
    let lines: Vec<String> = String::from_utf8(traced.stderr)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();
    assert!(
        lines
            .iter()
            .all(|line| line.starts_with("chough-trace: ") && line.len() <= 1024),
        "{lines:#?}"
    );
    let native: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("chough-trace: native "))
        .collect();
    assert_eq!(native.len(), calls, "{lines:#?}");
    assert!(
        native
            .iter()
            .all(|line| line.contains("specsync change status")
                && line.contains(" state=")
                && line.contains(" elapsed_ms=")),
        "{native:#?}"
    );
    assert!(native.iter().any(|line| line.contains("state=failed")));
    for phase in ["transitions", "tests", "validation", "native", "render"] {
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with(&format!("chough-trace: phase {phase} elapsed_ms="))),
            "{phase}: {lines:#?}"
        );
    }

    // Text output and a refused request keep the same output and exit code.
    for args in [&["timing"][..], &["timing", "--cycle", "current"][..]] {
        let plain = run_env(&fixture, args, &[]);
        let traced = run_env(&fixture, args, &[("CHOUGH_TRACE", "1")]);
        assert_eq!(plain.status.code(), traced.status.code(), "{args:?}");
        assert!(plain.stderr.is_empty());
    }
    let plain = run_env(
        &fixture,
        &["timing", "--json"],
        &[("CHOUGH_TIMING_NATIVE_BUDGET_SECONDS", "0")],
    );
    let traced = run_env(
        &fixture,
        &["timing", "--json"],
        &[
            ("CHOUGH_TIMING_NATIVE_BUDGET_SECONDS", "0"),
            ("CHOUGH_TRACE", "1"),
        ],
    );
    assert_eq!(plain.status.code(), traced.status.code());
    assert_eq!(plain.stdout, traced.stdout);
    assert!(!plain.status.success());
}

#[cfg(unix)]
#[test]
fn a_maximal_change_id_trace_keeps_state_and_elapsed_before_the_command() {
    // The longest change ID the CLI accepts (255 bytes) is the longest real native
    // command; its trace line keeps the outcome first and the whole command intact.
    let fixture = native_fixture();
    let id = format!("long-{}", "x".repeat(250));
    assert_eq!(id.len(), 255);
    fixture.ok(&["checkpoint", "--native-change", &id]);
    let output = run_env(
        &fixture,
        &["timing", "--json", "--cycle", "current"],
        &[("CHOUGH_TRACE", "1")],
    );
    let report = json_ok(&output);
    assert_eq!(report["native"]["current"]["status_read"], "observed");
    let stderr = String::from_utf8(output.stderr).unwrap();
    let native: Vec<&str> = stderr
        .lines()
        .filter(|line| line.starts_with("chough-trace: native "))
        .collect();
    assert_eq!(native.len(), 1, "{stderr}");
    let line = native[0];
    assert!(line.len() <= 1024, "{} bytes", line.len());
    assert!(
        line.starts_with("chough-trace: native state=observed elapsed_ms="),
        "{line}"
    );
    assert!(
        line.ends_with(&format!(" command=specsync change status {id} --json")),
        "{line}"
    );
}

#[cfg(unix)]
#[test]
fn next_status_and_stage_moves_keep_one_authoritative_native_read_outside_the_report_budget() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(".stub-count", "");
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "gate-change"]);
    fixture.no_spec_approval("gate-change");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.native_calls();
    // The report budget does not apply here: even a value timing refuses leaves next,
    // status and stage moves to their one native read.
    for budget in ["0", "1"] {
        let env = [
            ("CHOUGH_TIMING_NATIVE_BUDGET_SECONDS", budget),
            ("CHOUGH_TRACE", "1"),
        ];
        for args in [&["next", "--json"][..], &["status", "--json"][..]] {
            let output = run_env(&fixture, args, &env);
            json_ok(&output);
            assert_eq!(fixture.native_calls(), 1, "{args:?} with budget {budget}");
            let native_lines = String::from_utf8_lossy(&output.stderr)
                .lines()
                .filter(|line| line.starts_with("chough-trace: native "))
                .count();
            assert_eq!(native_lines, 1, "{args:?}");
        }
        fixture.write(".stub-fail", "");
        let refused = run_env(&fixture, &["stage", "advance"], &env);
        assert!(
            !refused.status.success(),
            "a failed native read opened a gate"
        );
        assert!(
            String::from_utf8_lossy(&refused.stderr).contains("refused:native-unknown"),
            "{}",
            String::from_utf8_lossy(&refused.stderr)
        );
        fs::remove_file(fixture.root().join(".stub-fail")).unwrap();
        fixture.native_calls();
    }
    assert_eq!(fixture.session()["stage"]["current"], "plan");
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

/// Run `examples/measure-report.sh` with `args`, the fixture's stand-ins first on PATH and
/// exactly `env` set among the native variables.
#[cfg(unix)]
fn measure(fixture: &Fixture, args: &[&Path], env: &[(&str, &str)]) -> std::process::Output {
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/measure-report.sh");
    let mut command = std::process::Command::new("sh");
    command.arg(script).args(args);
    for name in NATIVE_VARIABLES {
        command.env_remove(name);
    }
    command.envs(env.iter().copied());
    command.env(
        "PATH",
        std::env::join_paths(std::iter::once(fixture.root().join("bin")).chain(
            std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()),
        ))
        .unwrap(),
    );
    command.output().unwrap()
}

/// REQ-reporting-003, last criterion (#138, audit assertion review): a before/after
/// measurement with `examples/measure-report.sh` uses one input root and names it with its
/// revision, names both binaries by SHA-256 and version, keeps first-process and repeated
/// runs apart, reports native wait apart from local time, and turns a timed-out native read
/// into an incomplete result with no native comparison, so local work is never reported as
/// fixing native slowness.
#[cfg(unix)]
#[test]
fn measure_report_pairs_two_binaries_on_one_root_and_keeps_native_wait_apart() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("implementing");
    fixture.init();
    fixture.commit("measured root");
    let revision = String::from_utf8(
        std::process::Command::new("git")
            .arg("-C")
            .arg(fixture.root())
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let binaries = tempfile::tempdir().unwrap();
    let mut digests = vec![];
    for side in ["old", "new"] {
        let path = binaries.path().join(side);
        fs::copy(env!("CARGO_BIN_EXE_chough"), &path).unwrap();
        digests.push(format!("{:x}", Sha256::digest(fs::read(&path).unwrap())));
    }
    let (old, new) = (binaries.path().join("old"), binaries.path().join("new"));
    let root = fixture.root().canonicalize().unwrap();
    let runs = Path::new("2");

    let output = measure(&fixture, &[&old, &new, fixture.root(), runs], &[]);
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    assert!(
        output.status.success(),
        "{text}{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(
        lines[0],
        format!(
            "root {} revision {} dirty no",
            root.display(),
            revision.trim()
        ),
        "{text}"
    );
    let version = format!("version chough {}", env!("CARGO_PKG_VERSION"));
    for (side, digest) in ["old", "new"].iter().zip(&digests) {
        assert!(
            lines.contains(&format!("binary {side} sha256 {digest} {version}").as_str()),
            "{text}"
        );
    }
    let runs: Vec<&str> = lines
        .iter()
        .copied()
        .filter(|line| line.starts_with("run "))
        .collect();
    assert_eq!(runs.len(), 4, "{text}");
    for side in ["old", "new"] {
        assert!(
            runs.iter()
                .any(|line| line.starts_with(&format!("run {side} 1 first-process exit=0 "))),
            "{text}"
        );
        assert!(
            runs.iter()
                .any(|line| line.starts_with(&format!("run {side} 2 repeated exit=0 "))),
            "{text}"
        );
    }
    for line in &runs {
        for field in ["wall_ms=", "native_ms=", "local_ms="] {
            assert!(line.contains(field), "{line}");
        }
        assert!(
            line.ends_with("native_reads=1 incomplete_reads=0"),
            "{line}"
        );
    }
    assert!(
        lines
            .iter()
            .any(|line| line.starts_with("summary local old_median_ms=")),
        "{text}"
    );
    assert!(
        lines
            .iter()
            .any(|line| line.starts_with("summary native old_median_ms=")),
        "{text}"
    );
    assert!(!text.contains("incomplete:"), "{text}");

    // A native read that times out makes the pair incomplete: native wait is not compared,
    // while local time still is.
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nsleep 5\nprintf '{\"summary\":{\"id\":\"%s\",\"state\":\"implementing\",\"next_action\":\"stub\",\"approval_valid\":true,\"correction_valid\":true}}\\n' \"$3\"\n",
    );
    fixture.executable("bin/specsync");
    let output = measure(
        &fixture,
        &[&old, &new, fixture.root(), Path::new("1")],
        &[("CHOUGH_NATIVE_TIMEOUT_SECONDS", "1")],
    );
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    assert!(output.status.success(), "{text}");
    assert!(text.contains("native_timeout_seconds 1"), "{text}");
    for side in ["old", "new"] {
        assert!(
            text.lines().any(
                |line| line.starts_with(&format!("run {side} 1 first-process "))
                    && line.ends_with("native_reads=1 incomplete_reads=1")
            ),
            "{text}"
        );
    }
    assert!(
        text.contains("summary native incomplete: 2 native reads timed out or were unavailable; no native comparison"),
        "{text}"
    );
    assert!(!text.contains("summary native old_median_ms="), "{text}");
    assert!(
        text.lines()
            .any(|line| line.starts_with("summary local old_median_ms=")),
        "{text}"
    );

    // A missing binary is refused with the usage line that names the command to run.
    let output = measure(
        &fixture,
        &[&binaries.path().join("absent"), &new, fixture.root()],
        &[],
    );
    assert_eq!(output.status.code(), Some(2));
    let refusal = String::from_utf8_lossy(&output.stderr);
    assert!(
        refusal.contains("usage: sh examples/measure-report.sh OLD NEW ROOT [RUNS]"),
        "{refusal}"
    );
}
