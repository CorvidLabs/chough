//! Preferences, repeated validation, librarian and host skills
//! (REQ-cli-011, REQ-cli-013, REQ-cli-015, REQ-cli-016).

mod common;

use common::Fixture;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[test]
fn standing_preferences_need_a_source_and_stay_separate_from_consent() {
    let fixture = Fixture::new();
    fixture.init();
    let defaults = fixture.ok(&["pref", "show"]);
    for (key, value) in [
        ("repeat-validation", "ask"),
        ("monitoring", "ask"),
        ("release", "on-request"),
        ("monitor-initiation", "capture-only"),
        ("librarian", "off"),
    ] {
        assert_eq!(defaults["effective"][key]["value"], value);
        assert_eq!(defaults["effective"][key]["origin"], "default");
    }
    assert!(
        fixture
            .fails(&["pref", "set", "release", "always", "--source", "x"])
            .contains("accepts")
    );
    assert!(
        fixture
            .fails(&["pref", "set", "librarian", "on", "--source", " "])
            .contains("source")
    );
    fixture.ok(&[
        "pref",
        "set",
        "monitoring",
        "off",
        "--source",
        "Human: never monitor this repo",
    ]);
    let stored = fixture.read(".chough/preferences.json");
    fixture.ok(&[
        "consent",
        "grant",
        "monitoring",
        "--source",
        "Human: just this once",
    ]);
    assert_eq!(fixture.read(".chough/preferences.json"), stored);
    let shown = fixture.ok(&["pref", "show"]);
    assert_eq!(shown["effective"]["monitoring"]["value"], "off");
    assert_eq!(
        fixture.ok(&["consent", "show"])["run_consents"][0]["key"],
        "monitoring"
    );
    fixture.ok(&["consent", "revoke", "monitoring"]);
    assert!(
        fixture
            .fails(&["consent", "revoke", "monitoring"])
            .contains("no run consent")
    );
}

#[cfg(unix)]
#[test]
fn repeated_validation_needs_authority_honors_stop_and_persists_findings() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Health probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"Probe reports healthy\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo unhealthy\"]\nexpect_output = [\"healthy: yes\"]\n",
    );
    fixture.init();
    fixture.commit("fixture");
    let refused = fixture.fails(&[
        "validate",
        "run",
        "TC-cli-0001",
        "--times",
        "3",
        "--environment",
        "test-local",
    ]);
    assert!(refused.contains("not authorized"), "{refused}");
    fixture.ok(&[
        "consent",
        "grant",
        "repeat-validation",
        "--source",
        "Human: repeat the probe three times",
    ]);
    fixture.ok(&[
        "pref",
        "set",
        "monitor-initiation",
        "propose-change",
        "--source",
        "Human: suggest a change when a probe fails",
    ]);
    let run = fixture.ok(&[
        "validate",
        "run",
        "TC-cli-0001",
        "--times",
        "3",
        "--environment",
        "test-local",
    ]);
    assert_eq!(run["run"]["completed"], 3);
    assert_eq!(run["run"]["failures"], 3);
    assert!(
        run["run"]["authority"]
            .as_str()
            .unwrap()
            .starts_with("run consent")
    );
    assert!(run["label"].as_str().unwrap().contains("not a soak test"));
    assert!(
        run["proposal"]
            .as_str()
            .unwrap()
            .starts_with("Proposed only, not run: specsync change new")
    );
    assert_eq!(run["captured_locally"].as_array().unwrap().len(), 1);

    // A fresh process still finds the finding with every failing observation.
    let bugs = fixture.ok(&["bug", "list"]);
    assert_eq!(bugs["bugs"].as_array().unwrap().len(), 1);
    let bug = fixture.ok(&["bug", "show", "BUG-0001"]);
    assert_eq!(bug["bug"]["observations"].as_array().unwrap().len(), 3);
    assert_eq!(bug["verification"], "reproduced");

    fixture.ok(&["validate", "stop"]);
    let stopped = fixture.ok(&[
        "validate",
        "run",
        "TC-cli-0001",
        "--times",
        "2",
        "--environment",
        "test-local",
    ]);
    assert_eq!(stopped["run"]["state"], "stopped");
    assert_eq!(stopped["run"]["completed"], 0);
    let id = stopped["run"]["id"].as_str().unwrap().to_string();
    let resumed = fixture.ok(&["validate", "resume", &id]);
    assert_eq!(resumed["run"]["state"], "complete");
    assert_eq!(resumed["run"]["times"], 2);
    assert_eq!(resumed["run"]["completed"], 2);
    assert!(!fixture.exists(".chough/validation.stop"));

    // A new objective ends the run consent, so repetition is refused again.
    fixture.ok(&["checkpoint", "--objective", "Something else"]);
    assert!(
        fixture
            .fails(&["validate", "resume", &id])
            .contains("complete")
    );
    assert!(
        fixture
            .fails(&[
                "validate",
                "run",
                "TC-cli-0001",
                "--times",
                "1",
                "--environment",
                "test-local",
            ])
            .contains("not authorized")
    );
}

/// Start chough in the background, as a second agent session would.
#[cfg(unix)]
fn spawn(fixture: &Fixture, args: &[&str]) -> std::process::Child {
    let mut paths = vec![fixture.root().join("bin")];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    std::process::Command::new(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(fixture.root())
        .args(args)
        .env("PATH", std::env::join_paths(paths).unwrap())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap()
}

/// Wait until a running repeated run has recorded its first attempt; returns its ID.
#[cfg(unix)]
fn first_attempt_recorded(fixture: &Fixture) -> String {
    for _ in 0..200 {
        if let Ok(entries) = fs::read_dir(fixture.root().join(".chough/validation")) {
            for entry in entries {
                let run: Value = serde_json::from_str(
                    &fs::read_to_string(entry.unwrap().path()).unwrap_or_default(),
                )
                .unwrap_or(Value::Null);
                if run["state"] == "running" && run["completed"] == 1 {
                    return run["id"].as_str().unwrap().to_string();
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    panic!("the first attempt was never recorded");
}

/// Wait for a background run and return its report and how long it took after `since`.
#[cfg(unix)]
fn finish(child: std::process::Child, since: std::time::Instant) -> (Value, u64) {
    let output = child.wait_with_output().unwrap();
    let elapsed = since.elapsed().as_secs();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    (serde_json::from_slice(&output.stdout).unwrap(), elapsed)
}

#[cfg(unix)]
#[test]
fn repeated_runs_halt_promptly_for_a_stop_or_pause_during_the_interval() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Health probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"Probe reports healthy\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo healthy: yes\"]\nexpect_output = [\"healthy: yes\"]\n",
    );
    fixture.init();
    fixture.commit("fixture");
    fixture.ok(&[
        "consent",
        "grant",
        "repeat-validation",
        "--source",
        "Human: repeat the probe",
    ]);
    let repeat = |times: &'static str, interval: &'static str| {
        [
            "validate",
            "run",
            "TC-cli-0001",
            "--times",
            times,
            "--interval",
            interval,
            "--environment",
            "test-local",
        ]
    };

    // A paused checkpoint is checked before the first attempt; resuming the run after
    // the stage resumes continues with its stored limits.
    fixture.ok(&["stage", "pause", "--reason", "Human stepped away"]);
    let paused = fixture.ok(&repeat("2", "0"));
    assert_eq!(paused["run"]["state"], "paused");
    assert_eq!(paused["run"]["completed"], 0);
    let id = paused["run"]["id"].as_str().unwrap().to_string();
    assert_eq!(
        fixture.ok(&["validate", "resume", &id])["run"]["state"],
        "paused"
    );
    fixture.ok(&["stage", "resume"]);
    let resumed = fixture.ok(&["validate", "resume", &id]);
    assert_eq!(resumed["run"]["state"], "complete");
    assert_eq!(resumed["run"]["completed"], 2);

    // A stop requested during a 30 s interval ends the run without another attempt.
    let started = std::time::Instant::now();
    let child = spawn(&fixture, &repeat("3", "30"));
    first_attempt_recorded(&fixture);
    fixture.ok(&["validate", "stop"]);
    let stop_requested = std::time::Instant::now();
    let (stopped, after_stop) = finish(child, stop_requested);
    assert_eq!(stopped["run"]["state"], "stopped");
    assert_eq!(stopped["run"]["completed"], 1, "{stopped}");
    assert!(
        after_stop < 5,
        "the stop took {after_stop} s to take effect"
    );
    assert!(started.elapsed().as_secs() < 25);
    fs::remove_file(fixture.root().join(".chough/validation.stop")).unwrap();

    // A pause during the interval is honored the same way.
    let child = spawn(&fixture, &repeat("3", "30"));
    first_attempt_recorded(&fixture);
    fixture.ok(&["stage", "pause", "--reason", "Human asked to pause"]);
    let (paused, after_pause) = finish(child, std::time::Instant::now());
    assert_eq!(paused["run"]["state"], "paused");
    assert_eq!(paused["run"]["completed"], 1, "{paused}");
    assert!(
        after_pause < 5,
        "the pause took {after_pause} s to take effect"
    );
}

#[test]
fn librarian_snapshots_originals_and_indexes_the_open_backlog() {
    let fixture = Fixture::new();
    fixture.init();
    assert!(
        fixture
            .fails(&["library", "init"])
            .contains("optional and off")
    );
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "on",
        "--source",
        "Human: keep a project library",
    ]);
    fixture.write("design/prototype/COMPANION.md", "# Companion\n");
    fixture.write("design/prototype/index.html", "<p>v1</p>\n");
    fixture.ok(&["library", "init"]);
    let source = fixture.read("design/prototype/index.html");
    let snapshot = fixture.ok(&[
        "library",
        "snapshot",
        "design/prototype/index.html",
        "--reason",
        "Approved version 1",
    ]);
    let base = snapshot["snapshot"].as_str().unwrap().to_string();
    let entry = &snapshot["manifest"]["entries"][0];
    assert_eq!(entry["source"], "design/prototype/index.html");
    assert_eq!(
        fixture.read(entry["copy"].as_str().unwrap()),
        source,
        "snapshot copies the original bytes"
    );
    assert_eq!(fixture.read("design/prototype/index.html"), source);
    assert_eq!(entry["sha256"].as_str().unwrap().len(), 64);
    assert!(base.starts_with("library/snapshots/"));
    for bad in ["../outside.txt", "library/INDEX.md"] {
        assert!(
            !fixture
                .run(&["library", "snapshot", bad, "--reason", "x"])
                .status
                .success()
        );
    }

    fixture.ok(&[
        "bug",
        "add",
        "--title",
        "Prototype focus ring missing",
        "--symptom",
        "No visible focus",
        "--trigger",
        "Tab to the retry link",
        "--expected",
        "Visible focus",
        "--actual",
        "Nothing visible",
        "--environment",
        "chromium",
        "--scope",
        "out-of-scope",
    ]);
    fixture.ok(&["library", "index"]);
    let index = fixture.read("library/INDEX.md");
    assert!(index.contains("BUG-0001 Prototype focus ring missing"));
    assert!(index.contains("[design/prototype/](../design/prototype/) with COMPANION.md"));
    assert!(index.contains("Approved version 1"));
    assert!(
        fixture
            .read("library/README.md")
            .contains("Nothing here is a second current version")
    );

    fixture.ok(&[
        "stage",
        "enter",
        "librarian",
        "--reason",
        "Snapshot the design",
    ]);
    assert_eq!(fixture.ok(&["stage", "return"])["applied"]["to"], "intent");
}

#[cfg(unix)]
#[test]
fn snapshotting_a_symlink_is_refused() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["pref", "set", "librarian", "on", "--source", "Human: yes"]);
    std::os::unix::fs::symlink(
        fixture.root().join("INTENT.md"),
        fixture.root().join("link.md"),
    )
    .unwrap();
    assert!(
        fixture
            .fails(&["library", "snapshot", "link.md", "--reason", "x"])
            .contains("symlink")
    );
}

fn bundle_files() -> Vec<String> {
    fn walk(base: &Path, path: &Path, files: &mut Vec<String>) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(base, &path, files);
            } else {
                files.push(
                    path.strip_prefix(base)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("skills/chough-workflow");
    let mut files = vec![];
    walk(&base, &base, &mut files);
    files.sort();
    files
}

#[test]
fn skills_install_per_host_and_update_only_unmodified_files() {
    let fixture = Fixture::new();
    let output = fixture.ok(&[
        "init",
        "--objective",
        "Install for two hosts",
        "--host",
        "claude",
        "--host",
        "openai",
    ]);
    assert_eq!(output["hosts"], serde_json::json!(["claude", "openai"]));
    let files = bundle_files();
    assert!(files.len() >= 20);
    for host in [".claude/skills", ".agents/skills"] {
        for file in &files {
            let installed = fixture.root().join(host).join("chough-workflow").join(file);
            let canonical = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("skills/chough-workflow")
                .join(file);
            assert_eq!(
                fs::read(&installed).unwrap(),
                fs::read(canonical).unwrap(),
                "{host}/{file}"
            );
        }
    }

    let old = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/skill-0.1.0/SKILL.md"),
    )
    .unwrap()
    // The 0.1.0 digest is of LF bytes; a Windows checkout may have converted them.
    .replace("\r\n", "\n");
    fixture.write(".claude/skills/chough-workflow/SKILL.md", &old);
    fixture.write(
        ".claude/skills/chough-workflow/references/bugs.md",
        "my own notes\n",
    );
    fs::remove_file(
        fixture
            .root()
            .join(".claude/skills/chough-workflow/references/roles.md"),
    )
    .unwrap();
    let state = |report: &Value, file: &str| {
        report["hosts"][0]["files"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["file"] == file)
            .unwrap()
            .clone()
    };
    let status = fixture.ok(&["skill", "status", "--host", "claude"]);
    assert_eq!(state(&status, "SKILL.md")["state"], "known-outdated");
    assert_eq!(state(&status, "references/bugs.md")["state"], "edited");
    assert_eq!(state(&status, "references/roles.md")["state"], "missing");
    assert_eq!(state(&status, "references/stages.md")["state"], "current");

    let updated = fixture.ok(&["skill", "update", "--host", "claude"]);
    assert_eq!(state(&updated, "SKILL.md")["action"], "updated");
    assert_eq!(
        state(&updated, "references/bugs.md")["action"],
        "preserved-edit"
    );
    assert_eq!(state(&updated, "references/roles.md")["action"], "added");
    assert_eq!(
        fixture.read(".claude/skills/chough-workflow/references/bugs.md"),
        "my own notes\n"
    );
    assert_ne!(fixture.read(".claude/skills/chough-workflow/SKILL.md"), old);
    assert!(
        fixture
            .fails(&["init", "--host", "unknown"])
            .contains("invalid value")
    );
}
