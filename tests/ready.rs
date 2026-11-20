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
# SpecSync 6.4.0's approval dry run (#406): the change's dry-run.json when it has one,
# exiting 1 when it refuses; otherwise an accepting document. READY_TEST_DRY_RUN=none
# plays a SpecSync without the dry run.
if [ "${1:-}" = --json ] && [ "${2:-}" = change ] && [ "${3:-}" = approve ]; then
  if [ "${READY_TEST_DRY_RUN:-}" = none ]; then
    echo "error: unexpected argument '--dry-run' found" >&2
    exit 2
  fi
  if [ -f ".specsync/changes/$4/dry-run.json" ]; then
    cat ".specsync/changes/$4/dry-run.json"
    if grep -q '"would_approve":false' ".specsync/changes/$4/dry-run.json"; then
      exit 1
    fi
    exit 0
  fi
  printf '{"id":"%s","dry_run":true,"would_approve":true,"checks":[]}\n' "$4"
  exit 0
fi
if [ "${1:-}" = change ] && [ "${2:-}" = show ]; then
  cat ".specsync/changes/$3/show.json"
  exit 0
fi
if [ "${1:-}" = check ]; then
  # Whether a hash file was there when the check started (#404).
  if [ -n "${READY_TEST_SEEN:-}" ]; then
    if [ -e .specsync/hashes.json ]; then echo present; else echo absent; fi >>"$READY_TEST_SEEN"
  fi
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
        "{\"errors\":[],\"passed\":true,\"specs_checked\":1,\"warnings\":[\"specs/cli/cli.spec.md: 1/3 exports documented\",\"specs/cli/cli.spec.md: Undocumented export 'hidden' from src/lib.rs\",\"specs/cli/cli.spec.md: Undocumented export 'other' from src/other.rs\"]}\n",
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

/// A remote `origin` whose `main` and `HEAD` name `rev`, with no fetch: the default
/// branch the report compares with (REQ-cli-067, #406).
fn origin_main(fixture: &Fixture, rev: &str) {
    fixture.git(&[
        "remote",
        "add",
        "origin",
        "https://example.invalid/SYNTHETIC.git",
    ]);
    fixture.git(&["update-ref", "refs/remotes/origin/main", rev]);
    fixture.git(&[
        "symbolic-ref",
        "refs/remotes/origin/HEAD",
        "refs/remotes/origin/main",
    ]);
}

/// The fix for the one file outside `src/` these tests name.
const OWN_FIX: &str = "chough change answer demo affected_paths \"src/,own-outside.txt\"";

/// The change's own work on a branch `work` from the base: one file inside its paths and
/// one outside them.
fn own_work(fixture: &Fixture) {
    fixture.git(&["checkout", "-q", "-b", "work"]);
    fixture.write("src/owned.rs", "fn owned() {}\n");
    fixture.write("own-outside.txt", "own\n");
    fixture.git(&["add", "src/owned.rs", "own-outside.txt"]);
    fixture.git(&["commit", "-q", "-m", "the change's own work"]);
}

/// The default branch moves on: a file it changes and one it adds.
fn main_moves_on(fixture: &Fixture) -> String {
    fixture.git(&["checkout", "-q", "main"]);
    fixture.write("specs/cli/requirements.md", NEW_REQ);
    fixture.write("docs/from-main.md", "from main\n");
    fixture.git(&["add", "specs/cli/requirements.md", "docs/from-main.md"]);
    fixture.git(&["commit", "-q", "-m", "main moved on"]);
    let tip = head(fixture);
    fixture.git(&["checkout", "-q", "work"]);
    tip
}

/// #406 (REQ-cli-067): on a branch rebased onto the default branch after the change
/// recorded its base, a file only the default branch's commits changed is not named, and
/// a file outside the declared paths that the change's own commits changed is.
#[test]
fn after_a_rebase_onto_main_only_the_change_s_own_files_are_named() {
    let fixture = project(OLD_REQ, &["src/"]);
    let base = head(&fixture);
    own_work(&fixture);
    let tip = main_moves_on(&fixture);
    origin_main(&fixture, &tip);
    fixture.git(&["rebase", "-q", "main"]);
    write_change(&fixture, &base, "draft", &["src/"], false);
    assert_report(
        &fixture,
        &[(
            "changed file own-outside.txt is outside the declared paths",
            OWN_FIX,
        )],
    );
}

/// #406: on a branch that squash-merged the default branch, a file whose content at HEAD
/// is the default branch's is not named, and the change's own file outside its paths is.
#[test]
fn after_a_squash_merge_of_main_only_the_change_s_own_files_are_named() {
    let fixture = project(OLD_REQ, &["src/"]);
    let base = head(&fixture);
    own_work(&fixture);
    let tip = main_moves_on(&fixture);
    origin_main(&fixture, &tip);
    fixture.git(&["merge", "-q", "--squash", "main"]);
    fixture.git(&["commit", "-q", "-m", "main, squashed"]);
    write_change(&fixture, &base, "draft", &["src/"], false);
    assert_report(
        &fixture,
        &[(
            "changed file own-outside.txt is outside the declared paths",
            OWN_FIX,
        )],
    );
}

/// #406: the comparison never switches the check off. With HEAD on the default branch,
/// or `origin/HEAD` naming HEAD's own branch, a file outside the declared paths that the
/// change changed is still named.
#[test]
fn on_the_default_branch_itself_the_change_s_files_are_still_named() {
    // Work done on main, with origin/main at HEAD.
    let fixture = project(OLD_REQ, &["src/"]);
    let base = head(&fixture);
    fixture.write("own-outside.txt", "own\n");
    fixture.git(&["add", "own-outside.txt"]);
    fixture.git(&["commit", "-q", "-m", "work on main"]);
    origin_main(&fixture, &head(&fixture));
    write_change(&fixture, &base, "draft", &["src/"], false);
    assert_report(
        &fixture,
        &[(
            "changed file own-outside.txt is outside the declared paths",
            OWN_FIX,
        )],
    );

    // origin/HEAD naming the branch HEAD is on, whose remote copy already holds the
    // file: origin/main is compared instead, so the file is still named.
    let fixture = project(OLD_REQ, &["src/"]);
    let base = head(&fixture);
    origin_main(&fixture, &base);
    own_work(&fixture);
    fixture.git(&["update-ref", "refs/remotes/origin/work", "HEAD"]);
    fixture.git(&[
        "symbolic-ref",
        "refs/remotes/origin/HEAD",
        "refs/remotes/origin/work",
    ]);
    fixture.write("src/later.rs", "fn later() {}\n");
    fixture.git(&["add", "src/later.rs"]);
    fixture.git(&["commit", "-q", "-m", "more own work"]);
    write_change(&fixture, &base, "draft", &["src/"], false);
    assert_report(
        &fixture,
        &[(
            "changed file own-outside.txt is outside the declared paths",
            OWN_FIX,
        )],
    );
}

/// #406: with a `base_commit` the repository does not have, the report says so, naming
/// it, prints no `fatal:` line, and counts from HEAD's merge base with the default
/// branch; with no default branch either, it names one finding that the changed files
/// could not be listed, never that nothing is missing.
#[test]
fn a_base_commit_the_repository_lacks_is_named_plainly() {
    let gone = "0123456789abcdef0123456789abcdef01234567";
    let fixture = project(OLD_REQ, &["src/"]);
    let base = head(&fixture);
    origin_main(&fixture, &base);
    own_work(&fixture);
    write_change(&fixture, gone, "draft", &["src/"], false);
    let output = fixture.run(&["change", "ready", ID]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        !stdout.contains("fatal") && !stderr.contains("fatal"),
        "{stdout}{stderr}"
    );
    assert!(
        stderr.contains(gone) && stderr.contains("not in this repository"),
        "{stderr}"
    );
    assert_eq!(
        stdout,
        report(&[(
            "changed file own-outside.txt is outside the declared paths",
            OWN_FIX
        )])
    );

    // No default branch: no remote and no local main.
    let fixture = project(OLD_REQ, &["src/"]);
    own_work(&fixture);
    fixture.git(&["branch", "-q", "-m", "main", "trunk"]);
    write_change(&fixture, gone, "draft", &["src/"], false);
    let output = fixture.run(&["change", "ready", ID]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains("Nothing is missing"), "{stdout}");
    assert!(!stdout.contains("fatal"), "{stdout}");
    assert!(
        stdout.contains(gone) && stdout.contains("could not be listed"),
        "{stdout}"
    );
    assert_eq!(output.status.code(), Some(1));
}

/// A dry-run document refusing `check` with `message`, and `fix` when given (#406).
fn refusing(check: &str, message: &str, fix: Option<&str>) -> String {
    let mut entry = serde_json::json!({"check": check, "result": "refused", "message": message});
    if let Some(fix) = fix {
        entry["fix"] = fix.into();
    }
    serde_json::json!({
        "id": ID,
        "dry_run": true,
        "would_approve": false,
        "checks": [
            {"check": "change", "result": "passed"},
            entry,
            {"check": "approval_record", "result": "skipped", "skipped_because": check},
        ],
    })
    .to_string()
}

/// REQ-cli-067 (#406, #307), TC-cli-ready-names-the-approval-dry-run-s-refusals: a check
/// SpecSync's approval dry run refuses is named in SpecSync's words with SpecSync's fix,
/// shown and never run; a refused check without one names `chough change ready` again;
/// a skipped check is not a finding.
#[test]
fn a_refusal_of_the_approval_dry_run_is_named_with_its_fix() {
    let fixture = project(OLD_REQ, &["src/"]);
    let message = "no declared module canonically owns these affected paths: src/free.rs";
    let fix = "specsync change answer demo affected_specs \"<modules>\"";
    fixture.write(
        &format!(".specsync/changes/{ID}/dry-run.json"),
        &refusing("ownership", message, Some(fix)),
    );
    assert_report(
        &fixture,
        &[(
            &format!("the approval's ownership check refuses: {message}"),
            fix,
        )],
    );
    let message = "the semantic delta for cli has no SHALL";
    fixture.write(
        &format!(".specsync/changes/{ID}/dry-run.json"),
        &refusing("deltas", message, None),
    );
    assert_report(
        &fixture,
        &[(
            &format!("the approval's deltas check refuses: {message}"),
            "chough change ready demo",
        )],
    );
}

/// REQ-cli-067 (#406, Q1 A), TC-cli-ready-leaves-the-delta-set-to-specsync: a missing
/// delta is named once, by SpecSync's dry run; ready has no delta-set rule of its own.
#[test]
fn a_missing_delta_is_named_by_specsync_alone() {
    let fixture = project(OLD_REQ, &["src/"]);
    std::fs::remove_file(
        fixture
            .root()
            .join(format!(".specsync/changes/{ID}/deltas/cli.md")),
    )
    .unwrap();
    let message =
        "semantic delta modules must exactly match affected specs (missing: cli; extra: none)";
    fixture.write(
        &format!(".specsync/changes/{ID}/dry-run.json"),
        &refusing("deltas", message, None),
    );
    assert_report(
        &fixture,
        &[(
            &format!("the approval's deltas check refuses: {message}"),
            "chough change ready demo",
        )],
    );
}

/// REQ-cli-067 (#406), TC-cli-ready-without-a-dry-run-names-6-4-0: a SpecSync that gives
/// no dry-run document names the approval as unchecked with the 6.4.0 install command,
/// and the report never says that nothing is missing.
#[test]
fn a_specsync_without_the_dry_run_names_the_install_command() {
    let fixture = project(OLD_REQ, &["src/"]);
    let output = fixture.run_with_env(&["change", "ready", ID], &[("READY_TEST_DRY_RUN", "none")]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(1), "{stdout}");
    assert!(!stdout.contains("Nothing is missing."), "{stdout}");
    assert!(
        stdout.contains("the approval could not be checked")
            && stdout.contains("Fix: `cargo install --locked specsync@6.4.0`"),
        "{stdout}"
    );
}

/// REQ-cli-067 (#406), TC-cli-ready-leaves-out-specsync-s-own-files: in a project that never
/// ran SpecSync's adoption, its own untracked `.specsync/change.lock` and
/// `.specsync/workflow-v2-baseline.json` are not outside the declared paths; another
/// untracked file there still is.
#[test]
fn specsync_s_own_untracked_files_are_not_outside_the_declared_paths() {
    let fixture = project(OLD_REQ, &["src/"]);
    fixture.write(".specsync/change.lock", "");
    fixture.write(".specsync/workflow-v2-baseline.json", "{}\n");
    fixture.write(".specsync/other.json", "{}\n");
    assert_report(
        &fixture,
        &[(
            "changed file .specsync/other.json is outside the declared paths",
            "chough change answer demo affected_paths \"src/,.specsync/other.json\"",
        )],
    );
}

/// REQ-cli-067 (#406, Q2 A), TC-cli-ready-leaves-out-another-change-s-work: with two active
/// changes on one branch, the other change's folder and the files it declares are not
/// named for this one; a file neither declares still is.
#[test]
fn another_active_change_s_work_is_not_this_change_s() {
    let fixture = project(OLD_REQ, &["src/"]);
    let base = head(&fixture);
    fixture.write(
        ".specsync/changes/other/state.json",
        &format!(
            "{{\"state\":\"draft\",\"base_commit\":\"{base}\",\"affected_specs\":[],\"affected_paths\":[\"docs/other.md\"],\"no_spec_change\":true,\"selected_artifacts\":[]}}\n"
        ),
    );
    fixture.write("docs/other.md", "the other change's work\n");
    fixture.write("docs/neither.md", "nobody's\n");
    fixture.commit("two changes' work");
    assert_report(
        &fixture,
        &[(
            "changed file docs/neither.md is outside the declared paths",
            "chough change answer demo affected_paths \"src/,docs/neither.md\"",
        )],
    );
}

/// REQ-cli-067 (#404), TC-cli-ready-makes-no-hash-file: in a project with no
/// `.specsync/hashes.json`, the SpecSync check ready runs finds none.
#[test]
fn ready_makes_no_hash_file_for_specsync_to_find() {
    let fixture = project(OLD_REQ, &["src/"]);
    std::fs::remove_file(fixture.root().join(".specsync/hashes.json")).unwrap();
    fixture.commit("no hash file");
    let base = head(&fixture);
    write_change(&fixture, &base, "draft", &["src/"], false);
    let seen = tempfile::tempdir().unwrap();
    let log = seen.path().join("seen");
    let _ = fixture.run_with_env(
        &["change", "ready", ID],
        &[("READY_TEST_SEEN", log.to_str().unwrap())],
    );
    assert_eq!(std::fs::read_to_string(&log).unwrap(), "absent\n");
}
