//! `chough doctor` finds and fixes the Chough debt that stops a fresh worktree from
//! entering Chough (REQ-cli-055, REQ-history-012, DRIVE-17).

mod common;

use common::Fixture;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn json(fixture: &Fixture, name: &str) -> Value {
    serde_json::from_str(&fixture.read(name)).unwrap()
}

/// Run `chough doctor` with `args`, returning its exit code and JSON report.
fn doctor(fixture: &Fixture, args: &[&str]) -> (i32, Value) {
    let mut all = vec!["doctor"];
    all.extend_from_slice(args);
    let output = fixture.run(&all);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let report = serde_json::from_str(&stdout).unwrap_or_else(|error| {
        panic!(
            "{all:?}: {error}: stdout {stdout} stderr {}",
            String::from_utf8_lossy(&output.stderr)
        )
    });
    (output.status.code().unwrap_or(-1), report)
}

/// The findings of `report` as (kind, target, status) triples, sorted.
fn findings(report: &Value) -> Vec<(String, String, String)> {
    let mut found: Vec<_> = report["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("{report:#}"))
        .iter()
        .map(|finding| {
            (
                finding["kind"].as_str().unwrap().to_string(),
                finding["target"].as_str().unwrap().to_string(),
                finding["status"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    found.sort();
    found
}

fn triple(kind: &str, target: &str, status: &str) -> (String, String, String) {
    (kind.to_string(), target.to_string(), status.to_string())
}

/// Every file under `.chough/` with its bytes, so a test can show nothing changed.
fn snapshot(fixture: &Fixture) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                let name = path.strip_prefix(base).unwrap().display().to_string();
                out.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    let root = fixture.root();
    walk(root, &root.join(".chough"), &mut out);
    out
}

/// A `specsync` on PATH that records any call and fails, so a test sees a native read.
#[cfg(unix)]
fn no_native_reads(fixture: &Fixture) {
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\necho \"$*\" >>.native-calls\nexit 2\n",
    );
    fixture.executable("bin/specsync");
}

/// Set a checkpoint file's native change and archive that change on HEAD.
fn finalized_change(fixture: &Fixture, file: &str, change: &str) {
    let mut session = json(fixture, file);
    session["native_change"] = Value::String(change.to_string());
    fixture.write(
        file,
        &(serde_json::to_string_pretty(&session).unwrap() + "\n"),
    );
    fixture.write(
        &format!(".specsync/archive/changes/2026-11-06-{change}/finalization.json"),
        &format!("{{\"schema_version\": 2, \"change_id\": \"{change}\", \"timestamp\": 700}}\n"),
    );
}

/// Alpha's native change is archived at stage intent; beta reached done; gamma is open
/// work; the pointer names a session that does not exist.
fn indebted(fixture: &Fixture) {
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&["session", "new", "beta", "--objective", "Ship beta"]);
    fixture.ok(&["session", "new", "gamma", "--objective", "Ship gamma"]);
    finalized_change(fixture, ".chough/sessions/alpha.json", "ch-alpha");
    let mut beta = json(fixture, ".chough/sessions/beta.json");
    beta["stage"]["current"] = Value::String("done".to_string());
    fixture.write(
        ".chough/sessions/beta.json",
        &(serde_json::to_string_pretty(&beta).unwrap() + "\n"),
    );
    fixture.write(".chough/local/active-session", "ghost\n");
}

/// DRIVE-17: with no flag the doctor reports each finding and its fix, as `--dry-run`
/// does, changes nothing, makes no native read, and exits 1 while there is debt.
#[cfg(unix)]
#[test]
fn doctor_reports_debt_and_changes_nothing() {
    let fixture = Fixture::new();
    indebted(&fixture);
    no_native_reads(&fixture);
    let before = snapshot(&fixture);
    let (code, report) = doctor(&fixture, &[]);
    assert_eq!(code, 1, "{report:#}");
    assert_eq!(report["mode"], "report", "{report:#}");
    assert_eq!(
        findings(&report),
        vec![
            triple("finished_session", "alpha", "found"),
            triple("finished_session", "beta", "found"),
            triple("stale_pointer", "ghost", "found"),
        ],
        "{report:#}"
    );
    for finding in report["findings"].as_array().unwrap() {
        assert!(
            finding["fix"]
                .as_str()
                .is_some_and(|fix| fix.starts_with("chough ")),
            "{finding:#}"
        );
        assert_eq!(finding["decision"], false, "{finding:#}");
    }
    let (code, dry) = doctor(&fixture, &["--dry-run"]);
    assert_eq!(code, 1, "{dry:#}");
    assert_eq!(dry["mode"], "dry-run", "{dry:#}");
    assert_eq!(findings(&dry), findings(&report), "{dry:#}");
    assert_eq!(snapshot(&fixture), before);
    assert!(!fixture.exists(".native-calls"));
}

/// DRIVE-17: `--fix` archives finished sessions byte for byte and removes the stale
/// pointer; open work stays; a second run is clean and exits 0.
#[cfg(unix)]
#[test]
fn doctor_fix_archives_finished_sessions_and_removes_a_stale_pointer() {
    let fixture = Fixture::new();
    indebted(&fixture);
    no_native_reads(&fixture);
    let alpha = fixture.read(".chough/sessions/alpha.json");
    let beta = fixture.read(".chough/sessions/beta.json");
    let gamma = fixture.read(".chough/sessions/gamma.json");
    let (code, report) = doctor(&fixture, &["--fix"]);
    assert_eq!(code, 0, "{report:#}");
    assert_eq!(report["mode"], "fix", "{report:#}");
    assert_eq!(
        findings(&report),
        vec![
            triple("finished_session", "alpha", "fixed"),
            triple("finished_session", "beta", "fixed"),
            triple("stale_pointer", "ghost", "fixed"),
        ],
        "{report:#}"
    );
    assert_eq!(fixture.read(".chough/sessions/archive/alpha.json"), alpha);
    assert_eq!(fixture.read(".chough/sessions/archive/beta.json"), beta);
    assert_eq!(fixture.read(".chough/sessions/gamma.json"), gamma);
    assert!(!fixture.exists(".chough/local/active-session"));
    // With the debt gone, a fresh worktree enters Chough: gamma is the only open session.
    fixture.ok(&["checkpoint", "--decision", "gamma decision"]);
    let (code, clean) = doctor(&fixture, &[]);
    assert_eq!(code, 0, "{clean:#}");
    assert_eq!(clean["findings"], serde_json::json!([]), "{clean:#}");
    assert!(!fixture.exists(".native-calls"));
}

/// DRIVE-17: a legacy `.chough/session.json` is adopted under its native change's ID,
/// adopting history first when it has no cursor; one with no native change needs
/// `--adopt-as <key>` and is otherwise reported as skipped with that fix.
#[cfg(unix)]
#[test]
fn doctor_fix_adopts_a_legacy_checkpoint() {
    let fixture = Fixture::new();
    fixture.init();
    let mut legacy = json(&fixture, ".chough/session.json");
    legacy["native_change"] = Value::String("ch-legacy".to_string());
    fixture.write(
        ".chough/session.json",
        &(serde_json::to_string_pretty(&legacy).unwrap() + "\n"),
    );
    no_native_reads(&fixture);
    let (code, report) = doctor(&fixture, &[]);
    assert_eq!(code, 1, "{report:#}");
    assert_eq!(
        findings(&report),
        vec![triple("legacy_checkpoint", ".chough/session.json", "found")],
        "{report:#}"
    );
    let (code, fixed) = doctor(&fixture, &["--fix"]);
    assert_eq!(code, 0, "{fixed:#}");
    assert!(!fixture.exists(".chough/session.json"));
    let adopted = json(&fixture, ".chough/sessions/ch-legacy.json");
    assert!(adopted["history_cursor"].is_object(), "{adopted:#}");
    assert_eq!(adopted["native_change"], "ch-legacy");
    assert_eq!(
        fixture.read(".chough/local/active-session").trim(),
        "ch-legacy"
    );

    let bare = Fixture::new();
    bare.init();
    no_native_reads(&bare);
    let before = bare.read(".chough/session.json");
    let (code, skipped) = doctor(&bare, &["--fix"]);
    assert_eq!(code, 1, "{skipped:#}");
    assert_eq!(
        findings(&skipped),
        vec![triple(
            "legacy_checkpoint",
            ".chough/session.json",
            "skipped"
        )],
        "{skipped:#}"
    );
    assert!(
        skipped["findings"][0]["fix"]
            .as_str()
            .is_some_and(|fix| fix.contains("--adopt-as")),
        "{skipped:#}"
    );
    assert_eq!(bare.read(".chough/session.json"), before);
    let (code, adopted) = doctor(&bare, &["--fix", "--adopt-as", "legacy"]);
    assert_eq!(code, 0, "{adopted:#}");
    assert!(bare.exists(".chough/sessions/legacy.json"));
    assert!(!bare.exists(".native-calls"));
    assert!(!fixture.exists(".native-calls"));
}

/// DRIVE-17: withdrawing a stale gate and closing unfinished work are decisions. Without
/// `--reason` the doctor skips them and reports them as skipped, still applying the
/// other fixes; with `--reason` it withdraws the gate without recording a human
/// decision and closes the named session with the reason.
#[cfg(unix)]
#[test]
fn doctor_decision_fixes_need_a_reason_and_record_no_human_decision() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Native approval of ch-alpha",
    ]);
    fixture.ok(&["session", "new", "gamma", "--objective", "Abandoned work"]);
    fixture.ok(&["session", "new", "delta", "--objective", "Live work"]);
    finalized_change(&fixture, ".chough/sessions/alpha.json", "ch-alpha");
    no_native_reads(&fixture);
    let alpha = fixture.read(".chough/sessions/alpha.json");
    let gamma = fixture.read(".chough/sessions/gamma.json");

    let (code, skipped) = doctor(&fixture, &["--fix", "--close", "gamma"]);
    assert_eq!(code, 1, "{skipped:#}");
    assert_eq!(
        findings(&skipped),
        vec![
            triple("stale_gates", "alpha", "skipped"),
            triple("unfinished_session", "gamma", "skipped"),
        ],
        "{skipped:#}"
    );
    for finding in skipped["findings"].as_array().unwrap() {
        assert_eq!(finding["decision"], true, "{finding:#}");
        assert!(
            finding["fix"]
                .as_str()
                .is_some_and(|fix| fix.contains("--reason")),
            "{finding:#}"
        );
    }
    assert_eq!(fixture.read(".chough/sessions/alpha.json"), alpha);
    assert_eq!(fixture.read(".chough/sessions/gamma.json"), gamma);

    let (code, fixed) = doctor(
        &fixture,
        &[
            "--fix",
            "--close",
            "gamma",
            "--reason",
            "Superseded; ch-alpha is archived",
        ],
    );
    assert_eq!(code, 0, "{fixed:#}");
    assert_eq!(
        findings(&fixed),
        vec![
            triple("finished_session", "alpha", "fixed"),
            triple("stale_gates", "alpha", "fixed"),
            triple("unfinished_session", "gamma", "fixed"),
        ],
        "{fixed:#}"
    );
    let archived = json(&fixture, ".chough/sessions/archive/alpha.json");
    assert_eq!(
        archived["awaiting_human"],
        serde_json::json!([]),
        "{archived:#}"
    );
    // Withdrawing a gate records no human decision.
    assert!(
        !archived["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|decision| decision
                .as_str()
                .is_some_and(|text| text.contains("ch-alpha"))),
        "{archived:#}"
    );
    let closed = json(&fixture, ".chough/sessions/archive/gamma.json");
    assert!(
        closed["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|decision| decision
                .as_str()
                .is_some_and(|text| text.contains("Superseded; ch-alpha is archived"))),
        "{closed:#}"
    );
    assert!(fixture.exists(".chough/sessions/delta.json"));
    assert!(!fixture.exists(".native-calls"));
}

/// DRIVE-17: a finished session with unfolded inbox entries is not archived, so its
/// entries are not stranded; it is reported as skipped, naming `chough inbox fold`.
#[cfg(unix)]
#[test]
fn doctor_fix_keeps_a_finished_session_with_unfolded_entries_open() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&["session", "new", "beta", "--objective", "Ship beta"]);
    finalized_change(&fixture, ".chough/sessions/alpha.json", "ch-alpha");
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        "alpha",
        "--from",
        "worker",
        "--kind",
        "decision",
        "--text",
        "Fold me first",
    ]);
    no_native_reads(&fixture);
    let alpha = fixture.read(".chough/sessions/alpha.json");
    let (code, report) = doctor(&fixture, &["--fix"]);
    assert_eq!(code, 1, "{report:#}");
    assert_eq!(
        findings(&report),
        vec![triple("finished_session", "alpha", "skipped")],
        "{report:#}"
    );
    assert!(
        report["findings"][0]["fix"]
            .as_str()
            .is_some_and(|fix| fix.contains("chough inbox fold")),
        "{report:#}"
    );
    assert_eq!(fixture.read(".chough/sessions/alpha.json"), alpha);
    assert!(!fixture.exists(".native-calls"));
}
