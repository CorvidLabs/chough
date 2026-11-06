mod common;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

struct Fixture(TempDir);

impl Drop for Fixture {
    fn drop(&mut self) {
        common::remove_checkpoint_lock(self.root());
    }
}

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
        let run = |id: &str| {
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
        };
        let allocated = format!("OBS-{id}-000000000000000000000001");
        let output = run(&allocated);
        if !output.status.success() {
            return output;
        }
        let allocated_name = format!(".chough/evidence/{allocated}.json");
        let legacy_name = format!(".chough/evidence/{id}.json");
        if self.root().join(&legacy_name).exists() {
            fs::remove_file(self.root().join(allocated_name)).unwrap();
            return run(id);
        }
        let mut record: Value =
            serde_json::from_str(&fs::read_to_string(self.root().join(&allocated_name)).unwrap())
                .unwrap();
        record["id"] = json!(id);
        self.write(
            &legacy_name,
            &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
        );
        fs::remove_file(self.root().join(allocated_name)).unwrap();
        output
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
fn init_leaves_the_checkpoint_merge_driver_unconfigured() {
    let fixture = Fixture::new();
    fixture.write(
        ".gitattributes",
        ".chough/session.json merge=chough-session\n",
    );
    let git = |args: &[&str]| {
        Command::new("git")
            .current_dir(fixture.root())
            .args(args)
            .env("HOME", fixture.root().join("isolated-home"))
            .env(
                "GIT_CONFIG_GLOBAL",
                fixture.root().join("isolated-global-config"),
            )
            .env(
                "GIT_CONFIG_SYSTEM",
                fixture.root().join("isolated-system-config"),
            )
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .output()
            .unwrap()
    };
    assert!(git(&["init"]).status.success());
    let config = fixture.root().join(".git/config");
    let before = fs::read(&config).unwrap();
    let digest = format!("{:x}", Sha256::digest(&before));
    assert!(
        !git(&["config", "--local", "--get", "merge.chough-session.driver"])
            .status
            .success()
    );
    fixture.init();
    let after = fs::read(&config).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(&after)), digest);
    assert_eq!(after, before);
    assert!(
        !git(&["config", "--local", "--get", "merge.chough-session.driver"])
            .status
            .success()
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
    fixture.write(
        ".chough/session.json",
        &" ".repeat(common::CHECKPOINT_CAP + 1),
    );
    let output = fixture.run(&["context", "--json"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("1048576"));
}

fn session_padded_to(fixture: &Fixture, session: &Value, bytes: usize) -> String {
    let compact = serde_json::to_string(session).unwrap();
    assert!(compact.len() <= bytes, "{}", compact.len());
    let padded = format!("{compact}{}", " ".repeat(bytes - compact.len()));
    fixture.write(".chough/session.json", &padded);
    padded
}

#[test]
fn checkpoint_byte_bound_is_exactly_1048576() {
    let fixture = Fixture::new();
    fixture.init();
    let path = fixture.root().join(".chough/session.json");
    let session: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();

    session_padded_to(&fixture, &session, common::CHECKPOINT_CAP);
    let output = fixture.run(&["context", "--json"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let over = session_padded_to(&fixture, &session, common::CHECKPOINT_CAP + 1);
    let output = fixture.run(&["context", "--json"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("1048576"));
    assert_eq!(fs::read_to_string(&path).unwrap(), over);

    // A checkpoint whose content (not padding) exceeds the bound is refused too.
    let mut large = session.clone();
    for list in ["decisions", "completed"] {
        large[list] = json!(
            (0..140)
                .map(|index| format!("{list}: {index} {}", "d".repeat(4_000)))
                .collect::<Vec<_>>()
        );
    }
    assert!(serde_json::to_vec(&large).unwrap().len() > common::CHECKPOINT_CAP);
    fixture.write(
        ".chough/session.json",
        &serde_json::to_string(&large).unwrap(),
    );
    let output = fixture.run(&["context", "--json"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("1048576"));
}

#[test]
fn observation_pointer_bound_is_400_and_other_lists_stay_at_200() {
    let fixture = Fixture::new();
    fixture.init();
    let path = fixture.root().join(".chough/session.json");
    let session: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    let observations = |count: usize| {
        json!(
            (0..count)
                .map(|index| format!("OBS-bound-{index:03}"))
                .collect::<Vec<_>>()
        )
    };

    let mut at_bound = session.clone();
    at_bound["observations"] = observations(400);
    fixture.write(
        ".chough/session.json",
        &serde_json::to_string(&at_bound).unwrap(),
    );
    let output = fixture.run(&["context", "--json"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let mut over = session.clone();
    over["observations"] = observations(401);
    fixture.write(
        ".chough/session.json",
        &serde_json::to_string(&over).unwrap(),
    );
    let output = fixture.run(&["context", "--json"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("too many observation"));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("limit 400"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    for list in ["decisions", "completed", "bugs"] {
        let mut other = session.clone();
        other[list] = json!(
            (0..201)
                .map(|index| format!("BUG-bound-{index:03}"))
                .collect::<Vec<_>>()
        );
        fixture.write(
            ".chough/session.json",
            &serde_json::to_string(&other).unwrap(),
        );
        let output = fixture.run(&["context", "--json"]);
        assert!(!output.status.success(), "{list} accepted 201 entries");
        assert!(String::from_utf8_lossy(&output.stderr).contains("too many"));
    }
}

fn managed_record_bytes(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, directory: &Path, records: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if entry.file_type().unwrap().is_dir() {
                visit(root, &path, records);
            } else {
                assert!(entry.file_type().unwrap().is_file());
                records.insert(
                    path.strip_prefix(root).unwrap().to_path_buf(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut records = BTreeMap::new();
    visit(root, &root.join(".chough"), &mut records);
    records
}

fn checkpoint_capacity_fixture(history: &str) -> (Fixture, Value) {
    let fixture = Fixture::new();
    fixture.init();
    if history == "selected" {
        fixture.ok(&["checkpoint", "--awaiting-human", "initial recorded gate"]);
    }
    let mut session: Value = serde_json::from_str(
        &fs::read_to_string(fixture.root().join(".chough/session.json")).unwrap(),
    )
    .unwrap();
    if history == "legacy" {
        session.as_object_mut().unwrap().remove("history_cursor");
        fixture.write(
            ".chough/transitions.jsonl",
            "{\"schema_version\":1,\"at_unix_seconds\":100,\"kind\":\"reset\",\"from\":\"done\",\"to\":\"intent\",\"refresh\":[]}\n",
        );
    }
    session["awaiting_human"] = json!(["existing-one", "existing-two"]);
    fixture.write(".chough/artifacts/retained.log", "retained proof bytes\n");
    fixture.write(".chough/session.v1.json", "retained migration bytes\n");
    (fixture, session)
}

fn fill_checkpoint_to_pretty_bytes(session: &mut Value, bytes: usize) {
    // Full-size entries across the text lists (each list holds at most 200), then one
    // entry padded to land on exactly `bytes` pretty-printed plus a newline.
    let lists = ["decisions", "completed", "open_questions", "blockers"];
    let size = |session: &Value| serde_json::to_string_pretty(session).unwrap().len() + 1;
    for list in lists {
        session[list] = json!([]);
    }
    'fill: for list in lists {
        // At most 190 per list, so a test that adds one more entry meets the byte
        // bound, not the 200-entry list bound.
        for _ in 0..190 {
            if size(session) + 4_200 > bytes {
                break 'fill;
            }
            session[list]
                .as_array_mut()
                .unwrap()
                .push(json!("d".repeat(4_000)));
        }
    }
    let last = lists
        .iter()
        .find(|list| session[**list].as_array().unwrap().len() < 190)
        .unwrap();
    session[*last]
        .as_array_mut()
        .unwrap()
        .push(json!(String::new()));
    let index = session[*last].as_array().unwrap().len() - 1;
    let current = size(session);
    assert!(
        current <= bytes && bytes - current <= 4_096,
        "{current} {bytes}"
    );
    session[*last][index] = json!("d".repeat(bytes - current));
    assert_eq!(size(session), bytes);
}

fn assert_checkpoint_refusal_preserves_records(fixture: &Fixture, args: &[&str], bound: &str) {
    // A writer has run before, so the project's lock (#83) already exists, as in any
    // project; the comparison still covers every file, `.chough/local/` included.
    fixture.write(".chough/local/.gitignore", "*\n");
    fixture.write(".chough/local/checkpoint.lock", "");
    let before = managed_record_bytes(fixture.root());
    let output = fixture.run(args);
    assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(bound),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let after = managed_record_bytes(fixture.root());
    let changed: Vec<_> = before
        .keys()
        .chain(after.keys())
        .filter(|path| before.get(*path) != after.get(*path))
        .collect();
    assert!(
        changed.is_empty(),
        "refused checkpoint changed managed records: {changed:?}"
    );
}

#[test]
fn checkpoint_writer_observation_overflow_preserves_all_managed_records() {
    for history in ["empty", "selected", "legacy"] {
        let (fixture, mut session) = checkpoint_capacity_fixture(history);
        session["observations"] = json!(
            (0..400)
                .map(|index| format!("OBS-bound-{index:03}"))
                .collect::<Vec<_>>()
        );
        fixture.write(".chough/session.json", &session.to_string());
        assert_checkpoint_refusal_preserves_records(
            &fixture,
            &[
                "checkpoint",
                "--observation",
                "OBS-bound-400",
                "--clear-awaiting-human",
                "--awaiting-human",
                "existing-one",
                "--awaiting-human",
                "replacement",
            ],
            "limit 400",
        );
    }
}

#[test]
fn checkpoint_writer_byte_overflow_preserves_all_managed_records() {
    for history in ["empty", "selected", "legacy"] {
        let (fixture, mut session) = checkpoint_capacity_fixture(history);
        fill_checkpoint_to_pretty_bytes(&mut session, common::CHECKPOINT_CAP - 1);
        let mut candidate = session.clone();
        candidate["decisions"].as_array_mut().unwrap().push(json!(
            "another decision that exceeds the serialized byte bound"
        ));
        candidate["awaiting_human"] = json!(["existing-one", "replacement"]);
        assert!(serde_json::to_vec(&candidate).unwrap().len() <= common::CHECKPOINT_CAP);
        assert!(
            serde_json::to_string_pretty(&candidate).unwrap().len() + 1 > common::CHECKPOINT_CAP
        );
        fixture.write(".chough/session.json", &session.to_string());
        assert_checkpoint_refusal_preserves_records(
            &fixture,
            &[
                "checkpoint",
                "--decision",
                "another decision that exceeds the serialized byte bound",
                "--clear-awaiting-human",
                "--awaiting-human",
                "existing-one",
                "--awaiting-human",
                "replacement",
            ],
            "1048576",
        );
    }
}

#[test]
fn checkpoint_writer_final_cursor_byte_bound_is_checked_before_gate_events() {
    // An empty cursor only grows when the command actually records a gate event.
    let (fixture, mut session) = checkpoint_capacity_fixture("empty");
    fill_checkpoint_to_pretty_bytes(&mut session, common::CHECKPOINT_CAP);
    fixture.write(".chough/session.json", &session.to_string());
    let next_action = session["next_action"].as_str().unwrap();
    let result = fixture.ok(&["checkpoint", "--next-action", next_action]);
    assert!(result["checkpoint"]["history_cursor"]["head"].is_null());
    assert_eq!(
        fs::read(fixture.root().join(".chough/session.json"))
            .unwrap()
            .len(),
        common::CHECKPOINT_CAP
    );

    for final_bytes in [common::CHECKPOINT_CAP, common::CHECKPOINT_CAP + 1] {
        let (fixture, mut candidate) = checkpoint_capacity_fixture("empty");
        candidate["awaiting_human"] = json!(["existing-one", "replacement"]);
        candidate["history_cursor"]["head"] = json!("EVT-000000000000000000000000");
        fill_checkpoint_to_pretty_bytes(&mut candidate, final_bytes);
        let mut initial = candidate.clone();
        initial["awaiting_human"] = json!(["existing-one", "existing-two"]);
        initial["history_cursor"]["head"] = Value::Null;
        let initial_bytes = serde_json::to_string_pretty(&initial).unwrap().len() + 1;
        assert!(initial_bytes <= common::CHECKPOINT_CAP);
        fixture.write(".chough/session.json", &initial.to_string());
        let args = [
            "checkpoint",
            "--clear-awaiting-human",
            "--awaiting-human",
            "existing-one",
            "--awaiting-human",
            "replacement",
        ];
        if final_bytes > common::CHECKPOINT_CAP {
            assert_checkpoint_refusal_preserves_records(&fixture, &args, "1048576");
            continue;
        }
        let result = fixture.ok(&args);
        assert_eq!(
            fs::read(fixture.root().join(".chough/session.json"))
                .unwrap()
                .len(),
            final_bytes
        );
        let cursor = &result["checkpoint"]["history_cursor"];
        let cycle = cursor["cycle"].as_str().unwrap();
        let mut head = cursor["head"].as_str().map(str::to_owned);
        let mut transitions = vec![];
        while let Some(id) = head {
            let path = format!(".chough/history/cycles/{cycle}/{id}.json");
            let event: Value =
                serde_json::from_str(&fs::read_to_string(fixture.root().join(path)).unwrap())
                    .unwrap();
            transitions.push((
                event["transition"]["kind"].as_str().unwrap().to_owned(),
                event["transition"]["reason"].as_str().unwrap().to_owned(),
            ));
            head = event["parent"].as_str().map(str::to_owned);
        }
        transitions.reverse();
        assert_eq!(
            transitions,
            [
                ("gate-closed".into(), "existing-one".into()),
                ("gate-closed".into(), "existing-two".into()),
                ("gate-opened".into(), "existing-one".into()),
                ("gate-opened".into(), "replacement".into()),
            ]
        );
    }
}

#[test]
fn records_other_than_the_checkpoint_keep_the_65536_byte_bound() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.write(
        ".chough/preferences.json",
        &format!(
            "{}{}",
            r#"{"schema_version":1,"preferences":[]}"#,
            " ".repeat(65_536)
        ),
    );
    let output = fixture.run(&["pref", "show"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("65536"));
}

/// `init` validates the whole checkpoint before it writes anything. Since the 1 MiB cap
/// (DRIVE-14), no command-line objective can serialize past the cap within the 128 KiB
/// Linux per-argument limit, so the over-cap refusal is covered at the unit level by
/// `checkpoint::tests::validation_refuses_a_checkpoint_over_the_cap`; here a module list
/// over its bound proves that a refused `init` writes nothing.
#[test]
fn init_rejects_unreadable_session_sizes_before_writing() {
    let fixture = Fixture::new();
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
    // The budget is a display bound: its markers send the reader to the source and
    // never ask for a shorter one (LIFE-15).
    for state in ["omitted-budget", "truncated-budget"] {
        let section = sections
            .iter()
            .find(|section| section["state"] == state)
            .unwrap();
        let shown = section["content"].as_str().unwrap();
        assert!(shown.contains("never shorten"), "{state}: {shown}");
    }
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
        "OBS-product-test-000000000000000000000001",
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
        let mut arguments = vec![
            "evidence",
            "add",
            "OBS-structural-000000000000000000000002",
            "--criterion",
            criterion,
        ];
        arguments.extend(common);
        assert!(!fixture.run(&arguments).status.success());
    }
    let mut arguments = vec![
        "evidence",
        "add",
        "OBS-structural-000000000000000000000003",
        "--criterion",
        "LIFE-1",
    ];
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
    use std::time::Instant;
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    fixture.write(".specsync/sdd.json", "{\"enabled\": true}\n");
    // The checkpoint's status read must name the change (review thread on #71).
    common::stand_in(
        &fixture.root().join("bin"),
        "specsync",
        r#"{"summary":{"id":"bounded-change","state":"draft"}}"#,
    );
    fixture.ok(&["checkpoint", "--native-change", "bounded-change"]);
    fixture.write(
        ".specsync/changes/bounded-change/approvals.json",
        "{\"approvals\":[{\"gate\":\"definition\",\"approved_delta_digests\":{}}]}\n",
    );
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    let stand_in = |script: &str| {
        common::install_executable(
            &fixture.root().join("bin/specsync"),
            format!("#!/bin/sh\n{script}\n").as_bytes(),
        );
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
    assert!(shown.starts_with("exit 2\n"), "{shown}");
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

#[test]
fn build_reference_from_main_4daf861_is_known_outdated() {
    let fixture = Fixture::new();
    fixture.init();
    let installed = fixture.root().join(".agents/skills/chough-workflow");
    fs::write(installed.join("references/build.md"), BUILD_AT_4DAF861).unwrap();
    let state = |report: &Value| {
        report["hosts"][0]["files"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["file"] == "references/build.md")
            .unwrap()["state"]
            .clone()
    };
    assert_eq!(state(&fixture.ok(&["skill", "status"])), "known-outdated");
    fixture.ok(&["skill", "update"]);
    assert_eq!(state(&fixture.ok(&["skill", "status"])), "current");
    assert!(
        fs::read_to_string(installed.join("references/build.md"))
            .unwrap()
            .contains("## Keep it composable")
    );
}

/// `skills/chough-workflow/references/build.md` as shipped by main at 4daf861.
const BUILD_AT_4DAF861: &str = r#"# Build loop

`chough stage advance` from plan enters build only when native status shows the
definition approved. Then:

1. Take the next task from the native `tasks.md`. Stay inside the approved paths.
2. Implement with the project's conventions. Keep contracts, requirements
   traceability, tests and documentation in step.
3. Add or update case definitions for the affected requirements (see
   [testing.md](testing.md)).
4. Run focused checks through Fledge (`fledge run test`, `fledge run lint`, project
   lanes) and inspect the output, not just the exit code.
5. Fix failures and repeat. Record a genuine product failure as a bug.
6. Mark the task done in `tasks.md`, then save a checkpoint:
   `chough checkpoint --completed "<increment>" --next-action "<next step>"`.

## Ownership

When subagents help, give each explicit files or modules, the relevant contract and
requirements, and instructions to preserve others' edits. Only one agent edits a
shared file at a time. The checkpoint has one writer; other agents report back
instead of editing it.

## Escalate only what matters

Ordinary choices inside the approved scope are yours. Escalate a genuine blocker or
a decision that would change intent, contract, acceptance criteria or scope. Record
it with `--blocker` or `--awaiting-human`, and go back to plan if scope must change.

A model saying a task is done is not a native transition, a passed test or a review.
Native records and process results are what count.
"#;

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
            "OBS-product-test-000000000000000000000004",
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
            .join(".chough/evidence/OBS-product-test-000000000000000000000004.json")
            .exists()
    );
    // With the owning contract named, the same link is accepted.
    let output = fixture.run(&[
        "evidence",
        "add",
        "OBS-product-test-000000000000000000000004",
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

/// The exercise's exact work-directory setup and exit cleanup, run with a relative TMPDIR:
/// a created directory is removed only after a passing full run, and every other directory is kept.
#[cfg(unix)]
#[test]
fn exercise_cleanup_handles_a_relative_tmpdir() {
    let script = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/feature-bug-loop.sh"),
    )
    .unwrap();
    let begin = script.find("# work-dir setup: begin").unwrap();
    let end = script.find("# work-dir setup: end").unwrap();
    let setup = &script[begin..end];
    let run = |args: &[&str], status: i32| {
        let cwd = TempDir::new().unwrap();
        let body = format!("set -eu\n{setup}cd \"$work\"\nexit {status}\n");
        let out = Command::new("sh")
            .arg("-c")
            .arg(body)
            .arg("feature-bug-loop.sh")
            .args(args)
            .current_dir(cwd.path())
            .env("TMPDIR", ".")
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(status), "{out:?}");
        let left: Vec<String> = fs::read_dir(cwd.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        (cwd, left, String::from_utf8_lossy(&out.stderr).into_owned())
    };
    let created = |left: &[String]| left.iter().any(|n| n.starts_with("chough-exercise."));

    let (_cwd, left, _) = run(&[], 0);
    assert!(left.is_empty(), "a passing full run left {left:?}");

    let (_cwd, left, err) = run(&[], 1);
    assert!(created(&left) && err.contains("Kept /"), "{left:?} {err}");

    let (_cwd, left, err) = run(&["", "prepare-cold-start"], 0);
    assert!(created(&left) && err.contains("Kept /"), "{left:?} {err}");

    let cwd = TempDir::new().unwrap();
    fs::create_dir(cwd.path().join("given")).unwrap();
    let body = format!("set -eu\n{setup}cd \"$work\"\nexit 0\n");
    let out = Command::new("sh")
        .args(["-c", &body, "feature-bug-loop.sh", "given"])
        .current_dir(cwd.path())
        .env("TMPDIR", ".")
        .output()
        .unwrap();
    assert!(out.status.success(), "{out:?}");
    assert!(
        cwd.path().join("given").is_dir(),
        "a passed-in directory was removed"
    );
}

/// Spec files under `specs/**` are read at any size (Leif, 2026-11-03 10:05 AM MT:
/// "Spec files can grow to whatever number they want"), while Chough's own
/// records keep their bounds (see the preferences test above).
#[test]
fn spec_requirements_companions_are_read_at_any_size() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    // Over the 65,536-byte record bound and over the 4 MiB native record bound.
    let padding = "Filler line that keeps this companion large.\n".repeat(120_000);
    assert!(padding.len() > 4 * 1_048_576);
    fixture.write(
        "specs/cli/requirements.md",
        &format!("{padding}### REQ-cli-001\nPreserve files.\n"),
    );
    fixture.write("specs/checkout-ui/checkout-ui.spec.md", "# Checkout UI\n");
    fixture.write(
        "specs/checkout-ui/requirements.md",
        &format!("{padding}### REQ-checkout-ui-004\nShow the order total.\n"),
    );
    let add = |id: &str, requirement: &str| {
        fixture.run(&[
            "evidence",
            "add",
            id,
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
            "Observed preserved files",
        ])
    };
    let accepted = add("OBS-product-test-000000000000000000000071", "REQ-cli-001");
    assert!(
        accepted.status.success(),
        "{}",
        String::from_utf8_lossy(&accepted.stderr)
    );
    let refused = add(
        "OBS-product-test-000000000000000000000072",
        "REQ-checkout-ui-004",
    );
    let error = String::from_utf8_lossy(&refused.stderr).into_owned();
    assert_eq!(refused.status.code(), Some(1), "{error}");
    assert!(!error.contains("exceeds"), "{error}");
    assert!(
        error.contains(
            "REQ-checkout-ui-004 belongs to module checkout-ui (specs/checkout-ui/requirements.md)"
        ),
        "{error}"
    );
}

// Issues 62 and 64 (Kimi lab round 2): concurrent checkpoint writers and temp files
// left by a writer killed mid-write.

const LOCK_HOLDER_MODE: &str = "CHOUGH_TEST_LOCK_HOLDER";
const LOCK_HOLDER_ROOT: &str = "CHOUGH_TEST_LOCK_ROOT";
const LOCK_HOLDER_READY: &str = "chough-test-lock-holder-ready";
const HOLDER_DECISION: &str = "Written by the lock holder";

/// The lock file Chough uses for this project (see `common::checkpoint_lock_path`).
fn checkpoint_lock_path(root: &Path) -> PathBuf {
    common::checkpoint_lock_path(root).expect("the project has a .chough directory")
}

fn session_temps(root: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(root.join(".chough"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with(".session.json.") && name.ends_with(".tmp"))
        .collect();
    names.sort();
    names
}

fn chough_command(root: &Path, args: &[&str]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
    command
        .arg("--root")
        .arg(root)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    command
}

/// Not a check on its own. The writer-lock tests below re-run this test binary with
/// `CHOUGH_TEST_LOCK_HOLDER` set, so this runs as a separate process that takes the
/// checkpoint writer lock the way a Chough writer does, writes its temp file (and, in
/// `after-rename` mode, renames it over the session), then waits to be killed.
#[test]
fn checkpoint_writer_lock_holder_process() {
    let Ok(mode) = std::env::var(LOCK_HOLDER_MODE) else {
        return;
    };
    use std::io::Write;
    let root = PathBuf::from(std::env::var(LOCK_HOLDER_ROOT).unwrap());
    // The lock lives in the project's `.chough/local/` (#83), as a writer creates it.
    fs::create_dir_all(checkpoint_lock_path(&root).parent().unwrap()).unwrap();
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(checkpoint_lock_path(&root))
        .unwrap();
    <fs::File as fs4::FileExt>::lock(&lock).unwrap();
    let session_path = root.join(".chough/session.json");
    let mut session: Value = serde_json::from_slice(&fs::read(&session_path).unwrap()).unwrap();
    session["decisions"]
        .as_array_mut()
        .unwrap()
        .push(json!(HOLDER_DECISION));
    let temp = root.join(format!(".chough/.session.json.{}.tmp", std::process::id()));
    fs::write(
        &temp,
        serde_json::to_string_pretty(&session).unwrap() + "\n",
    )
    .unwrap();
    if mode == "after-rename" {
        fs::rename(&temp, &session_path).unwrap();
    }
    let mut stdout = std::io::stdout();
    writeln!(stdout, "{LOCK_HOLDER_READY}").unwrap();
    stdout.flush().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(120));
}

/// Start the lock holder in `mode` and return once it holds the lock.
fn start_lock_holder(root: &Path, mode: &str) -> std::process::Child {
    use std::io::BufRead;
    let mut holder = Command::new(std::env::current_exe().unwrap())
        .args([
            "checkpoint_writer_lock_holder_process",
            "--exact",
            "--nocapture",
            "--test-threads=1",
        ])
        .env(LOCK_HOLDER_MODE, mode)
        .env(LOCK_HOLDER_ROOT, root)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let mut lines = std::io::BufReader::new(holder.stdout.take().unwrap()).lines();
    loop {
        let line = lines
            .next()
            .expect("lock holder exited before it was ready")
            .unwrap();
        if line.contains(LOCK_HOLDER_READY) {
            break;
        }
    }
    holder
}

#[test]
fn checkpoint_writers_in_parallel_keep_every_update() {
    let fixture = Fixture::new();
    fixture.init();
    let lanes = 3;
    for round in 0..8 {
        let children: Vec<_> = (1..=lanes)
            .map(|lane| {
                let decision = format!("Parallel lane {lane} round {round}");
                let observation = format!("OBS-lane-{lane}-round-{round}");
                let bug = format!("BUG-lane-{lane}-round-{round}");
                let change = format!("parallel-lane-{lane}");
                chough_command(
                    fixture.root(),
                    &[
                        "checkpoint",
                        "--decision",
                        &decision,
                        "--observation",
                        &observation,
                        "--bug",
                        &bug,
                        "--native-change",
                        &change,
                    ],
                )
                .spawn()
                .unwrap()
            })
            .collect();
        for child in children {
            let output = child.wait_with_output().unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let response: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(response["saved"], true);
        }
        let session: Value = serde_json::from_str(
            &fs::read_to_string(fixture.root().join(".chough/session.json")).unwrap(),
        )
        .unwrap();
        for lane in 1..=lanes {
            for (list, value) in [
                ("decisions", format!("Parallel lane {lane} round {round}")),
                ("observations", format!("OBS-lane-{lane}-round-{round}")),
                ("bugs", format!("BUG-lane-{lane}-round-{round}")),
            ] {
                assert!(
                    session[list]
                        .as_array()
                        .unwrap()
                        .contains(&Value::from(value.clone())),
                    "round {round}: a successful writer's {value} was lost from {list}"
                );
            }
        }
        let change = session["native_change"].as_str().unwrap();
        assert!(
            (1..=lanes).any(|lane| change == format!("parallel-lane-{lane}")),
            "{change}"
        );
        assert!(session_temps(fixture.root()).is_empty());
    }
}

#[test]
fn checkpoint_writer_killed_before_rename_leaves_a_temp_the_next_writer_removes() {
    let fixture = Fixture::new();
    fixture.init();
    let before = fs::read(fixture.root().join(".chough/session.json")).unwrap();
    let mut holder = start_lock_holder(fixture.root(), "before-rename");
    let temps = session_temps(fixture.root());
    assert_eq!(temps.len(), 1, "{temps:?}");

    // While the writer is alive, the next writer waits and leaves its temp alone.
    let mut next = chough_command(
        fixture.root(),
        &["checkpoint", "--decision", "After the killed writer"],
    )
    .spawn()
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(700));
    assert!(
        next.try_wait().unwrap().is_none(),
        "the next writer did not wait for the live writer"
    );
    assert_eq!(session_temps(fixture.root()), temps);
    assert_eq!(
        fs::read(fixture.root().join(".chough/session.json")).unwrap(),
        before
    );

    // SIGKILL (TerminateProcess on Windows) before the rename: the lock is released,
    // the temp stays behind, and the waiting writer removes it and saves its own update.
    holder.kill().unwrap();
    holder.wait().unwrap();
    let output = next.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains(&temps[0]), "{stderr}");
    assert!(stderr.contains("removed"), "{stderr}");
    assert!(session_temps(fixture.root()).is_empty());
    let session: Value = serde_json::from_str(
        &fs::read_to_string(fixture.root().join(".chough/session.json")).unwrap(),
    )
    .unwrap();
    let decisions = session["decisions"].as_array().unwrap();
    assert!(decisions.contains(&json!("After the killed writer")));
    assert!(!decisions.contains(&json!(HOLDER_DECISION)));
}

#[test]
fn checkpoint_writer_killed_after_rename_keeps_its_saved_update() {
    let fixture = Fixture::new();
    fixture.init();
    let mut holder = start_lock_holder(fixture.root(), "after-rename");
    assert!(session_temps(fixture.root()).is_empty());
    holder.kill().unwrap();
    holder.wait().unwrap();
    let output = chough_command(
        fixture.root(),
        &["checkpoint", "--decision", "After the renamed write"],
    )
    .output()
    .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(!stderr.contains("removed"), "{stderr}");
    let session: Value = serde_json::from_str(
        &fs::read_to_string(fixture.root().join(".chough/session.json")).unwrap(),
    )
    .unwrap();
    let decisions = session["decisions"].as_array().unwrap();
    assert!(decisions.contains(&json!(HOLDER_DECISION)));
    assert!(decisions.contains(&json!("After the renamed write")));
    assert!(session_temps(fixture.root()).is_empty());
}

#[test]
fn checkpoint_writer_removes_only_orphaned_session_temps() {
    let fixture = Fixture::new();
    fixture.init();
    let before = fs::read(fixture.root().join(".chough/session.json")).unwrap();
    fixture.write(".chough/.session.json.4194304.tmp", "{\"partial\":");
    fixture.write(".chough/.session.json.notapid.tmp", "kept\n");
    fixture.write(".chough/.preferences.json.4194304.tmp", "kept\n");
    // Reading does not take the writer lock and changes nothing.
    fixture.ok(&["status"]);
    assert!(
        fixture
            .root()
            .join(".chough/.session.json.4194304.tmp")
            .exists()
    );
    assert_eq!(
        fs::read(fixture.root().join(".chough/session.json")).unwrap(),
        before
    );
    let output = fixture.run(&["checkpoint", "--decision", "After an orphan"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains(".chough/.session.json.4194304.tmp"),
        "{stderr}"
    );
    assert!(
        !fixture
            .root()
            .join(".chough/.session.json.4194304.tmp")
            .exists()
    );
    assert!(
        fixture
            .root()
            .join(".chough/.session.json.notapid.tmp")
            .exists()
    );
    assert!(
        fixture
            .root()
            .join(".chough/.preferences.json.4194304.tmp")
            .exists()
    );
}

/// Read-only commands that take the checkpoint lock leave session temps in place: only
/// a checkpoint write removes them (review of #70, the approved criterion's "read-only
/// commands remove nothing").
#[test]
fn read_only_commands_leave_session_temps_in_place() {
    let fixture = Fixture::new();
    fixture.init();
    let temp = ".chough/.session.json.4194304.tmp";
    for args in [
        vec!["context"],
        vec!["stage", "show"],
        vec!["history", "show"],
        vec!["history", "prune"],
    ] {
        fixture.write(temp, "{\"partial\":");
        let before = fs::read(fixture.root().join(".chough/session.json")).unwrap();
        let output = fixture.run(&args);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{args:?} {stderr}");
        assert!(!stderr.contains("removed"), "{args:?} {stderr}");
        assert!(
            fixture.root().join(temp).is_file(),
            "{args:?} removed the session temp"
        );
        assert_eq!(
            fs::read(fixture.root().join(".chough/session.json")).unwrap(),
            before,
            "{args:?}"
        );
    }
}

/// The exercise's checkpoint lock lives inside its project (#83, DRIVE-11): a Chough write
/// creates `.chough/local/checkpoint.lock` there, and nothing named `chough-checkpoint-*`
/// is left in the run's TMPDIR, whether the run passed, failed or prepared a cold start.
#[cfg(unix)]
#[test]
fn exercise_keeps_its_checkpoint_lock_inside_the_project() {
    let script = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/feature-bug-loop.sh"),
    )
    .unwrap();
    let begin = script.find("# work-dir setup: begin").unwrap();
    let end = script.find("# work-dir setup: end").unwrap();
    let setup = &script[begin..end];
    for (args, status) in [
        (vec![], 0),
        (vec![], 1),
        (vec!["", "prepare-cold-start"], 0),
    ] {
        let temp = TempDir::new().unwrap();
        let cwd = TempDir::new().unwrap();
        let chough = env!("CARGO_BIN_EXE_chough");
        let body = format!(
            "set -eu\n{setup}cd \"$work\"\n\"{chough}\" init --objective Probe --intent-scope LIFE --module cli >/dev/null\n\"{chough}\" checkpoint --decision 'Probe the lock' >/dev/null\nlock=\"$work/.chough/local/checkpoint.lock\"\nprintf '%s\\n' \"$lock\"\n[ -f \"$lock\" ] || exit 9\nexit {status}\n"
        );
        let out = Command::new("sh")
            .arg("-c")
            .arg(body)
            .arg("feature-bug-loop.sh")
            .args(&args)
            .current_dir(cwd.path())
            .env("TMPDIR", temp.path())
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(status), "{args:?} {out:?}");
        let named = String::from_utf8_lossy(&out.stdout).trim().to_string();
        assert!(named.ends_with("/.chough/local/checkpoint.lock"), "{named}");
        let locks: Vec<String> = fs::read_dir(temp.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name.starts_with("chough-checkpoint-"))
            .collect();
        assert!(locks.is_empty(), "{args:?} exit {status} left {locks:?}");
    }
}

/// Issue #83 (DRIVE-11): the writer lock lives in the project, so a writer started with
/// another `TMPDIR` waits for a holder of the project's lock and then saves.
#[cfg(unix)]
#[test]
fn writers_with_different_temp_directories_share_the_checkpoint_lock() {
    use std::process::Stdio;
    use std::time::Duration;
    let fixture = Fixture::new();
    fixture.init();
    let lock_path = fixture.root().join(".chough/local/checkpoint.lock");
    fs::create_dir_all(lock_path.parent().unwrap()).unwrap();
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .unwrap();
    <fs::File as fs4::FileExt>::lock(&lock).unwrap();
    let other_tmp = tempfile::tempdir().unwrap();
    let decision = "Written with another TMPDIR";
    let mut writer = Command::new(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(fixture.root())
        .args(["checkpoint", "--decision", decision])
        .env("TMPDIR", other_tmp.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    std::thread::sleep(Duration::from_secs(1));
    assert!(
        writer.try_wait().unwrap().is_none(),
        "the writer with another TMPDIR did not wait for the project's lock"
    );
    drop(lock);
    let output = writer.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let session: Value = serde_json::from_str(
        &fs::read_to_string(fixture.root().join(".chough/session.json")).unwrap(),
    )
    .unwrap();
    assert!(
        session["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry == decision),
        "{session:#}"
    );
}

/// Issue #83: the lock file is inside `.chough/local/`, which Git ignores.
#[test]
fn the_checkpoint_lock_lives_in_the_project_and_git_ignores_it() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["checkpoint", "--decision", "Take the lock"]);
    assert!(
        fixture
            .root()
            .join(".chough/local/checkpoint.lock")
            .exists()
    );
    fixture.git(&["init", "-q"]);
    fixture.commit_all("fixture");
    fixture.git(&["check-ignore", "-q", ".chough/local/checkpoint.lock"]);
    let tracked = Command::new("git")
        .current_dir(fixture.root())
        .args(["ls-files", ".chough/local"])
        .output()
        .unwrap();
    assert!(
        tracked.stdout.is_empty(),
        "{}",
        String::from_utf8_lossy(&tracked.stdout)
    );
}

/// Issue #83 item d: legacy writers racing `session adopt` each either save their
/// decision in the one remaining checkpoint or refuse with nothing saved. (#77's
/// `legacy_writers_racing_adopt_end_in_one_mode` asserts only the single mode.)
#[test]
fn legacy_writers_racing_adopt_each_save_or_refuse_with_nothing_saved() {
    for trial in 0..10 {
        let fixture = Fixture::new();
        fixture.init();
        let outputs: Vec<(String, Output)> = std::thread::scope(|scope| {
            let fixture = &fixture;
            let writers: Vec<_> = (0..4)
                .map(|writer| {
                    scope.spawn(move || {
                        let decision = format!("writer {writer} of trial {trial}");
                        let output = fixture.run(&["checkpoint", "--decision", &decision]);
                        (decision, output)
                    })
                })
                .collect();
            let adopt = scope.spawn(move || fixture.run(&["session", "adopt", "alpha"]));
            let outputs = writers
                .into_iter()
                .map(|writer| writer.join().unwrap())
                .collect();
            adopt.join().unwrap();
            outputs
        });
        let file = if fixture.root().join(".chough/session.json").exists() {
            ".chough/session.json"
        } else {
            ".chough/sessions/alpha.json"
        };
        let session: Value =
            serde_json::from_str(&fs::read_to_string(fixture.root().join(file)).unwrap()).unwrap();
        let decisions = session["decisions"].as_array().unwrap();
        for (decision, output) in &outputs {
            let present = decisions.iter().any(|entry| entry == decision.as_str());
            if output.status.success() {
                assert!(
                    present,
                    "trial {trial}: {decision} reported saved but is not in {file}"
                );
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(
                    stderr.contains("was adopted") && stderr.contains("nothing was saved"),
                    "trial {trial}: {stderr}"
                );
                assert!(!present, "trial {trial}: refused {decision} is in {file}");
            }
        }
    }
}

/// Issue #100 (LIFE-15): a HI family export over Chough's 16 KiB display bound is parsed
/// in full to validate criteria. Shortening the human's intent is never the remedy.
#[test]
fn criteria_validate_against_a_hi_family_export_over_16_kib() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    let intent = "The owner own words about this family, kept whole. ".repeat(800);
    let criteria: Vec<Value> = (1..=40)
        .map(|n| json!({"id": format!("LIFE-{n}"), "text": format!("Criterion {n}: {intent}")}))
        .filter(|criterion| criterion["id"] != "LIFE-2")
        .collect();
    let export = json!({
        "hi": 1,
        "export": 1,
        "files": [{"intent": intent, "criteria": criteria, "retired": [{"id": "LIFE-2"}]}]
    })
    .to_string();
    assert!(export.len() > 16_384 * 4, "{}", export.len());
    common::stand_in(&fixture.root().join("bin"), "hi", &export);
    let add = |id: &str, criterion: &str| {
        fixture.run(&[
            "evidence",
            "add",
            id,
            "--criterion",
            criterion,
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
            "Observed against a large family",
        ])
    };
    // The last criterion sits far past the first 16 KiB of the export.
    let accepted = add("OBS-product-test-000000000000000000000100", "LIFE-40");
    assert!(
        accepted.status.success(),
        "{}",
        String::from_utf8_lossy(&accepted.stderr)
    );
    let refused = add("OBS-product-test-000000000000000000000101", "LIFE-2");
    let stderr = String::from_utf8_lossy(&refused.stderr);
    assert!(!refused.status.success());
    assert!(
        stderr.contains("unknown or retired criterion: LIFE-2"),
        "{stderr}"
    );
    assert!(!stderr.contains("16384"), "{stderr}");
    // `cases check` validates a case's criterion links through the same full export.
    let case = |criterion: &str| {
        format!(
            "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Large family\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"{criterion}\"]\nrequirements = [\"REQ-cli-001\"]\npaths = [\"src/\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"git prints its version\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"git\", \"--version\"]\nexpect_output = [\"git version\"]\n"
        )
    };
    fixture.write(".chough/cases/cli.cases.toml", &case("LIFE-40"));
    fixture.ok(&["cases", "check"]);
    fixture.write(".chough/cases/cli.cases.toml", &case("LIFE-2"));
    let refused = fixture.run(&["cases", "check"]);
    assert!(!refused.status.success());
    assert!(
        String::from_utf8_lossy(&refused.stderr).contains("unknown or retired criterion"),
        "{}",
        String::from_utf8_lossy(&refused.stderr)
    );
}

impl Fixture {
    /// An evidence fixture whose SpecSync stand-in reports `big-change` archived with no
    /// acceptance entries, and the archive directory of that change.
    fn big_archive(&self) -> &'static str {
        self.evidence_fixture();
        common::stand_in(
            &self.root().join("bin"),
            "specsync",
            r#"{"summary":{"id":"big-change","state":"archived"},"acceptance_entries":[]}"#,
        );
        let archive = ".specsync/archive/changes/2026-11-04-big-change";
        self.write(
            &format!("{archive}/finalization.json"),
            "{\"schema_version\": 2, \"change_id\": \"big-change\", \"timestamp\": 100}\n",
        );
        archive
    }
}

/// Issue #100 (LIFE-15): SpecSync's own files are read within SpecSync's bound, never
/// Chough's 65,536-byte record bound. Retention reads an archived `finalization.json`
/// larger than that bound to find the latest native manifest.
#[test]
fn an_archived_finalization_over_the_record_bound_is_read() {
    let fixture = Fixture::new();
    let archive = fixture.big_archive();
    let finalization = json!({
        "schema_version": 2,
        "change_id": "big-change",
        "timestamp": 100,
        "note": "x".repeat(70_000)
    })
    .to_string();
    assert!(finalization.len() > 65_536);
    fixture.write(&format!("{archive}/finalization.json"), &finalization);
    let preview = fixture.ok(&["evidence", "prune"]);
    assert_eq!(preview["native"]["state"], "observed", "{preview:#}");
    assert!(
        preview["native"]["detail"]
            .as_str()
            .is_some_and(|detail| !detail.contains("65536")),
        "{preview:#}"
    );
}

/// Issue #100 (LIFE-15): the librarian reads an archived `change.md` larger than the
/// record bound for its title and criteria instead of silently dropping them.
#[test]
fn an_archived_change_md_over_the_record_bound_is_read() {
    let fixture = Fixture::new();
    let archive = fixture.big_archive();
    fixture.write(
        &format!("{archive}/verification.json"),
        "{\"acceptance_manifest\":{\"entries\":[]}}\n",
    );
    fixture.write(
        &format!("{archive}/change.md"),
        &format!(
            "---\nid: big-change\nstate: archived\n---\n\n# A large archived change\n\n## Acceptance Criteria\n\n- It is read whole\n\n## Notes\n\n{}\n",
            "x".repeat(70_000)
        ),
    );
    fixture.ok(&["library", "index"]);
    let entry =
        fs::read_to_string(fixture.root().join("knowledge/archives/big-change.md")).unwrap();
    assert!(entry.contains("A large archived change"), "{entry}");
    assert!(entry.contains("It is read whole"), "{entry}");
}

/// Issue #100 (LIFE-15): a `.specsync/sdd.json` policy larger than the record bound is
/// SpecSync's to judge; a valid disabled policy is not refused as unreadable.
#[test]
fn an_sdd_policy_over_the_record_bound_is_read() {
    let fixture = Fixture::new();
    fixture.evidence_fixture();
    fixture.write(
        ".specsync/sdd.json",
        &format!("{}{{\"enabled\": false}}\n", " ".repeat(70_000)),
    );
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    let advanced = fixture.ok(&["stage", "advance"]);
    assert_eq!(
        advanced["applied"]["gate"], "ungated:sdd-disabled",
        "{advanced:#}"
    );
}

/// Display bounds stay, and their marker sends the reader to the complete source rather
/// than suggesting the source should shrink (LIFE-15).
#[test]
fn display_truncation_points_to_the_complete_source() {
    let fixture = Fixture::new();
    fixture.write("INTENT.md", &"Intent kept whole. ".repeat(2_000));
    fixture.native_fixtures();
    fixture.init();
    let context = fixture.ok(&["context", "--json"]);
    let intent = context["sections"]
        .as_array()
        .unwrap()
        .iter()
        .find(|section| section["source"] == "INTENT.md")
        .unwrap();
    let shown = intent["content"].as_str().unwrap();
    assert!(
        shown.contains("[truncated at 16384 bytes for display only"),
        "{shown}"
    );
    assert!(shown.contains("read the complete source"), "{shown}");
    assert!(shown.contains("never shorten the source"), "{shown}");
}

/// Issue #100 (LIFE-15): the workflow skill says that Chough's bounds never justify
/// editing HI, specs or SpecSync files, and that an oversized native source is a Chough
/// bug to report. The installed copy carries the same words.
#[test]
fn the_skill_says_chough_bounds_never_justify_editing_native_sources() {
    let bundle = Path::new(env!("CARGO_MANIFEST_DIR")).join("skills/chough-workflow");
    let skill = fs::read_to_string(bundle.join("SKILL.md")).unwrap();
    let build = fs::read_to_string(bundle.join("references/build.md")).unwrap();
    for (name, text) in [("SKILL.md", &skill), ("build.md", &build)] {
        let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(
            flat.contains("HI intent or criteria, specs") && flat.contains("SpecSync"),
            "{name}"
        );
        assert!(flat.contains("report a Chough bug"), "{name}");
    }
    let flat = skill.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("Chough's size bounds cover only its `.chough/` records and displays"),
        "{skill}"
    );
    let fixture = Fixture::new();
    fixture.init();
    assert_eq!(
        fs::read_to_string(
            fixture
                .root()
                .join(".agents/skills/chough-workflow/SKILL.md")
        )
        .unwrap(),
        skill
    );
}

/// The text of a repository file with every run of whitespace (line ends included, so CRLF
/// and LF alike) collapsed to one space.
fn flat_repository_text(path: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// REQ-cli-002, third criterion (#138, audit assertion review): the canonical skill and its
/// installed copy state the three certainty labels, that each lesson names its source and
/// limits, and that a project decision outranks a lesson.
#[test]
fn the_skill_states_lesson_labels_sources_and_limits_and_that_decisions_outrank_lessons() {
    for path in [
        "skills/chough-workflow/references/accept.md",
        ".agents/skills/chough-workflow/references/accept.md",
    ] {
        let text = flat_repository_text(path);
        for needed in [
            "Each lesson carries a certainty label:",
            "- `observation`:",
            "- `hypothesis`:",
            "- `verified`:",
            "Each lesson names its source (an observation ID, a bug ID or a commit) and its limits",
            "always outranks a lesson",
            "Chough keeps no lesson store of its own",
        ] {
            assert!(text.contains(needed), "{path} lacks {needed:?}");
        }
    }
}

/// REQ-cli-002, third criterion (#138): Chough points to lessons and keeps no lesson store,
/// index or ledger of its own. A lesson in the module's `## Lessons` section and one in an
/// archived lesson bundle reach no file under `.chough/` or `knowledge/`; the module note
/// links the source and `next` names it without printing either lesson.
#[cfg(unix)]
#[test]
fn chough_points_to_lessons_and_keeps_no_lesson_store_index_or_ledger() {
    const LESSON: &str = "walrus lesson 7f3a stays only in its module source";
    const BUNDLED: &str = "walrus bundle 9c2e stays only in its archive";
    let fixture = Fixture::new();
    fixture.native_fixtures();
    fs::remove_file(fixture.root().join("specs/cli/cli.3md")).unwrap();
    fixture.write(
        "specs/cli/context.md",
        &format!(
            "# Context\n\n## Lessons\n\n- 2026-11-05, {LESSON} (observation). Source: OBS-a. Limits: this fixture.\n"
        ),
    );
    let archive = ".specsync/archive/changes/2026-01-01-old-change";
    fixture.write(
        &format!("{archive}/change.md"),
        &format!(
            "---\nid: old-change\nstate: archived\nbase_commit: {}\n---\n\n# Old change\n\n## Acceptance Criteria\n\n- it works\n",
            "3".repeat(40)
        ),
    );
    fixture.write(
        &format!("{archive}/finalization.json"),
        &format!(
            "{{\"schema_version\": 2, \"change_id\": \"old-change\", \"timestamp\": 1767225600, \"implementation_commit\": \"{}\"}}\n",
            "1".repeat(40)
        ),
    );
    fixture.write(
        &format!("{archive}/lesson-bundle.md"),
        &format!("# Lessons\n\n- {BUNDLED}\n"),
    );
    fixture.init();
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "decided: keep lessons in their source because the skill says so",
    ]);
    let next = fixture.run(&["next"]);
    assert!(next.status.success());
    let next = String::from_utf8_lossy(&next.stdout).into_owned();
    assert!(
        next.contains("specs/cli/context.md (## Lessons at line"),
        "{next}"
    );
    assert!(
        next.contains(&format!("{archive}/lesson-bundle.md")),
        "{next}"
    );
    fixture.ok(&["status"]);
    fixture.ok(&["library", "index"]);
    assert!(
        !next.contains(LESSON) && !next.contains(BUNDLED),
        "next printed a lesson: {next}"
    );
    let mut stored = vec![];
    for dir in [".chough", "knowledge"] {
        let mut pending = vec![fixture.root().join(dir)];
        while let Some(path) = pending.pop() {
            if path.is_dir() {
                pending.extend(
                    fs::read_dir(&path)
                        .unwrap()
                        .map(|entry| entry.unwrap().path()),
                );
            } else if let Ok(text) = fs::read_to_string(&path)
                && (text.contains(LESSON) || text.contains(BUNDLED))
            {
                stored.push(path);
            }
        }
    }
    assert!(stored.is_empty(), "lesson text was stored in {stored:?}");
    let note = fs::read_to_string(fixture.root().join("knowledge/modules/cli.md")).unwrap();
    assert!(
        note.contains("recorded in the `## Lessons` section of")
            && note.contains("[specs/cli/context.md](../../specs/cli/context.md)"),
        "{note}"
    );
}

/// REQ-cli-031, last criterion (#138): the walkthrough begins with the new-project setup
/// path, which covers a minimal HI criteria file, `specsync init` and scaffolded module
/// files, `specsync change adopt`, and what `ungated:sdd-disabled` means.
#[test]
fn the_walkthrough_begins_with_the_new_project_setup_path() {
    let raw = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/walkthrough.md"))
        .unwrap();
    // A Windows checkout has CRLF line endings (#131), so check both forms.
    for text in [
        raw.replace("\r\n", "\n"),
        raw.replace("\r\n", "\n").replace('\n', "\r\n"),
    ] {
        let text = text.replace("\r\n", "\n");
        let first = text
            .split("\n## ")
            .nth(1)
            .unwrap_or_else(|| panic!("the walkthrough has no section"));
        assert!(first.starts_with("New project\n"), "{first:.120}");
        for needed in [
            "hi: 1",
            "## Criteria",
            "hi check",
            "specsync init",
            "specsync scaffold",
            "specsync change adopt",
            "ungated:sdd-disabled",
            "chough init",
        ] {
            assert!(first.contains(needed), "New project lacks {needed:?}");
        }
    }
}
