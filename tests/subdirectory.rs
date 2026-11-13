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

/// The repository-relative file of evidence entry `id` in the project's one open session.
fn entry_file(repo: &Fixture, id: &str) -> String {
    let sessions = repo.root().join("app/.chough/sessions");
    let open: Vec<String> = std::fs::read_dir(&sessions)
        .unwrap()
        .flatten()
        .filter(|entry| entry.path().join("session.md").is_file())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();
    assert_eq!(open.len(), 1, "{open:?}");
    format!("app/.chough/sessions/{}/evidence/{id}.md", open[0])
}

/// One scalar front matter value of a Markdown record.
fn front(text: &str, key: &str) -> Option<String> {
    text.strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .and_then(|(front, _)| {
            front.lines().find_map(|line| {
                line.split_once(": ")
                    .filter(|(name, _)| *name == key)
                    .map(|(_, value)| value.trim_matches('"').to_string())
            })
        })
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
    // The entry is in the project's session, under app/ (REQ-records-004).
    let entry = repo.read(&entry_file(&repo, &id));
    assert_eq!(
        front(&entry, "revision").as_deref(),
        Some(head(repo.root()).as_str()),
        "{entry}"
    );
    // An entry records the project's content identity, not a dirty flag: it is the
    // identity of the clean project, so nothing outside app/ counted.
    let clean = ok(&repo, &["status"])["snapshot"].clone();
    assert_eq!(clean["dirty"], false, "{clean:#}");
    assert_eq!(
        front(&entry, "content").as_deref(),
        clean["content"].as_str(),
        "{entry}"
    );

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
    // An automatic date-slug ID (REQ-records-003), recorded in the project's session.
    let observation = added["observation"].as_str().unwrap_or_default();
    assert!(
        observation.starts_with(|c: char| c.is_ascii_digit()) && !observation.contains("OBS-"),
        "{added:#}"
    );
    assert_eq!(
        format!("app/{}", added["recorded"].as_str().unwrap_or_default()),
        entry_file(&repo, observation),
        "{added:#}"
    );

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

/// Fresh-eyes finding 5 (t38j, 2026-11-12): `chough context` reads each AGENTS.md from the
/// project's Git root down to the project, and none above the repository (a home folder's,
/// say), naming each relative to the project, so the packet never prints an absolute path.
#[cfg(unix)]
#[test]
fn context_reads_agents_md_from_the_git_root_down_only() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write("AGENTS.md", "SYNTHETIC-ABOVE-THE-REPOSITORY\n");
    fixture.write("repo/AGENTS.md", "SYNTHETIC-REPOSITORY-ROOT\n");
    for (name, content) in [
        ("AGENTS.md", "SYNTHETIC-PROJECT\n"),
        ("INTENT.md", "A user wants a reliable product.\n"),
        ("specs/cli/cli.spec.md", "# CLI contract\n"),
        (
            "specs/cli/requirements.md",
            "### REQ-cli-001\nPreserve files.\n",
        ),
    ] {
        fixture.write(&format!("repo/app/{name}"), content);
    }
    let repo = fixture.root().join("repo");
    assert!(
        Command::new("git")
            .arg("-C")
            .arg(&repo)
            .args(["init", "-q"])
            .status()
            .unwrap()
            .success()
    );
    let app = repo.join("app");
    let chough = |args: &[&str]| {
        let mut paths = vec![fixture.root().join("bin")];
        paths.extend(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        ));
        let output = Command::new(env!("CARGO_BIN_EXE_chough"))
            .arg("--root")
            .arg(&app)
            .args(args)
            .current_dir(&app)
            .env("PATH", std::env::join_paths(paths).unwrap())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    };
    chough(&[
        "init",
        "--objective",
        "Ship a focused change",
        "--module",
        "cli",
    ]);

    let packet: Value = serde_json::from_str(&chough(&["context", "--json"])).unwrap();
    let sections = packet["sections"].as_array().unwrap();
    let agents: Vec<(&str, &str)> = sections
        .iter()
        .filter(|section| {
            section["source"]
                .as_str()
                .is_some_and(|source| source.ends_with("AGENTS.md"))
        })
        .map(|section| {
            (
                section["source"].as_str().unwrap(),
                section["content"].as_str().unwrap_or_default(),
            )
        })
        .collect();
    assert_eq!(
        agents,
        vec![
            ("../AGENTS.md", "SYNTHETIC-REPOSITORY-ROOT\n"),
            ("AGENTS.md", "SYNTHETIC-PROJECT\n"),
        ],
        "{packet:#}"
    );
    let text = chough(&["context"]);
    let root = fixture.root().canonicalize().unwrap();
    for shown in [packet.to_string(), text] {
        assert!(!shown.contains("SYNTHETIC-ABOVE-THE-REPOSITORY"), "{shown}");
        assert!(!shown.contains(&*root.to_string_lossy()), "{shown}");
        assert!(
            !shown.contains(&*fixture.root().to_string_lossy()),
            "{shown}"
        );
    }
}
