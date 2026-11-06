//! A Chough project in a subdirectory of its Git repository (REQ-cli-056, DRIVE-18).

mod common;

use common::Fixture;
use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};

const CASES: &str = "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"Probe reports healthy\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo healthy: yes\"]\nexpect_output = [\"healthy: yes\"]\n";

/// Run chough with `--root <repository>/app`, as an agent working in `app/` does.
fn in_app(repo: &Fixture, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
    command
        .arg("--root")
        .arg(repo.root().join("app"))
        .args(args)
        .current_dir(repo.root().join("app"));
    let mut paths = vec![repo.root().join("bin")];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    command.env("PATH", std::env::join_paths(paths).unwrap());
    command.output().unwrap()
}

fn ok(repo: &Fixture, args: &[&str]) -> Value {
    let output = in_app(repo, args);
    assert!(
        output.status.success(),
        "{args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn head(root: &Path) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

/// A repository whose Chough project lives in `app/`, committed, with stub `hi` and
/// `specsync` on PATH.
fn repository() -> Fixture {
    let repo = Fixture::new();
    repo.stubs();
    for (name, content) in [
        ("AGENTS.md", "Project instructions\n"),
        ("INTENT.md", "A user wants a reliable product.\n"),
        ("specs/cli/cli.spec.md", "# CLI contract\n"),
        (
            "specs/cli/requirements.md",
            "### REQ-cli-001\nPreserve files.\n\n### REQ-cli-002\nResume.\n\n### REQ-cli-003\nDelegate.\n",
        ),
        ("src/input.txt", "first version\n"),
        (".gitignore", "bin/\n"),
        (".chough/cases/cli.cases.toml", CASES),
    ] {
        repo.write(&format!("app/{name}"), content);
    }
    ok(
        &repo,
        &[
            "init",
            "--objective",
            "Ship a focused change",
            "--intent-scope",
            "LIFE",
            "--module",
            "cli",
        ],
    );
    repo.write("README.md", "The repository root, outside the project.\n");
    repo.commit("a project in app/");
    repo
}

/// DRIVE-18: `cases run` and `evidence add` work in a project in a subdirectory and
/// record the repository's HEAD against the project's own files; `status` reports a
/// known snapshot.
#[cfg(unix)]
#[test]
fn a_project_in_a_subdirectory_records_evidence() {
    let repo = repository();
    let run = ok(
        &repo,
        &["cases", "run", "TC-cli-0001", "--environment", "local"],
    );
    assert_eq!(run["outcome"], "passed", "{run:#}");
    let id = run["observation"]
        .as_str()
        .unwrap_or_else(|| panic!("{run:#}"))
        .to_string();
    let record: Value =
        serde_json::from_str(&repo.read(&format!("app/.chough/evidence/{id}.json"))).unwrap();
    assert_eq!(
        record["snapshot"]["revision"],
        head(repo.root()),
        "{record:#}"
    );
    assert_eq!(record["snapshot"]["dirty"], false, "{record:#}");

    let added = ok(
        &repo,
        &[
            "evidence",
            "add",
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
            "Checked by hand in app/",
        ],
    );
    assert!(added.to_string().contains("OBS-"), "{added:#}");

    let status = ok(&repo, &["status"]);
    assert!(
        status["snapshot"]["unknown"].is_null(),
        "{:#}",
        status["snapshot"]
    );
    assert_eq!(status["snapshot"]["revision"], head(repo.root()));
}

/// DRIVE-18: the snapshot covers the project's own files. An edit in `app/` makes it
/// dirty; an edit elsewhere in the repository leaves it clean and its fingerprints
/// unchanged.
#[cfg(unix)]
#[test]
fn a_subdirectory_snapshot_covers_only_the_project() {
    let repo = repository();
    let clean = ok(&repo, &["status"])["snapshot"].clone();
    assert_eq!(clean["dirty"], false, "{clean:#}");
    repo.write("README.md", "Edited outside the project.\n");
    repo.write("other/new.txt", "An untracked file outside the project.\n");
    let outside = ok(&repo, &["status"])["snapshot"].clone();
    assert_eq!(outside, clean);
    repo.write("app/src/input.txt", "second version\n");
    let inside = ok(&repo, &["status"])["snapshot"].clone();
    assert_eq!(inside["dirty"], true, "{inside:#}");
    assert_ne!(inside["content"], clean["content"], "{inside:#}");
}

/// DRIVE-18: the same project has the same content fingerprint at the repository root
/// and in a subdirectory, so moving the snapshot's scope changes nothing at the root.
#[cfg(unix)]
#[test]
fn a_project_has_the_same_snapshot_at_the_root_and_in_a_subdirectory() {
    let nested = repository();
    let root = Fixture::new();
    root.stubs();
    root.write(".gitignore", "bin/\n");
    root.write(".chough/cases/cli.cases.toml", CASES);
    root.ok(&[
        "init",
        "--objective",
        "Ship a focused change",
        "--intent-scope",
        "LIFE",
        "--module",
        "cli",
    ]);
    root.commit("a project at the root");
    let at_root = root.ok(&["status"])["snapshot"].clone();
    let in_app = ok(&nested, &["status"])["snapshot"].clone();
    assert!(at_root["content"].is_string(), "{at_root:#}");
    assert_eq!(
        in_app["content"], at_root["content"],
        "{in_app:#} {at_root:#}"
    );
    assert_eq!(in_app["dirty"], at_root["dirty"]);
}
