mod common;

use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

struct Fixture(TempDir);

impl Fixture {
    fn new() -> Self {
        let fixture = Self(tempfile::tempdir().unwrap());
        fixture.write("AGENTS.md", "Project instructions\n");
        fixture.write("INTENT.md", "A user wants a reliable product.\n");
        fixture.write("specs/cli/cli.spec.md", "# CLI contract\n");
        fixture.write(
            "specs/cli/requirements.md",
            "### REQ-cli-001\nPreserve files.\n",
        );
        fixture.write(
            "specs/cli/context.md",
            "Fallback context, not the lesson source.\n",
        );
        fixture.write("specs/cli/cli.3md", "context plane\n---\nlesson plane\n");
        fixture.write("src/input.txt", "first version\n");
        fixture
    }

    fn root(&self) -> &Path {
        self.0.path()
    }

    fn write(&self, name: &str, content: &str) {
        let path = self.root().join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn run(&self, args: &[&str]) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
        command.arg("--root").arg(self.root()).args(args);
        let mut paths = vec![self.root().join("bin")];
        paths.extend(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        ));
        command.env("PATH", std::env::join_paths(paths).unwrap());
        command.output().unwrap()
    }

    fn ok(&self, args: &[&str]) -> Value {
        let output = self.run(args);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice(&output.stdout).unwrap()
    }

    fn init(&self) {
        self.ok(&[
            "init",
            "--objective",
            "Fix a reported bug",
            "--intent-scope",
            "LIFE",
            "--module",
            "cli",
        ]);
    }

    fn git(&self, args: &[&str]) {
        let output = Command::new("git")
            .current_dir(self.root())
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn native_fixtures(&self) {
        let bin = self.root().join("bin");
        common::stand_in(
            &bin,
            "hi",
            r#"{"hi":1,"export":1,"files":[{"criteria":[{"id":"LIFE-1","text":"Preserve intent"}],"retired":[{"id":"LIFE-2"}]}]}"#,
        );
        common::stand_in(
            &bin,
            "specsync",
            r#"{"next_action":"human_scope_decision","state":"draft"}"#,
        );
        self.write(".specsync/config.toml", "source_dirs = [\"src\"]\n");
    }

    fn evidence_fixture(&self) {
        self.native_fixtures();
        self.init();
        self.git(&["init"]);
        self.git(&["add", "."]);
        self.git(&[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-m",
            "fixture",
        ]);
    }

    fn add(&self, id: &str, kind: &str, outcome: &str) -> Output {
        self.run(&[
            "evidence",
            "add",
            id,
            "--criterion",
            "LIFE-1",
            "--requirement",
            "REQ-cli-001",
            "--contract",
            "specs/cli/cli.spec.md",
            "--kind",
            kind,
            "--outcome",
            outcome,
            "--environment",
            "test-local",
            "--summary",
            "Observed behavior in isolated fixture",
        ])
    }
}

#[test]
fn init_preserves_user_files_and_installs_a_complete_skill() {
    let fixture = Fixture::new();
    fixture.init();
    for name in [
        "SKILL.md",
        "references/lifecycle.md",
        "references/bugs.md",
        "agents/openai.yaml",
    ] {
        assert!(
            fixture
                .root()
                .join(".agents/skills/chough-workflow")
                .join(name)
                .is_file()
        );
    }
    fixture.write(
        ".chough/session.json",
        "user owned bytes, even if malformed\n",
    );
    fixture.write(
        ".agents/skills/chough-workflow/SKILL.md",
        "user owned skill\n",
    );
    let output = fixture.ok(&["init", "--objective", "Do not overwrite"]);
    assert!(output["created"].as_array().unwrap().is_empty());
    assert_eq!(
        fs::read_to_string(fixture.root().join(".chough/session.json")).unwrap(),
        "user owned bytes, even if malformed\n"
    );
    assert_eq!(
        fs::read_to_string(
            fixture
                .root()
                .join(".agents/skills/chough-workflow/SKILL.md")
        )
        .unwrap(),
        "user owned skill\n"
    );
}

#[test]
fn invalid_modules_and_future_schemas_are_rejected() {
    let fixture = Fixture::new();
    assert!(
        !fixture
            .run(&["init", "--module", "../../outside"])
            .status
            .success()
    );
    assert!(!fixture.root().join(".chough").exists());
    fixture.init();
    let path = fixture.root().join(".chough/session.json");
    let mut session: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    session["schema_version"] = json!(99);
    fixture.write(
        ".chough/session.json",
        &serde_json::to_string(&session).unwrap(),
    );
    assert!(!fixture.run(&["context", "--json"]).status.success());
}

#[test]
fn oversized_records_fail_explicitly() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.write(".chough/session.json", &" ".repeat(65_537));
    let output = fixture.run(&["context", "--json"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("65536"));
}

#[test]
fn init_rejects_unreadable_session_sizes_before_writing() {
    let fixture = Fixture::new();
    // JSON escapes each control character to six bytes. Keep the actual argument
    // below Windows' process limit while still exceeding the serialized record cap.
    let objective = "\u{0001}".repeat(12_000);
    assert!(
        !fixture
            .run(&["init", "--objective", &objective])
            .status
            .success()
    );
    assert!(!fixture.root().join(".chough").exists());
    let mut arguments = vec!["init"];
    for _ in 0..17 {
        arguments.extend(["--module", "cli"]);
    }
    assert!(!fixture.run(&arguments).status.success());
    assert!(!fixture.root().join(".chough").exists());
}

#[cfg(unix)]
#[test]
fn aggregate_context_budget_is_explicit() {
    let fixture = Fixture::new();
    fixture.native_fixtures();
    fixture.init();
    for name in [
        "cli.spec.md",
        "requirements.md",
        "tasks.md",
        "testing.md",
        "cli.3md",
    ] {
        fixture.write(&format!("specs/cli/{name}"), &"x".repeat(20_000));
    }
    let session_path = fixture.root().join(".chough/session.json");
    let mut session: Value =
        serde_json::from_str(&fs::read_to_string(&session_path).unwrap()).unwrap();
    session["modules"] = json!(["cli", "other"]);
    fixture.write(
        ".chough/session.json",
        &serde_json::to_string(&session).unwrap(),
    );
    for name in [
        "other.spec.md",
        "requirements.md",
        "tasks.md",
        "testing.md",
        "context.md",
    ] {
        fixture.write(&format!("specs/other/{name}"), &"x".repeat(20_000));
    }
    let report = fixture.ok(&["context", "--json"]);
    let sections = report["sections"].as_array().unwrap();
    assert!(
        sections
            .iter()
            .any(|section| section["state"] == "omitted-budget")
    );
    let source_bytes: usize = sections
        .iter()
        .map(|section| section["content"].as_str().unwrap().len())
        .sum();
    assert!(source_bytes < 135_000);
}

#[cfg(unix)]
#[test]
fn artifacts_inside_managed_records_are_independently_bound() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    fixture.write(".chough/test-output.txt", "passed\n");
    let output = fixture.run(&[
        "evidence",
        "add",
        "artifact",
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
        "test-local",
        "--summary",
        "Observed fixture",
        "--artifact",
        ".chough/test-output.txt",
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fixture.ok(&["status", "--environment", "test-local", "--json"])["observations"][0]["freshness"],
        "current-observation"
    );
    fixture.write(".chough/test-output.txt", "edited\n");
    assert_eq!(
        fixture.ok(&["status", "--environment", "test-local", "--json"])["observations"][0]["freshness"],
        "stale-artifact"
    );
}

#[cfg(unix)]
#[test]
fn executable_mode_changes_stale_observations() {
    use std::os::unix::fs::PermissionsExt;
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    assert!(fixture.add("mode", "structural", "passed").status.success());
    fs::set_permissions(
        fixture.root().join("src/input.txt"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    assert_eq!(
        fixture.ok(&["status", "--environment", "test-local", "--json"])["observations"][0]["freshness"],
        "stale-revision-or-inputs"
    );
}

#[cfg(unix)]
#[test]
fn symlink_write_escape_is_rejected_before_any_file_is_created() {
    let fixture = Fixture::new();
    let outside = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(outside.path(), fixture.root().join(".agents")).unwrap();
    assert!(!fixture.run(&["init"]).status.success());
    assert!(!fixture.root().join(".chough").exists());
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
}

#[cfg(unix)]
#[test]
fn context_resumes_deterministically_and_prefers_native_lesson_source() {
    let fixture = Fixture::new();
    fixture.native_fixtures();
    fixture.init();
    fixture.write("specs/cli/tasks.md", &"x".repeat(20_000));
    let first = fixture.ok(&["context", "--json"]);
    let second = fixture.ok(&["context", "--json"]);
    assert_eq!(first, second);
    assert_eq!(first["session"]["objective"], "Fix a reported bug");
    let sections = first["sections"].as_array().unwrap();
    assert!(sections.iter().any(|section| {
        section["source"] == "specs/cli/cli.3md"
            && section["content"]
                .as_str()
                .unwrap()
                .contains("lesson plane")
    }));
    assert!(
        !sections
            .iter()
            .any(|section| section["source"] == "specs/cli/context.md")
    );
    assert!(sections.iter().any(
        |section| section["source"] == "specs/cli/testing.md" && section["state"] == "missing"
    ));
    let tasks = sections
        .iter()
        .find(|section| section["source"] == "specs/cli/tasks.md")
        .unwrap();
    assert!(tasks["content"].as_str().unwrap().len() < 17_000);
    assert!(
        sections
            .iter()
            .any(|section| section["source"] == "hi export -- LIFE"
                && section["state"] == "observed")
    );
}

#[cfg(unix)]
#[test]
fn observations_bind_revision_inputs_and_environment_without_granting_acceptance() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    assert!(
        fixture
            .add("first", "product-test", "passed")
            .status
            .success()
    );
    let current = fixture.ok(&["status", "--environment", "test-local", "--json"]);
    assert_eq!(
        current["observations"][0]["freshness"],
        "current-observation"
    );
    assert_eq!(current["native_specsync"]["state"], "observed");
    assert_eq!(
        serde_json::from_str::<Value>(current["native_specsync"]["content"].as_str().unwrap())
            .unwrap()["next_action"],
        "human_scope_decision"
    );
    assert!(
        current["human_approval"]
            .as_str()
            .unwrap()
            .starts_with("unknown")
    );
    assert!(current["delivery"].as_str().unwrap().starts_with("unknown"));
    assert_eq!(
        fixture.ok(&["status", "--json"])["observations"][0]["freshness"],
        "environment-unchecked"
    );
    assert_eq!(
        fixture.ok(&["status", "--environment", "other", "--json"])["observations"][0]["freshness"],
        "stale-environment"
    );
    assert!(
        !fixture
            .add("first", "structural", "failed")
            .status
            .success()
    );
    assert!(
        fixture
            .add("second", "human-observation", "passed")
            .status
            .success()
    );
    assert_eq!(
        fixture.ok(&["status", "--environment", "test-local", "--json"])["observations"][0]["freshness"],
        "current-observation"
    );
    fixture.write("src/input.txt", "changed without a commit\n");
    assert_eq!(
        fixture.ok(&["status", "--environment", "test-local", "--json"])["observations"][0]["freshness"],
        "stale-revision-or-inputs"
    );
}

#[cfg(unix)]
#[test]
fn untracked_inputs_also_stale_observations() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    assert!(
        fixture
            .add("failed-reproduction", "product-test", "failed")
            .status
            .success()
    );
    fixture.write("src/untracked.txt", "new work not yet staged\n");
    let report = fixture.ok(&["status", "--environment", "test-local", "--json"]);
    assert_eq!(
        report["observations"][0]["freshness"],
        "stale-revision-or-inputs"
    );
    assert_eq!(report["observations"][0]["record"]["outcome"], "failed");
}

#[cfg(unix)]
#[test]
fn invalid_traceability_and_escaped_artifacts_are_rejected() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    let common = [
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "structural",
        "--outcome",
        "passed",
        "--environment",
        "test-local",
        "--summary",
        "Observation",
    ];
    for criterion in ["NOPE-1", "LIFE-2"] {
        let mut arguments = vec!["evidence", "add", "invalid", "--criterion", criterion];
        arguments.extend(common);
        assert!(!fixture.run(&arguments).status.success());
    }
    let mut arguments = vec!["evidence", "add", "escaped", "--criterion", "LIFE-1"];
    arguments.extend(common);
    arguments.extend(["--artifact", "../outside"]);
    assert!(!fixture.run(&arguments).status.success());
    assert!(!fixture.root().join(".chough/evidence").exists());
}

#[cfg(unix)]
#[test]
fn missing_native_tools_remain_unknown() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    let output = Command::new(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(fixture.root())
        .args(["status", "--json"])
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["native_specsync"]["state"], "unavailable");
    assert!(report["snapshot"]["unknown"].is_string());
    assert_eq!(report["evidence_state"], "missing");
}

#[cfg(unix)]
#[test]
fn successful_native_process_with_invalid_json_does_not_establish_lifecycle_state() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nprintf '%s\\n' 'not JSON; exit zero is not a lifecycle record'\n",
    );
    let report = fixture.ok(&["status", "--json"]);
    assert_eq!(report["native_specsync"]["state"], "invalid-output");
    assert!(
        report["native_specsync"]["content"]
            .as_str()
            .unwrap()
            .contains("unknown")
    );
    assert!(
        report["native_specsync"]["content"]
            .as_str()
            .unwrap()
            .contains("not JSON")
    );
    fixture.write("bin/specsync", "#!/bin/sh\nprintf '%s\\n' 'true'\n");
    assert_eq!(
        fixture.ok(&["status", "--json"])["native_specsync"]["state"],
        "invalid-output"
    );
    assert_eq!(
        fixture.ok(&["next", "--json"])["native"]["read"],
        "invalid-output"
    );
}

#[cfg(unix)]
#[test]
fn native_reads_end_by_their_deadline_and_retain_bounded_output() {
    use std::os::unix::fs::PermissionsExt;
    use std::time::Instant;
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    fixture.write(".specsync/sdd.json", "{\"enabled\": true}\n");
    fixture.ok(&["checkpoint", "--native-change", "bounded-change"]);
    fixture.write(
        ".specsync/changes/bounded-change/approvals.json",
        "{\"approvals\":[{\"gate\":\"definition\",\"approved_delta_digests\":{}}]}\n",
    );
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    let stand_in = |script: &str| {
        fixture.write("bin/specsync", &format!("#!/bin/sh\n{script}\n"));
        fs::set_permissions(
            fixture.root().join("bin/specsync"),
            fs::Permissions::from_mode(0o755),
        )
        .unwrap();
    };
    let next = |timeout: Option<&str>| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
        command
            .arg("--root")
            .arg(fixture.root())
            .args(["status", "--json"]);
        let mut paths = vec![fixture.root().join("bin")];
        paths.extend(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        ));
        command.env("PATH", std::env::join_paths(paths).unwrap());
        command.env_remove("CHOUGH_NATIVE_TIMEOUT_SECONDS");
        if let Some(seconds) = timeout {
            command.env("CHOUGH_NATIVE_TIMEOUT_SECONDS", seconds);
        }
        let started = Instant::now();
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        (report, started.elapsed().as_secs())
    };

    // A native command that never answers (and whose descendant holds its output) is
    // reported as timed-out by the deadline, and the build gate refuses it.
    stand_in("sleep 43 &\nsleep 43");
    let (report, elapsed) = next(Some("2"));
    assert!(elapsed < 14, "next returned after {elapsed} s");
    assert_eq!(report["native_specsync"]["state"], "timed-out");
    let shown = report["native_specsync"]["content"].as_str().unwrap();
    assert!(shown.contains("no response within 2 s"), "{shown}");
    assert_eq!(report["native_summary"], Value::Null);
    assert_eq!(
        report["checkpoint"]["gate"]["label"],
        "refused:native-unknown"
    );
    assert!(
        report["checkpoint"]["gate"]["detail"]
            .as_str()
            .unwrap()
            .contains("timed-out")
    );
    let survived = (0..30).all(|_| {
        let running = Command::new("pgrep")
            .args(["-f", "sleep 43"])
            .output()
            .unwrap()
            .status
            .success();
        if running {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        running
    });
    assert!(!survived, "a native descendant survived");

    // An invalid override is refused visibly instead of being replaced.
    let (report, _) = next(Some("forever"));
    assert_eq!(report["native_specsync"]["state"], "unavailable");
    assert!(
        report["native_specsync"]["content"]
            .as_str()
            .unwrap()
            .contains(
                "CHOUGH_NATIVE_TIMEOUT_SECONDS must be a whole number of seconds from 1 to 3600"
            )
    );

    // Output beyond the 4 MiB parse bound is counted exactly, not retained.
    stand_in("yes x | head -c 6000000");
    let (report, _) = next(None);
    assert_eq!(report["native_specsync"]["state"], "oversized");
    let shown = report["native_specsync"]["content"].as_str().unwrap();
    assert!(shown.contains("output of 6000000 bytes exceeds"), "{shown}");
    assert!(shown.len() < 17_000);
    assert_eq!(
        report["checkpoint"]["gate"]["label"],
        "refused:native-unknown"
    );

    // A stderr flood is drained beside stdout and shown within the display bound.
    stand_in(
        "yes e | head -c 5000000 >&2\nprintf '{\"summary\":{\"state\":\"approved\",\"next_action\":\"implement\",\"approval_valid\":true},\"change\":{\"affected_specs\":[],\"no_spec_change\":true}}\\n'",
    );
    let (report, _) = next(None);
    assert_eq!(report["native_specsync"]["state"], "observed");
    assert_eq!(report["native_summary"]["state"], "approved");
    assert_eq!(report["checkpoint"]["gate"]["label"], "native:approved");
    stand_in("yes e | head -c 5000000 >&2\nexit 2");
    let (report, _) = next(None);
    assert_eq!(report["native_specsync"]["state"], "failed");
    let shown = report["native_specsync"]["content"].as_str().unwrap();
    assert!(shown.starts_with("exit=Some(2)"), "{shown}");
    assert!(shown.contains("[truncated at 16384 bytes"), "{shown}");
    assert!(shown.len() < 17_000, "{}", shown.len());
}

impl Fixture {
    /// Stage everything and commit, as `git add -A && git commit` would.
    fn commit_all(&self, message: &str) {
        self.git(&["add", "-A"]);
        self.git(&[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "--allow-empty",
            "-m",
            message,
        ]);
    }

    fn freshness(&self, id: &str) -> String {
        let status = self.ok(&["status", "--environment", "test-local", "--json"]);
        status["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|observation| observation["record"]["id"] == id)
            .unwrap_or_else(|| panic!("no observation {id}"))["freshness"]
            .as_str()
            .unwrap()
            .to_string()
    }

    fn observe(&self, id: &str) {
        let output = self.add(id, "product-test", "passed");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(self.freshness(id), "current-observation", "{id}");
    }

    fn head(&self) -> String {
        let output = Command::new("git")
            .current_dir(self.root())
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap();
        String::from_utf8(output.stdout).unwrap().trim().to_string()
    }

    /// A committed fixture with a stub native change in `.specsync/changes/<id>/`.
    fn change_fixture(&self) {
        self.write(
            &format!("{CHANGE}/change.md"),
            "---\nid: demo-change\nstate: draft\ntype: bug_fix\n---\n\n# Demo change\n\nstate: as noted in the body\n",
        );
        self.write(
            &format!("{CHANGE}/deltas/cli.md"),
            "## MODIFIED\n\n### REQUIREMENT REQ-cli-001\n\nPreserve files.\n",
        );
        for artifact in ["requirements.md", "testing.md", "tasks.md", "design.md"] {
            self.write(&format!("{CHANGE}/{artifact}"), "# Artifact\n");
        }
        self.write(&format!("{CHANGE}/state.json"), "{\"state\":\"draft\"}\n");
        self.evidence_fixture();
    }
}

const CHANGE: &str = ".specsync/changes/demo-change";
const ARCHIVE: &str = ".specsync/archive/changes/2026-10-31-demo-change";
const BOOKKEEPING: [&str; 9] = [
    "state.json",
    "approvals.json",
    "verification.json",
    "verification-attempts.json",
    "review.json",
    "review-attempts.json",
    "finalization.json",
    "accepted-state.json",
    "lesson-bundle.md",
];

#[test]
fn committing_only_chough_records_or_an_empty_commit_keeps_an_observation_current() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    let revision = fixture.head();
    fixture.observe("observed");
    fixture.git(&["add", ".chough"]);
    fixture.commit_all("record the observation");
    assert_eq!(fixture.freshness("observed"), "current-observation");
    fixture.commit_all("an empty commit");
    assert_eq!(fixture.freshness("observed"), "current-observation");
    // The revision is still recorded and shown; it no longer decides freshness.
    let status = fixture.ok(&["status", "--environment", "test-local", "--json"]);
    assert_eq!(
        status["observations"][0]["record"]["snapshot"]["revision"],
        revision.as_str()
    );
    assert_ne!(status["snapshot"]["revision"], revision.as_str());
}

#[test]
fn committing_edits_already_present_when_observed_keeps_an_observation_current() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    fixture.write("src/input.txt", "edited before the observation\n");
    fixture.write("src/added.txt", "untracked when observed\n");
    fs::remove_file(fixture.root().join("AGENTS.md")).unwrap();
    fixture.observe("observed");
    fixture.commit_all("commit what was observed");
    assert_eq!(fixture.freshness("observed"), "current-observation");
    // Restoring the deleted file is a content change.
    fixture.write("AGENTS.md", "Project instructions\n");
    assert_eq!(fixture.freshness("observed"), "stale-revision-or-inputs");
}

#[test]
fn any_fingerprinted_content_change_stales_an_observation() {
    let fixture = Fixture::new();
    fixture.change_fixture();
    for (index, (name, content)) in [
        ("src/input.txt", "a tracked edit\n"),
        ("src/untracked.txt", "a new untracked file\n"),
        ("specs/cli/requirements.md", "### REQ-cli-001\nChanged.\n"),
        (".specsync/config.toml", "source_dirs = [\"lib\"]\n"),
        (".specsync/registry.toml", "[registry]\n"),
    ]
    .into_iter()
    .enumerate()
    {
        let id = format!("before-{index}");
        fixture.observe(&id);
        fixture.write(name, content);
        assert_eq!(fixture.freshness(&id), "stale-revision-or-inputs", "{name}");
    }
}

#[test]
fn specsync_bookkeeping_writes_keep_an_observation_current() {
    let fixture = Fixture::new();
    fixture.change_fixture();
    fixture.observe("observed");
    for name in BOOKKEEPING {
        fixture.write(
            &format!("{CHANGE}/{name}"),
            &format!("{{\"written_by\":\"specsync\",\"file\":\"{name}\"}}\n"),
        );
        assert_eq!(
            fixture.freshness("observed"),
            "current-observation",
            "{name}"
        );
    }
    // Materialization changes only the front-matter state.
    fixture.write(
        &format!("{CHANGE}/change.md"),
        "---\nid: demo-change\nstate: implementing\ntype: bug_fix\n---\n\n# Demo change\n\nstate: as noted in the body\n",
    );
    assert_eq!(fixture.freshness("observed"), "current-observation");
    // What `specsync change check --commit` commits.
    fixture.commit_all("specsync verification");
    assert_eq!(fixture.freshness("observed"), "current-observation");
    // A `change.md` with CRLF line endings, as a Windows checkout may write it, has only
    // its front-matter `state:` value blanked too.
    let crlf = |state: &str, body: &str| {
        format!(
            "---\r\nid: demo-change\r\nstate: {state}\r\ntype: bug_fix\r\n---\r\n\r\n# Demo change\r\n\r\nstate: {body}\r\n"
        )
    };
    let change_md = format!("{CHANGE}/change.md");
    fixture.write(&change_md, &crlf("implementing", "as noted in the body"));
    fixture.observe("crlf");
    fixture.write(&change_md, &crlf("verifying", "as noted in the body"));
    assert_eq!(fixture.freshness("crlf"), "current-observation");
    fixture.write(&change_md, &crlf("verifying", "rewritten in the body"));
    assert_eq!(fixture.freshness("crlf"), "stale-revision-or-inputs");
}

#[test]
fn edits_to_a_native_change_scope_stale_an_observation() {
    let fixture = Fixture::new();
    fixture.change_fixture();
    for (index, (name, content)) in [
        ("deltas/cli.md", "## MODIFIED\n\n### REQUIREMENT REQ-cli-001\n\nRewritten.\n"),
        ("requirements.md", "# Requirements, edited\n"),
        ("testing.md", "# Testing, edited\n"),
        ("tasks.md", "- [x] done\n"),
        ("research.md", "# A new artifact\n"),
        ("deltas/state.json", "{\"not\":\"bookkeeping here\"}\n"),
        (
            "change.md",
            "---\nid: demo-change\nstate: draft\ntype: bug_fix\n---\n\n# Demo change\n\nstate: rewritten in the body\n",
        ),
        (
            "change.md",
            "---\nid: demo-change\nstate: draft\ntype: feature\n---\n\n# Demo change\n\nstate: rewritten in the body\n",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let id = format!("before-{index}");
        fixture.observe(&id);
        fixture.write(&format!("{CHANGE}/{name}"), content);
        assert_eq!(fixture.freshness(&id), "stale-revision-or-inputs", "{name}");
    }
}

#[test]
fn archiving_a_change_keeps_an_observation_current_until_an_artifact_changes() {
    let fixture = Fixture::new();
    fixture.change_fixture();
    fixture.observe("observed");
    fs::create_dir_all(fixture.root().join(".specsync/archive/changes")).unwrap();
    fs::rename(fixture.root().join(CHANGE), fixture.root().join(ARCHIVE)).unwrap();
    for name in [
        "review.json",
        "finalization.json",
        "accepted-state.json",
        "lesson-bundle.md",
    ] {
        fixture.write(&format!("{ARCHIVE}/{name}"), "{\"finalized\":true}\n");
    }
    fixture.write(
        &format!("{ARCHIVE}/change.md"),
        "---\nid: demo-change\nstate: archived\ntype: bug_fix\n---\n\n# Demo change\n\nstate: as noted in the body\n",
    );
    assert_eq!(fixture.freshness("observed"), "current-observation");
    fixture.commit_all("archive the change");
    assert_eq!(fixture.freshness("observed"), "current-observation");

    fixture.write(
        &format!("{ARCHIVE}/deltas/cli.md"),
        "## MODIFIED\n\nEdited.\n",
    );
    assert_eq!(fixture.freshness("observed"), "stale-revision-or-inputs");
    fixture.observe("after-edit");
    fixture.write(
        &format!("{ARCHIVE}/notes.md"),
        "not on the bookkeeping list\n",
    );
    assert_eq!(fixture.freshness("after-edit"), "stale-revision-or-inputs");
}

#[test]
fn observations_without_a_content_identity_stay_stale_after_a_commit() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    fixture.observe("legacy");
    let name = fixture.root().join(".chough/evidence/legacy.json");
    let mut record: Value = serde_json::from_str(&fs::read_to_string(&name).unwrap()).unwrap();
    record["snapshot"]
        .as_object_mut()
        .unwrap()
        .remove("content");
    fs::write(&name, serde_json::to_string_pretty(&record).unwrap() + "\n").unwrap();
    assert_eq!(fixture.freshness("legacy"), "current-observation");
    // A schema 1 record has no content identity in its schema; one that carries a
    // `content` field anyway keeps the schema 1 revision-bound freshness.
    fixture.observe("schema-one");
    let name = fixture.root().join(".chough/evidence/schema-one.json");
    let mut record: Value = serde_json::from_str(&fs::read_to_string(&name).unwrap()).unwrap();
    record["schema_version"] = json!(1);
    for field in [
        "cases",
        "method",
        "track",
        "argv",
        "exit_code",
        "tests_run",
        "limitations",
    ] {
        record.as_object_mut().unwrap().remove(field);
    }
    assert!(record["snapshot"]["content"].is_string());
    fs::write(&name, serde_json::to_string_pretty(&record).unwrap() + "\n").unwrap();
    assert_eq!(fixture.freshness("schema-one"), "current-observation");
    fixture.commit_all("an empty commit");
    assert_eq!(fixture.freshness("legacy"), "stale-revision-or-inputs");
    assert_eq!(fixture.freshness("schema-one"), "stale-revision-or-inputs");
}

#[test]
fn a_malformed_content_identity_is_refused() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    fixture.observe("malformed");
    let name = fixture.root().join(".chough/evidence/malformed.json");
    let mut record: Value = serde_json::from_str(&fs::read_to_string(&name).unwrap()).unwrap();
    record["snapshot"]["content"] = json!("not-a-sha256-digest");
    fs::write(&name, serde_json::to_string_pretty(&record).unwrap() + "\n").unwrap();
    let output = fixture.run(&["status", "--environment", "test-local", "--json"]);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("invalid evidence"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn accept_lifecycle_and_bugs_references_from_main_2756dc2_are_known_outdated() {
    let fixture = Fixture::new();
    fixture.init();
    let installed = fixture.root().join(".agents/skills/chough-workflow");
    let shipped = [
        ("references/accept.md", ACCEPT_AT_2756DC2),
        ("references/lifecycle.md", LIFECYCLE_AT_2756DC2),
        ("references/bugs.md", BUGS_AT_2756DC2),
    ];
    for (file, content) in shipped {
        fs::write(installed.join(file), content).unwrap();
    }
    let state = |report: &Value, file: &str| {
        report["hosts"][0]["files"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["file"] == file)
            .unwrap()["state"]
            .clone()
    };
    let status = fixture.ok(&["skill", "status"]);
    for (file, _) in shipped {
        assert_eq!(state(&status, file), "known-outdated", "{file}");
    }
    fixture.ok(&["skill", "update"]);
    let status = fixture.ok(&["skill", "status"]);
    for (file, _) in shipped {
        assert_eq!(state(&status, file), "current", "{file}");
    }
}

/// `skills/chough-workflow/references/accept.md` as shipped by main at 2756dc2.
const ACCEPT_AT_2756DC2: &str = r#"# Acceptance and native closeout

Acceptance is where the human decides whether the implementation is what they
wanted. Agent review, a passing command, scope approval or an actor label never
stands in for that decision.

## Prepare the review

1. Run the project lanes and the native scoped check when native status allows it:
   `specsync change check <id> --commit` (stage new source files first).
2. Push the branch, open or update the pull request, and observe hosted CI at the
   real tip. Report product tests, structural checks, native lifecycle state,
   provenance and platforms separately.
3. Present to the human: what changed, the current verification, the relevant case
   observations with freshness, open bugs and limits, and what was not checked.
4. Record the gate: `chough checkpoint --awaiting-human "Implementation review of <id>"`.

If native archive checks are red because review has not happened, say exactly that.
Do not claim all checks are green and do not bypass the gate.

## After the human decides

Only after a real review decision, record it and close natively, following native
status rather than a remembered sequence:

```sh
specsync change review <id> --reviewer "<the human>"
specsync change ship <id>        # or: specsync change finalize <id>
```

Do not commit between review and finalize. Push the archive commit on the same pull
request, wait for required checks, then merge through the repository's normal
process. If several changes share a pull request, review and finalize them one at a
time. If evidence went stale, use the recovery action that `specsync change status`
names.

Then `chough stage advance` moves accept to done; it is refused until native state is
`archived`. Finishing never enters release.

## Compounding

Lessons can be recorded at any stage. Put them in the module's native lesson source
(`<module>.3md` later plane when present, otherwise `context.md`) with the source,
the scope where it applies and the future action it changes. A lesson that changes
behavior or policy may need its own native change. Archival does not mean no bug
remains; open bugs stay in the backlog.
"#;

/// `skills/chough-workflow/references/lifecycle.md` as shipped by main at 2756dc2.
const LIFECYCLE_AT_2756DC2: &str = r#"# Native lifecycle and evidence

Installed `specsync change status <id> --json`, `specsync change ship-status <id>` and
command help are authoritative for the current state and recovery path. This
reference follows SpecSync 6.1; do not hardcode it over another version's output.
Match the SpecSync version used by lifecycle writers and CI.

## Shape of a change

```text
new → interview → artifacts and delta → one human scope approval → implement →
check --commit → push, CI → human implementation review → review → ship/finalize →
push archive, CI → merge
```

| Native state | Chough stage | Gate |
|---|---|---|
| draft | intent, design, plan | build, test, accept and done refused |
| approved, implementing, verifying, with `approval_valid` true | build, test, accept | build, test and accept allowed; done refused |
| archived, with authenticated `terminal_evidence` | test, accept, done | done allowed; build refused (new work needs a new change) |

Chough reads `summary.state`, `summary.approval_valid`, `summary.correction_valid` and,
for archived changes, `summary.terminal_evidence.validity`. The same prerequisite
applies whichever command moves the stage (advance, skip, back, reset, return or
release → done). It refuses when status is missing, cannot be decoded or reports an
invalid approval, and when `.specsync/sdd.json` is not a valid SpecSync policy. It never runs `approve`, `review` or `finalize` for you.

## Approval

Record `specsync change approve <id> --actor "<human>"` only after a real human
decision about this concrete definition. A general instruction to keep working is
not approval of a new definition. Actor text is a portable claim, not identity.

## Verification

`specsync change check <id> --commit` materializes the approved delta and records
targeted structural verification. Stage new source files first. It does not run the
project's tests; product checks are separate observations. It refuses an edited
delta and incomplete tasks. Fix real failures; if evidence is stale, follow the exact
recovery action status names.

## Review, archive, merge

Push only after the applicable local gates and observe required CI at the actual tip.
After the human reviews the change package, diff, delta and evidence:

```sh
specsync change review <id> --reviewer "<human>"
specsync change ship <id>
```

The reviewer may be the scope approver. Do not commit between review and ship or
finalize. Archive on the same pull request, push the archive commit, wait for its
checks, and merge only when every change on that pull request is archived. Review and
finalize several changes one at a time.

Merge, package publication, release, deployment and live health are separate
observations; see [release.md](release.md).

## Evidence records

`chough cases run` and `chough evidence add` bind an observation to the Git revision,
working-tree fingerprint, environment, artifacts and, for cases, the definition
digest. They do not authenticate a witness or verify a remote artifact. A changed
input, artifact, case definition or environment makes the observation stale.
"#;

/// `skills/chough-workflow/references/bugs.md` as shipped by main at 2756dc2.
const BUGS_AT_2756DC2: &str = r#"# Reproduce, classify, deduplicate and record a bug

Use this procedure for a suspected defect, including a failing case. Bugs live
locally first in `.chough/bugs/`. A request to investigate does not authorize
posting an issue, comment or message; publication needs an explicit request or an
authorization the human already established.

## Establish the failure

Find the applicable HI criterion, requirement and case. If the intended behavior is
unclear, keep the observation and ask the smallest question that separates a defect
from a new request. Do not rewrite intent to match the implementation.

Reduce the reproduction to the smallest safe fixture and steps. Run it and record the
result as an observation: `chough cases run <case> --environment <env>` for a bound
case, or `chough evidence add … --outcome failed` with the saved output. A bug without
a linked failed observation is reported as `unverified`. Avoid real credentials,
private data or destructive production actions when a local fixture suffices.

## Classify from evidence

| Class | Evidence needed |
|---|---|
| product-defect | Observed behavior contradicts an applicable criterion or contract |
| regression | Fails now, and a passed observation at an earlier ancestor revision exists (`--regression-baseline <observation>`) |
| environment | The failure is isolated to an identified environment or tool input |
| requirement-gap | Behavior is observed, but the intended outcome is unresolved |
| evidence-gap | Existing claims cannot be reproduced or their evidence is missing or stale |

A plausible cause is a hypothesis until a test supports it. A remembered "it used to
work" is not a regression baseline. State impact separately from priority.

## Search, then record

```sh
chough bug search "<symptom and trigger>" --requirement <REQ> --case <case>
chough bug add --title "…" --symptom "…" --trigger "…" --expected "…" --actual "…" \
  --environment <env> --requirement <REQ> --case <case> --observation <failed-obs> \
  --step "…" --scope blocking|in-scope|out-of-scope --classification product-defect
```

`bug add` refuses when a similar bug exists. Compare symptom, trigger and affected
behavior, not titles. For the same defect, append to the existing record with
`chough bug update <BUG> --observation <id> --note "<what this adds>"`; for a related
but distinct one, add with `--distinct "<why>"` and link it with `--related`.

Search wider sources only when they are in scope, for example `fledge github issues
--state all --limit 100 --json`. That list is bounded and can hide fetch errors; say
what you searched and its limits.

## Scope decides the next step

- `blocking`: the acceptance-relevant behavior is wrong. `chough stage advance` from
  test refuses while it is open; go back to build, fix, and rerun the focused cases.
- `in-scope`: fix within the approved scope when practical.
- `out-of-scope`: keep it visible in the backlog for later work. It does not expand
  the current change.

After a fix, link the passing observation and set `--status verified`, which needs a
linked passed observation. Bugs found after closure normally become new native
changes; use native recovery only when the original recorded evidence needs repair.

## Report candidates

`chough bug report <BUG>` prints a Markdown candidate built from the record, marked as
not published. Review it, and post it only when the human asks. Then return the
resulting URL and add it to the record with a note. Record a lesson in the native
module source only when its source, scope and future use are clear.
"#;

#[test]
fn evidence_add_names_the_contract_for_another_modules_requirement() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    fixture.write("specs/checkout-ui/checkout-ui.spec.md", "# Checkout UI\n");
    fixture.write(
        "specs/checkout-ui/requirements.md",
        "### REQ-checkout-ui-004\nShow the order total.\n",
    );
    let add = |requirement: &str| {
        fixture.run(&[
            "evidence",
            "add",
            "cross-module",
            "--criterion",
            "LIFE-1",
            "--requirement",
            requirement,
            "--contract",
            "specs/cli/cli.spec.md",
            "--kind",
            "product-test",
            "--outcome",
            "passed",
            "--environment",
            "test-local",
            "--summary",
            "Observed the order total",
        ])
    };
    for (requirement, hints, absent) in [
        (
            "REQ-checkout-ui-004",
            vec![
                "requirement REQ-checkout-ui-004 not found in specs/cli/requirements.md",
                "REQ-checkout-ui-004 belongs to module checkout-ui (specs/checkout-ui/requirements.md)",
                "add --contract specs/checkout-ui/checkout-ui.spec.md",
            ],
            "specsync change check",
        ),
        (
            "REQ-cli-999",
            vec![
                "requirement REQ-cli-999 not found in specs/cli/requirements.md",
                "it may be a typo, or a requirement an approved native change adds that is not yet materialized: run specsync change check <change>",
            ],
            "belongs to module",
        ),
    ] {
        let output = add(requirement);
        let error = String::from_utf8_lossy(&output.stderr).into_owned();
        assert_eq!(output.status.code(), Some(1), "{error}");
        for hint in hints {
            assert!(error.contains(hint), "expected {hint:?} in {error}");
        }
        assert!(!error.contains(absent), "{error}");
    }
    assert!(
        !fixture
            .root()
            .join(".chough/evidence/cross-module.json")
            .exists()
    );
    // With the owning contract named, the same link is accepted.
    let output = fixture.run(&[
        "evidence",
        "add",
        "cross-module",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-checkout-ui-004",
        "--contract",
        "specs/checkout-ui/checkout-ui.spec.md",
        "--kind",
        "product-test",
        "--outcome",
        "passed",
        "--environment",
        "test-local",
        "--summary",
        "Observed the order total",
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
