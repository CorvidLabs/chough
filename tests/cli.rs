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

    #[cfg(unix)]
    fn native_fixtures(&self) {
        use std::os::unix::fs::PermissionsExt;
        self.write("bin/hi", "#!/bin/sh\nprintf '%s\\n' '{\"hi\":1,\"export\":1,\"files\":[{\"criteria\":[{\"id\":\"LIFE-1\",\"text\":\"Preserve intent\"}],\"retired\":[{\"id\":\"LIFE-2\"}]}]}'\n");
        self.write("bin/specsync", "#!/bin/sh\nprintf '%s\\n' '{\"next_action\":\"human_scope_decision\",\"state\":\"draft\"}'\n");
        for name in ["hi", "specsync"] {
            fs::set_permissions(
                self.root().join("bin").join(name),
                fs::Permissions::from_mode(0o755),
            )
            .unwrap();
        }
        self.write(".specsync/config.toml", "source_dirs = [\"src\"]\n");
    }

    #[cfg(unix)]
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

    #[cfg(unix)]
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
    let objective = "x".repeat(65_537);
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
    let report = fixture.ok(&["next", "--environment", "test-local", "--json"]);
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
        fixture.ok(&["next", "--json"])["native_specsync"]["state"],
        "invalid-output"
    );
}
