//! REQ-cli-069 (#266): during a Git merge or rebase, `chough next`, `chough status` and
//! `chough doctor` say so, and every command that writes a Chough record refuses while
//! a path under `.chough/` is unmerged, so nothing is written into a conflicted tree.
#![cfg(unix)]

mod common;

use common::Fixture;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Every file under `root` but `.git` and `.chough/local`, with its bytes.
fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        for entry in fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            let name = path
                .strip_prefix(base)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if name == ".git" || name.starts_with(".chough/local") {
                continue;
            }
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                out.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}

fn said(output: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// A project stopped in a `git merge` that conflicts on the session record (`chough`)
/// or on a product file (otherwise); returns the fixture and the conflicted path.
fn stopped_merge(in_chough: bool) -> (Fixture, String) {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.configure_human();
    fixture.init();
    fixture.write("src/product.txt", "base\n");
    fixture.commit("base");
    let session = format!(".chough/sessions/{}/session.md", fixture.active_id());
    let side = |text: &str| {
        if in_chough {
            fixture.ok(&["checkpoint", "--next-action", text]);
        } else {
            fixture.write("src/product.txt", &format!("{text}\n"));
        }
    };
    fixture.git(&["checkout", "-q", "-b", "other"]);
    side("SYNTHETIC the other branch's next step");
    fixture.commit("other side");
    fixture.git(&["checkout", "-q", "main"]);
    side("SYNTHETIC this branch's next step");
    fixture.commit("our side");
    let merge = Command::new("git")
        .current_dir(fixture.root())
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
        ])
        .args(["merge", "--no-edit", "other"])
        .output()
        .unwrap();
    assert!(!merge.status.success(), "the merge did not conflict");
    assert!(fixture.root().join(".git/MERGE_HEAD").is_file());
    let path = if in_chough {
        session
    } else {
        "src/product.txt".to_string()
    };
    (fixture, path)
}

fn evidence_add() -> Vec<&'static str> {
    vec![
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
        "local",
        "--summary",
        "SYNTHETIC entry during a merge",
    ]
}

fn bug_add() -> Vec<&'static str> {
    vec![
        "bug",
        "add",
        "--title",
        "SYNTHETIC bug during a merge",
        "--symptom",
        "s",
        "--trigger",
        "t",
        "--expected",
        "e",
        "--actual",
        "a",
        "--environment",
        "local",
    ]
}

/// The `git_operation` findings of `chough doctor`.
fn doctor_git_findings(fixture: &Fixture) -> Vec<serde_json::Value> {
    let output = fixture.run(&["doctor"]);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|_| panic!("doctor printed no JSON: {}", said(&output)));
    report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| finding["kind"] == "git_operation")
        .cloned()
        .collect()
}

/// REQ-cli-069, TC-cli-git-merge-in-progress-warns-and-refuses-writes: with the session
/// record unmerged, `next` and `status` name the merge and the path and exit 0, `doctor`
/// reports one `git_operation` finding, and a decision, an evidence entry and a bug are
/// each refused as `refused:unmerged` with every file unchanged; once the merge is
/// aborted, nothing of it shows and a write succeeds.
#[test]
fn a_merge_conflict_in_chough_warns_and_refuses_writes() {
    let (fixture, path) = stopped_merge(true);
    for args in [&["next"][..], &["status"][..], &["next", "--json"][..]] {
        let output = fixture.run(args);
        let text = said(&output);
        assert!(output.status.success(), "{args:?}: {text}");
        assert!(
            text.contains("merge"),
            "{args:?} does not name the merge: {text}"
        );
        assert!(
            text.contains(&path),
            "{args:?} does not name {path}: {text}"
        );
        assert!(
            text.contains("finish or abort"),
            "{args:?} does not say what to do: {text}"
        );
    }
    let findings = doctor_git_findings(&fixture);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    assert!(findings[0].to_string().contains(&path), "{findings:#?}");

    let decision = [
        "checkpoint",
        "--decision",
        "decided: SYNTHETIC during a merge",
    ];
    for args in [&decision[..], &evidence_add()[..], &bug_add()[..]] {
        let before = tree(fixture.root());
        let output = fixture.run(args);
        let text = said(&output);
        assert!(!output.status.success(), "{args:?} was written: {text}");
        assert!(text.contains("refused:unmerged"), "{args:?}: {text}");
        assert!(
            text.contains(&path),
            "{args:?} does not name {path}: {text}"
        );
        assert_eq!(tree(fixture.root()), before, "{args:?} changed a file");
    }

    fixture.git(&["merge", "--abort"]);
    let next = fixture.run(&["next"]);
    assert!(next.status.success(), "{}", said(&next));
    assert!(!said(&next).contains("Git:"), "{}", said(&next));
    assert!(doctor_git_findings(&fixture).is_empty());
    fixture.ok(&decision);
}

/// REQ-cli-069: with only a product file unmerged, `next`, `status` and `doctor` still
/// name the merge, and Chough's own writes go on.
#[test]
fn a_merge_conflict_outside_chough_warns_without_refusing() {
    let (fixture, path) = stopped_merge(false);
    for args in [&["next"][..], &["status"][..]] {
        let output = fixture.run(args);
        let text = said(&output);
        assert!(output.status.success(), "{args:?}: {text}");
        assert!(
            text.contains("merge"),
            "{args:?} does not name the merge: {text}"
        );
        assert!(
            text.contains("finish or abort"),
            "{args:?} does not say what to do: {text}"
        );
    }
    assert_eq!(doctor_git_findings(&fixture).len(), 1);
    assert!(!path.starts_with(".chough/"));
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "decided: SYNTHETIC during a product merge",
    ]);
}

/// REQ-cli-069 (#330 review): a command that only reads still runs while a path under
/// `.chough/` is unmerged, though it takes the writer lock to read: `context`, `stage
/// show` and a bare `checkpoint`, while a write is still refused.
#[test]
fn read_only_commands_still_run_while_chough_is_unmerged() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.configure_human();
    fixture.init();
    fixture.commit("base");
    let readme = ".chough/README.md";
    let side = |text: &str| {
        let mut body = fixture.read(readme);
        body.push_str(&format!("\n{text}\n"));
        fixture.write(readme, &body);
    };
    fixture.git(&["checkout", "-q", "-b", "other"]);
    side("SYNTHETIC the other branch's note");
    fixture.commit("other side");
    fixture.git(&["checkout", "-q", "main"]);
    side("SYNTHETIC this branch's note");
    fixture.commit("our side");
    let merge = Command::new("git")
        .current_dir(fixture.root())
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
        ])
        .args(["merge", "--no-edit", "other"])
        .output()
        .unwrap();
    assert!(!merge.status.success(), "the merge did not conflict");

    for args in [
        &["context"][..],
        &["context", "--json"][..],
        &["stage", "show"][..],
        &["checkpoint"][..],
    ] {
        let before = tree(fixture.root());
        let output = fixture.run(args);
        let text = said(&output);
        assert!(output.status.success(), "{args:?} was refused: {text}");
        assert!(!text.contains("refused:unmerged"), "{args:?}: {text}");
        assert_eq!(tree(fixture.root()), before, "{args:?} changed a file");
    }
    let output = fixture.run(&["checkpoint", "--decision", "decided: SYNTHETIC mid-merge"]);
    assert!(
        said(&output).contains("refused:unmerged"),
        "{}",
        said(&output)
    );
}

/// REQ-cli-069 (#330 review): a rebase is in progress while its `rebase-merge` or
/// `rebase-apply` folder is there, as `git status` reads it; a `REBASE_HEAD` that Git
/// left behind after the rebase finished is not one.
#[test]
fn a_finished_rebase_is_not_in_progress() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.commit("base");
    let head = Command::new("git")
        .current_dir(fixture.root())
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    fs::write(fixture.root().join(".git/REBASE_HEAD"), &head.stdout).unwrap();
    let next = fixture.run(&["next"]);
    assert!(next.status.success(), "{}", said(&next));
    assert!(!said(&next).contains("Git:"), "{}", said(&next));
    assert!(doctor_git_findings(&fixture).is_empty());

    fs::create_dir_all(fixture.root().join(".git/rebase-merge")).unwrap();
    let next = fixture.run(&["next"]);
    assert!(
        said(&next).contains("a git rebase is in progress"),
        "{}",
        said(&next)
    );
    assert_eq!(doctor_git_findings(&fixture).len(), 1);
}

/// REQ-cli-069 (#330 review, P2): `pref set` and `pref unset` write
/// `.chough/preferences.json`, a Chough record, so while a path under `.chough/` is
/// unmerged they refuse as `refused:unmerged` with every file unchanged, as every other
/// writer does; `pref show` still runs, and after the merge is aborted they write again.
#[test]
fn preference_writes_refuse_while_chough_is_unmerged() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.configure_human();
    fixture.init();
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "off",
        "--source",
        "SYNTHETIC fixture",
    ]);
    fixture.commit("base");
    let readme = ".chough/README.md";
    let side = |text: &str| {
        let mut body = fixture.read(readme);
        body.push_str(&format!("\n{text}\n"));
        fixture.write(readme, &body);
    };
    fixture.git(&["checkout", "-q", "-b", "other"]);
    side("SYNTHETIC the other branch's note");
    fixture.commit("other side");
    fixture.git(&["checkout", "-q", "main"]);
    side("SYNTHETIC this branch's note");
    fixture.commit("our side");
    let merge = Command::new("git")
        .current_dir(fixture.root())
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
        ])
        .args(["merge", "--no-edit", "other"])
        .output()
        .unwrap();
    assert!(!merge.status.success(), "the merge did not conflict");

    for args in [
        &[
            "pref",
            "set",
            "librarian",
            "on",
            "--source",
            "SYNTHETIC mid-merge",
        ][..],
        &[
            "pref",
            "unset",
            "librarian",
            "--source",
            "SYNTHETIC mid-merge",
        ][..],
    ] {
        let before = tree(fixture.root());
        let output = fixture.run(args);
        let text = said(&output);
        assert!(!output.status.success(), "{args:?} ran mid-merge: {text}");
        assert!(text.contains("refused:unmerged"), "{args:?}: {text}");
        assert!(text.contains(readme), "{args:?}: {text}");
        assert_eq!(tree(fixture.root()), before, "{args:?} changed a file");
    }
    let before = tree(fixture.root());
    let shown = fixture.run(&["pref", "show"]);
    assert!(shown.status.success(), "{}", said(&shown));
    assert_eq!(tree(fixture.root()), before, "pref show changed a file");

    fixture.git(&["merge", "--abort"]);
    fixture.ok(&[
        "pref",
        "unset",
        "librarian",
        "--source",
        "SYNTHETIC after the merge",
    ]);
}
