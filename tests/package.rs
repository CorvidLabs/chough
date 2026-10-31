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
