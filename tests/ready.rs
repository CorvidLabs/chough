//! REQ-cli-067 (DRIVE-29): `chough change ready` lists everything still missing
//! and names the command that fixes each piece. Unix only: the stand-in is a shell script.
#![cfg(unix)]

mod common;

use common::Fixture;
use sha2::{Digest, Sha256};
use std::process::Command;

const ID: &str = "demo";
const STUB: &str = r#"#!/bin/sh
set -eu
if [ "${1:-}" = change ] && [ "${2:-}" = show ]; then
  cat ".specsync/changes/$3/show.json"
  exit 0
fi
if [ "${1:-}" = check ]; then
  printf 'dirty\n' >.specsync/hashes.json
  # Another process replacing the cache while the check runs (the review of #311).
  case "${READY_TEST_CACHE:-}" in
    regular)
      printf 'SECOND_WRITER\n' >.specsync/second
      mv .specsync/second .specsync/hashes.json
      ;;
    symlink)
      rm .specsync/hashes.json
      ln -s ../specs/cli/requirements.md .specsync/hashes.json
      ;;
  esac
  name=$(ls -1 .specsync/changes | head -n 1)
  cat ".specsync/changes/$name/check.json"
  exit 0
fi
printf '{}\n'
"#;
const OLD_DELTA: &str = "## MODIFIED\n\n### REQUIREMENT REQ-cli-001\n\nOld line.\n";
const OLD_REQ: &str = "### REQ-cli-001\n\nOld line.\n";
const NEW_DELTA: &str = "## MODIFIED\n\n### REQUIREMENT REQ-cli-001\n\nNew line.\n";
const NEW_REQ: &str = "### REQ-cli-001\n\nNew line.\n";

fn project(requirements: &str, paths: &[&str]) -> Fixture {
    let fixture = Fixture::new();
    fixture.write("specs/cli/requirements.md", requirements);
    fixture.write("bin/specsync", STUB);
    fixture.executable("bin/specsync");
    fixture.write(
        ".specsync/sdd.json",
        "{\"verification_ignored_paths\":[\".chough/archive/\",\".chough/bugs/\",\".chough/sessions/\"]}\n",
    );
    fixture.write(".specsync/hashes.json", "original\n");
    fixture.commit("base");
    let base = head(&fixture);
    write_change(&fixture, &base, "draft", paths, false);
    fixture
}

fn write_change(fixture: &Fixture, base: &str, state: &str, paths: &[&str], no_spec: bool) {
    let specs = if no_spec { "[]" } else { "[\"cli\"]" };
    let paths = serde_json::to_string(paths).unwrap();
    let no_spec = if no_spec { "true" } else { "false" };
    fixture.write(
        &format!(".specsync/changes/{ID}/state.json"),
        &format!(
            "{{\"state\":\"{state}\",\"base_commit\":\"{base}\",\"affected_specs\":{specs},\"affected_paths\":{paths},\"no_spec_change\":{no_spec},\"selected_artifacts\":[\"context\"]}}\n"
        ),
    );
    fixture.write(
        &format!(".specsync/changes/{ID}/show.json"),
        "{\"questions\":[]}\n",
    );
    fixture.write(
        &format!(".specsync/changes/{ID}/check.json"),
        "{\"warnings\":[]}\n",
    );
    fixture.write(
        &format!(".specsync/changes/{ID}/deltas/cli.md"),
        "## ADDED\n\n### REQUIREMENT REQ-cli-067\n\nReadiness.\n",
    );
    fixture.write(
        &format!(".specsync/changes/{ID}/context.md"),
        "The change adds a readiness report.\n",
    );
}

fn head(fixture: &Fixture) -> String {
    let output = Command::new("git")
        .current_dir(fixture.root())
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn porcelain(fixture: &Fixture) -> String {
    let output = Command::new("git")
        .current_dir(fixture.root())
        .args(["status", "--porcelain"])
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

fn report(findings: &[(&str, &str)]) -> String {
    let mut out = format!("chough change ready {ID}\n\n");
    if findings.is_empty() {
        out.push_str("Nothing is missing.\n");
        return out;
    }
    for (text, fix) in findings {
        out.push_str(&format!("- {text}\n  Fix: `{fix}`\n"));
    }
    out.push('\n');
    let noun = if findings.len() == 1 {
        "finding"
    } else {
        "findings"
    };
    out.push_str(&format!("{} {noun}.\n", findings.len()));
    out
}

fn assert_report(fixture: &Fixture, findings: &[(&str, &str)]) {
    let before = porcelain(fixture);
    let hashes = fixture.read(".specsync/hashes.json");
    let output = fixture.run(&["change", "ready", ID]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(stdout, report(findings), "stderr:\n{stderr}");
    assert!(stderr.is_empty(), "{stderr}");
    let code = if findings.is_empty() { 0 } else { 1 };
    assert_eq!(output.status.code(), Some(code));
    assert_eq!(fixture.read(".specsync/hashes.json"), hashes);
    assert_eq!(porcelain(fixture), before);
}

fn frame(tag: &[u8], value: &[u8], hasher: &mut Sha256) {
    hasher.update((tag.len() as u64).to_be_bytes());
    hasher.update(tag);
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
}

fn framed(domain: &[u8], fields: &[(&[u8], &[u8])]) -> String {
    let mut hasher = Sha256::new();
    frame(b"domain", domain, &mut hasher);
    for (tag, value) in fields {
        frame(tag, value, &mut hasher);
    }
    format!("{:x}", hasher.finalize())
}

fn delta_digest(body: &str) -> String {
    framed(
        b"specsync.approved-delta.v1",
        &[(b"module", b"cli"), (b"body", body.as_bytes())],
    )
}

fn base_digest(requirements: &str) -> String {
    let block = requirements.trim_matches(['\n', ' ', '\t']);
    framed(
        b"specsync.approved-delta-base.v1",
        &[
            (b"key", b"cli: REQUIREMENT REQ-cli-001"),
            (b"block", block.as_bytes()),
        ],
    )
}

fn approve(fixture: &Fixture, delta: &str, requirements: &str) {
    let body = delta_digest(delta);
    let base = base_digest(requirements);
    fixture.write(
        &format!(".specsync/changes/{ID}/approvals.json"),
        &format!(
            "{{\"approvals\":[{{\"gate\":\"definition\",\"approved_delta_digests\":{{\"cli\":\"{body}\"}},\"approved_delta_bases\":{{\"cli: REQUIREMENT REQ-cli-001\":\"{base}\"}}}}]}}\n"
        ),
    );
}

#[test]
fn a_draft_names_each_missing_fix_and_passes_once_they_are_fixed() {
    let fixture = project(OLD_REQ, &["src/"]);
    std::fs::remove_file(
        fixture
            .root()
            .join(format!(".specsync/changes/{ID}/deltas/cli.md")),
    )
    .unwrap();
    fixture.write(
        &format!(".specsync/changes/{ID}/deltas/notes.md"),
        "extra\n",
    );
    fixture.write(
        &format!(".specsync/changes/{ID}/show.json"),
        "{\"questions\":[{\"id\":\"acceptance_criteria\"}]}\n",
    );
    fixture.write(&format!(".specsync/changes/{ID}/context.md"), "TODO\n");
    assert_report(
        &fixture,
        &[
            (
                "missing delta for cli",
                "write .specsync/changes/demo/deltas/cli.md",
            ),
            (
                "extra delta notes.md",
                "remove .specsync/changes/demo/deltas/notes.md",
            ),
            (
                "unanswered question acceptance_criteria",
                "chough change answer demo acceptance_criteria \"<answer>\"",
            ),
            (
                ".specsync/changes/demo/context.md is still a TODO placeholder",
                "complete .specsync/changes/demo/context.md",
            ),
        ],
    );
    fixture.write(
        &format!(".specsync/changes/{ID}/deltas/cli.md"),
        "## ADDED\n\n### REQUIREMENT REQ-cli-067\n\nReadiness.\n",
    );
    std::fs::remove_file(
        fixture
            .root()
            .join(format!(".specsync/changes/{ID}/deltas/notes.md")),
    )
    .unwrap();
    fixture.write(
        &format!(".specsync/changes/{ID}/show.json"),
        "{\"questions\":[]}\n",
    );
    fixture.write(
        &format!(".specsync/changes/{ID}/context.md"),
        "The change adds a readiness report.\n",
    );
    assert_report(&fixture, &[]);
}

#[test]
fn a_stale_delta_is_named_until_the_body_is_rewritten() {
    assert_eq!(
        delta_digest(OLD_DELTA),
        "26e8d49573f595e176975eb425d1e937808a08a86943633e66461a0d56cc9b07"
    );
    assert_eq!(
        base_digest(OLD_REQ),
        "1d58afd5faaa83ca253b19d8f3031eeb35e76ba2aa45c824f15fd062300aaf34"
    );
    let fixture = project(OLD_REQ, &["specs/cli/requirements.md"]);
    fixture.write(&format!(".specsync/changes/{ID}/deltas/cli.md"), OLD_DELTA);
    approve(&fixture, OLD_DELTA, OLD_REQ);
    fixture.write("specs/cli/requirements.md", NEW_REQ);
    assert_report(
        &fixture,
        &[(
            "stale canonical text for cli REQ-cli-001",
            "rewrite .specsync/changes/demo/deltas/cli.md and then chough change ready demo",
        )],
    );
    fixture.write(&format!(".specsync/changes/{ID}/deltas/cli.md"), NEW_DELTA);
    assert_report(&fixture, &[]);
}

#[test]
fn a_delta_that_already_carries_the_canonical_move_is_not_named() {
    let committed = "### REQ-cli-001\n\nalpha\ntail\n";
    let current = "### REQ-cli-001\n\nalpha\ngamma\ntail\n";
    let delta = "## MODIFIED\n\n### REQUIREMENT REQ-cli-001\n\nalpha\ngamma\ntail\nbeta\n";
    let fixture = project(committed, &["specs/cli/requirements.md"]);
    fixture.write(&format!(".specsync/changes/{ID}/deltas/cli.md"), delta);
    approve(&fixture, delta, committed);
    fixture.write("specs/cli/requirements.md", current);
    assert_report(&fixture, &[]);
}

#[test]
fn an_undocumented_export_in_a_declared_file_is_named() {
    let fixture = project(OLD_REQ, &["src/lib.rs"]);
    fixture.write(
        &format!(".specsync/changes/{ID}/check.json"),
        "{\"warnings\":[\"Undocumented export 'hidden' from src/lib.rs\",\"Undocumented export 'other' from src/other.rs\"]}\n",
    );
    assert_report(
        &fixture,
        &[(
            "undocumented export 'hidden' from src/lib.rs",
            "document hidden in specs/cli/cli.spec.md",
        )],
    );
}

#[test]
fn a_case_that_links_another_modules_requirement_is_named() {
    let path = ".chough/cases/cli.cross.cases.toml";
    let fixture = project(OLD_REQ, &[path]);
    fixture.write(
        path,
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-own\"\nrequirements = [\"REQ-cli-001\"]\n\n[[case]]\nid = \"TC-cross\"\nrequirements = [\"REQ-records-002\", \"REQ-cli-002\"]\n",
    );
    assert_report(
        &fixture,
        &[(
            "TC-cross links REQ-records-002, owned by records",
            "move TC-cross to .chough/cases/records.cross.cases.toml",
        )],
    );
}

#[test]
fn a_changed_file_outside_the_declared_paths_is_named() {
    let fixture = project(OLD_REQ, &["src/"]);
    let base = head(&fixture);
    fixture.write("notes.txt", "note\n");
    fixture.write("src/owned.rs", "fn owned() {}\n");
    fixture.write(".chough/sessions/note.md", "session\n");
    write_change(&fixture, &base, "Draft", &["src/"], false);
    assert_report(
        &fixture,
        &[(
            "changed file notes.txt is outside the declared paths",
            "chough change answer demo affected_paths \"src/,notes.txt\"",
        )],
    );
    write_change(&fixture, &base, "approved", &["src/"], false);
    assert_report(
        &fixture,
        &[(
            "changed file notes.txt is outside the declared paths",
            "specsync change send-back demo --actor <name> --reason \"notes.txt\"",
        )],
    );
}

#[test]
fn a_no_spec_change_names_every_delta_for_removal() {
    let fixture = project(OLD_REQ, &["src/"]);
    let base = head(&fixture);
    write_change(&fixture, &base, "draft", &["src/"], true);
    assert_report(
        &fixture,
        &[(
            "delta cli.md is present on a change that declares no spec change",
            "remove .specsync/changes/demo/deltas/cli.md",
        )],
    );
}

#[test]
fn an_unknown_change_names_status() {
    let fixture = Fixture::new();
    for id in ["missing", "../secret"] {
        let output = fixture.run(&["change", "ready", id]);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        // An id that is not a valid change id is not shown as typed.
        let named = if id == "missing" {
            format!("no change `{id}` to read")
        } else {
            "no change to read: that is not a valid change id".to_string()
        };
        assert_eq!(stderr, format!("{named}\nFix: `chough change status`\n"));
    }
}

#[test]
fn a_missing_specsync_names_the_install_command() {
    let fixture = project(OLD_REQ, &["src/"]);
    std::fs::remove_file(
        fixture
            .root()
            .join(format!(".specsync/changes/{ID}/deltas/cli.md")),
    )
    .unwrap();
    let before = porcelain(&fixture);
    let hashes = fixture.read(".specsync/hashes.json");
    let output = fixture.run_with_env(&["change", "ready", ID], &[("PATH", "/usr/bin:/bin")]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        stdout,
        report(&[
            (
                "missing delta for cli",
                "write .specsync/changes/demo/deltas/cli.md",
            ),
            (
                "specsync cannot be started, so interview questions and undocumented exports were not read",
                "cargo install --locked specsync@6.3.0",
            ),
        ]),
        "stderr:\n{stderr}"
    );
    assert!(stderr.is_empty(), "{stderr}");
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        stdout
            .matches("cargo install --locked specsync@6.3.0")
            .count(),
        1
    );
    assert!(!stdout.contains("unanswered"));
    assert_eq!(fixture.read(".specsync/hashes.json"), hashes);
    assert_eq!(porcelain(&fixture), before);
}

/// REQ-cli-067 (#363): after another branch is merged in, the files that merge brought
/// are not the change's own, so they are not named as outside the declared paths. A
/// file the change's own commit touches outside its paths still is, and so is one left
/// uncommitted.
#[test]
fn files_a_merge_brought_in_are_not_outside_the_declared_paths() {
    let fixture = project(OLD_REQ, &["src/"]);
    let base = head(&fixture);
    // The change's own work, committed.
    fixture.write("src/owned.rs", "fn owned() {}\n");
    fixture.commit("the change's own work");
    let work = head(&fixture);
    // Another branch, from the base, adds files of its own; the work branch merges it.
    fixture.git(&["checkout", "-q", "-b", "other", &base]);
    fixture.write("docs/from-main.md", "from main\n");
    fixture.write("notes-from-main.txt", "from main\n");
    fixture.commit("main moved on");
    fixture.git(&["checkout", "-q", "-"]);
    assert_eq!(head(&fixture), work);
    fixture.git(&["merge", "-q", "--no-ff", "--no-edit", "other"]);
    write_change(&fixture, &base, "draft", &["src/"], false);
    assert_report(&fixture, &[]);

    // The change's own file outside its paths is still named, committed or not.
    fixture.write("own-outside.txt", "own\n");
    fixture.git(&["add", "own-outside.txt"]);
    fixture.git(&["commit", "-q", "-m", "an own file outside the paths"]);
    fixture.write("uncommitted-outside.txt", "own\n");
    write_change(&fixture, &base, "draft", &["src/"], false);
    assert_report(
        &fixture,
        &[
            (
                "changed file own-outside.txt is outside the declared paths",
                "chough change answer demo affected_paths \"src/,own-outside.txt,uncommitted-outside.txt\"",
            ),
            (
                "changed file uncommitted-outside.txt is outside the declared paths",
                "chough change answer demo affected_paths \"src/,own-outside.txt,uncommitted-outside.txt\"",
            ),
        ],
    );
}

/// REQ-cli-067 (the review of #311): what a merge commit itself changes is the change's
/// own work. A file outside the declared paths that only the merge's resolution
/// changed is named, while the files the other branch brought in are not.
#[test]
fn a_file_changed_only_in_a_merge_resolution_is_named() {
    let fixture = project(OLD_REQ, &["src/"]);
    let base = head(&fixture);
    fixture.write("src/owned.rs", "fn owned() {}\n");
    fixture.commit("the change's own work");
    fixture.git(&["checkout", "-q", "-b", "other", &base]);
    fixture.write("docs/from-main.md", "from main\n");
    fixture.write("shared.txt", "from main\n");
    fixture.commit("main moved on");
    fixture.git(&["checkout", "-q", "-"]);
    fixture.git(&["merge", "-q", "--no-ff", "--no-commit", "other"]);
    // The resolution edits a file the other branch brought, and adds one of its own.
    fixture.write("shared.txt", "changed while merging\n");
    fixture.write("added-while-merging.txt", "own\n");
    fixture.git(&["add", "shared.txt", "added-while-merging.txt"]);
    fixture.git(&["commit", "-q", "-m", "merge other, with edits"]);
    write_change(&fixture, &base, "draft", &["src/"], false);
    let fix =
        "chough change answer demo affected_paths \"src/,added-while-merging.txt,shared.txt\"";
    assert_report(
        &fixture,
        &[
            (
                "changed file added-while-merging.txt is outside the declared paths",
                fix,
            ),
            ("changed file shared.txt is outside the declared paths", fix),
        ],
    );
}

/// REQ-cli-067, REQ-records-006: an id that is not a valid change id is not shown as
/// typed, since it can hold anything; a valid one that names no change is.
#[test]
fn an_invalid_id_is_not_echoed() {
    let fixture = project(OLD_REQ, &["src/"]);
    let typed = "SYNTHETIC/secret value";
    let output = fixture.run(&["change", "ready", typed]);
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(!stderr.contains("SYNTHETIC"), "{stderr}");
    assert!(stderr.contains("chough change status"), "{stderr}");
    let output = fixture.run(&["change", "ready", "no-such-change"]);
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("no change `no-such-change` to read"),
        "{stderr}"
    );
}

/// REQ-cli-067 (the review of #311): the report restores only the cache its own
/// SpecSync runs wrote. A cache another process replaced meanwhile, by a regular file
/// or by a symlink to a tracked file, stays as that process left it, nothing is written
/// through the symlink, and the report says the cache was left.
#[cfg(unix)]
#[test]
fn a_cache_replaced_while_the_report_runs_is_left_alone() {
    // Said on stderr, not as a finding: REQ-cli-067 lists the findings, and no other.
    let left =
        ".specsync/hashes.json was replaced while this report ran, so it was left as it is now\n";
    for kind in ["regular", "symlink"] {
        let fixture = project(OLD_REQ, &["src/"]);
        let output = fixture.run_with_env(&["change", "ready", ID], &[("READY_TEST_CACHE", kind)]);
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(String::from_utf8(output.stderr).unwrap(), left, "{kind}");
        assert!(!stdout.contains("was replaced"), "{kind}: {stdout}");
        let cache = fixture.root().join(".specsync/hashes.json");
        // The tracked file the symlink points to keeps its text either way.
        assert_eq!(fixture.read("specs/cli/requirements.md"), OLD_REQ, "{kind}");
        if kind == "regular" {
            assert_eq!(std::fs::read_to_string(&cache).unwrap(), "SECOND_WRITER\n");
        } else {
            assert!(
                std::fs::symlink_metadata(&cache)
                    .unwrap()
                    .file_type()
                    .is_symlink(),
                "the replacement symlink stays"
            );
        }
    }
}

/// REQ-cli-067 (the review of #311): with no cache before the report, a cache another
/// process puts there while the check runs is not the report's own, so it stays, and
/// stderr says the cache was left.
#[cfg(unix)]
#[test]
fn a_cache_another_process_made_while_the_report_ran_is_kept() {
    let fixture = project(OLD_REQ, &["src/"]);
    let cache = fixture.root().join(".specsync/hashes.json");
    std::fs::remove_file(&cache).unwrap();
    let output = fixture.run_with_env(&["change", "ready", ID], &[("READY_TEST_CACHE", "regular")]);
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        ".specsync/hashes.json was replaced while this report ran, so it was left as it is now\n"
    );
    assert_eq!(std::fs::read_to_string(&cache).unwrap(), "SECOND_WRITER\n");
}

/// REQ-cli-067 (the review of #311): a delta item whose body holds a short `### `
/// heading, such as `### Notes`, gives a report; the heading is body text, and the
/// command does not panic.
#[test]
fn a_short_heading_inside_a_delta_item_gives_a_report() {
    // An approved delta, so the report parses it to look for stale canonical text.
    let delta = "## MODIFIED\n\n### REQUIREMENT REQ-cli-001\n\nOld line.\n\n### Notes\n\nÜber die Änderung.\n";
    let fixture = project(OLD_REQ, &["specs/cli/requirements.md"]);
    fixture.write(&format!(".specsync/changes/{ID}/deltas/cli.md"), delta);
    approve(&fixture, delta, OLD_REQ);
    fixture.write("specs/cli/requirements.md", NEW_REQ);
    let output = fixture.run(&["change", "ready", ID]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stdout.starts_with("chough change ready demo\n"),
        "{stdout}{stderr}"
    );
    assert!(matches!(output.status.code(), Some(0 | 1)), "{stderr}");
    assert!(!stderr.contains("panicked"), "{stderr}");
}

/// REQ-cli-067: with no cache before the report, the one SpecSync wrote is removed.
#[test]
fn a_cache_the_report_made_is_removed() {
    let fixture = project(OLD_REQ, &["src/"]);
    let cache = fixture.root().join(".specsync/hashes.json");
    std::fs::remove_file(&cache).unwrap();
    let output = fixture.run(&["change", "ready", ID]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains("was replaced"), "{stdout}");
    assert!(!cache.exists());
}
