//! Preferences, repeated validation, librarian and host skills
//! (REQ-cli-011, REQ-cli-013, REQ-cli-015, REQ-cli-016).

mod common;

use common::Fixture;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

fn assert_new_bug_id(id: &str) {
    // `<YYYY-MM-DD>-<slug>`, at most 80 bytes (REQ-records-003).
    assert!(id.len() <= 80, "bug name exceeds 80 bytes: {id}");
    let (date, slug) = id.split_at(10);
    assert!(
        date.bytes()
            .enumerate()
            .all(|(at, byte)| if at == 4 || at == 7 {
                byte == b'-'
            } else {
                byte.is_ascii_digit()
            }),
        "bug name does not start with a date: {id}"
    );
    let slug = slug.strip_prefix('-').expect("a slug after the date");
    assert!(
        !slug.is_empty()
            && slug
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'),
        "slug is not lowercase ASCII: {id}"
    );
    assert!(!slug.starts_with('-') && !slug.ends_with('-') && !slug.contains("--"));
}

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
        ("librarian", "on"),
        ("library-dir", "knowledge"),
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
    let bug_id = run["captured_locally"][0].as_str().unwrap().to_string();
    assert_new_bug_id(&bug_id);

    // A fresh process still finds the finding with every failing observation.
    let bugs = fixture.ok(&["bug", "list"]);
    assert_eq!(bugs["bugs"].as_array().unwrap().len(), 1);
    let bug = fixture.ok(&["bug", "show", &bug_id]);
    assert_eq!(bug["bug"]["evidence"].as_array().unwrap().len(), 3);
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
    assert_eq!(resumed["captured_locally"][0], bug_id);
    let appended = fixture.ok(&["bug", "show", &bug_id]);
    assert_eq!(appended["bug"]["id"], bug_id);
    assert_eq!(appended["bug"]["evidence"].as_array().unwrap().len(), 5);
    assert!(fixture.exists(&format!(".chough/bugs/{bug_id}.md")));
    assert!(!fixture.exists(".chough/local/validation.stop"));

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
        let output = fixture.run(&["validate", "status"]);
        if output.status.success()
            && let Ok(status) = serde_json::from_slice::<Value>(&output.stdout)
        {
            for run in status["runs"].as_array().into_iter().flatten() {
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
    fs::remove_file(fixture.root().join(".chough/local/validation.stop")).unwrap();

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

/// A date and slug record name (REQ-records-003): `<YYYY-MM-DD>-<slug>`, with at most
/// one clash suffix of 6 lowercase hexadecimal digits.
fn assert_dated_name(id: &str, slug: &str) {
    assert!(id.len() <= 80, "record name exceeds 80 bytes: {id}");
    let (date, rest) = id.split_at(10);
    assert!(
        date.as_bytes()[4] == b'-' && date.as_bytes()[7] == b'-',
        "no date: {id}"
    );
    let rest = rest
        .strip_prefix('-')
        .unwrap_or_else(|| panic!("no slug: {id}"));
    match rest.strip_prefix(slug) {
        Some("") => {}
        Some(suffix) => {
            let suffix = suffix
                .strip_prefix('-')
                .unwrap_or_else(|| panic!("bad suffix: {id}"));
            assert!(
                suffix.len() == 6
                    && suffix
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
                "bad clash suffix: {id}"
            );
        }
        None => panic!("expected the slug {slug}: {id}"),
    }
}

/// A project with case TC-cli-0001 and a run consent, in session `key`.
fn validation_fixture(key: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Version probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"Git reports its version\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"git\", \"--version\"]\nexpect_output = [\"git version\"]\n",
    );
    fixture.commit("fixture");
    fixture.ok(&["session", "new", key, "--objective", "Repeat the probe"]);
    fixture.ok(&[
        "consent",
        "grant",
        "repeat-validation",
        "--source",
        "Human: repeat the probe",
    ]);
    fixture
}

/// One single-attempt run: its entry path and the path of its attempt's entry.
fn validation_attempt(fixture: &Fixture) -> (String, String) {
    let run = fixture.ok(&[
        "validate",
        "run",
        "TC-cli-0001",
        "--times",
        "1",
        "--environment",
        "local-macos-arm64",
    ]);
    let id = run["run"]["id"].as_str().unwrap().to_string();
    assert_dated_name(&id, "tc-cli-0001-run");
    let entry = run["run"]["entry"].as_str().unwrap().to_string();
    let session = fixture.active_id();
    assert_eq!(
        entry,
        format!(".chough/sessions/{session}/evidence/{id}.md")
    );
    assert!(fixture.read(&entry).contains("validation_run: true"));
    let observations = run["run"]["observations"].as_array().unwrap();
    assert_eq!(observations.len(), 1, "one attempt needs one observation");
    let observation = observations[0].as_str().unwrap();
    assert_dated_name(observation, "tc-cli-0001");
    let attempt = format!(".chough/sessions/{session}/evidence/{observation}.md");
    assert!(fixture.exists(&attempt), "{attempt}");
    // The raw log stays machine-local (REQ-records-005).
    assert!(
        fixture
            .read(&format!(".chough/local/logs/{observation}.log"))
            .contains("git version")
    );
    (entry, attempt)
}

/// REQ-cli-023: identical repeated runs of one case on one day in one session get
/// distinct names, and two independent roots, each in its own session, never share an
/// entry path.
#[test]
fn independent_roots_and_identical_repeated_runs_allocate_distinct_validation_runs() {
    let first = validation_fixture("first-root");
    let second = validation_fixture("second-root");
    let attempts = [
        validation_attempt(&first),
        validation_attempt(&first),
        validation_attempt(&second),
        validation_attempt(&second),
    ];
    let runs = attempts
        .iter()
        .map(|(run, _)| run.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let observations = attempts
        .iter()
        .map(|(_, observation)| observation.clone())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(runs.len(), attempts.len(), "run entries: {attempts:?}");
    assert_eq!(
        observations.len(),
        attempts.len(),
        "attempt entries: {attempts:?}"
    );
}

/// REQ-cli-023: a 0.5.x run the migration carried resumes by its old ID, which its
/// entry keeps as an alias, with its stored limits and start time.
#[test]
fn legacy_validation_runs_resume_with_their_ids_and_schema_unchanged() {
    let fixture = validation_fixture("legacy-run");
    let old = "tc-cli-0001-1700000000-7";
    let id = "2023-11-14-tc-cli-0001-run";
    let entry = format!(".chough/sessions/{}/evidence/{id}.md", fixture.active_id());
    fixture.write(
        &entry,
        &format!(
            "---\nkind: evidence\nid: {id}\naliases: [{old}]\nvalidation_run: true\nclaim: Repeated validation run of TC-cli-0001\nresult: not-run\nevidence_kind: structural\nrun_case: TC-cli-0001\nenvironment: local-macos-arm64\nauthority: \"run consent: retained legacy authority\"\ntimes: 1\ninterval_seconds: 0\nstate: stopped\nstarted: 2023-11-14T22:13:20Z\nfailures: 0\nnot_run: 0\nelapsed_seconds: 9\nbugs: []\nattempts: []\nobserved: 2023-11-14T22:13:20Z\nrevision: {}\n---\n**A repeated validation run of TC-cli-0001.**\n",
            "0".repeat(40)
        ),
    );
    let resumed = fixture.ok(&["validate", "resume", old]);
    assert_eq!(resumed["run"]["id"], id);
    assert_eq!(resumed["run"]["aliases"][0], old);
    assert_eq!(resumed["run"]["started_at_unix_seconds"], 1_700_000_000_u64);
    assert_eq!(resumed["run"]["completed"], 1);
    assert_eq!(resumed["run"]["times"], 1);
    assert_eq!(resumed["run"]["interval_seconds"], 0);
    assert_eq!(resumed["run"]["environment"], "local-macos-arm64");
    let observation = resumed["run"]["observations"][0].as_str().unwrap();
    assert_dated_name(observation, "tc-cli-0001");
    let status = fixture.ok(&["validate", "status"]);
    assert!(status.to_string().contains(old), "{status:#}");
    let timing = fixture.ok(&["timing", "--json"]);
    assert!(timing.to_string().contains(id), "{timing:#}");
    let stored = fixture.read(&entry);
    for kept in [
        format!("aliases: [{old}]"),
        "started: 2023-11-14T22:13:20Z".to_string(),
        "state: complete".to_string(),
    ] {
        assert!(stored.contains(&kept), "{kept} missing:\n{stored}");
    }
}

#[test]
fn librarian_snapshots_originals_and_indexes_the_open_backlog() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "off",
        "--source",
        "Human: no library yet",
    ]);
    assert!(
        fixture
            .fails(&["library", "init"])
            .contains("librarian is off")
    );
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "on",
        "--source",
        "Human: keep a project library",
    ]);
    // `library snapshot` is retired (REQ-cli-015); init builds the knowledge directory.
    fixture.ok(&["library", "init"]);
    assert!(fixture.exists("knowledge/index.md"));

    let added = fixture.ok(&[
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
    let id = added["id"].as_str().unwrap();
    assert_new_bug_id(id);
    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/index.md");
    assert!(
        index.contains(&format!("- [{id}](../.chough/bugs/{id}."))
            && index.contains(") Prototype focus ring missing ("),
        "{index}"
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

const BASE_5AF3F78_SKILL: &str = r#"---
name: chough-workflow
description: Carry a request or bug from human intent through design, one scope approval, focused testing, local bugs, human acceptance and optional release, with a checkpoint any agent can resume from. Use for Chough-guided development, not unrelated single-file edits.
---

# Chough workflow

You are the agent doing the work. Chough keeps what was wanted, what was checked and
what comes next in project files, so you or another agent can continue later.
Native tools keep their records: HI owns human wording and criterion IDs, SpecSync
owns contracts, changes, approval, verification, review and archive, Fledge runs
tasks, and the actual environment produces evidence. Never copy their state into a
second ledger, and never record a human decision that a human did not make.

## Start or resume

1. Read applicable `AGENTS.md` files. Prefer Fledge for tasks it supports.
2. Find the checkpoint: run `chough next --environment <env>`. Without the helper,
   read `.chough/session.json`, the last lines of `.chough/transitions.jsonl`,
   `.chough/preferences.json`, open bugs in `.chough/bugs/`, and
   `specsync change status <native_change> --json`.
3. Check what is still true: native state, evidence freshness, open blocking bugs,
   and `awaiting_human`. A saved stage or next action is a suggestion; native status
   decides gates.
4. Read [the stage contract](references/stages.md), then only the reference for the
   current stage.

With no checkpoint, run `chough init --objective "<outcome>" --intent-scope <HI-family>
--module <module>` (add `--host claude` or another host as needed); it never
overwrites existing files. A schema 1 session is migrated only with `chough migrate`.

## Stages

| Stage | Reference | Human decision at the end |
|---|---|---|
| intent | [intent.md](references/intent.md), [roles.md](references/roles.md) | Approve the intent as a whole |
| design (only when visuals or interactions change) | [design.md](references/design.md) | Approve rendered prototype and companion together |
| plan | [plan.md](references/plan.md) | One native scope approval |
| build | [build.md](references/build.md) | None for routine work inside scope |
| test | [testing.md](references/testing.md) | None; failures go to bugs |
| accept | [accept.md](references/accept.md), [lifecycle.md](references/lifecycle.md) | Native implementation review |
| release (optional) | [release.md](references/release.md) | Only on request or standing setting |
| bugs, validate, librarian (side stages) | [bugs.md](references/bugs.md), [validate.md](references/validate.md), [librarian.md](references/librarian.md) | Publication or optional runs need a human |

Checkpointing is mandatory in every stage: see [checkpoint.md](references/checkpoint.md).

## Working rules

- Talk naturally. Map requests such as "go back to design", "pause", "skip design,
  it is a CLI change" or "check the library" to `chough stage …` commands, and say
  what you recorded.
- Ask the human only when the answer changes the product, a consequential
  assumption, the approved scope or an optional execution policy. Synthesize review
  findings first; do not forward every observation as a question.
- Keep doing independent authorized work while a human gate is pending, and record
  the gate with `chough checkpoint --awaiting-human "<decision>"`.
- Evidence states what ran, where and on which inputs. A structural pass, exit code,
  actor label or agent review is not behavioral proof, human approval or deployment.
- Nothing is published, released or deployed as a side effect of finishing work.

## Host use

Chough runs in the current agent; no provider API or runtime is required. `chough init
--host <openai|claude|codex|cursor|gemini|grok>` copies this bundle into that host's
project skill directory, and `chough skill status` shows whether a copy is current,
edited or outdated. Confirm that the host actually lists the skill; otherwise give
it this file directly. Subagents are optional: [roles.md](references/roles.md) also
works as sequential passes.
"#;

#[test]
fn skills_install_per_host_and_update_only_unmodified_files() {
    let fixture = Fixture::new();
    let output = fixture.ok(&[
        "init",
        "--objective",
        "Install for two hosts",
        "--host",
        "codex",
        "--host",
        "openai",
    ]);
    assert_eq!(output["hosts"], serde_json::json!(["codex", "openai"]));
    let files = bundle_files();
    assert!(files.len() >= 20);
    for host in [".codex/skills", ".agents/skills"] {
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
    fixture.write(".codex/skills/chough-workflow/SKILL.md", &old);
    fixture.write(
        ".codex/skills/chough-workflow/references/bugs.md",
        "my own notes\n",
    );
    fs::remove_file(
        fixture
            .root()
            .join(".codex/skills/chough-workflow/references/roles.md"),
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
    let status = fixture.ok(&["skill", "status", "--host", "codex"]);
    assert_eq!(state(&status, "SKILL.md")["state"], "known-outdated");
    assert_eq!(state(&status, "references/bugs.md")["state"], "edited");
    assert_eq!(state(&status, "references/roles.md")["state"], "missing");
    assert_eq!(state(&status, "references/stages.md")["state"], "current");

    let updated = fixture.ok(&["skill", "update", "--host", "codex"]);
    assert_eq!(state(&updated, "SKILL.md")["action"], "updated");
    assert_eq!(
        state(&updated, "references/bugs.md")["action"],
        "preserved-edit"
    );
    assert_eq!(state(&updated, "references/roles.md")["action"], "added");
    assert_eq!(
        fixture.read(".codex/skills/chough-workflow/references/bugs.md"),
        "my own notes\n"
    );
    assert_ne!(fixture.read(".codex/skills/chough-workflow/SKILL.md"), old);
    assert!(
        fixture
            .fails(&["init", "--host", "unknown"])
            .contains("invalid value")
    );

    let digest = Sha256::digest(BASE_5AF3F78_SKILL.as_bytes());
    assert_eq!(
        format!("{digest:x}"),
        "d350cc91271e6d942bef516a2b2ea8a6009697a2e0d034b9ff48c8591be2ee5c"
    );
    fixture.write(
        ".agents/skills/chough-workflow/SKILL.md",
        BASE_5AF3F78_SKILL,
    );
    let status = fixture.ok(&["skill", "status", "--host", "openai"]);
    assert_eq!(state(&status, "SKILL.md")["state"], "known-outdated");
    assert_ne!(state(&status, "SKILL.md")["state"], "edited");

    let updated = fixture.ok(&["skill", "update", "--host", "openai"]);
    assert_eq!(state(&updated, "SKILL.md")["action"], "updated");
    let canonical =
        fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("skills/chough-workflow/SKILL.md"))
            .unwrap();
    assert_eq!(
        fs::read(
            fixture
                .root()
                .join(".agents/skills/chough-workflow/SKILL.md"),
        )
        .unwrap(),
        canonical
    );
    let status = fixture.ok(&["skill", "status", "--host", "openai"]);
    assert_eq!(state(&status, "SKILL.md")["state"], "current");

    let edited = format!("{BASE_5AF3F78_SKILL}local edit\n");
    fixture.write(".agents/skills/chough-workflow/SKILL.md", &edited);
    let status = fixture.ok(&["skill", "status", "--host", "openai"]);
    assert_eq!(state(&status, "SKILL.md")["state"], "edited");
    let preserved = fixture.ok(&["skill", "update", "--host", "openai"]);
    assert_eq!(state(&preserved, "SKILL.md")["action"], "preserved-edit");
    assert_eq!(
        fixture.read(".agents/skills/chough-workflow/SKILL.md"),
        edited
    );
}

#[test]
fn checkpoint_reference_updates_only_known_outdated_copies() {
    let fixture = Fixture::new();
    fixture.init();
    let installed = fixture
        .root()
        .join(".agents/skills/chough-workflow/references/checkpoint.md");
    let old = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/skill-ee6e3d2/references/checkpoint.md"),
    )
    .unwrap()
    .replace("\r\n", "\n");
    assert_eq!(
        format!("{:x}", Sha256::digest(old.as_bytes())),
        "20daaaa0cec972169abcedd508cd9ad18ff93ef7ec9d1f4e7330fcd18ca80ac0"
    );
    fixture.write(
        ".agents/skills/chough-workflow/references/checkpoint.md",
        &old,
    );
    let state = |report: &Value| {
        report["hosts"][0]["files"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["file"] == "references/checkpoint.md")
            .unwrap()
            .clone()
    };

    let status = fixture.ok(&["skill", "status", "--host", "openai"]);
    assert_eq!(state(&status)["state"], "known-outdated");
    let updated = fixture.ok(&["skill", "update", "--host", "openai"]);
    assert_eq!(state(&updated)["action"], "updated");
    assert_eq!(
        fs::read(&installed).unwrap(),
        fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("skills/chough-workflow/references/checkpoint.md")
        )
        .unwrap()
    );

    let edited = format!("{old}\nA local checkpoint note.\n");
    fixture.write(
        ".agents/skills/chough-workflow/references/checkpoint.md",
        &edited,
    );
    let preserved = fixture.ok(&["skill", "update", "--host", "openai"]);
    assert_eq!(state(&preserved)["state"], "edited");
    assert_eq!(state(&preserved)["action"], "preserved-edit");
    assert_eq!(fs::read_to_string(installed).unwrap(), edited);
}

#[test]
fn the_librarian_is_on_by_default_in_knowledge_and_can_be_turned_off() {
    // KNOW-1, KNOW-5, KNOW-6: on by default in `knowledge/`, with a recorded opt-out.
    let fixture = Fixture::new();
    fixture.init();
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["knowledge"]["librarian"], "on");
    // REQ-cli-043: init builds the knowledge directory with the librarian on.
    assert_eq!(next["knowledge"]["index"], "knowledge/index.md");
    fixture.ok(&["library", "init"]);
    assert!(fixture.exists("knowledge/index.md"));
    assert!(fixture.exists("knowledge/journal/feed.xml"));
    assert!(fixture.exists("knowledge/lessons"));
    assert!(!fixture.exists("library"));
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["knowledge"]["index"], "knowledge/index.md");
    let text = fixture.run(&["next"]);
    assert!(
        String::from_utf8_lossy(&text.stdout).contains("Knowledge index: knowledge/index.md"),
        "{}",
        String::from_utf8_lossy(&text.stdout)
    );
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "off",
        "--source",
        "Human: no knowledge folder here",
    ]);
    assert!(fixture.fails(&["library", "index"]).contains("off"));
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["knowledge"]["librarian"], "off");
    fixture.ok(&["status"]);
}

#[test]
fn claude_host_installs_the_shared_agents_copy_once() {
    // LIFE-12, REQ-cli-016: one full copy, in the shared `.agents/skills` directory.
    let fixture = Fixture::new();
    let output = fixture.ok(&[
        "init",
        "--objective",
        "One shared copy",
        "--host",
        "claude",
        "--host",
        "openai",
    ]);
    assert!(
        !fixture.root().join(".claude").exists(),
        "init --host claude created .claude/"
    );
    let canonical = Path::new(env!("CARGO_MANIFEST_DIR")).join("skills/chough-workflow");
    for file in bundle_files() {
        assert_eq!(
            fs::read(
                fixture
                    .root()
                    .join(".agents/skills/chough-workflow")
                    .join(&file)
            )
            .unwrap(),
            fs::read(canonical.join(&file)).unwrap(),
            "{file}"
        );
    }
    let created: Vec<&str> = output["created"]
        .as_array()
        .unwrap()
        .iter()
        .map(|path| path.as_str().unwrap())
        .collect();
    let skill_files: Vec<&&str> = created
        .iter()
        .filter(|path| path.contains("skills/chough-workflow/"))
        .collect();
    assert_eq!(skill_files.len(), bundle_files().len(), "{created:?}");
    assert!(skill_files.iter().all(|path| path.starts_with(".agents/")));
    assert_eq!(output["preserved"], serde_json::json!([]));
    let status = fixture.ok(&["skill", "status", "--host", "claude"]);
    assert_eq!(status["hosts"].as_array().unwrap().len(), 1);
    assert_eq!(
        status["hosts"][0]["directory"],
        ".agents/skills/chough-workflow"
    );
    let updated = fixture.ok(&["skill", "update", "--host", "claude", "--host", "openai"]);
    assert_eq!(updated["hosts"].as_array().unwrap().len(), 1);
    assert!(!fixture.root().join(".claude").exists());
}

#[test]
fn legacy_claude_copy_is_reported_and_removed_only_explicitly() {
    // LIFE-13, REQ-cli-016: a legacy `.claude` copy is never deleted silently.
    let fixture = Fixture::new();
    let canonical = Path::new(env!("CARGO_MANIFEST_DIR")).join("skills/chough-workflow");
    let legacy = ".claude/skills/chough-workflow";
    for file in bundle_files() {
        let bytes = fs::read_to_string(canonical.join(&file)).unwrap();
        fixture.write(&format!("{legacy}/{file}"), &bytes);
    }
    fixture.write(&format!("{legacy}/SKILL.md"), BASE_5AF3F78_SKILL);
    fixture.write(&format!("{legacy}/references/bugs.md"), "my own notes\n");
    fixture.write(&format!("{legacy}/NOTES.md"), "unknown file\n");
    let snapshot = |fixture: &Fixture| -> Vec<(String, String)> {
        let mut files: Vec<(String, String)> = bundle_files()
            .into_iter()
            .chain(["NOTES.md".to_string()])
            .map(|file| {
                let path = format!("{legacy}/{file}");
                let body = fixture.read(&path);
                (path, body)
            })
            .collect();
        files.sort();
        files
    };
    let before = snapshot(&fixture);

    // Without the shared copy, removal refuses and changes nothing.
    let refused = fixture.fails(&["skill", "remove-legacy", "--host", "claude"]);
    assert!(
        refused.contains(".agents/skills/chough-workflow"),
        "{refused}"
    );
    assert_eq!(snapshot(&fixture), before);

    fixture.ok(&["init", "--objective", "Legacy copy", "--host", "claude"]);
    assert_eq!(snapshot(&fixture), before, "init touched the legacy copy");
    for command in ["status", "update"] {
        let report = fixture.ok(&["skill", command]);
        let legacy_report = &report["legacy"][0];
        assert_eq!(legacy_report["directory"], legacy, "{command}: {report}");
        assert_eq!(legacy_report["host"], "claude");
        let state = |file: &str| {
            legacy_report["files"]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["file"] == file)
                .unwrap()["state"]
                .clone()
        };
        assert_eq!(state("SKILL.md"), "known-outdated");
        assert_eq!(state("references/bugs.md"), "edited");
        assert_eq!(state("references/stages.md"), "current");
        assert_eq!(state("NOTES.md"), "unknown");
        assert!(
            legacy_report["remove"]
                .as_str()
                .unwrap()
                .contains("chough skill remove-legacy --host claude")
        );
        assert_eq!(
            snapshot(&fixture),
            before,
            "{command} touched the legacy copy"
        );
    }

    let removed = fixture.ok(&["skill", "remove-legacy", "--host", "claude"]);
    let kept: Vec<&str> = removed["legacy"][0]["kept"]
        .as_array()
        .unwrap()
        .iter()
        .map(|file| file.as_str().unwrap())
        .collect();
    assert_eq!(kept, ["NOTES.md", "references/bugs.md"]);
    assert!(!fixture.exists(&format!("{legacy}/SKILL.md")));
    assert!(!fixture.exists(&format!("{legacy}/references/stages.md")));
    assert_eq!(
        fixture.read(&format!("{legacy}/references/bugs.md")),
        "my own notes\n"
    );
    assert_eq!(
        fixture.read(&format!("{legacy}/NOTES.md")),
        "unknown file\n"
    );
    assert!(fixture.exists(".agents/skills/chough-workflow/SKILL.md"));
}

/// D8 (#124): the requirements give the librarian one default, the one the runtime
/// reports (`on`, asserted above). Any requirement sentence that names the librarian, a
/// default and `off` must say it is on by default ("on by default" or "the librarian is
/// on (the default)").
#[test]
fn the_requirements_give_the_librarian_one_default_and_it_is_on() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs/cli/requirements.md");
    let raw = std::fs::read_to_string(path).unwrap();
    // #131: a Windows checkout has CRLF line endings, so check both forms.
    for text in [
        raw.replace("\r\n", "\n"),
        raw.replace("\r\n", "\n").replace('\n', "\r\n"),
    ] {
        let contradicting = librarian_default_contradictions(&text);
        assert!(contradicting.is_empty(), "{contradicting:#?}");
    }
}

/// Requirement sentences that name the librarian, a default and `off` without saying it
/// is on by default.
fn librarian_default_contradictions(text: &str) -> Vec<String> {
    text.replace("\r\n", "\n")
        .replace('\n', " ")
        .split(". ")
        .filter(|sentence| {
            let lower = sentence.to_lowercase();
            lower.contains("librarian")
                && lower.contains("default")
                && lower
                    .split(|c: char| !c.is_alphanumeric())
                    .any(|word| word == "off")
                && !lower.contains("on by default")
                && !lower.contains("librarian is on (the default)")
        })
        .map(str::to_string)
        .collect()
}
