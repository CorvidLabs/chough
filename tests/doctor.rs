//! `chough doctor` finds and fixes the Chough debt that stops a fresh worktree from
//! entering Chough (REQ-cli-055, REQ-history-012, DRIVE-17).

mod common;

use common::Fixture;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

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

/// Set session `key`'s native change and archive that change on HEAD.
fn finalized_change(fixture: &Fixture, key: &str, change: &str) {
    fixture.ok(&["session", "use", key]);
    fixture.set_front("native_change", change);
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
    finalized_change(fixture, "alpha", "ch-alpha");
    fixture.ok(&["session", "use", "beta"]);
    fixture.set_front("stage", "done");
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
            triple("finished_session", &fixture.id_of("alpha"), "found"),
            triple("finished_session", &fixture.id_of("beta"), "found"),
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
    // Committed, so archiving removes the finished sessions' folders (REQ-records-012).
    fixture.commit("finished sessions");
    no_native_reads(&fixture);
    let gamma = fixture.read(&fixture.session_file_of("gamma"));
    let (code, report) = doctor(&fixture, &["--fix"]);
    assert_eq!(code, 0, "{report:#}");
    assert_eq!(report["mode"], "fix", "{report:#}");
    assert_eq!(
        findings(&report),
        vec![
            triple("finished_session", &fixture.id_of("alpha"), "fixed"),
            triple("finished_session", &fixture.id_of("beta"), "fixed"),
            triple("stale_pointer", "ghost", "fixed"),
        ],
        "{report:#}"
    );
    assert!(fixture.is_archived("alpha") && fixture.is_archived("beta"));
    assert_eq!(fixture.session_json("alpha")["outcome"], "done");
    assert_eq!(fixture.read(&fixture.session_file_of("gamma")), gamma);
    assert!(!fixture.exists(".chough/local/active-session"));
    // With the debt gone, a fresh worktree enters Chough: gamma is the only open session.
    fixture.ok(&["checkpoint", "--decision", "gamma decision"]);
    let (code, clean) = doctor(&fixture, &[]);
    assert_eq!(code, 0, "{clean:#}");
    assert_eq!(clean["findings"], serde_json::json!([]), "{clean:#}");
    assert!(!fixture.exists(".native-calls"));
}

/// REQ-records-010: a 0.5.x `.chough/session.json` means the project is not migrated;
/// doctor reports it with `chough migrate` as the fix, never migrates itself, and its
/// --fix changes nothing.
#[cfg(unix)]
#[test]
fn doctor_reports_an_unmigrated_checkpoint_and_never_migrates() {
    let fixture = Fixture::new();
    fixture.write(".chough/session.json", "{\"schema_version\": 2}\n");
    no_native_reads(&fixture);
    let before = snapshot(&fixture);
    let (code, report) = doctor(&fixture, &[]);
    assert_eq!(code, 1, "{report:#}");
    assert_eq!(report["findings"][0]["fix"], "chough migrate", "{report:#}");
    let (code, fixed) = doctor(&fixture, &["--fix"]);
    assert_eq!(code, 1, "{fixed:#}");
    // Only Git-ignored local state (the writer lock) may appear.
    let mut after = snapshot(&fixture);
    after.retain(|path, _| !path.starts_with(".chough/local/"));
    assert_eq!(after, before);
    assert!(fixture.fails(&["next"]).contains("refused:unmigrated"));
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
    finalized_change(&fixture, "alpha", "ch-alpha");
    no_native_reads(&fixture);
    let alpha = fixture.read(&fixture.session_file_of("alpha"));
    let gamma = fixture.read(&fixture.session_file_of("gamma"));

    let (code, skipped) = doctor(&fixture, &["--fix", "--close", "gamma"]);
    assert_eq!(code, 1, "{skipped:#}");
    assert_eq!(
        findings(&skipped),
        vec![
            triple("stale_gates", &fixture.id_of("alpha"), "skipped"),
            triple("unfinished_session", &fixture.id_of("gamma"), "skipped"),
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
    assert_eq!(fixture.read(&fixture.session_file_of("alpha")), alpha);
    assert_eq!(fixture.read(&fixture.session_file_of("gamma")), gamma);

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
            triple("finished_session", &fixture.id_of("alpha"), "fixed"),
            triple("stale_gates", &fixture.id_of("alpha"), "fixed"),
            triple("unfinished_session", &fixture.id_of("gamma"), "fixed"),
        ],
        "{fixed:#}"
    );
    let archived = fixture.session_json("alpha");
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
    let closed = fixture.session_json("gamma");
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
    assert!(!fixture.is_archived("delta"));
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
    finalized_change(&fixture, "alpha", "ch-alpha");
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
    let alpha = fixture.read(&fixture.session_file_of("alpha"));
    let (code, report) = doctor(&fixture, &["--fix"]);
    assert_eq!(code, 1, "{report:#}");
    assert_eq!(
        findings(&report),
        vec![triple(
            "finished_session",
            &fixture.id_of("alpha"),
            "skipped"
        )],
        "{report:#}"
    );
    assert!(
        report["findings"][0]["fix"]
            .as_str()
            .is_some_and(|fix| fix.contains("chough inbox fold")),
        "{report:#}"
    );
    assert_eq!(fixture.read(&fixture.session_file_of("alpha")), alpha);
    assert!(!fixture.exists(".native-calls"));
}

/// A `.specsync/sdd.json` that lists `entries` in `verification_ignored_paths`.
fn policy(entries: &[&str]) -> String {
    format!(
        "{{\n  \"version\": 2,\n  \"enabled\": true,\n  \"verification_ignored_paths\": {}\n}}\n",
        serde_json::to_string(entries).unwrap()
    )
}

/// The `records_in_verification` finding of `report`, if any.
fn exclusions_finding(report: &Value) -> Option<Value> {
    report["findings"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|finding| finding["kind"] == "records_in_verification")
        .cloned()
}

/// REQ-records-014, TC-records-doctor-names-missing-exclusions: with a record prefix
/// missing from `verification_ignored_paths`, `chough doctor` reports
/// `records_in_verification` naming the missing entries, the SpecSync version and that
/// adding them is an ordinary reviewed change, and writes nothing, even with `--fix`;
/// with all three listed it reports nothing; `.specsync/sdd.json` is byte-identical after
/// every Chough command.
#[test]
fn doctor_names_the_missing_verification_exclusions() {
    let fixture = Fixture::new();
    fixture.init();
    // No policy: no SDD verification for Chough's records to stale, so no finding.
    let (code, report) = doctor(&fixture, &[]);
    assert_eq!(code, 0, "{report:#}");
    assert!(exclusions_finding(&report).is_none(), "{report:#}");

    let partial = policy(&[".chough/sessions/"]);
    fixture.write(".specsync/sdd.json", &partial);
    // `--fix` takes the project lock in the Git-ignored `.chough/local/`; records only.
    let records = || {
        let mut files = snapshot(&fixture);
        files.retain(|name, _| !name.starts_with(".chough/local/"));
        files
    };
    let before = records();
    for args in [&[][..], &["--dry-run"][..], &["--fix"][..]] {
        let (code, report) = doctor(&fixture, args);
        assert_eq!(code, 1, "{args:?}: {report:#}");
        let finding = exclusions_finding(&report).unwrap_or_else(|| panic!("{report:#}"));
        assert_eq!(finding["target"], ".specsync/sdd.json", "{finding:#}");
        assert_eq!(
            finding["missing"],
            serde_json::json!([".chough/archive/", ".chough/bugs/"]),
            "{finding:#}"
        );
        assert_eq!(finding["specsync"], "6.1.2 and later", "{finding:#}");
        let advice = finding["advice"].as_str().unwrap_or_default();
        assert!(
            advice.contains(".chough/archive/, .chough/bugs/")
                && advice.contains("ordinary reviewed change")
                && advice.contains("never writes"),
            "{advice}"
        );
        let status = if args == ["--fix"] {
            "skipped"
        } else {
            "found"
        };
        assert_eq!(finding["status"], status, "{finding:#}");
        assert_eq!(fixture.read(".specsync/sdd.json"), partial, "{args:?}");
        assert_eq!(records(), before, "{args:?} wrote a record under .chough/");
    }

    // A broader entry hides case definitions too; the finding says so.
    fixture.write(".specsync/sdd.json", &policy(&[".chough/"]));
    let (_, report) = doctor(&fixture, &[]);
    let advice = exclusions_finding(&report).unwrap_or_else(|| panic!("{report:#}"))["advice"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    assert!(
        advice.contains(".chough/ also leaves .chough/cases/"),
        "{advice}"
    );

    let complete = policy(&[".chough/archive/", ".chough/bugs/", ".chough/sessions/"]);
    fixture.write(".specsync/sdd.json", &complete);
    let (code, report) = doctor(&fixture, &[]);
    assert!(exclusions_finding(&report).is_none(), "{report:#}");
    assert_eq!(code, 0, "{report:#}");
    for args in [
        &["next", "--json"][..],
        &["status", "--json"][..],
        &["context", "--json"][..],
        &["checkpoint", "--completed", "SYNTHETIC: a step"][..],
        &["doctor", "--fix"][..],
    ] {
        fixture.run(args);
        assert_eq!(fixture.read(".specsync/sdd.json"), complete, "{args:?}");
    }
}

/// REQ-records-014: `chough next` says when Chough's writes are safe, at any point with
/// the three entries in force and commit-first without them, and `chough init` prints the
/// doctor's advice while an entry is missing.
#[test]
fn next_says_when_chough_writes_are_safe_and_init_gives_the_advice() {
    let fixture = Fixture::new();
    fixture.write(".specsync/sdd.json", &policy(&[]));
    let init = fixture.ok(&[
        "init",
        "--objective",
        "SYNTHETIC objective",
        "--intent-scope",
        "LIFE",
    ]);
    let advice = init["records_in_verification"].as_str().unwrap_or_default();
    assert!(
        advice.contains(".chough/sessions/, .chough/archive/, .chough/bugs/")
            && advice.contains("6.1.2 and later")
            && advice.contains("ordinary reviewed change"),
        "{init:#}"
    );
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["chough_writes"]["state"], "commit-first", "{next:#}");
    let text = String::from_utf8_lossy(&fixture.run(&["next"]).stdout).into_owned();
    assert!(
        text.contains("Chough writes: commit Chough's records before `specsync change check --commit` and write nothing under .chough/ until finalize"),
        "{text}"
    );

    fixture.write(
        ".specsync/sdd.json",
        &policy(&[".chough/archive/", ".chough/bugs/", ".chough/sessions/"]),
    );
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["chough_writes"]["state"], "any-point", "{next:#}");
    let text = String::from_utf8_lossy(&fixture.run(&["next"]).stdout).into_owned();
    assert!(
        text.contains("may be written and committed at any point of the native order"),
        "{text}"
    );
    let fresh = Fixture::new();
    fresh.write(
        ".specsync/sdd.json",
        &policy(&[".chough/archive/", ".chough/bugs/", ".chough/sessions/"]),
    );
    let init = fresh.ok(&["init"]);
    assert!(init["records_in_verification"].is_null(), "{init:#}");
}
