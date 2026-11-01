//! Installable package and fresh installs (REQ-cli-019, LIFE-9, LIFE-4).

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

/// Paths a fresh project or the published crate must never carry: Chough's own
/// development records, examples, tests, documentation and host skill copies.
const FORBIDDEN: &[&str] = &[
    ".chough/",
    ".specsync/",
    "examples/",
    "tests/",
    "docs/",
    ".agents/",
    ".claude/",
];

fn manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn chough(root: &Path, args: &[&str]) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// An empty Git project with a local identity, as a new user would start from.
fn empty_project() -> tempfile::TempDir {
    let project = tempfile::tempdir().unwrap();
    git(project.path(), &["init", "-q"]);
    git(project.path(), &["config", "user.name", "Test"]);
    git(
        project.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(project.path(), &["config", "commit.gpgsign", "false"]);
    project
}

fn bundle_files() -> BTreeSet<String> {
    let base = manifest_dir().join("skills/chough-workflow");
    let mut files = BTreeSet::new();
    let mut pending = vec![base.clone()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else {
                let relative = path.strip_prefix(&base).unwrap();
                files.insert(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    files
}

fn skill_state<'a>(status: &'a Value, file: &str) -> &'a Value {
    status["hosts"][0]["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["file"] == file)
        .unwrap_or_else(|| panic!("{file} missing from {status}"))
}

#[test]
fn the_crate_package_holds_only_the_helper_source_and_canonical_skill() {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output: Output = match Command::new(cargo)
        .current_dir(manifest_dir())
        .args(["package", "--list", "--allow-dirty", "--offline"])
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            eprintln!("skipping: cargo is unavailable ({error})");
            return;
        }
    };
    assert!(
        output.status.success(),
        "cargo package --list: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let listed: BTreeSet<String> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| line.trim().replace('\\', "/"))
        .filter(|line| !line.is_empty())
        .collect();
    for path in &listed {
        assert!(
            !FORBIDDEN.iter().any(|prefix| path.starts_with(prefix)),
            "{path} must not be packaged"
        );
        assert!(
            path.starts_with("src/")
                || path.starts_with("skills/chough-workflow/")
                || [
                    "Cargo.toml",
                    "Cargo.toml.orig",
                    "Cargo.lock",
                    "README.md",
                    "LICENSE",
                    ".cargo_vcs_info.json",
                ]
                .contains(&path.as_str()),
            "{path} is outside the package include list"
        );
    }
    for required in [
        "Cargo.toml",
        "Cargo.lock",
        "README.md",
        "LICENSE",
        "src/main.rs",
    ] {
        assert!(listed.contains(required), "{required} is not packaged");
    }
    for file in bundle_files() {
        let path = format!("skills/chough-workflow/{file}");
        assert!(listed.contains(&path), "{path} is not packaged");
    }
}

#[test]
fn init_in_a_new_project_installs_the_skill_and_an_empty_checkpoint_only() {
    let project = empty_project();
    let root = project.path();
    let created = chough(root, &["init", "--objective", "Try Chough"]);
    let created: BTreeSet<String> = created["created"]
        .as_array()
        .unwrap()
        .iter()
        .map(|path| path.as_str().unwrap().to_string())
        .collect();
    let mut expected: BTreeSet<String> = bundle_files()
        .into_iter()
        .map(|file| format!(".agents/skills/chough-workflow/{file}"))
        .collect();
    expected.insert(".chough/session.json".to_string());
    assert_eq!(created, expected);

    let session: Value =
        serde_json::from_str(&fs::read_to_string(root.join(".chough/session.json")).unwrap())
            .unwrap();
    for empty in [
        "bugs",
        "observations",
        "completed",
        "decisions",
        "blockers",
        "awaiting_human",
        "run_consents",
    ] {
        assert_eq!(session[empty], Value::Array(vec![]), "{empty} is not empty");
    }
    assert_eq!(session["native_change"], Value::Null);
    let records: Vec<String> = fs::read_dir(root.join(".chough"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(records, ["session.json"]);
    for absent in [".specsync", "examples", "tests", "docs", ".claude"] {
        assert!(!root.join(absent).exists(), "{absent} was created");
    }
    let status = chough(root, &["skill", "status"]);
    for file in bundle_files() {
        assert_eq!(skill_state(&status, &file)["state"], "current", "{file}");
    }
}

#[test]
fn a_skill_copy_from_an_earlier_main_build_is_known_outdated_and_updated() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow/references/stages.md");
    assert_eq!(
        format!("{:x}", Sha256::digest(STAGES_AT_F19823D.as_bytes())),
        "3a6d304596b52c88fe33b45ce28ae70701c2c075511e81da18f0d006adc2575a"
    );
    fs::write(&installed, STAGES_AT_F19823D).unwrap();

    let status = chough(root, &["skill", "status"]);
    assert_eq!(
        skill_state(&status, "references/stages.md")["state"],
        "known-outdated"
    );

    chough(root, &["skill", "update"]);
    assert_eq!(
        fs::read(&installed).unwrap(),
        fs::read(manifest_dir().join("skills/chough-workflow/references/stages.md")).unwrap()
    );
    let status = chough(root, &["skill", "status"]);
    assert_eq!(
        skill_state(&status, "references/stages.md")["state"],
        "current"
    );

    // An edit nobody shipped stays the user's and is never overwritten.
    fs::write(&installed, "my own notes\n").unwrap();
    let status = chough(root, &["skill", "update"]);
    assert_eq!(
        skill_state(&status, "references/stages.md")["state"],
        "edited"
    );
    assert_eq!(fs::read_to_string(&installed).unwrap(), "my own notes\n");
}

/// `skills/chough-workflow/references/stages.md` as shipped by main at f19823d.
const STAGES_AT_F19823D: &str = r##"# Stage contract

Every stage uses the same shape so any agent can tell where work stands. The helper
embeds only the stage names, order and gates; this file holds the meaning. Use the
fields as a checklist for your own reasoning, not as a form to fill in front of the
human.

Each stage below states: purpose; inputs; outputs and their owners; useful roles
and tools; completion evidence; approval and preference policy; consequential
questions; what to checkpoint; and transitions.

## Transitions shared by every stage

| Intent | Command | Notes |
|---|---|---|
| Next stage | `chough stage advance` | Native gates apply when entering build and leaving accept |
| Not applicable | `chough stage skip --reason "<why>"` | The reason is required and kept |
| Another pass | `chough stage iterate --note "<what changed>"` | |
| Go back | `chough stage back --to <stage> --reason "<why>"` | Lists what to refresh; history is kept |
| Start over | `chough stage reset [--to <stage>] --reason "<why>"` | History is kept |
| Pause, resume | `chough stage pause --reason "<why>"`, `chough stage resume` | Resume re-checks native state and freshness |
| Side stage | `chough stage enter <side> --reason "<why>"`, then `chough stage return` | Sides: bugs, validate, librarian; the caller is restored |

Going back to intent or plan after native approval means native reapproval if
intent, contract, acceptance criteria or scope change. Follow `specsync change
status`; Chough never changes native state. Dependent design approval, test
observations and evidence named in the refresh list must be redone by their owners.

## intent

- Purpose: capture what the human wants and derive an appropriately sized FRD.
- Inputs: conversation, `INTENT.md`, `hi export <scope>`, existing FRD and module requirements.
- Outputs: HI criteria (HI), FRD sections (`docs/FRD.md`), review findings (working notes).
- Roles and tools: product and user advocate, QA, design, accessibility, engineering, architecture; `hi`.
- Completion evidence: criteria captured with `hi`, FRD traced to them, findings synthesized.
- Approval: the human approves the intent stage as a whole; no per-criterion ceremony.
- Questions: only those that change the product or a consequential assumption.
- Checkpoint: criteria and requirement references, decisions, open questions.
- Transitions: advance to design; back is not available; reset restarts here.

## design

- Purpose: define visuals and interactions in one canonical local HTML and CSS prototype.
- Applicability: skip with a reason when the change has no relevant visuals or interactions.
- Inputs: approved intent, FRD, project conventions and design tokens.
- Outputs: canonical prototype and `COMPANION.md` (design owner), rendered review notes.
- Roles and tools: design, accessibility, user advocate; a browser-capable agent for rendered review.
- Completion evidence: rendered review of states and viewports, companion mappings to HI and FRD.
- Approval: the human approves the rendered prototype and companion together.
- Questions: design choices that change scope go back to intent.
- Checkpoint: prototype and companion paths in `references.design`, pending design approval in `awaiting_human`.
- Transitions: advance to plan; back to intent.

## plan

- Purpose: technical design, task plan, estimates and ownership in the native change.
- Inputs: intent, design, contracts, architecture, native lessons and principles.
- Outputs: native change package and deltas (SpecSync), estimates and ownership (plan artifact).
- Roles and tools: engineering, architecture, QA; `specsync change new/answer`.
- Completion evidence: native artifacts complete and status ready for approval.
- Approval: one human scope approval over the concrete definition, recorded natively.
- Questions: consequential open choices, batched into the one approval question.
- Checkpoint: `native_change`, the approval request in `awaiting_human`.
- Transitions: advance to build only after native approval; back to intent or design.

## build

- Purpose: implement the approved tasks, keeping contracts, tests and docs in sync.
- Inputs: approved native definition, owned files, case definitions.
- Outputs: code, tests, case definitions, documentation (project owners).
- Roles and tools: engineering; Fledge tasks and lanes; subagents with explicit file ownership.
- Completion evidence: focused checks pass locally, tasks updated, checkpoint saved.
- Approval: none for routine work inside scope; scope changes go back to plan.
- Questions: genuine blockers and scope-changing decisions only.
- Checkpoint: completed work, files, blockers, next action after each increment.
- Transitions: advance to test; back to plan, design or intent.

## test

- Purpose: focused verification of the affected criteria and behavior.
- Inputs: selected cases (`chough cases select`), project-required lane.
- Outputs: observations (`.chough/evidence/`), artifacts, bugs for failures.
- Roles and tools: QA; `chough cases run/record`; browser for design QA.
- Completion evidence: selected cases observed; failures captured; omissions explained.
- Approval: none; blocking bugs send work back to build.
- Questions: whether a gap is a product decision rather than a defect.
- Checkpoint: case and observation references, open bugs, test debt.
- Transitions: advance to accept (refused while blocking bugs are open); back to build.

## accept

- Purpose: present the implementation and evidence for the human's review, then close natively.
- Inputs: verification, product evidence, remaining bugs and limits.
- Outputs: native review and archive (SpecSync), lessons (native module sources).
- Roles and tools: the human reviewer; `specsync change check/review/ship/finalize`.
- Completion evidence: native state `archived`.
- Approval: the human implementation review, recorded natively only after it happens.
- Questions: the review itself.
- Checkpoint: the pending review in `awaiting_human`, PR and CI references.
- Transitions: advance to done only once archived; back to build or test.

## done

- Purpose: the change is archived. Merge follows the repository's process.
- Transitions: release only with authorization; reset to start new work.

## release

- Purpose: optional publication, installation or deployment with real project commands.
- Entry: a human request recorded as run consent, or a standing project setting.
- Outputs: distinct publication, installation, deployment and live-health observations.
- Transitions: advance to done.

## bugs

- Side stage for reproducing, classifying, deduplicating and recording a defect locally.
- Returns to its caller; blocking bugs send the main flow back to build.

## validate

- Side stage for optional repeated validation or monitoring.
- Entry needs a run consent or standing preference; findings are kept locally.

## librarian

- Side stage for adding to or organizing the optional project library and preparing briefs.
- Returns to its caller; the librarian never becomes a second current source.
"##;

#[test]
fn bundle_files_from_the_latest_main_build_are_never_reported_as_edited() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow");
    for (file, content, digest) in [
        (
            "references/checkpoint.md",
            CHECKPOINT_AT_9CB1C35,
            "569ff9df89f3564bd1b91c8f231cc6b56af377ab8e2411c57cf052bf5aff71aa",
        ),
        (
            "references/testing.md",
            TESTING_AT_9CB1C35,
            "26029a2ee652ddc55b2ef6a68ed3edfa6a699f3a61a8fffa8d4d485f1180f4fb",
        ),
    ] {
        assert_eq!(
            format!("{:x}", Sha256::digest(content.as_bytes())),
            digest,
            "{file}"
        );
        fs::write(installed.join(file), content).unwrap();
    }

    let status = chough(root, &["skill", "status"]);
    for file in ["references/checkpoint.md", "references/testing.md"] {
        let state = &skill_state(&status, file)["state"];
        assert!(
            *state == "current" || *state == "known-outdated",
            "{file}: {state}"
        );
    }

    chough(root, &["skill", "update"]);
    let status = chough(root, &["skill", "status"]);
    for file in bundle_files() {
        assert_eq!(skill_state(&status, &file)["state"], "current", "{file}");
    }
}

#[test]
fn skill_and_checkpoint_copies_from_main_ee6e3d2_are_known_outdated_and_edited_copies_stay() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow");
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/skill-ee6e3d2");
    let files = [
        (
            "SKILL.md",
            "d350cc91271e6d942bef516a2b2ea8a6009697a2e0d034b9ff48c8591be2ee5c",
        ),
        (
            "references/checkpoint.md",
            "20daaaa0cec972169abcedd508cd9ad18ff93ef7ec9d1f4e7330fcd18ca80ac0",
        ),
    ];
    for (file, digest) in files {
        // The known digests are of LF bytes; a Windows checkout may have converted them.
        let old = fs::read_to_string(fixtures.join(file))
            .unwrap()
            .replace("\r\n", "\n");
        assert_eq!(
            format!("{:x}", Sha256::digest(old.as_bytes())),
            digest,
            "{file}"
        );
        fs::write(installed.join(file), old).unwrap();
    }
    let status = chough(root, &["skill", "status"]);
    for (file, _) in files {
        assert_eq!(
            skill_state(&status, file)["state"],
            "known-outdated",
            "{file}"
        );
    }
    chough(root, &["skill", "update"]);
    let status = chough(root, &["skill", "status"]);
    for file in bundle_files() {
        assert_eq!(skill_state(&status, &file)["state"], "current", "{file}");
    }

    // A copy edited from the ee6e3d2 version is edited, and update keeps it.
    for (file, _) in files {
        let mut edited = fs::read_to_string(fixtures.join(file))
            .unwrap()
            .replace("\r\n", "\n");
        edited.push_str("\nA local note.\n");
        fs::write(installed.join(file), &edited).unwrap();
        let status = chough(root, &["skill", "status"]);
        assert_eq!(skill_state(&status, file)["state"], "edited", "{file}");
        chough(root, &["skill", "update"]);
        assert_eq!(
            fs::read_to_string(installed.join(file)).unwrap(),
            edited,
            "{file}"
        );
    }
}

#[test]
fn checkpoint_and_plan_references_from_main_2756dc2_are_known_outdated() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow");
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/skill-2756dc2");
    for file in ["references/checkpoint.md", "references/plan.md"] {
        // The known digests are of LF bytes; a Windows checkout may have converted them.
        let old = fs::read_to_string(fixtures.join(file))
            .unwrap()
            .replace("\r\n", "\n");
        fs::write(installed.join(file), old).unwrap();
    }
    let status = chough(root, &["skill", "status"]);
    for file in ["references/checkpoint.md", "references/plan.md"] {
        assert_eq!(
            skill_state(&status, file)["state"],
            "known-outdated",
            "{file}"
        );
    }
    chough(root, &["skill", "update"]);
    let status = chough(root, &["skill", "status"]);
    for file in bundle_files() {
        assert_eq!(skill_state(&status, &file)["state"], "current", "{file}");
    }
}

/// `skills/chough-workflow/references/checkpoint.md` as shipped by main at 9cb1c35.
const CHECKPOINT_AT_9CB1C35: &str = r##"# Checkpoint and cold resume

Essential state must never live only in model context. `.chough/session.json` is the
single writable checkpoint. It works without the librarian and without any other
storage adapter.

## What it holds

| Field | Content |
|---|---|
| `objective`, `stage` | Current outcome, stage, status, side-stage callers and iteration |
| `native_change` | The SpecSync change ID; native status decides its state |
| `references` | Criteria, requirements, cases, design sources and files in scope |
| `decisions`, `open_questions` | Decisions with their reason in the sentence; questions that matter |
| `completed`, `observations`, `bugs`, `blockers` | Work done and the records that show it |
| `awaiting_human` | Real gates still waiting for a person |
| `run_consents` | Consent for this objective only |
| `next_action` | Suggested next step; never overrides native state |

Standing preferences are in `.chough/preferences.json`, transitions in
`.chough/transitions.jsonl`, observations in `.chough/evidence/`, bugs in
`.chough/bugs/` and case definitions in `.chough/cases/`.

## When to save

Save after each meaningful increment, before handing off, before context is cleared,
and whenever a human gate opens or closes:

```sh
chough checkpoint --completed "<what was done>" --observation <id> \
  --decision "<decision, because reason>" --next-action "<next step>"
```

Write decision summaries and observations, not private reasoning transcripts.
SpecSync's `Handoff:` line says whether its own records are safe to clear; follow it
too.

## Resume cold

A fresh agent with none of the earlier conversation:

1. Reads `AGENTS.md`, then this skill.
2. Runs `chough next --environment <env>` (or reads the files above directly).
3. Checks native status for `native_change`, freshness of the referenced observations,
   open blocking bugs and `awaiting_human`.
4. Continues with `next_valid_action` under the same scope, preferences and gates. It
   never records a pending human decision itself.

## Schema and concurrency

A schema 1 session keeps working for `context` and `status`. `chough migrate`
converts it after saving the original bytes to `.chough/session.v1.json`. Hand edits
are allowed and validated on every read. With several agents, one agent owns the
checkpoint; others report to it. APS was evaluated as a backend and deferred: it lost
concurrent read-modify-write updates without error and accepts unknown fields.
"##;

/// `skills/chough-workflow/references/testing.md` as shipped by main at 9cb1c35.
const TESTING_AT_9CB1C35: &str = r##"# Focused testing

Test what the change affects, not every criterion the product has.

## Choose the checks

Run `chough cases select --requirement <REQ> --criterion <ID> --path <changed path>`
for the affected scope. Add `--track copy-localization` when visible text,
terminology, comprehension or locale behavior changed, and `--track design-qa` when
visuals or interactions changed. The output lists selected cases, omitted cases with
reasons, and the project-required lane if one is configured
(`chough pref set required-lane <lane> --source …`). Also run proportionate
regressions for nearby behavior and the project's mandatory gates. Explain any
meaningful omission and record test debt in the native `testing.md`.

| Track | When | Typical method |
|---|---|---|
| functional | Always | Automated unit, integration or UI tests; manual checks where needed |
| copy-localization | Visible text or locale behavior changed | Deterministic string assertions plus reviewer notes |
| design-qa | Visuals or interactions changed | Rendered comparison with the canonical prototype at the target viewports and states |

## Definitions are not observations

Definitions live in `.chough/cases/<module>.cases.toml` (profile `chough-testing/1`,
see [the template](../templates/cases.toml)). Each case has a stable ID
`TC-<module>-NNNN`, links to HI criteria and requirements, setup, steps, explicit
expected outcomes, method, environments and expected artifact types. Several cases
can cover one criterion. Never edit an expected outcome to match a failure; retire a
case (`status = "retired"`) instead of deleting or reusing its ID.

`chough cases check` validates every definition, including its HI and requirement
links. The digest of a case ignores its position and title, so reordering never
changes identity, while editing what it asserts makes earlier observations
`stale-case-definition`.

## Execute honestly

- Automated: `chough cases run <ID> --environment <env>` runs the binding without a
  shell and records the observation with the captured log. `cargo-test` passes only
  when exactly one test ran and passed; a filter matching nothing is `not-run`.
  `fledge-task` and `command` pass only with exit 0 and every `expect_output`
  fragment present. The command exits 3 when the outcome is not passed.
- Manual or agent-assisted: `chough cases record <ID> --method manual --verdict 1=pass
  --verdict 2=fail … --summary "…" --artifact <screenshot>` gives a verdict for every
  expected outcome; any fail makes the observation failed.

A failing case stays failed even when every other case passes. Do not rerun until
it passes and report only the pass. Capture the failure with
[the bug procedure](bugs.md).

AI judgment is never the only oracle when an assertion or a reproducible procedure
can decide. Keep rendered, copy and human judgments as observations with method and
limits. Magpie can run real commands, but its generic mode judges exit codes and
records skipped steps as passes; bind a Magpie run only through `command` with an
`expect_output` assertion, and only where it is installed.
"##;

#[test]
fn the_shipped_bundle_holds_no_em_or_en_dash() {
    let base = manifest_dir().join("skills/chough-workflow");
    let mut found = vec![];
    for file in bundle_files() {
        let text = fs::read_to_string(base.join(&file)).unwrap();
        for (index, line) in text.lines().enumerate() {
            if line.contains(['\u{2013}', '\u{2014}']) {
                found.push(format!("{file}:{}: em or en dash", index + 1));
            }
        }
    }
    assert!(found.is_empty(), "{}", found.join("\n"));
}

#[test]
fn dashed_copies_from_main_2756dc2_are_known_outdated_and_updated() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow");
    let earlier = [
        (
            "templates/frd.md",
            FRD_AT_2756DC2,
            "5647b56e21f6667d1ed97381bcbd67a7baf2f60dd8332abcb269e72d4759fd23",
        ),
        (
            "templates/cases.toml",
            CASES_AT_2756DC2,
            "6c7ebaa10b099829bc0db4529e28286ded721b0dfa9b5635a3ae0f37d9d91d4a",
        ),
        (
            "references/testing.md",
            TESTING_AT_2756DC2,
            "eac153b213914bf71a84cb08cda5f0810730a324b112e3caf96ee3be2672b3e9",
        ),
    ];
    for (file, content, digest) in earlier {
        assert_eq!(
            format!("{:x}", Sha256::digest(content.as_bytes())),
            digest,
            "{file}"
        );
        fs::write(installed.join(file), content).unwrap();
    }

    let status = chough(root, &["skill", "status"]);
    for (file, _, _) in earlier {
        assert_eq!(
            skill_state(&status, file)["state"],
            "known-outdated",
            "{file}"
        );
    }
    chough(root, &["skill", "update"]);
    for (file, _, _) in earlier {
        assert_eq!(
            fs::read(installed.join(file)).unwrap(),
            fs::read(manifest_dir().join("skills/chough-workflow").join(file)).unwrap(),
            "{file}"
        );
    }
    let status = chough(root, &["skill", "status"]);
    for file in bundle_files() {
        assert_eq!(skill_state(&status, &file)["state"], "current", "{file}");
    }

    // An edited copy of an earlier file stays the user's.
    let edited = format!("{FRD_AT_2756DC2}\nOur own section.\n");
    fs::write(installed.join("templates/frd.md"), &edited).unwrap();
    let status = chough(root, &["skill", "update"]);
    assert_eq!(skill_state(&status, "templates/frd.md")["state"], "edited");
    assert_eq!(
        fs::read_to_string(installed.join("templates/frd.md")).unwrap(),
        edited
    );
}

/// `skills/chough-workflow/templates/frd.md` as shipped by main at 2756dc2.
const FRD_AT_2756DC2: &str = r##"# <Feature> functional requirements

Status: draft for the intent decision. Derived by the agent from the HI criteria
below; not human wording.

## Purpose

<One paragraph: the outcome the human wants and why it matters.>

## Traceability

| Requirement | Summary | HI criteria | Native requirement |
|---|---|---|---|
| FR-1 | <observable behavior> | <FAMILY-N> | <REQ-module-NNN, or "to be added by the native delta"> |

## Scope

- In: <behaviors this change delivers>
- Out: <explicit exclusions>

## Consequential assumptions

- <Assumption> — why it matters, and what changes if it is wrong.

## Dependencies

- <Module, tool, data or decision this relies on>

## Open questions for the human

- <Only questions whose answer changes the product or scope>
"##;

/// `skills/chough-workflow/templates/cases.toml` as shipped by main at 2756dc2.
const CASES_AT_2756DC2: &str = r##"# .chough/cases/<module>.cases.toml — test-case definitions only.
# Observations live in .chough/evidence/. Unknown fields are rejected.
profile = "chough-testing/1"
module = "<module>"

[[case]]
id = "TC-<module>-0001"            # stable; never reused, retire instead
title = "<what this case shows>"
status = "active"                  # active | retired
track = "functional"               # functional | copy-localization | design-qa
method = "automated"               # automated | manual | agent-assisted
criteria = ["<FAMILY-N>"]
requirements = ["REQ-<module>-001"]
paths = ["src/"]                   # optional: changed paths that select this case
environments = ["local", "ci-linux"]
artifacts = ["test-log"]           # test-log | report-json | screenshot | recording | transcript | reviewer-note
setup = ["<preconditions>"]
steps = ["<what runs>"]
expected = ["<explicit observable outcome>"]

[case.bind]
adapter = "cargo-test"             # cargo-test | fledge-task | command
test_target = "<integration test file stem>"
test = "<exact test name>"         # exactly one test must run for a pass

[[case]]
id = "TC-<module>-0002"
title = "<what a person checks>"
status = "active"
track = "design-qa"
method = "manual"                  # manual cases have no binding
criteria = ["<FAMILY-N>"]
requirements = ["REQ-<module>-001"]
environments = ["chromium-1280x800", "chromium-390x844"]
artifacts = ["screenshot", "reviewer-note"]
setup = ["Open design/prototype/index.html and the implementation side by side"]
steps = ["Compare the empty, error and success states at each viewport"]
expected = [
  "Layout and spacing match the prototype at each viewport",
  "Focus order and visible focus match the companion notes",
]
"##;

/// `skills/chough-workflow/references/testing.md` as shipped by main at 2756dc2.
const TESTING_AT_2756DC2: &str = r##"# Focused testing

Test what the change affects, not every criterion the product has.

## Choose the checks

Run `chough cases select --requirement <REQ> --criterion <ID> --path <changed path>`
for the affected scope. Add `--track copy-localization` when visible text,
terminology, comprehension or locale behavior changed, and `--track design-qa` when
visuals or interactions changed. The output lists selected cases, omitted cases with
reasons, and the project-required lane if one is configured
(`chough pref set required-lane <lane> --source …`). Also run proportionate
regressions for nearby behavior and the project's mandatory gates. Explain any
meaningful omission and record test debt in the native `testing.md`.

| Track | When | Typical method |
|---|---|---|
| functional | Always | Automated unit, integration or UI tests; manual checks where needed |
| copy-localization | Visible text or locale behavior changed | Deterministic string assertions plus reviewer notes |
| design-qa | Visuals or interactions changed | Rendered comparison with the canonical prototype at the target viewports and states |

## Definitions are not observations

Definitions live in `.chough/cases/<module>.cases.toml` (profile `chough-testing/1`,
see [the template](../templates/cases.toml)). Each case has a stable ID
`TC-<module>-NNNN`, links to HI criteria and requirements, setup, steps, explicit
expected outcomes, method, environments and expected artifact types. Several cases
can cover one criterion. Never edit an expected outcome to match a failure; retire a
case (`status = "retired"`) instead of deleting or reusing its ID.

`chough cases check` validates every definition, including its HI and requirement
links. The digest of a case ignores its position and title, so reordering never
changes identity, while editing what it asserts makes earlier observations
`stale-case-definition`.

## Execute honestly

- Automated: `chough cases run <ID> --environment <env>` runs the binding without a
  shell and records the observation with the captured log. `cargo-test` passes only
  when exactly one test ran and passed; a filter matching nothing is `not-run`.
  `fledge-task` and `command` pass only with exit 0 and every `expect_output`
  fragment present. The command exits 3 when the outcome is not passed.
- Manual or agent-assisted: `chough cases record <ID> --method manual --verdict 1=pass
  --verdict 2=fail … --summary "…" --artifact <screenshot>` gives a verdict for every
  expected outcome; any fail makes the observation failed.

A failing case stays failed even when every other case passes. Do not rerun until
it passes and report only the pass. Capture the failure with
[the bug procedure](bugs.md).

AI judgment is never the only oracle when an assertion or a reproducible procedure
can decide. Keep rendered, copy and human judgments as observations with method and
limits. Magpie can run real commands, but its generic mode judges exit codes and
records skipped steps as passes; bind a Magpie run only through `command` with an
`expect_output` assertion, and only where it is installed.

## Retention

Routine reruns are prunable; proof that something still references is not.
`chough evidence prune` previews eligible and protected observation sets with reasons
and bytes, and changes nothing. It protects what the checkpoint, a bug, an active
native change or the most recent archived one lists, non-routine records, the newest
3 automated runs and the newest failure per case and environment, validation failures
and boundary passes, and anything younger than 14 days. Apply only with `--apply` under
`chough consent grant retention --source "<their words>"`, or when a human set
`chough pref set retention auto --source …` (default `off`), which also prunes after
`cases run` and `validate run`. Removals are logged in `.chough/pruned.jsonl`. Never
delete evidence by hand, and never report a `pruned` or `unavailable` observation as
passed.
"##;

/// The tool pins and the minimum Rust version, as each file states them.
struct PinSources<'a> {
    readme: &'a str,
    agents: &'a str,
    ci: &'a str,
    cargo: &'a str,
    ci_doc: &'a str,
    integrations: &'a str,
}

fn version_token(word: &str) -> Option<String> {
    let word = word
        .trim_start_matches('*')
        .trim_end_matches(['.', ',', ';', ')', '`', '*']);
    let starts_with_digit = word.chars().next().is_some_and(|c| c.is_ascii_digit());
    (starts_with_digit && word.chars().all(|c| c.is_ascii_digit() || c == '.'))
        .then(|| word.to_string())
}

/// The `cargo install` lines in the code blocks of README's `Install the toolchain`
/// section.
fn readme_toolchain_commands(readme: &str) -> Vec<&str> {
    let mut in_block = false;
    readme
        .lines()
        .skip_while(|line| line.trim() != "## Install the toolchain")
        .skip(1)
        .take_while(|line| !line.starts_with("## "))
        .filter(|line| {
            if line.trim_start().starts_with("```") {
                in_block = !in_block;
                return false;
            }
            in_block
        })
        .map(str::trim)
        .filter(|line| line.starts_with("cargo install"))
        .collect()
}

/// The `<crate>@<version>` tokens of README's toolchain install command.
fn readme_tool_pins(readme: &str) -> Vec<(String, String)> {
    let Some(line) = readme_toolchain_commands(readme)
        .into_iter()
        .find(|line| line.starts_with("cargo install --locked") && line.contains('@'))
    else {
        return vec![];
    };
    line.split_whitespace()
        .filter_map(|token| token.split_once('@'))
        .map(|(name, version)| (name.to_string(), version.to_string()))
        .collect()
}

/// The `<tool> <version>` pairs of AGENTS.md's "Match tool versions to CI" sentence.
fn agents_tool_pins(agents: &str) -> Vec<(String, String)> {
    let text = agents.split_whitespace().collect::<Vec<_>>().join(" ");
    let Some((_, rest)) = text.split_once("Match tool versions to CI:") else {
        return vec![];
    };
    let words: Vec<&str> = rest.split_whitespace().take(12).collect();
    words
        .windows(2)
        .filter_map(|pair| {
            let name = pair[0].trim_end_matches(',');
            version_token(pair[1]).map(|version| (name.to_string(), version))
        })
        .collect()
}

/// The lines of the step that uses `action`, up to the next step.
fn ci_steps<'a>(ci: &'a str, action: &str) -> Vec<Vec<&'a str>> {
    let lines: Vec<&str> = ci.lines().collect();
    let mut steps = vec![];
    for (index, line) in lines.iter().enumerate() {
        if line.trim_start().starts_with(&format!("- uses: {action}@")) {
            let step = lines[index + 1..]
                .iter()
                .take_while(|next| !next.trim_start().starts_with("- "))
                .copied()
                .collect();
            steps.push(step);
        }
    }
    steps
}

fn ci_input(step: &[&str], input: &str) -> Option<String> {
    step.iter().find_map(|line| {
        line.trim()
            .strip_prefix(&format!("{input}:"))
            .map(|value| value.trim().trim_matches(['\'', '"']).to_string())
    })
}

/// The lines of the CI job `job`, up to the next job.
fn ci_job<'a>(ci: &'a str, job: &str) -> Vec<&'a str> {
    let header = format!("  {job}:");
    ci.lines()
        .skip_while(|line| *line != header)
        .skip(1)
        .take_while(|line| line.is_empty() || line.starts_with("   "))
        .collect()
}

/// Every distinct version written within two words after a mention of `tool`, so a
/// stale second mention shows up as `<one> and <other>`.
fn doc_versions(text: &str, tool: &str) -> Option<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut versions: Vec<String> = vec![];
    for (index, word) in words.iter().enumerate() {
        let word = word.trim_start_matches(['[', '`', '*', '(']);
        let Some(rest) = word.strip_prefix(tool) else {
            continue;
        };
        if rest.chars().next().is_some_and(|c| c.is_alphanumeric()) {
            continue;
        }
        for next in words.iter().skip(index + 1).take(2) {
            let next = next.trim_start_matches(['`', '*']);
            match version_token(next.strip_prefix('v').unwrap_or(next)) {
                Some(version) if !versions.contains(&version) => versions.push(version),
                _ => {}
            }
        }
    }
    (!versions.is_empty()).then(|| versions.join(" and "))
}

/// Shape problems that a version comparison alone would miss: README's toolchain
/// command must stay one command naming all three tools, and the MSRV job must need
/// the Archive job and run the full suite.
fn pin_shape_problems(sources: &PinSources) -> Vec<String> {
    let mut problems = vec![];
    let commands = readme_toolchain_commands(sources.readme);
    let one_command = match commands.as_slice() {
        [command] => {
            let crates: BTreeSet<&str> = command
                .split_whitespace()
                .filter_map(|token| token.split_once('@').map(|(name, _)| name))
                .collect();
            !command.contains("&&")
                && !command.contains(';')
                && !command.ends_with('\\')
                && crates == BTreeSet::from(["fledge", "human-intent", "specsync"])
        }
        _ => false,
    };
    if !one_command {
        problems.push(format!(
            "README.md: Install the toolchain is not one cargo install command naming \
             human-intent, fledge and specsync: {commands:?}"
        ));
    }
    let msrv = ci_job(sources.ci, "msrv");
    if !msrv.iter().any(|line| line.trim() == "needs: archive-gate") {
        problems.push(".github/workflows/ci.yml MSRV job: does not need archive-gate".into());
    }
    if !msrv
        .iter()
        .any(|line| line.trim() == "- run: cargo test --locked")
    {
        problems.push(".github/workflows/ci.yml MSRV job: does not run cargo test --locked".into());
    }
    problems
}

fn major_minor(version: &str) -> String {
    version.split('.').take(2).collect::<Vec<_>>().join(".")
}

/// Every missing or disagreeing pin, as `<file>: <tool> pinned <found>, expected
/// <expected>` or `<file>: <tool> not found`.
fn pin_disagreements(sources: &PinSources) -> Vec<String> {
    let readme = readme_tool_pins(sources.readme);
    let agents = agents_tool_pins(sources.agents);
    let find = |pins: &[(String, String)], name: &str| {
        pins.iter()
            .find(|(found, _)| found == name)
            .map(|(_, version)| version.clone())
    };
    let hi_in_ci = sources.ci.lines().find_map(|line| {
        line.trim()
            .strip_prefix("- run: cargo install human-intent --version ")
            .and_then(|rest| rest.strip_suffix(" --locked"))
            .map(str::to_string)
    });
    let fledge_in_ci = ci_steps(sources.ci, "CorvidLabs/fledge")
        .first()
        .and_then(|step| ci_input(step, "version"))
        .and_then(|version| version.strip_prefix('v').map(str::to_string));
    let specsync_steps = ci_steps(sources.ci, "CorvidLabs/spec-sync");
    let specsync_versions: BTreeSet<String> = specsync_steps
        .iter()
        .filter_map(|step| ci_input(step, "version"))
        .collect();
    let specsync_in_ci = match specsync_versions.len() {
        0 => None,
        1 => specsync_versions.into_iter().next(),
        _ => Some(
            specsync_versions
                .into_iter()
                .collect::<Vec<_>>()
                .join(" and "),
        ),
    };

    let tools = [
        (
            "HI (human-intent)",
            [
                ("README.md", find(&readme, "human-intent")),
                ("AGENTS.md", find(&agents, "hi")),
                (".github/workflows/ci.yml", hi_in_ci),
                ("docs/ci.md", doc_versions(sources.ci_doc, "HI")),
                (
                    "docs/integrations.md",
                    doc_versions(sources.integrations, "HI"),
                ),
            ],
        ),
        (
            "SpecSync",
            [
                ("README.md", find(&readme, "specsync")),
                ("AGENTS.md", find(&agents, "specsync")),
                (".github/workflows/ci.yml", specsync_in_ci),
                ("docs/ci.md", doc_versions(sources.ci_doc, "SpecSync")),
                (
                    "docs/integrations.md",
                    doc_versions(sources.integrations, "SpecSync"),
                ),
            ],
        ),
        (
            "Fledge",
            [
                ("README.md", find(&readme, "fledge")),
                ("AGENTS.md", find(&agents, "fledge")),
                (".github/workflows/ci.yml", fledge_in_ci),
                ("docs/ci.md", doc_versions(sources.ci_doc, "Fledge")),
                (
                    "docs/integrations.md",
                    doc_versions(sources.integrations, "Fledge"),
                ),
            ],
        ),
    ];

    let cargo_rust = sources.cargo.lines().find_map(|line| {
        line.trim()
            .strip_prefix("rust-version = \"")
            .and_then(|rest| rest.strip_suffix('"'))
            .map(str::to_string)
    });
    let readme_rust = {
        let text = sources
            .readme
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        text.split_once("Chough needs Rust ").and_then(|(_, rest)| {
            let (version, tail) = rest.split_once(' ')?;
            tail.starts_with("or later").then(|| version.to_string())
        })
    };
    let msrv_rust = ci_job(sources.ci, "msrv").iter().find_map(|line| {
        line.trim()
            .strip_prefix("toolchain:")
            .map(|value| major_minor(value.trim().trim_matches(['\'', '"'])))
    });

    let mut problems = pin_shape_problems(sources);
    let mut compare = |tool: &str, places: &[(&str, Option<String>)]| {
        let mut counts: Vec<(String, usize)> = vec![];
        for (_, found) in places {
            if let Some(found) = found {
                match counts.iter_mut().find(|(version, _)| version == found) {
                    Some((_, count)) => *count += 1,
                    None => counts.push((found.clone(), 1)),
                }
            }
        }
        let expected = counts
            .iter()
            .fold(None::<&(String, usize)>, |best, entry| match best {
                Some(best) if best.1 >= entry.1 => Some(best),
                _ => Some(entry),
            })
            .map(|(version, _)| version.clone());
        for (file, found) in places {
            match (found, &expected) {
                (None, _) => problems.push(format!("{file}: {tool} not found")),
                (Some(found), Some(expected)) if found != expected => problems.push(format!(
                    "{file}: {tool} pinned {found}, expected {expected}"
                )),
                _ => {}
            }
        }
    };
    for (tool, places) in &tools {
        compare(tool, places);
    }
    compare(
        "minimum Rust version",
        &[
            ("Cargo.toml", cargo_rust),
            ("README.md", readme_rust),
            (".github/workflows/ci.yml MSRV job", msrv_rust),
        ],
    );
    problems
}

#[test]
fn tool_and_rust_version_pins_agree() {
    let read = |file: &str| fs::read_to_string(manifest_dir().join(file)).unwrap();
    let (readme, agents, ci, cargo, ci_doc, integrations) = (
        read("README.md"),
        read("AGENTS.md"),
        read(".github/workflows/ci.yml"),
        read("Cargo.toml"),
        read("docs/ci.md"),
        read("docs/integrations.md"),
    );
    let sources = PinSources {
        readme: &readme,
        agents: &agents,
        ci: &ci,
        cargo: &cargo,
        ci_doc: &ci_doc,
        integrations: &integrations,
    };
    let problems = pin_disagreements(&sources);
    assert!(problems.is_empty(), "{}", problems.join("\n"));

    // The checker itself: changing any one pin fails, naming that file and tool.
    let cases = [
        (
            "README.md",
            "HI (human-intent)",
            "human-intent@0.8.0",
            "human-intent@9.9.9",
        ),
        ("README.md", "SpecSync", "specsync@6.1.2", "specsync@9.9.9"),
        ("README.md", "Fledge", "fledge@1.8.0", "fledge@9.9.9"),
        (
            "README.md",
            "minimum Rust version",
            "Chough needs Rust 1.88",
            "Chough needs Rust 9.9",
        ),
        ("AGENTS.md", "HI (human-intent)", "hi 0.8.0", "hi 9.9.9"),
        ("AGENTS.md", "SpecSync", "specsync 6.1.2", "specsync 9.9.9"),
        ("AGENTS.md", "Fledge", "fledge 1.8.0", "fledge 9.9.9"),
        (
            ".github/workflows/ci.yml",
            "HI (human-intent)",
            "--version 0.8.0",
            "--version 9.9.9",
        ),
        (
            ".github/workflows/ci.yml",
            "SpecSync",
            "version: '6.1.2'",
            "version: '9.9.9'",
        ),
        (
            ".github/workflows/ci.yml",
            "Fledge",
            "version: v1.8.0",
            "version: v9.9.9",
        ),
        (
            ".github/workflows/ci.yml MSRV job",
            "minimum Rust version",
            "toolchain: 1.88.0",
            "toolchain: 9.9.0",
        ),
        (
            "Cargo.toml",
            "minimum Rust version",
            "rust-version = \"1.88\"",
            "rust-version = \"9.9\"",
        ),
        (
            "docs/ci.md",
            "HI (human-intent)",
            "is 0.8.0, installed",
            "is 9.9.9, installed",
        ),
        (
            "docs/ci.md",
            "SpecSync",
            "SpecSync **6.1.2**",
            "SpecSync **9.9.9**",
        ),
        ("docs/ci.md", "Fledge", "Fledge is 1.8.0", "Fledge is 9.9.9"),
        (
            "docs/ci.md",
            "Fledge",
            "Fledge is 1.8.0",
            "Fledge is 1.8.0 (Fledge remains v1.7.2)",
        ),
        (
            "docs/integrations.md",
            "HI (human-intent)",
            "CorvidLabs/hi) 0.8.0",
            "CorvidLabs/hi) 9.9.9",
        ),
        (
            "docs/integrations.md",
            "SpecSync",
            "CorvidLabs/spec-sync) 6.1.2",
            "CorvidLabs/spec-sync) 9.9.9",
        ),
        (
            "docs/integrations.md",
            "Fledge",
            "CorvidLabs/fledge) 1.8.0",
            "CorvidLabs/fledge) 9.9.9",
        ),
    ];
    let changed_sources = |slot: usize, from: &str, to: &str| {
        let mut changed = [
            readme.clone(),
            agents.clone(),
            ci.clone(),
            cargo.clone(),
            ci_doc.clone(),
            integrations.clone(),
        ];
        assert!(changed[slot].contains(from), "slot {slot} lacks {from}");
        changed[slot] = changed[slot].replacen(from, to, 1);
        changed
    };
    let problems_in = |changed: &[String; 6]| {
        pin_disagreements(&PinSources {
            readme: &changed[0],
            agents: &changed[1],
            ci: &changed[2],
            cargo: &changed[3],
            ci_doc: &changed[4],
            integrations: &changed[5],
        })
    };
    for (file, tool, from, to) in cases {
        let slot = match file {
            "README.md" => 0,
            "AGENTS.md" => 1,
            "Cargo.toml" => 3,
            "docs/ci.md" => 4,
            "docs/integrations.md" => 5,
            _ => 2,
        };
        let problems = problems_in(&changed_sources(slot, from, to));
        assert!(
            problems
                .iter()
                .any(|problem| problem.starts_with(&format!("{file}: {tool} pinned "))),
            "changing {from} in {file}: {problems:?}"
        );
    }

    // A pin removed from a file is reported as not found there.
    let problems = problems_in(&changed_sources(
        1,
        "Match tool versions to CI",
        "Tool versions",
    ));
    for tool in ["HI (human-intent)", "SpecSync", "Fledge"] {
        assert!(
            problems.contains(&format!("AGENTS.md: {tool} not found")),
            "{problems:?}"
        );
    }

    // The shape around the pins: README's toolchain command stays one command naming
    // all three tools, and the MSRV job needs the Archive job and runs the suite.
    let toolchain = "cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.1.2";
    let shapes = [
        (
            0,
            toolchain,
            "cargo install --locked human-intent@0.8.0 fledge@1.8.0\ncargo install --locked specsync@6.1.2",
            "README.md: Install the toolchain is not one cargo install command",
        ),
        (
            0,
            toolchain,
            "cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.1.2 && hi --version",
            "README.md: Install the toolchain is not one cargo install command",
        ),
        (
            0,
            toolchain,
            "cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.1.2 cargo-nextest@0.9.0",
            "README.md: Install the toolchain is not one cargo install command",
        ),
        (
            2,
            "    needs: archive-gate\n    runs-on: ubuntu-latest\n    timeout-minutes: 20\n    steps:\n      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\n        with:\n          persist-credentials: false\n      - uses: dtolnay/rust-toolchain@b725eda525362e7cf34af9a7243cac1e3f01a7ec # stable\n        with:\n          toolchain: 1.88.0",
            "    runs-on: ubuntu-latest\n    timeout-minutes: 20\n    steps:\n      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\n        with:\n          persist-credentials: false\n      - uses: dtolnay/rust-toolchain@b725eda525362e7cf34af9a7243cac1e3f01a7ec # stable\n        with:\n          toolchain: 1.88.0",
            ".github/workflows/ci.yml MSRV job: does not need archive-gate",
        ),
        (
            2,
            "          toolchain: 1.88.0\n      - uses: Swatinem/rust-cache@f13886b937689c021905a6b90929199931d60db1 # v2.8.1\n      - run: cargo test --locked",
            "          toolchain: 1.88.0\n      - uses: Swatinem/rust-cache@f13886b937689c021905a6b90929199931d60db1 # v2.8.1\n      - run: cargo build --locked",
            ".github/workflows/ci.yml MSRV job: does not run cargo test --locked",
        ),
    ];
    for (slot, from, to, expected) in shapes {
        let problems = problems_in(&changed_sources(slot, from, to));
        assert!(
            problems.iter().any(|problem| problem.starts_with(expected)),
            "changing {from:?}: {problems:?}"
        );
    }
}

#[test]
fn a_skill_copy_from_main_before_the_release_prep_is_known_outdated_and_updated() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow");
    let earlier = [
        (
            "SKILL.md",
            SKILL_AT_5AF3F78,
            "d350cc91271e6d942bef516a2b2ea8a6009697a2e0d034b9ff48c8591be2ee5c",
        ),
        (
            "references/stages.md",
            STAGES_AT_5AF3F78,
            "5127182e148521e9c9df2490eb45055d0c5edb3302f245bc4f7aab114b591a24",
        ),
        (
            "references/validate.md",
            VALIDATE_AT_5AF3F78,
            "5b7ea4ec1a12c745e7d5a14625f4f98ab2731bf13b588986e4f0a6b7686b4aed",
        ),
        (
            "references/testing.md",
            TESTING_AT_5AF3F78,
            "a056931078182d3bd887cfa96c6bc4bd68a83b955e479bca95266bf1d1c1ee7c",
        ),
        (
            "references/checkpoint.md",
            CHECKPOINT_AT_5AF3F78,
            "20daaaa0cec972169abcedd508cd9ad18ff93ef7ec9d1f4e7330fcd18ca80ac0",
        ),
        (
            "references/lifecycle.md",
            LIFECYCLE_AT_5AF3F78,
            "a776c2f85d8803d78d24bd124773b73be1b09dd190c5cc9b6dd5a11fe4cc09c3",
        ),
        (
            "templates/cases.toml",
            CASES_AT_5AF3F78,
            "8a07e72f5f72668f6b440a14d457a8352850a94413ba7e1b5962abc5cdae234a",
        ),
    ];
    for (file, content, digest) in earlier {
        assert_eq!(
            format!("{:x}", Sha256::digest(content.as_bytes())),
            digest,
            "{file}"
        );
        fs::write(installed.join(file), content).unwrap();
    }

    let status = chough(root, &["skill", "status"]);
    for (file, _, _) in earlier {
        assert_eq!(
            skill_state(&status, file)["state"],
            "known-outdated",
            "{file}"
        );
    }
    chough(root, &["skill", "update"]);
    for (file, _, _) in earlier {
        assert_eq!(
            fs::read(installed.join(file)).unwrap(),
            fs::read(manifest_dir().join("skills/chough-workflow").join(file)).unwrap(),
            "{file}"
        );
    }
    let status = chough(root, &["skill", "status"]);
    for file in bundle_files() {
        assert_eq!(skill_state(&status, &file)["state"], "current", "{file}");
    }

    // An edited copy of a 5af3f78 file stays the user's.
    let edited = format!("{SKILL_AT_5AF3F78}\nOur own section.\n");
    fs::write(installed.join("SKILL.md"), &edited).unwrap();
    let status = chough(root, &["skill", "update"]);
    assert_eq!(skill_state(&status, "SKILL.md")["state"], "edited");
    assert_eq!(
        fs::read_to_string(installed.join("SKILL.md")).unwrap(),
        edited
    );
}

/// `skills/chough-workflow/SKILL.md` as shipped by main at 5af3f78 (PR #19).
const SKILL_AT_5AF3F78: &str = r##"---
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
"##;

/// `skills/chough-workflow/references/stages.md` as shipped by main at 5af3f78 (PR #19).
const STAGES_AT_5AF3F78: &str = r##"# Stage contract

Every stage uses the same shape so any agent can tell where work stands. The helper
embeds only the stage names, order and gates; this file holds the meaning. Use the
fields as a checklist for your own reasoning, not as a form to fill in front of the
human.

Each stage below states: purpose; inputs; outputs and their owners; useful roles
and tools; completion evidence; approval and preference policy; consequential
questions; what to checkpoint; and transitions.

## Transitions shared by every stage

| Intent | Command | Notes |
|---|---|---|
| Next stage | `chough stage advance` | Every move meets its destination's native prerequisite (build, test, accept, done) |
| Not applicable | `chough stage skip --reason "<why>"` | The reason is required and kept |
| Another pass | `chough stage iterate --note "<what changed>"` | |
| Go back | `chough stage back --to <stage> --reason "<why>"` | Lists what to refresh; history is kept |
| Start over | `chough stage reset [--to <stage>] --reason "<why>"` | History is kept; intent, design and plan are always reachable |
| Pause, resume | `chough stage pause --reason "<why>"`, `chough stage resume` | Resume re-checks native state and freshness |
| Side stage | `chough stage enter <side> --reason "<why>"`, then `chough stage return` | Sides: bugs, validate, librarian; the caller is restored |

Going back to intent or plan after native approval means native reapproval if
intent, contract, acceptance criteria or scope change. Follow `specsync change
status`; Chough never changes native state. Dependent design approval, test
observations and evidence named in the refresh list must be redone by their owners.

## intent

- Purpose: capture what the human wants and derive an appropriately sized FRD.
- Inputs: conversation, `INTENT.md`, `hi export <scope>`, existing FRD and module requirements.
- Outputs: HI criteria (HI), FRD sections (`docs/FRD.md`), review findings (working notes).
- Roles and tools: product and user advocate, QA, design, accessibility, engineering, architecture; `hi`.
- Completion evidence: criteria captured with `hi`, FRD traced to them, findings synthesized.
- Approval: the human approves the intent stage as a whole; no per-criterion ceremony.
- Questions: only those that change the product or a consequential assumption.
- Checkpoint: criteria and requirement references, decisions, open questions.
- Transitions: advance to design; back is not available; reset restarts here.

## design

- Purpose: define visuals and interactions in one canonical local HTML and CSS prototype.
- Applicability: skip with a reason when the change has no relevant visuals or interactions.
- Inputs: approved intent, FRD, project conventions and design tokens.
- Outputs: canonical prototype and `COMPANION.md` (design owner), rendered review notes.
- Roles and tools: design, accessibility, user advocate; a browser-capable agent for rendered review.
- Completion evidence: rendered review of states and viewports, companion mappings to HI and FRD.
- Approval: the human approves the rendered prototype and companion together.
- Questions: design choices that change scope go back to intent.
- Checkpoint: prototype and companion paths in `references.design`, pending design approval in `awaiting_human`.
- Transitions: advance to plan; back to intent.

## plan

- Purpose: technical design, task plan, estimates and ownership in the native change.
- Inputs: intent, design, contracts, architecture, native lessons and principles.
- Outputs: native change package and deltas (SpecSync), estimates and ownership (plan artifact).
- Roles and tools: engineering, architecture, QA; `specsync change new/answer`.
- Completion evidence: native artifacts complete and status ready for approval.
- Approval: one human scope approval over the concrete definition, recorded natively.
- Questions: consequential open choices, batched into the one approval question.
- Checkpoint: `native_change`, the approval request in `awaiting_human`.
- Transitions: advance to build only after native approval; back to intent or design.

## build

- Purpose: implement the approved tasks, keeping contracts, tests and docs in sync.
- Inputs: approved native definition, owned files, case definitions.
- Outputs: code, tests, case definitions, documentation (project owners).
- Roles and tools: engineering; Fledge tasks and lanes; subagents with explicit file ownership.
- Completion evidence: focused checks pass locally, tasks updated, checkpoint saved.
- Approval: none for routine work inside scope; scope changes go back to plan.
- Questions: genuine blockers and scope-changing decisions only.
- Checkpoint: completed work, files, blockers, next action after each increment.
- Transitions: advance to test; back to plan, design or intent.

## test

- Purpose: focused verification of the affected criteria and behavior.
- Inputs: selected cases (`chough cases select`), project-required lane.
- Outputs: observations (`.chough/evidence/`), artifacts, bugs for failures.
- Roles and tools: QA; `chough cases run/record`; browser for design QA.
- Completion evidence: selected cases observed; failures captured; omissions explained.
- Approval: none; blocking bugs send work back to build.
- Questions: whether a gap is a product decision rather than a defect.
- Checkpoint: case and observation references, open bugs, test debt.
- Transitions: advance to accept (refused while blocking bugs are open); back to build.

## accept

- Purpose: present the implementation and evidence for the human's review, then close natively.
- Inputs: verification, product evidence, remaining bugs and limits.
- Outputs: native review and archive (SpecSync), lessons (native module sources).
- Roles and tools: the human reviewer; `specsync change check/review/ship/finalize`.
- Completion evidence: native state `archived`.
- Approval: the human implementation review, recorded natively only after it happens.
- Questions: the review itself.
- Checkpoint: the pending review in `awaiting_human`, PR and CI references.
- Transitions: advance to done only once archived; back to build or test.

## done

- Purpose: the change is archived. Merge follows the repository's process.
- Transitions: release only with authorization; reset to start new work.

## release

- Purpose: optional publication, installation or deployment with real project commands.
- Entry: a human request recorded as run consent, or a standing project setting.
- Outputs: distinct publication, installation, deployment and live-health observations.
- Transitions: advance to done.

## bugs

- Side stage for reproducing, classifying, deduplicating and recording a defect locally.
- Returns to its caller; blocking bugs send the main flow back to build.

## validate

- Side stage for optional repeated validation or monitoring.
- Entry needs a run consent or standing preference; findings are kept locally.

## librarian

- Side stage for adding to or organizing the optional project library and preparing briefs.
- Returns to its caller; the librarian never becomes a second current source.
"##;

/// `skills/chough-workflow/references/validate.md` as shipped by main at 5af3f78 (PR #19).
const VALIDATE_AT_5AF3F78: &str = r##"# Optional repeated validation and monitoring

Ordinary functional checks and project-required gates always run during approved
work. Repetition beyond that, and monitoring, are optional.

## Who decides

| Situation | What to do |
|---|---|
| Standing preference `always` | Run within its stated limits |
| Preference `off` | Do not run unless the human asks for this run |
| Preference `ask` (default) and the choice matters | Ask one short contextual question |
| The human agrees for this run | `chough consent grant repeat-validation --source "<their words>"` |
| The human says "always for this project" | `chough pref set repeat-validation always --source "<their words>"` |

A run consent ends with the objective and never becomes a standing preference.
Record preferences only from an explicit human statement. `chough pref show` lists
effective values and where each came from.

## Run, stop, resume

`chough validate run <case> --times N [--interval S] --environment <env>` repeats an
automated case in the foreground and records every attempt. `chough validate stop`
asks it to halt before the next attempt; `chough validate resume <run>` continues
with the same stored limits after re-checking authorization. `chough validate
status` lists runs.

Each failure is kept locally: it opens or extends a bug for that case, which stays in
the backlog after a restart. Results are labelled as repeated runs with attempts,
failures and elapsed time. Retrying until one success is not soak testing, a file
watcher is not product health, and neither is authority to repair anything.

## Monitoring sources

Monitor with the project's own health, log or error sources, wrapped as cases with a
`command` binding and an `expect_output` assertion. No general production-health
service is assumed. Any real or live job needs explicit authorization.

Whether a monitor may open an investigation is unsettled. The default,
`monitor-initiation capture-only`, only records local findings. With
`propose-change`, Chough prints a suggested `specsync change new` command; it never
runs it. Any repair follows the normal scope and gates.
"##;

/// `skills/chough-workflow/references/testing.md` as shipped by main at 5af3f78 (PR #19).
const TESTING_AT_5AF3F78: &str = r##"# Focused testing

Test what the change affects, not every criterion the product has.

## Choose the checks

Run `chough cases select --requirement <REQ> --criterion <ID> --path <changed path>`
for the affected scope. Add `--track copy-localization` when visible text,
terminology, comprehension or locale behavior changed, and `--track design-qa` when
visuals or interactions changed. The output lists selected cases, omitted cases with
reasons, and the project-required lane if one is configured
(`chough pref set required-lane <lane> --source …`). Also run proportionate
regressions for nearby behavior and the project's mandatory gates. Explain any
meaningful omission and record test debt in the native `testing.md`.

| Track | When | Typical method |
|---|---|---|
| functional | Always | Automated unit, integration or UI tests; manual checks where needed |
| copy-localization | Visible text or locale behavior changed | Deterministic string assertions plus reviewer notes |
| design-qa | Visuals or interactions changed | Rendered comparison with the canonical prototype at the target viewports and states |

## Definitions are not observations

Definitions live in `.chough/cases/<module>.cases.toml` (profile `chough-testing/1`,
see [the template](../templates/cases.toml)). Each case has a stable ID
`TC-<module>-NNNN`, links to HI criteria and requirements, setup, steps, explicit
expected outcomes, method, environments and expected artifact types. Several cases
can cover one criterion. Never edit an expected outcome to match a failure; retire a
case (`status = "retired"`) instead of deleting or reusing its ID.

`chough cases check` validates every definition, including its HI and requirement
links. The digest of a case ignores its position and title, so reordering never
changes identity, while editing what it asserts makes earlier observations
`stale-case-definition`.

## Execute honestly

- Automated: `chough cases run <ID> --environment <env>` runs the binding without a
  shell and records the observation with the captured log. `cargo-test` passes only
  when exactly one test ran and passed; a filter matching nothing is `not-run`.
  `fledge-task` and `command` pass only with exit 0 and every `expect_output`
  fragment present. The command exits 3 when the outcome is not passed.
- A binding may declare `env`, variables set on top of the inherited environment,
  for example `env = { NO_COLOR = "1" }` when a tool should print plain text. Chough
  sets none by default. `env` is committed with the case, so never put secrets in
  it. A `cargo-test` binding cannot set, in any ASCII case, names that choose what
  cargo builds or runs: `PATH`, `CARGO`, `CARGO_HOME`, `RUSTC`, `RUSTDOC`,
  `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`, `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`,
  `RUSTDOCFLAGS`, `CARGO_ENCODED_RUSTDOCFLAGS`, `RUSTC_BOOTSTRAP`, or any name
  starting with `RUSTUP_`, `CARGO_BUILD_`, `CARGO_TARGET_` (`CARGO_TARGET_DIR` stays
  allowed), `CARGO_HOST_`, `CARGO_PROFILE_` or `CARGO_UNSTABLE_`. Observations name
  the declared variables, not their values, and always state that inherited
  variables were not recorded.
- `expect_output` and the `cargo-test` count read the output with complete terminal
  escape sequences (colours, hyperlinks) removed, while the artifact keeps the raw
  bytes. Write fragments as the visible text; a fragment containing ESC is refused.
- Manual or agent-assisted: `chough cases record <ID> --method manual --verdict 1=pass
  --verdict 2=fail … --summary "…" --artifact <screenshot>` gives a verdict for every
  expected outcome; any fail makes the observation failed.

A failing case stays failed even when every other case passes. Do not rerun until
it passes and report only the pass. Capture the failure with
[the bug procedure](bugs.md).

AI judgment is never the only oracle when an assertion or a reproducible procedure
can decide. Keep rendered, copy and human judgments as observations with method and
limits. Magpie can run real commands, but its generic mode judges exit codes and
records skipped steps as passes; bind a Magpie run only through `command` with an
`expect_output` assertion, and only where it is installed.

## Artifacts

Artifacts stay inside the project; there are no external artifact paths. Keep
screenshots and other large files small. One may be ignored by Git: the observation
keeps its fingerprint, and a clone without the file reads `stale-artifact`, never
passed. Do not use Git LFS for them while SpecSync `change check` refuses tracked LFS
files (CorvidLabs/spec-sync#789).

## Retention

Routine reruns are prunable; proof that something still references is not.
`chough evidence prune` previews eligible and protected observation sets with reasons
and bytes, and changes nothing. It protects what the checkpoint, a bug, an active
native change or the most recent archived one lists, non-routine records, the newest
3 automated runs and the newest failure per case and environment, validation failures
and boundary passes, and anything younger than 14 days. Apply only with `--apply` under
`chough consent grant retention --source "<their words>"`, or when a human set
`chough pref set retention auto --source …` (default `off`), which also prunes after
`cases run` and `validate run`. Removals are logged in `.chough/pruned.jsonl`. Never
delete evidence by hand, and never report a `pruned` or `unavailable` observation as
passed.
"##;

/// `skills/chough-workflow/references/checkpoint.md` as shipped by main at 5af3f78 (PR #19).
const CHECKPOINT_AT_5AF3F78: &str = r##"# Checkpoint and cold resume

Essential state must never live only in model context. `.chough/session.json` is the
single writable checkpoint. It works without the librarian and without any other
storage adapter.

## What it holds

| Field | Content |
|---|---|
| `objective`, `stage` | Current outcome, stage, status, side-stage callers and iteration |
| `native_change` | The SpecSync change ID; native status decides its state |
| `references` | Criteria, requirements, cases, design sources and files in scope |
| `decisions`, `open_questions` | Decisions with their reason in the sentence; questions that matter |
| `completed`, `observations`, `bugs`, `blockers` | Work done and the records that show it |
| `awaiting_human` | Real gates still waiting for a person |
| `run_consents` | Consent for this objective only |
| `next_action` | Suggested next step; never overrides native state |

Standing preferences are in `.chough/preferences.json`, transitions in
`.chough/transitions.jsonl`, observations in `.chough/evidence/`, bugs in
`.chough/bugs/`, case definitions in `.chough/cases/` and retention removals in
`.chough/pruned.jsonl`.

## When to save

Save after each meaningful increment, before handing off, before context is cleared,
and whenever a human gate opens or closes:

```sh
chough checkpoint --completed "<what was done>" --observation <id> \
  --decision "<decision, because reason>" --next-action "<next step>"
```

Opening a gate with `--awaiting-human "<gate>"` appends one `gate-opened` record to
`.chough/transitions.jsonl` (only when that gate is not already open), and
`--clear-awaiting-human` appends one `gate-closed` record per gate it clears. They
record when a gate waited, never who decided or what, and `chough timing` reads them
to show waiting-on-human time. Open and clear gates through these flags rather than by
editing `session.json`, or the waiting time is lost.

Write decision summaries and observations, not private reasoning transcripts.
SpecSync's `Handoff:` line says whether its own records are safe to clear; follow it
too.

## Resume cold

A fresh agent with none of the earlier conversation:

1. Reads `AGENTS.md`, then this skill.
2. Runs `chough next --environment <env>` (or reads the files above directly).
3. Checks native status for `native_change`, freshness of the referenced observations,
   open blocking bugs and `awaiting_human`. A referenced observation is `current`,
   `historical` (present but stale), `pruned` (removed by retention, in the ledger) or
   `unavailable` (missing, not in the ledger); only a current one counts, so rerun the
   others rather than treat them as passed.
4. Continues with `next_valid_action` under the same scope, preferences and gates. It
   never records a pending human decision itself.

## Schema and concurrency

A schema 1 session keeps working for `context` and `status`. `chough migrate`
converts it after saving the original bytes to `.chough/session.v1.json`. Hand edits
are allowed and validated on every read. With several agents, one agent owns the
checkpoint; others report to it. APS was evaluated as a backend and deferred: it lost
concurrent read-modify-write updates without error and accepts unknown fields.
"##;

/// `skills/chough-workflow/references/lifecycle.md` as shipped by main at 5af3f78 (PR #19).
const LIFECYCLE_AT_5AF3F78: &str = r##"# Native lifecycle and evidence

Installed `specsync change status <id> --json`, `specsync change ship-status <id>` and
command help are authoritative for the current state and recovery path. This
reference follows SpecSync 6.1; do not hardcode it over another version's output.
Match the SpecSync version used by lifecycle writers and CI.

## Shape of a change

```text
new → interview → artifacts and delta → one human scope approval → implement →
check (when requirements change) → cases run → gate → commit with .chough/ →
check --commit → push, CI → human implementation review → review → ship/finalize →
clear gate, stage advance → push archive, CI → merge
```

Between `check --commit` and finalize, nothing writes to `.chough/` and nothing is
committed: SpecSync's verification digest covers `.chough/`, and until SpecSync offers
an opt-in verification exclusion (the proposed upstream `verification_ignored_paths`)
any Chough write in that window forces another `change check`. See
[accept.md](accept.md) for the full order.

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

`specsync change check <id>` materializes the approved delta; run it before cases
that link requirements the delta adds or modifies. `specsync change check <id>
--commit` also records targeted structural verification; run it after committing the
source with `.chough/`. Stage new source files first. It does not run the project's
tests; product checks are separate observations. It refuses an edited delta and
incomplete tasks. Fix real failures; if evidence is stale, follow the exact recovery
action status names.

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

`chough cases run` and `chough evidence add` record with an observation the Git
revision, a working content identity, the environment, artifacts and, for cases, the
definition digest. Freshness compares the content identity, not the revision: it
covers every tracked or untracked non-ignored file except `.chough/`, `target/` and
SpecSync lifecycle bookkeeping (a change's `state.json`, `approvals.json`,
`verification.json`, `verification-attempts.json`, `review.json`,
`review-attempts.json`, `finalization.json`, `accepted-state.json`,
`lesson-bundle.md` and the front-matter `state:` value of `change.md`). So committing
only Chough records, an empty commit, `change check --commit` and finalize keep an
observation current, while a change to source, specs, configuration or a change's
deltas and other artifacts makes it stale. A record without a content identity, from
an earlier Chough, keeps the revision-bound comparison and is stale after any commit.
The records do not authenticate a witness or verify a remote artifact. A changed
input, artifact, case definition or environment makes the observation stale.
"##;

/// `skills/chough-workflow/templates/cases.toml` as shipped by main at 5af3f78 (PR #19).
const CASES_AT_5AF3F78: &str = r##"# .chough/cases/<module>.cases.toml: test-case definitions only.
# Observations live in .chough/evidence/. Unknown fields are rejected.
profile = "chough-testing/1"
module = "<module>"

[[case]]
id = "TC-<module>-0001"            # stable; never reused, retire instead
title = "<what this case shows>"
status = "active"                  # active | retired
track = "functional"               # functional | copy-localization | design-qa
method = "automated"               # automated | manual | agent-assisted
criteria = ["<FAMILY-N>"]
requirements = ["REQ-<module>-001"]
paths = ["src/"]                   # optional: changed paths that select this case
environments = ["local", "ci-linux"]
artifacts = ["test-log"]           # test-log | report-json | screenshot | recording | transcript | reviewer-note
setup = ["<preconditions>"]
steps = ["<what runs>"]
expected = ["<explicit observable outcome>"]

[case.bind]
adapter = "cargo-test"             # cargo-test | fledge-task | command
test_target = "<integration test file stem>"
test = "<exact test name>"         # exactly one test must run for a pass

[[case]]
id = "TC-<module>-0002"
title = "<what a person checks>"
status = "active"
track = "design-qa"
method = "manual"                  # manual cases have no binding
criteria = ["<FAMILY-N>"]
requirements = ["REQ-<module>-001"]
environments = ["chromium-1280x800", "chromium-390x844"]
artifacts = ["screenshot", "reviewer-note"]
setup = ["Open design/prototype/index.html and the implementation side by side"]
steps = ["Compare the empty, error and success states at each viewport"]
expected = [
  "Layout and spacing match the prototype at each viewport",
  "Focus order and visible focus match the companion notes",
]
"##;
