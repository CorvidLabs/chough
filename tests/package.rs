//! Installable package and fresh installs (REQ-cli-019, LIFE-9, LIFE-4).

mod common;

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
    // init --objective creates the first session folder (TC-cli-0015).
    let session_rel = created
        .iter()
        .find(|path| {
            path.starts_with(".chough/sessions/") && path.ends_with("-try-chough/session.md")
        })
        .unwrap_or_else(|| panic!("no session record in {created:?}"))
        .clone();
    let expected: BTreeSet<String> = bundle_files()
        .into_iter()
        .map(|file| format!(".agents/skills/chough-workflow/{file}"))
        .chain(std::iter::once(session_rel.clone()))
        // REQ-records-002: the records tree's reading guide, create-only.
        .chain(std::iter::once(".chough/README.md".to_string()))
        .collect();
    assert_eq!(created, expected);

    let session = fs::read_to_string(root.join(&session_rel)).unwrap();
    for empty in [
        "bugs",
        "observations",
        "blockers",
        "waiting_on_human",
        "run_consents",
    ] {
        assert!(
            session.contains(&format!("{empty}: []")),
            "{empty} is not empty: {session}"
        );
    }
    assert!(session.contains("native_change: \"\""), "{session}");
    let mut records: Vec<String> = fs::read_dir(root.join(".chough"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    records.sort();
    // REQ-records-002: the reading guide, the cases folder, local state and the session.
    assert_eq!(records, ["README.md", "cases", "local", "sessions"]);
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

#[test]
fn slug_identity_bundle_predecessors_update_and_edited_copies_stay() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow");
    let earlier = [
        (
            "templates/cases.toml",
            CASES_BEFORE_SLUG_IDENTITIES,
            "5f839a81742b9dc3df3b1278e340061a08e95e2a00e203f0f2554eab3aa3fed9",
        ),
        (
            "references/testing.md",
            TESTING_BEFORE_SLUG_IDENTITIES,
            "8ca2b70f36ee162638cafd63bcc97e8c9ed688b59c2b67b98d61ba3be1d28f37",
        ),
        (
            "references/validate.md",
            VALIDATE_BEFORE_SLUG_IDENTITIES,
            "51281ff6c9af4b939105cdf6ecf80d9dd11080dc8d682ecbba9d5b5c17d75c42",
        ),
        (
            "references/librarian.md",
            LIBRARIAN_BEFORE_SLUG_IDENTITIES,
            "df33dfa9aa3db1355b47ef47a653e75ff4ae42a8c1de096bcd1718f6a4287c17",
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

    for (file, content, _) in earlier {
        let edited = format!("{content}\nA local identity note.\n");
        fs::write(installed.join(file), &edited).unwrap();
        let status = chough(root, &["skill", "update"]);
        assert_eq!(skill_state(&status, file)["state"], "edited", "{file}");
        assert_eq!(
            skill_state(&status, file)["action"],
            "preserved-edit",
            "{file}"
        );
        assert_eq!(
            fs::read_to_string(installed.join(file)).unwrap(),
            edited,
            "{file}"
        );
    }
}

const CASES_BEFORE_SLUG_IDENTITIES: &str = r##"# .chough/cases/<module>.cases.toml: test-case definitions only.
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
# timeout_seconds = 600            # optional, 1 to 3600 (default 600); a timeout is recorded failed
# env = { NO_COLOR = "1" }         # optional variables on top of the inherited environment; never secrets
# A fledge-task binding instead: adapter = "fledge-task", task = "<fledge task>",
#   expect_output = ["<visible text>"] (at least one fragment).
# A command binding instead: adapter = "command", argv = ["<program>", "<arg>"],
#   expect_output = ["<visible text>"] (at least one fragment; run without a shell).

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

const TESTING_BEFORE_SLUG_IDENTITIES: &str = r##"# Focused testing

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
  fragment present, so each needs at least one fragment. `timeout_seconds` bounds a
  run from 1 to 3600 (default 600); a run that times out is recorded failed. The
  command exits 3 when the outcome is not passed.
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
  expected outcome; any fail makes the observation failed. Like `cases run`, it
  exits 3 when the outcome is not passed.

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

const VALIDATE_BEFORE_SLUG_IDENTITIES: &str = r##"# Optional repeated validation and monitoring

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
automated case in the foreground and records every attempt. It needs a
`repeat-validation` run consent or the standing preference `always`; a `monitoring`
consent or preference only allows entering the validate stage, not `validate run`.

`chough validate stop` leaves a stop request in `.chough/validation.stop`. The running
repetition halts before its next attempt, and any later run halts at once, until
`chough validate resume <run>` clears the request and continues that run with the
same stored limits after re-checking authorization. Pausing the stage
(`chough stage pause`) pauses a run the same way. `chough validate status` lists
runs.

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

const LIBRARIAN_BEFORE_SLUG_IDENTITIES: &str = r##"# Optional project librarian

The librarian is a project-local knowledge folder, off by default. Turn it on only
when the human wants it: `chough pref set librarian on --source "<their words>"`.
The folder name defaults to `library` and can be changed with `chough pref set
library-dir <name> --source …`. It is not a shared cross-project vault.

## From any stage

Enter it as a side stage and return to the caller:

```sh
chough stage enter librarian --reason "Snapshot the approved prototype before redesign"
chough library snapshot design/prototype/index.html design/prototype/COMPANION.md --reason "Approved version 1"
chough library index
chough stage return --note "Snapshot 2026-10-31-1 taken"
```

- `library init` creates `README.md`, `INDEX.md` and `briefs/`.
- `library snapshot` copies originals into `snapshots/<date>-<n>/` with a manifest of
  source path, Git revision and SHA-256. Sources are never moved or edited.
- `library index` regenerates `INDEX.md`: living sources and their owners, active
  and archived native changes, the open bug backlog, recent observations, briefs and
  snapshots.
- Briefs in `briefs/` are focused next-session summaries written from
  [the brief template](../templates/brief.md), linking to originals.

## Boundaries

Living HI, prototypes, contracts and native archives stay in their owning locations;
a snapshot is history, not a second current copy. Summaries complement originals.
Unfixed bugs remain in the live backlog even when an old report is snapshotted. The
mandatory checkpoint does not depend on the librarian.
"##;

#[test]
fn the_stages_reference_from_main_01f1d023_is_known_outdated_and_names_advance_to() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow/references/stages.md");
    // Shipped unchanged from 0.1.0 (#22) to main 01f1d023, before `stage advance --to`
    // had a row in the transition table (REQ-cli-052, DRIVE-9).
    assert_eq!(
        format!("{:x}", Sha256::digest(STAGES_AT_01F1D023.as_bytes())),
        "658ce798826c4d685753af0735e5fc6a53685f1205b9b9902d625c63c71d277f"
    );
    fs::write(&installed, STAGES_AT_01F1D023).unwrap();
    let status = chough(root, &["skill", "status"]);
    assert_eq!(
        skill_state(&status, "references/stages.md")["state"],
        "known-outdated"
    );
    chough(root, &["skill", "update"]);
    let status = chough(root, &["skill", "status"]);
    assert_eq!(
        skill_state(&status, "references/stages.md")["state"],
        "current"
    );
    let updated = fs::read_to_string(&installed).unwrap();
    let row = updated
        .lines()
        .find(|line| line.contains("`chough stage advance --to <stage>`"))
        .expect("the transition table names stage advance --to");
    assert!(row.starts_with("| Several stages at once |"), "{row}");
    assert!(row.contains("one native read"), "{row}");
    assert!(row.contains("records nothing"), "{row}");
}

/// `skills/chough-workflow/references/stages.md` as shipped by main at 01f1d023.
const STAGES_AT_01F1D023: &str = r###"# Stage contract

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
| Pause, resume | `chough stage pause --reason "<why>"`, `chough stage resume` | Resume only marks the stage active and checks nothing itself; then run `chough next --environment <env>` to re-check native state and freshness |
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
"###;

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

#[test]
fn a_skill_copy_from_main_before_the_learning_handoff_is_known_outdated_and_updated() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow");
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/skill-fb6e487");
    let files = [
        (
            "references/accept.md",
            "4e085b88eaa66a59fd6c8ce7d3acb1201b46292603a2cf13ab5333fc5ea85d86",
        ),
        (
            "references/bugs.md",
            "df42d472ae25b0ed07ad3beb1257ce77c3fe4e159feefe8e70e7c18ec03a6945",
        ),
        (
            "references/checkpoint.md",
            "28eccd903a89ff815e71e0cc3f81250e6c63bb986d93b0ee76b323180c4d92bf",
        ),
        (
            "references/librarian.md",
            "df33dfa9aa3db1355b47ef47a653e75ff4ae42a8c1de096bcd1718f6a4287c17",
        ),
        (
            "references/lifecycle.md",
            "e232af339e12254fcc735ffa87b72c36d0e80a8b3efc585cb7be627973d42235",
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

    // A copy edited from the fb6e487 version is edited, and update keeps it.
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
fn a_checkpoint_reference_from_main_with_merge_guidance_is_known_outdated_and_updated() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow/references/checkpoint.md");
    // The known digest is of LF bytes; a Windows checkout may have converted them.
    let old = CHECKPOINT_AT_66BEF80.replace("\r\n", "\n");
    assert_eq!(
        format!("{:x}", Sha256::digest(old.as_bytes())),
        "70ec5dee386c4ef178abe76a45236072bea7342aa265192fd0210536fe2639e8"
    );
    fs::write(&installed, old).unwrap();
    let status = chough(root, &["skill", "status"]);
    assert_eq!(
        skill_state(&status, "references/checkpoint.md")["state"],
        "known-outdated"
    );
    chough(root, &["skill", "update"]);
    let status = chough(root, &["skill", "status"]);
    assert_eq!(
        skill_state(&status, "references/checkpoint.md")["state"],
        "current"
    );
}

#[test]
fn checkpoint_reference_before_merge_guidance_is_known_outdated_and_edited_copies_stay() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow/references/checkpoint.md");
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(CHECKPOINT_BEFORE_MERGE_GUIDANCE.as_bytes())
        ),
        "28eccd903a89ff815e71e0cc3f81250e6c63bb986d93b0ee76b323180c4d92bf"
    );
    fs::write(&installed, CHECKPOINT_BEFORE_MERGE_GUIDANCE).unwrap();

    let status = chough(root, &["skill", "status"]);
    assert_eq!(
        skill_state(&status, "references/checkpoint.md")["state"],
        "known-outdated"
    );
    chough(root, &["skill", "update"]);
    assert_eq!(
        fs::read(&installed).unwrap(),
        fs::read(manifest_dir().join("skills/chough-workflow/references/checkpoint.md")).unwrap()
    );
    let status = chough(root, &["skill", "status"]);
    assert_eq!(
        skill_state(&status, "references/checkpoint.md")["state"],
        "current"
    );

    let edited = format!("{CHECKPOINT_BEFORE_MERGE_GUIDANCE}\nA local checkpoint note.\n");
    fs::write(&installed, &edited).unwrap();
    let preserved = chough(root, &["skill", "update"]);
    assert_eq!(
        skill_state(&preserved, "references/checkpoint.md")["state"],
        "edited"
    );
    assert_eq!(
        skill_state(&preserved, "references/checkpoint.md")["action"],
        "preserved-edit"
    );
    assert_eq!(fs::read_to_string(installed).unwrap(), edited);
}

#[test]
fn plan_testing_and_accept_references_from_main_825a380_are_known_outdated_and_edited_copies_stay()
{
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow");
    let files = [
        (
            "references/plan.md",
            PLAN_AT_825A380,
            "1022db57d0c114effc8669ede9ebb453f932109f608f883712725e57ba53551d",
        ),
        (
            "references/testing.md",
            TESTING_AT_825A380,
            "426c2df68e8d25bfbdee4bf81b4deaa06eac4525b6dfad3c771311e0d6a4ac99",
        ),
        (
            "references/accept.md",
            ACCEPT_AT_825A380,
            "80f71366ad81e1720bdc5f91d51c5428cd6a60729c9d1675977e284f7b08e398",
        ),
    ];
    for (file, content, digest) in files {
        // The known digests are of LF bytes; a Windows checkout may have converted them.
        let old = content.replace("\r\n", "\n");
        assert_eq!(
            format!("{:x}", Sha256::digest(old.as_bytes())),
            digest,
            "{file}"
        );
        fs::write(installed.join(file), old).unwrap();
    }
    let status = chough(root, &["skill", "status"]);
    for (file, _, _) in files {
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

    // A copy edited from the 825a380 version is edited, and update keeps it.
    for (file, content, _) in files {
        let edited = format!("{}\nA local note.\n", content.replace("\r\n", "\n"));
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
fn the_skill_links_new_requirements_from_the_approved_delta_and_failing_first_evidence() {
    let reference = |file: &str| {
        fs::read_to_string(
            manifest_dir()
                .join("skills/chough-workflow/references")
                .join(file),
        )
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
    };
    for (file, phrases) in [
        (
            "plan.md",
            &[
                "and running or recording Chough cases, as plain text rather than checkboxes",
                "reaches the companion only when `change check` materializes it",
                "Chough cases may link it only while the checkpoint records the approved change and its delta is unchanged",
            ][..],
        ),
        (
            "testing.md",
            &[
                "is not in the companion until the scoped `specsync change check <id>` materializes it",
                "accept a case linking it when the checkpoint records that change (`chough checkpoint --native-change <id>`), the change passes the build gate's native checks and its delta for the case file's module adds or modifies the ID",
                "lists such links under `approved_delta_requirements`. Anything else is refused with the reason.",
                "Running or recording cases is never a `tasks.md` checkbox",
                "## Failing-first evidence in the test stage",
                "The commit that adds the failing tests, before the fix.",
                "records a failed observation, and `chough bug add ... --observation <id>`",
                "The passing run after the fix",
            ][..],
        ),
        (
            "accept.md",
            &["so the cases in step 2 are never a box there"][..],
        ),
    ] {
        let text = reference(file);
        for phrase in phrases {
            assert!(text.contains(phrase), "{file} lacks: {phrase}");
        }
    }
}

/// Frozen plan reference from main 825a380. Keep its bytes.
const PLAN_AT_825A380: &str = r##"# Technical design and plan

SpecSync drives this stage with its own adaptive artifacts. Read source
instructions, existing architecture and the native lessons first; `specsync change
new` prints the lessons of the modules you declare.

1. Create the change with installed help as the authority, for example
   `specsync change new "<one outcome>" --spec <module> --path <path> --json`.
   Declare every path you expect to touch: scope freezes at approval.
2. Answer the deterministic interview with `specsync change answer <id> <question>
   "<answer>"`.
3. Complete the selected artifacts: requirements, research, design, plan (approach,
   out of scope, steps, risks, constraints consulted), tasks, testing, docs, and a
   semantic delta per affected module.
4. Estimate meaningful increments with relative size and the main uncertainty. Do not
   invent precise hours. Run `chough timing` and use its history ranges (per stage, bug
   fix and human gate) as the basis: they are elapsed wall-clock time, not effort, and
   estimates from recorded history, not targets. When it reports too few samples, say
   that history is too thin and estimate by relative size alone; never present a range
   as a deadline.
5. If several agents will build, assign file or module ownership in the plan.
6. Record the change: `chough checkpoint --native-change <id>`.

## One approval

Present one coherent package: what will change, the contract delta, the plan, the
verification approach, and any consequential open choice with your recommendation.
Ask one concise question. Record `chough checkpoint --awaiting-human "Scope approval
of <id>"`, then keep doing independent preparation.

Only after the human actually approves, run `specsync change approve <id> --actor
"<their name>"` and clear the gate with `chough checkpoint --clear-awaiting-human`.
The actor is a label, not authenticated identity.

Lessons learned from rehearsing SpecSync 6.1: the delta is frozen once approved and
`change check` refuses an edited delta; `change check` also refuses while `tasks.md`
has unchecked boxes, so list human gates as plain text rather than checkboxes.

## What needs reapproval

A change to approved intent, contract, acceptance criteria or affected scope needs
native reapproval. Routine implementation, tests, evidence, delta materialization
and lifecycle metadata do not.
"##;

/// Frozen testing reference from main 825a380. Keep its bytes.
const TESTING_AT_825A380: &str = r##"# Focused testing

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
see [the template](../templates/cases.toml)). Each new case has a curated stable ID
`TC-<module>-<title-slug>`, with lowercase ASCII letters and digits in nonempty runs
separated by single hyphens. The complete ID stays within the module-aware 100-byte
bound, and duplicate IDs are refused globally. Exact legacy `TC-<module>-NNNN` cases
remain valid without renumbering or changing their definition digests. Each case
links to HI criteria and requirements, setup, steps, explicit expected outcomes,
method, environments and expected artifact types. Several cases can cover one
criterion. Never edit an expected outcome to match a failure; retire a case
(`status = "retired"`) instead of deleting or reusing its ID.

`chough cases check` validates every definition, including its HI and requirement
links. The digest of a case ignores its position and title, so reordering never
changes identity, while editing what it asserts makes earlier observations
`stale-case-definition`.

## Execute honestly

- Automated: `chough cases run <ID> --environment <env>` runs the binding without a
  shell and records an automatic `OBS-<slug>-<24-lowercase-hex>` observation with
  the captured log at exactly `.chough/artifacts/<observation-id>.log`. An optional
  `--observation-id` accepts only the new OBS form. An old-style ID supplied for a
  new write is refused with a remedy to omit the ID for automatic allocation.
  `cargo-test` passes only when exactly one test ran and passed; a filter matching
  nothing is `not-run`. A validator refusing to load or run a definition is not a
  behavioral `not-run` observation: run the bound regression directly when failure
  first evidence is needed before the validator can accept its new case ID.
  `fledge-task` and `command` pass only with exit 0 and every `expect_output`
  fragment present, so each needs at least one fragment. `timeout_seconds` bounds a
  run from 1 to 3600 (default 600); a run that times out is recorded failed. The
  command exits 3 when the outcome is not passed.
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
  expected outcome; any fail makes the observation failed. Like `cases run`, it
  allocates an OBS ID with a random 96-bit suffix when `--observation-id` is omitted
  and exits 3 when the outcome is not passed. Direct `chough evidence add [ID]` also
  allocates when ID is omitted and applies the same explicit new-form and old-style
  omit-ID rules. Stored legacy observations remain readable and linkable without
  migration.

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

/// Frozen accept reference from main 825a380. Keep its bytes.
const ACCEPT_AT_825A380: &str = r##"# Acceptance and native closeout

Acceptance is where the human decides whether the implementation is what they
wanted. Agent review, a passing command, scope approval or an actor label never
stands in for that decision.

## Prepare the review

Follow this order, so that one verification covers both the code and Chough's
records. Follow native status where it names a different next action.

1. When the delta adds or modifies requirements, run `specsync change check <id>`
   first, so the requirement text that cases link is materialized.
2. Run the affected cases: `chough cases run <ID> --environment <env>`, plus the
   project lanes.
3. Record the gate: `chough checkpoint --awaiting-human "Implementation review of <id>"`.
4. Commit the source together with `.chough/`.
5. Run `specsync change check <id> --commit` (stage new source files first).
6. Push the branch, open or update the pull request, and observe hosted CI at the
   real tip. Report product tests, structural checks, native lifecycle state,
   provenance and platforms separately.
7. Present to the human: what changed, the current verification, the relevant case
   observations with freshness, open bugs and limits, and what was not checked.

Between the check in step 5 and finalize, nothing writes to `.chough/` and nothing is
committed. Committing Chough records or SpecSync bookkeeping does not make Chough
observations stale, but SpecSync's verification digest covers every non-ignored file,
including `.chough/`. SpecSync 6.1.2 ships an opt-in verification exclusion,
`verification_ignored_paths`, but Chough's own repository has not adopted it: its
entries are directory prefixes, so covering `.chough/session.json` would also hide
`.chough/cases/`, whose case definitions must stay verified. Unless a project lists
only directories that are safe to leave out, any Chough write in that window forces
another `change check`.

If native archive checks are red because review has not happened, say exactly that.
Do not claim all checks are green and do not bypass the gate.

## After the human decides

Only after a real review decision, record it and close natively, following native
status rather than a remembered sequence:

```sh
specsync change review <id> --reviewer "<the human>"
specsync change ship <id>        # or: specsync change finalize <id>
```

Do not commit between review and finalize. Only then clear the gate with `chough
checkpoint --clear-awaiting-human` and run `chough stage advance`, which moves accept
to done; it is refused until native state is `archived`. Push the archive commit and
the checkpoint on the same pull request, wait for required checks, then merge through
the repository's normal process. If several changes share a pull request, review and
finalize them one at a time. If evidence went stale, use the recovery action that
`specsync change status` names. Finishing never enters release.

## Compounding

Lessons can be recorded at any stage. Put them in the module's native lesson source
(`<module>.3md` later plane when present, otherwise the Lessons section of
`context.md`) with the source, the scope where it applies and the future action it
changes. `chough next` points to that source; Chough keeps no lesson store of its own.

Each lesson carries a certainty label:

- `observation`: something seen once, with what was run and where.
- `hypothesis`: a likely explanation not yet tested. It is a prompt to check, not a
  rule.
- `verified`: supported by a test or a repeated observation.

Each lesson names its source (an observation ID, a bug ID or a commit) and its limits
(where it applies and where it was not checked). A project decision, meaning a
checkpoint decision, a `docs/DECISIONS.md` entry or an approved contract, always
outranks a lesson. A lesson never changes a decision; if a lesson argues against one,
raise it with the human. A lesson that changes behavior or policy may need its own
native change. Write lessons from a change during closeout, inside the open native
change (see [librarian.md](librarian.md)). Archival does not mean no bug remains;
open bugs stay in the backlog.
"##;

const CHECKPOINT_BEFORE_MERGE_GUIDANCE: &str = r##"# Checkpoint and cold resume

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

`--clear-awaiting-human` clears every open gate, not just one. To close some gates
and keep the others, pass the gates still waiting with `--awaiting-human` in the same
call; they are logged closed and opened again, so their waiting time restarts there.

Write decision summaries and observations, not private reasoning transcripts.
SpecSync's `Handoff:` line says whether its own records are safe to clear; follow it
too.

## Resume cold

A fresh agent with none of the earlier conversation:

1. Reads `AGENTS.md`, then this skill.
2. Runs `chough next --environment <env>` (or reads the files above directly); it is
   the focused view, and `chough status` is the full one.
3. Checks native status for `native_change`, freshness of the referenced observations,
   open blocking bugs and `awaiting_human`. A referenced observation is `current`,
   `historical` (present but stale), `pruned` (removed by retention, in the ledger) or
   `unavailable` (missing, not in the ledger); only a current one counts, so rerun the
   others rather than treat them as passed.
4. Continues with `next_valid_action` under the same scope, preferences and gates. It
   never records a pending human decision itself. In stage done with the native
   change archived, the cycle is closed: `chough next` recommends `chough stage reset
   --to intent --reason "<why>"` (or `--to plan` when the approved intent already
   covers the new work), then `specsync change new "<outcome>" --spec <module> --path
   <path>` and `chough checkpoint --native-change <id> --next-action "<next step>"`.
   A reset replaces the checkpoint's next action.

## Schema and concurrency

A schema 1 session keeps working for `context` and `status`. `chough migrate`
converts it after saving the original bytes to `.chough/session.v1.json`. Hand edits
are allowed and validated on every read. With several agents, one agent owns the
checkpoint; others report to it. APS was evaluated as a backend and deferred: it lost
concurrent read-modify-write updates without error and accepts unknown fields.
"##;

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
    /// `.specsync/config.toml`: its header comment names the pins too (BUG-0001).
    config: &'a str,
}

/// The leading comment lines of `.specsync/config.toml`.
fn config_header(config: &str) -> String {
    config
        .lines()
        .take_while(|line| line.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
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
                (
                    ".specsync/config.toml",
                    doc_versions(&config_header(sources.config), "HI"),
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
                (
                    ".specsync/config.toml",
                    doc_versions(&config_header(sources.config), "SpecSync"),
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
                (
                    ".specsync/config.toml",
                    doc_versions(&config_header(sources.config), "Fledge"),
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

/// TC-cli-bug-0022-toolchain-matches-ci (REQ-cli-019, BUG-0022): the `toolchain` task,
/// first in the verify lane, prints each installed tool beside the CI pin and fails
/// naming each one that differs, and records its verdict for evidence to read. Stand-ins
/// play every tool; nothing is installed.
#[cfg(unix)]
#[test]
fn toolchain_task_names_a_version_that_differs() {
    let fledge = fs::read_to_string(manifest_dir().join("fledge.toml")).unwrap();
    assert!(
        fledge
            .lines()
            .any(|line| line.trim() == "toolchain = \"sh tools/toolchain.sh\""),
        "fledge.toml defines no toolchain task:\n{fledge}"
    );
    let verify = fledge
        .split("[lanes.verify]")
        .nth(1)
        .and_then(|lane| lane.lines().find(|line| line.starts_with("steps = ")))
        .unwrap_or_default();
    assert!(
        verify.starts_with("steps = [\"toolchain\", "),
        "the verify lane does not run toolchain first: {verify}"
    );

    let pins = Command::new("sh")
        .arg(manifest_dir().join("tools/linux/pins.sh"))
        .output()
        .unwrap();
    assert!(pins.status.success());
    let pins = String::from_utf8(pins.stdout).unwrap();
    let pin = |tool: &str| {
        pins.lines()
            .find_map(|line| line.strip_prefix(&format!("{tool}=")))
            .unwrap()
            .to_string()
    };
    let (hi, specsync, fledge_pin) = (pin("human-intent"), pin("specsync"), pin("fledge"));

    let directory = tempfile::tempdir().unwrap();
    let bin = directory.path().join("bin");
    let log = directory.path().join("logs/toolchain.log");
    let stand_ins = |tools: &[(&str, &str)]| {
        let _ = fs::remove_dir_all(&bin);
        for (name, line) in tools {
            common::stand_in(&bin, name, line);
        }
    };
    let run = || {
        Command::new("sh")
            .arg(manifest_dir().join("tools/toolchain.sh"))
            .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
            .env("CHOUGH_TOOLCHAIN_LOG", &log)
            .output()
            .unwrap()
    };
    let rust = [
        ("rustc", "rustc 1.96.0 (SYNTHETIC 2026-01-01)"),
        ("cargo", "cargo 1.96.0 (SYNTHETIC 2026-01-01)"),
    ];

    // Fledge differs: the task fails naming Fledge, its version and the pin.
    let mut tools = vec![
        ("hi", format!("hi {hi}")),
        ("specsync", format!("specsync {specsync}")),
        ("fledge", "fledge 0.0.1".to_string()),
    ];
    let install = |tools: &[(&str, String)]| {
        let mut all: Vec<(&str, &str)> = tools
            .iter()
            .map(|(name, line)| (*name, line.as_str()))
            .collect();
        all.extend(rust);
        stand_ins(&all);
    };
    install(&tools);
    let output = run();
    let (stdout, stderr) = (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    );
    assert!(!output.status.success(), "{stdout}{stderr}");
    assert!(
        stderr.contains(&format!("Fledge 0.0.1 (CI {fledge_pin})")),
        "{stderr}"
    );
    assert!(
        !stderr.contains("HI ") && !stderr.contains("SpecSync "),
        "{stderr}"
    );
    for line in [
        format!("HI: installed {hi}, CI pins {hi}"),
        format!("SpecSync: installed {specsync}, CI pins {specsync}"),
        format!("Fledge: installed 0.0.1, CI pins {fledge_pin}"),
        "Rust (rustc): installed 1.96.0, CI stable and minimum 1.88".to_string(),
    ] {
        assert!(stdout.contains(&line), "missing {line:?} in {stdout}");
    }
    let verdict = fs::read_to_string(&log).unwrap();
    assert!(
        verdict.starts_with(&format!("differs: Fledge 0.0.1 (CI {fledge_pin})\n")),
        "{verdict}"
    );

    // Every tool at its pin, and a newer Rust than the minimum: the task passes.
    tools[2].1 = format!("fledge {fledge_pin}");
    install(&tools);
    let output = run();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("toolchain: matches CI"));
    assert!(fs::read_to_string(&log).unwrap().starts_with("matches\n"));

    // HI missing and a Rust older than the minimum: both are named.
    stand_ins(&[
        ("specsync", tools[1].1.as_str()),
        ("fledge", tools[2].1.as_str()),
        ("rustc", "rustc 1.80.0 (SYNTHETIC 2024-01-01)"),
        ("cargo", "cargo 1.96.0 (SYNTHETIC 2026-01-01)"),
    ]);
    let output = run();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(!output.status.success());
    assert!(
        stderr.contains(&format!("HI not installed (CI {hi})"))
            && stderr.contains("Rust (rustc) 1.80.0 (minimum 1.88)")
            && !stderr.contains("Rust (cargo)"),
        "{stderr}"
    );
}

/// TC-cli-bug-0022-toolchain-matches-ci, a second copy on PATH (BUG-0022): when a tool
/// differs, the failure names the copy PATH finds first, the one in use, and every
/// other copy on PATH, each with its version, so the human knows which binary to fix.
/// Two stand-in Fledge copies play the Homebrew 1.7.2 and the Cargo 1.8.0 found on the
/// coordinator's Mac. The verdict's first line, which evidence quotes, holds no path.
#[cfg(unix)]
#[test]
fn toolchain_failure_names_each_copy_on_path_with_its_version() {
    let pins = Command::new("sh")
        .arg(manifest_dir().join("tools/linux/pins.sh"))
        .output()
        .unwrap();
    let pins = String::from_utf8(pins.stdout).unwrap();
    let pin = |tool: &str| {
        pins.lines()
            .find_map(|line| line.strip_prefix(&format!("{tool}=")))
            .unwrap()
            .to_string()
    };
    let (hi, specsync, fledge) = (pin("human-intent"), pin("specsync"), pin("fledge"));
    let directory = tempfile::tempdir().unwrap();
    let first = directory.path().join("homebrew-bin");
    let second = directory.path().join("cargo-bin");
    for (name, line) in [
        ("hi", format!("hi {hi}")),
        ("specsync", format!("specsync {specsync}")),
        ("rustc", "rustc 1.96.0 (SYNTHETIC 2026-01-01)".to_string()),
        ("cargo", "cargo 1.96.0 (SYNTHETIC 2026-01-01)".to_string()),
    ] {
        common::stand_in(&second, name, &line);
    }
    common::stand_in(&first, "fledge", "fledge 0.0.1");
    common::stand_in(&second, "fledge", &format!("fledge {fledge}"));
    let log = directory.path().join("toolchain.log");
    let run = |path: String| {
        Command::new("sh")
            .arg(manifest_dir().join("tools/toolchain.sh"))
            .env("PATH", path)
            .env("CHOUGH_TOOLCHAIN_LOG", &log)
            .output()
            .unwrap()
    };
    let output = run(format!(
        "{}:{}:/usr/bin:/bin",
        first.display(),
        second.display()
    ));
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(!output.status.success(), "{stderr}");
    let used = format!("{}/fledge (0.0.1, used)", first.display());
    let other = format!("{}/fledge ({fledge})", second.display());
    assert!(
        stderr.contains(&format!(
            "Fledge on PATH, first found first: {used}, {other}"
        )),
        "{stderr}"
    );
    assert!(
        stderr.contains(&format!(
            "a copy at {fledge} is on PATH after the one used: put it first, or upgrade or remove {}/fledge",
            first.display()
        )),
        "{stderr}"
    );
    // The verdict evidence quotes names no path.
    let verdict = fs::read_to_string(&log).unwrap();
    let first_line = verdict.lines().next().unwrap();
    assert_eq!(first_line, format!("differs: Fledge 0.0.1 (CI {fledge})"));

    // Only the old copy on PATH: the fix is to upgrade it.
    let output = run(format!("{}:/usr/bin:/bin", first.display()));
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(
        stderr.contains(&format!(
            "Fledge on PATH, first found first: {used}; upgrade it to {fledge}"
        )),
        "{stderr}"
    );
}

/// TC-cli-0110 (REQ-cli-019, BUG-0001): README, AGENTS.md, CI, the docs and the header
/// comment of `.specsync/config.toml` name the same HI, SpecSync and Fledge versions,
/// and Cargo.toml, README and the MSRV job the same Rust version.
#[test]
fn versions_agree_across_readme_agents_ci_and_specsync_config() {
    let read = |file: &str| {
        fs::read_to_string(manifest_dir().join(file))
            .unwrap()
            .replace("\r\n", "\n")
    };
    let (readme, agents, ci, cargo, ci_doc, integrations, config) = (
        read("README.md"),
        read("AGENTS.md"),
        read(".github/workflows/ci.yml"),
        read("Cargo.toml"),
        read("docs/ci.md"),
        read("docs/integrations.md"),
        read(".specsync/config.toml"),
    );
    let sources = PinSources {
        readme: &readme,
        agents: &agents,
        ci: &ci,
        cargo: &cargo,
        ci_doc: &ci_doc,
        integrations: &integrations,
        config: &config,
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
        ("README.md", "SpecSync", "specsync@6.3.0", "specsync@9.9.9"),
        ("README.md", "Fledge", "fledge@1.8.0", "fledge@9.9.9"),
        (
            "README.md",
            "minimum Rust version",
            "Chough needs Rust 1.88",
            "Chough needs Rust 9.9",
        ),
        ("AGENTS.md", "HI (human-intent)", "hi 0.8.0", "hi 9.9.9"),
        ("AGENTS.md", "SpecSync", "specsync 6.3.0", "specsync 9.9.9"),
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
            "version: '6.3.0'",
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
            "SpecSync **6.3.0**",
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
            "CorvidLabs/spec-sync) 6.3.0",
            "CorvidLabs/spec-sync) 9.9.9",
        ),
        (
            "docs/integrations.md",
            "Fledge",
            "CorvidLabs/fledge) 1.8.0",
            "CorvidLabs/fledge) 9.9.9",
        ),
        (
            ".specsync/config.toml",
            "HI (human-intent)",
            "HI 0.8.0",
            "HI 9.9.9",
        ),
        (
            ".specsync/config.toml",
            "SpecSync",
            "SpecSync 6.3.0",
            "SpecSync 9.9.9",
        ),
        (
            ".specsync/config.toml",
            "Fledge",
            "Fledge 1.8.0",
            "Fledge 9.9.9",
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
            config.clone(),
        ];
        assert!(changed[slot].contains(from), "slot {slot} lacks {from}");
        changed[slot] = changed[slot].replacen(from, to, 1);
        changed
    };
    let problems_in = |changed: &[String; 7]| {
        pin_disagreements(&PinSources {
            readme: &changed[0],
            agents: &changed[1],
            ci: &changed[2],
            cargo: &changed[3],
            ci_doc: &changed[4],
            integrations: &changed[5],
            config: &changed[6],
        })
    };
    for (file, tool, from, to) in cases {
        let slot = match file {
            "README.md" => 0,
            "AGENTS.md" => 1,
            "Cargo.toml" => 3,
            "docs/ci.md" => 4,
            "docs/integrations.md" => 5,
            ".specsync/config.toml" => 6,
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
    let toolchain = "cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.3.0";
    let shapes = [
        (
            0,
            toolchain,
            "cargo install --locked human-intent@0.8.0 fledge@1.8.0\ncargo install --locked specsync@6.3.0",
            "README.md: Install the toolchain is not one cargo install command",
        ),
        (
            0,
            toolchain,
            "cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.3.0 && hi --version",
            "README.md: Install the toolchain is not one cargo install command",
        ),
        (
            0,
            toolchain,
            "cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.3.0 cargo-nextest@0.9.0",
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

/// The files that state where Chough runs and where CI tests it (FLOW-17).
struct PlatformSources<'a> {
    ci: &'a str,
    fledge: &'a str,
    readme: &'a str,
    ci_doc: &'a str,
    integrations: &'a str,
}

/// The lines of a Markdown section, from its `## ` heading to the next one.
fn markdown_section<'a>(text: &'a str, heading: &str) -> Vec<&'a str> {
    text.lines()
        .skip_while(|line| *line != heading)
        .skip(1)
        .take_while(|line| !line.starts_with("## "))
        .collect()
}

/// Text with every run of whitespace made one space, so a phrase matches however
/// the Markdown is wrapped.
fn flat(lines: &[&str]) -> String {
    lines
        .iter()
        .flat_map(|line| line.split_whitespace())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Every way CI or the platform statements disagree with FLOW-17: macOS and Linux,
/// Windows only through WSL 2 with Ubuntu, and a Linux build running the exercise.
fn platform_problems(sources: &PlatformSources) -> Vec<String> {
    let mut problems = vec![];
    let product = ci_job(sources.ci, "product");
    if !product
        .iter()
        .any(|line| line.trim() == "os: [ubuntu-latest, macos-latest]")
    {
        problems.push(
            ".github/workflows/ci.yml product job: matrix is not [ubuntu-latest, macos-latest]"
                .into(),
        );
    }
    if let Some(line) = sources
        .ci
        .lines()
        .find(|line| line.to_ascii_lowercase().contains("windows"))
    {
        problems.push(format!(
            ".github/workflows/ci.yml: names Windows: {}",
            line.trim()
        ));
    }
    // WSL 2 with Ubuntu is Linux: the Ubuntu verify lane runs the exercise.
    let gates = ci_job(sources.ci, "native-gates");
    if !gates
        .iter()
        .any(|line| line.trim() == "runs-on: ubuntu-latest")
        || !gates
            .iter()
            .any(|line| line.trim() == "- run: fledge lanes run verify")
    {
        problems.push(
            ".github/workflows/ci.yml native-gates job: does not run fledge lanes run verify \
             on ubuntu-latest"
                .into(),
        );
    }
    let fledge: toml::Table = toml::from_str(sources.fledge).unwrap();
    let verify_runs_exercise = fledge
        .get("lanes")
        .and_then(|lanes| lanes.get("verify"))
        .and_then(|verify| verify.get("steps"))
        .and_then(|steps| steps.as_array())
        .is_some_and(|steps| steps.iter().any(|step| step.as_str() == Some("exercise")));
    let exercise_is_the_loop = fledge
        .get("tasks")
        .and_then(|tasks| tasks.get("exercise"))
        .and_then(|task| task.as_str())
        .is_some_and(|task| task.contains("examples/feature-bug-loop.sh"));
    if !verify_runs_exercise || !exercise_is_the_loop {
        problems.push(
            "fledge.toml: the verify lane does not run the exercise task \
             (examples/feature-bug-loop.sh)"
                .into(),
        );
    }
    let support = [
        "Windows is supported through WSL 2 with Ubuntu",
        "native Windows is not supported",
    ];
    let tested = markdown_section(sources.readme, "## Tested platforms");
    for row in tested.iter().filter(|line| line.starts_with("| Hosted CI")) {
        if row.contains("Windows") {
            problems.push(format!(
                "README.md Tested platforms: claims Windows CI: {row}"
            ));
        }
    }
    if !tested
        .iter()
        .any(|line| line.starts_with("| Hosted CI: Ubuntu, macOS |"))
    {
        problems.push("README.md Tested platforms: no hosted CI row for Ubuntu and macOS".into());
    }
    let platforms = markdown_section(sources.integrations, "## Platforms");
    for (file, section) in [
        ("README.md Tested platforms", flat(&tested)),
        ("docs/integrations.md Platforms", flat(&platforms)),
    ] {
        for phrase in support {
            if !section.contains(phrase) {
                problems.push(format!("{file}: does not say {phrase:?}"));
            }
        }
        if !section.contains("WSL 1") || !section.contains("/mnt/c") {
            problems.push(format!(
                "{file}: does not name what the Ubuntu jobs do not cover (WSL 1, /mnt/c)"
            ));
        }
    }
    let ci_doc = flat(&sources.ci_doc.lines().collect::<Vec<_>>());
    if sources.ci_doc.contains("windows-latest")
        || !ci_doc.contains("release builds on Ubuntu and macOS")
    {
        problems.push("docs/ci.md: still lists Windows CI".into());
    }
    problems
}

#[test]
fn supported_platforms_are_macos_and_linux_with_windows_through_wsl() {
    let read = |file: &str| {
        fs::read_to_string(manifest_dir().join(file))
            .unwrap()
            .replace("\r\n", "\n")
    };
    let (ci, fledge, readme, ci_doc, integrations) = (
        read(".github/workflows/ci.yml"),
        read("fledge.toml"),
        read("README.md"),
        read("docs/ci.md"),
        read("docs/integrations.md"),
    );
    let sources = PlatformSources {
        ci: &ci,
        fledge: &fledge,
        readme: &readme,
        ci_doc: &ci_doc,
        integrations: &integrations,
    };
    let problems = platform_problems(&sources);
    assert!(problems.is_empty(), "{}", problems.join("\n"));

    // The checker itself: bringing Windows back or dropping the exercise fails.
    let ci_with_windows = ci.replace(
        "os: [ubuntu-latest, macos-latest]",
        "os: [ubuntu-latest, macos-latest, windows-latest]",
    );
    let fledge_without_exercise = fledge.replace("[lanes.verify]", "[lanes.unused]");
    let readme_with_windows = readme.replace(
        "| Hosted CI: Ubuntu, macOS |",
        "| Hosted CI: Ubuntu, macOS, Windows |",
    );
    let mutations = [
        (
            PlatformSources {
                ci: &ci_with_windows,
                ..sources
            },
            ".github/workflows/ci.yml product job",
        ),
        (
            PlatformSources {
                fledge: &fledge_without_exercise,
                ..sources
            },
            "fledge.toml",
        ),
        (
            PlatformSources {
                readme: &readme_with_windows,
                ..sources
            },
            "README.md Tested platforms: claims Windows CI",
        ),
    ];
    for (changed, expected) in mutations {
        let problems = platform_problems(&changed);
        assert!(
            problems.iter().any(|problem| problem.starts_with(expected)),
            "expected {expected:?}: {problems:?}"
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

/// `references/checkpoint.md` as shipped by main 66bef80 (safe checkpoint merges, #29).
const CHECKPOINT_AT_66BEF80: &str = r##"# Checkpoint and cold resume

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

`--clear-awaiting-human` clears every open gate, not just one. To close some gates
and keep the others, pass the gates still waiting with `--awaiting-human` in the same
call; they are logged closed and opened again, so their waiting time restarts there.

Write decision summaries and observations, not private reasoning transcripts.
SpecSync's `Handoff:` line says whether its own records are safe to clear; follow it
too.

## Resume cold

A fresh agent with none of the earlier conversation:

1. Reads `AGENTS.md`, then this skill.
2. Runs `chough next --environment <env>` (or reads the files above directly); it is
   the focused view, and `chough status` is the full one.
3. Checks native status for `native_change`, freshness of the referenced observations,
   open blocking bugs and `awaiting_human`. A referenced observation is `current`,
   `historical` (present but stale), `pruned` (removed by retention, in the ledger) or
   `unavailable` (missing, not in the ledger); only a current one counts, so rerun the
   others rather than treat them as passed.
4. Continues with `next_valid_action` under the same scope, preferences and gates. It
   never records a pending human decision itself. In stage done with the native
   change archived, the cycle is closed: `chough next` recommends `chough stage reset
   --to intent --reason "<why>"` (or `--to plan` when the approved intent already
   covers the new work), then `specsync change new "<outcome>" --spec <module> --path
   <path>` and `chough checkpoint --native-change <id> --next-action "<next step>"`.
   A reset replaces the checkpoint's next action.

## Schema and concurrency

A schema 1 session keeps working for `context` and `status`. `chough migrate`
converts it after saving the original bytes to `.chough/session.v1.json`. Hand edits
are allowed and validated on every read. With several agents, one agent owns live
checkpoint writes; others report to it. APS was evaluated as a backend and deferred:
it lost concurrent read-modify-write updates without error and accepts unknown fields.

For committed parallel schema 2 edits, the repository declares:

```gitattributes
.chough/session.json merge=chough-session
```

Each clone that wants automatic merging configures the driver locally:

```sh
git config --local merge.chough-session.name "Chough schema 2 checkpoint"
git config --local merge.chough-session.driver "chough checkpoint --merge %O %A %B"
```

Git supplies base, ours and theirs as `%O`, `%A` and `%B`. `chough init` never sets
these values. Without a driver, use the explicit fallback:

```sh
chough checkpoint --merge <base> <ours> <theirs>
```

The merge preserves ordered exact-string history and reference changes. It treats
objective, intent scope, modules, stage, native change, next action, run consents,
blockers and awaiting-human gates as one current-cycle group. If both sides change
that group differently, leave the conflict unresolved and ask the checkpoint owner
or human to select a coherent cycle. Never hand-resolve it by combining fields. A
merge is not approval, consent, gate closure or a lifecycle transition.

Success replaces only ours. Every refusal preserves all three inputs and writes no
Chough record. The merge adds no second checkpoint store and does not change the
one-owner rule for live writes.
"##;

#[test]
fn librarian_reference_from_identity_main_updates_without_overwriting_edits() {
    let project = empty_project();
    let root = project.path();
    chough(
        root,
        &["init", "--objective", "Resume the identity main skill"],
    );
    let file = "references/librarian.md";
    let installed = root.join(".agents/skills/chough-workflow").join(file);
    let old = IDENTITY_MAIN_LIBRARIAN.replace("\r\n", "\n");
    assert_eq!(
        format!("{:x}", Sha256::digest(old.as_bytes())),
        "4fa746a83405059803f3589579559dd2ac909d942e1fd8dd709158b488e5dc0d"
    );
    fs::write(&installed, &old).unwrap();
    assert_eq!(
        skill_state(&chough(root, &["skill", "status"]), file)["state"],
        "known-outdated"
    );
    chough(root, &["skill", "update"]);
    assert_eq!(
        skill_state(&chough(root, &["skill", "status"]), file)["state"],
        "current"
    );
    let edited = format!("{old}\nA local librarian note.\n");
    fs::write(&installed, &edited).unwrap();
    assert_eq!(
        skill_state(&chough(root, &["skill", "status"]), file)["state"],
        "edited"
    );
    chough(root, &["skill", "update"]);
    assert_eq!(fs::read_to_string(&installed).unwrap(), edited);
}

// Exact LF predecessor bytes from the integrated identity head, held inside the
// already-approved test file rather than a new fixture directory.
const IDENTITY_MAIN_LIBRARIAN: &str = r####"# Optional project librarian

The librarian is a project-local knowledge folder, off by default. Turn it on only
when the human wants it: `chough pref set librarian on --source "<their words>"`.
The folder name defaults to `library` and can be changed with `chough pref set
library-dir <name> --source …`. It is not a shared cross-project vault.

## From any stage

Enter it as a side stage and return to the caller:

```sh
chough stage enter librarian --reason "Snapshot the approved prototype before redesign"
chough library snapshot design/prototype/index.html design/prototype/COMPANION.md --reason "Approved version 1"
chough library index
chough stage return --note "Snapshot SNAP-approved-version-1-<24-lowercase-hex> taken"
```

- `library init` creates `README.md`, `INDEX.md` and `briefs/`.
- `library snapshot` copies originals into
  `snapshots/SNAP-<slug>-<24-lowercase-hex>/` with a random 96-bit suffix and a
  manifest of source path, Git revision, SHA-256 and creation time. Sources are
  never moved or edited, and their bytes and digests stay unchanged. Repeated
  snapshots of identical inputs remain distinct history.
- `library index` regenerates `INDEX.md`: living sources and their owners, active
  and archived native changes, the open bug backlog, recent observations, briefs and
  snapshots. Legacy `snapshots/<date>-<n>/` directories remain listed beside new
  SNAP directories without renaming or migration. Manifest timestamps determine
  chronology, never the snapshot ID or random suffix.
- Briefs in `briefs/` are focused next-session summaries written from
  [the brief template](../templates/brief.md), linking to originals.

## Boundaries

Living HI, prototypes, contracts and native archives stay in their owning locations;
a snapshot is history, not a second current copy. Summaries complement originals.
Unfixed bugs remain in the live backlog even when an old report is snapshotted. The
mandatory checkpoint does not depend on the librarian.
"####;

#[test]
fn a_skill_md_copy_from_main_118a895_is_known_outdated_and_edited_copies_stay() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let installed = root.join(".agents/skills/chough-workflow/SKILL.md");
    // The known digest is of LF bytes; a Windows checkout may have converted them.
    let old = SKILL_AT_118A895.replace("\r\n", "\n");
    assert_eq!(
        format!("{:x}", Sha256::digest(old.as_bytes())),
        "367a9873293e79ca40c9634e2b2a12c3223074aaec161c7444690c2d02e12199"
    );
    fs::write(&installed, &old).unwrap();
    let status = chough(root, &["skill", "status"]);
    assert_eq!(skill_state(&status, "SKILL.md")["state"], "known-outdated");
    chough(root, &["skill", "update"]);
    assert_eq!(
        fs::read(&installed).unwrap(),
        fs::read(manifest_dir().join("skills/chough-workflow/SKILL.md")).unwrap()
    );
    let status = chough(root, &["skill", "status"]);
    assert_eq!(skill_state(&status, "SKILL.md")["state"], "current");

    let edited = format!("{old}\nA local note.\n");
    fs::write(&installed, &edited).unwrap();
    let status = chough(root, &["skill", "status"]);
    assert_eq!(skill_state(&status, "SKILL.md")["state"], "edited");
    chough(root, &["skill", "update"]);
    assert_eq!(fs::read_to_string(&installed).unwrap(), edited);
}

/// LIFE-22: an agent with a small context, a local model included, finds one short
/// path for a routine change before the stage-by-stage detail, and that path names
/// the session, the native change with a delta per spec, the human's gates and the
/// checks in order.
#[test]
fn the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change() {
    let read = |path: &str| {
        fs::read_to_string(manifest_dir().join(path))
            .unwrap()
            .replace("\r\n", "\n")
    };
    let skill = read("skills/chough-workflow/SKILL.md");
    assert!(
        skill.contains("[the stage contract](references/stages.md) and its fast path"),
        "SKILL.md routes to the fast path"
    );
    let stages = read("skills/chough-workflow/references/stages.md");
    let start = stages
        .find("## Fast path for a routine change\n")
        .expect("a fast path section");
    let transitions = stages
        .find("## Transitions shared by every stage\n")
        .expect("the transitions section");
    assert!(start < transitions, "the fast path comes first");
    let path = &stages[start..transitions];
    assert!(
        path.lines().count() <= 32,
        "the fast path has {} lines",
        path.lines().count()
    );
    let mut at = 0;
    for step in [
        "`AGENTS.md`",
        "`chough session new <change-id>`",
        "`chough checkpoint --criterion <id>`",
        "`chough next`",
        "`chough change new",
        "`specsync change answer`",
        "`deltas/<module>.md` for each affected spec",
        "scope approval belongs to the human",
        "first code edit",
        "`.chough/cases/<module>.<topic>.cases.toml`",
        "`fledge run test`",
        "`chough change check",
        "stop for the human's review",
        "Work only in your own checkout and branch",
    ] {
        let found = path[at..]
            .find(step)
            .unwrap_or_else(|| panic!("the fast path names {step} in order"));
        at += found + step.len();
    }
}

#[test]
fn the_skill_asks_a_resuming_agent_to_state_the_handoff_before_its_first_edit() {
    let skill = fs::read_to_string(manifest_dir().join("skills/chough-workflow/SKILL.md"))
        .unwrap()
        .replace("\r\n", "\n");
    assert!(skill.len() < 6144, "SKILL.md is {} bytes", skill.len());
    let resume = skill
        .split("## Start or resume\n")
        .nth(1)
        .and_then(|rest| rest.split("\n## ").next())
        .expect("a Start or resume section");
    let still_true = resume
        .find("Check what is still true")
        .expect("the check step");
    let contract = resume
        .find("the stage contract")
        .expect("the stage contract step");
    let start = resume
        .find("Before your first edit")
        .expect("a step about the first edit");
    assert!(
        still_true < start && start < contract,
        "the step is out of order"
    );
    let step = &resume[start..contract];
    let step = &step[..step.rfind("\n").unwrap_or(step.len())];
    assert!(
        step.lines().count() <= 8,
        "the step has {} lines",
        step.lines().count()
    );
    for phrase in [
        "your own message",
        "the goal",
        "decision that",
        "with its reason",
        "rejected",
        "last failure",
        "its bug",
        "status",
        "lessons that apply",
        "where they",
        "the next task",
        "An edit is any file change",
        "Chough",
        "say so",
        "instead of guessing",
    ] {
        assert!(
            step.replace("\n   ", " ")
                .replace("\n", " ")
                .contains(phrase),
            "the step does not say {phrase:?}"
        );
    }
    assert!(!step.contains('\u{2013}') && !step.contains('\u{2014}'));
}

const SKILL_AT_118A895: &str = r##"---
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
--module <module>`; it never overwrites existing files. Without `--host` it installs
the skill for `openai`; any `--host` replaces that default, so repeat it for each
host you want, for example `--host openai --host claude`. A schema 1 session is migrated only with `chough migrate`.

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
project skill directory: openai, the default, writes `.agents/skills`, a shared
directory that more than one host may read, and gemini writes `.gemini/skills`, which
no run has exercised. For Kimi Code CLI, use `chough init --host openai`. The existing
`openai` mapping installs `.agents/skills/chough-workflow/`; there is no separate
`kimi` host value. `chough skill status` shows whether a copy is current, edited or
outdated. Which directory a host reads is the host's behavior: Claude Code was
observed reading `.claude/skills`, Kimi Code CLI 0.41.0 was observed discovering
`.agents/skills/chough-workflow/SKILL.md` in one macOS session, and one run, reported
narratively in closed PR #20 and not independently verified, said Google Antigravity
read `.agents/skills`. Confirm that the host actually lists the skill; otherwise give
it this file directly. Subagents are optional: [roles.md](references/roles.md) also
works as sequential passes.
"##;

/// Frozen checkpoint reference from the definition base 9a8b9b1. Keep its bytes.
const CHECKPOINT_BEFORE_IMMUTABLE_HISTORY: &str = r###"# Checkpoint and cold resume

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

`--clear-awaiting-human` clears every open gate, not just one. To close some gates
and keep the others, pass the gates still waiting with `--awaiting-human` in the same
call; they are logged closed and opened again, so their waiting time restarts there.

Write decisions with a prefix and a reason, so a fresh agent can tell what was chosen,
what was ruled out and what already failed:

```text
decided: <what> because <reason>
rejected: <approach> because <reason>
failed: <experiment> because <reason>
```

This is a convention, read best-effort: the prefix starts the entry, lowercase, with a
colon, and the reason is the text after the last ` because `. Any other entry, such as
older free text or a dated note, is still kept and shown whole. Decisions stay plain
strings in the checkpoint; nothing validates the convention. Record a human's decision
only in their words, and label an agent's own assumption as the agent's, without a
`decided:` prefix.

Write decision summaries and observations, not private reasoning transcripts.
SpecSync's `Handoff:` line says whether its own records are safe to clear; follow it
too.

## Resume cold

A fresh agent with none of the earlier conversation:

1. Reads `AGENTS.md`, then this skill.
2. Runs `chough next --environment <env>` (or reads the files above directly); it is
   the focused view, and `chough status` is the full one. Besides the stage, native
   state, next action and gates, it shows the 5 newest decisions with their kind and
   reason, the latest failed observation by recorded time with the status of each bug
   that links it, and where each module's lessons live (`<module>.3md`, or the
   `## Lessons` line of `context.md`) with the newest archived SpecSync lesson
   bundles. It points to lessons and never prints them; read the source it names.
3. Checks native status for `native_change`, freshness of the referenced observations,
   open blocking bugs and `awaiting_human`. A referenced observation is `current`,
   `historical` (present but stale), `pruned` (removed by retention, in the ledger) or
   `unavailable` (missing, not in the ledger); only a current one counts, so rerun the
   others rather than treat them as passed.
4. Continues with `next_valid_action` under the same scope, preferences and gates. It
   never records a pending human decision itself. In stage done with the native
   change archived, the cycle is closed: `chough next` recommends `chough stage reset
   --to intent --reason "<why>"` (or `--to plan` when the approved intent already
   covers the new work), then `specsync change new "<outcome>" --spec <module> --path
   <path>` and `chough checkpoint --native-change <id> --next-action "<next step>"`.
   The same holds at an earlier stage when the native change is already archived:
   `chough next` names the archive and the new cycle instead of continuing, after
   `chough stage advance` to done at accept. A reset replaces the checkpoint's next
   action.

## Schema and concurrency

A schema 1 session keeps working for `context` and `status`. `chough migrate`
converts it after saving the original bytes to `.chough/session.v1.json`. Hand edits
are allowed and validated on every read. With several agents, one agent owns live
checkpoint writes; others report to it. APS was evaluated as a backend and deferred:
it lost concurrent read-modify-write updates without error and accepts unknown fields.

For committed parallel schema 2 edits, the repository declares:

```gitattributes
.chough/session.json merge=chough-session
```

Each clone that wants automatic merging configures the driver locally:

```sh
git config --local merge.chough-session.name "Chough schema 2 checkpoint"
git config --local merge.chough-session.driver "chough checkpoint --merge %O %A %B"
```

Git supplies base, ours and theirs as `%O`, `%A` and `%B`. `chough init` never sets
these values. Without a driver, use the explicit fallback:

```sh
chough checkpoint --merge <base> <ours> <theirs>
```

The merge preserves ordered exact-string history and reference changes. It treats
objective, intent scope, modules, stage, native change, next action, run consents,
blockers and awaiting-human gates as one current-cycle group. If both sides change
that group differently, leave the conflict unresolved and ask the checkpoint owner
or human to select a coherent cycle. Never hand-resolve it by combining fields. A
merge is not approval, consent, gate closure or a lifecycle transition.

Success replaces only ours. Every refusal preserves all three inputs and writes no
Chough record. The merge adds no second checkpoint store and does not change the
one-owner rule for live writes.
"###;

#[test]
fn the_pre_history_checkpoint_reference_updates_only_when_unedited() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "History compatibility"]);
    let path = root.join(".agents/skills/chough-workflow/references/checkpoint.md");
    let old = CHECKPOINT_BEFORE_IMMUTABLE_HISTORY.replace("\r\n", "\n");
    assert_eq!(
        format!("{:x}", Sha256::digest(old.as_bytes())),
        "c21ddc44653079aaa61553c2a31389b4b942925267d4d9c847b09646e90fcf8b"
    );
    fs::write(&path, &old).unwrap();
    let status = chough(root, &["skill", "status"]);
    assert_eq!(
        skill_state(&status, "references/checkpoint.md")["state"],
        "known-outdated"
    );
    chough(root, &["skill", "update"]);
    assert_eq!(
        fs::read(&path).unwrap(),
        fs::read(manifest_dir().join("skills/chough-workflow/references/checkpoint.md")).unwrap()
    );
    let edited = format!("{old}\nOwner edit retained.\n");
    fs::write(&path, &edited).unwrap();
    chough(root, &["skill", "update"]);
    assert_eq!(fs::read_to_string(&path).unwrap(), edited);
}

/// #58, FLOW-4: the prototype's state switcher sets `hidden` on the state sections. The
/// browser's own `display: none` for `[hidden]` loses to any project rule that sets
/// `display` on those sections, which shows every state at once, so the template's
/// styles.css keeps hidden panels hidden itself.
#[test]
fn the_prototype_template_keeps_hidden_state_panels_hidden() {
    let template = manifest_dir().join("skills/chough-workflow/templates/prototype");
    let html = fs::read_to_string(template.join("index.html")).unwrap();
    assert!(
        html.contains("panel.hidden ="),
        "templates/prototype/index.html switches state panels with `hidden`"
    );
    let css = fs::read_to_string(template.join("styles.css"))
        .unwrap()
        .replace("\r\n", "\n");
    let guarded = css.split('}').any(|rule| {
        let Some((selector, body)) = rule.split_once('{') else {
            return false;
        };
        let selector = selector.rsplit("*/").next().unwrap().trim();
        selector == "[hidden]"
            && body.split(';').any(|declaration| {
                declaration.split_whitespace().collect::<Vec<_>>()
                    == ["display:", "none", "!important"]
            })
    });
    assert!(
        guarded,
        "templates/prototype/styles.css has no `[hidden] {{ display: none !important; }}` rule"
    );
}

/// #58: styles.css as releases 0.1.0 to 0.4.0 shipped it reads known-outdated and
/// `skill update` replaces it; an edited copy stays edited and is kept.
#[test]
fn the_prototype_styles_from_releases_0_1_0_to_0_4_0_are_known_outdated_and_edited_copies_stay() {
    let project = empty_project();
    let root = project.path();
    chough(root, &["init", "--objective", "Try Chough"]);
    let file = "templates/prototype/styles.css";
    let installed = root.join(".agents/skills/chough-workflow").join(file);
    // The known digest is of LF bytes; a Windows checkout may have converted them.
    let old = PROTOTYPE_STYLES_AT_0_4_0.replace("\r\n", "\n");
    assert_eq!(
        format!("{:x}", Sha256::digest(old.as_bytes())),
        "ce8f63741058cf79afa275e01fb7c6ab0c38dc242c35c87a8628c808bf386239"
    );
    fs::write(&installed, &old).unwrap();
    let status = chough(root, &["skill", "status"]);
    assert_eq!(skill_state(&status, file)["state"], "known-outdated");
    chough(root, &["skill", "update"]);
    let status = chough(root, &["skill", "status"]);
    for bundled in bundle_files() {
        assert_eq!(
            skill_state(&status, &bundled)["state"],
            "current",
            "{bundled}"
        );
    }

    let edited = format!("{old}\n.local-note {{ color: inherit; }}\n");
    fs::write(&installed, &edited).unwrap();
    let status = chough(root, &["skill", "status"]);
    assert_eq!(skill_state(&status, file)["state"], "edited");
    chough(root, &["skill", "update"]);
    assert_eq!(fs::read_to_string(&installed).unwrap(), edited);
}

const PROTOTYPE_STYLES_AT_0_4_0: &str = r##"/* Build on project tokens. The fallbacks below only keep the template readable
   when no token file is linked; replace them with the project's tokens. */
:root {
  --proto-ink: var(--ink, #15181b);
  --proto-paper: var(--paper, #faf9f6);
  --proto-surface: var(--surface, #ffffff);
  --proto-accent: var(--sheen, #0e6f66);
  --proto-danger: var(--danger, #b3261e);
  --proto-hairline: var(--hairline, rgba(21, 24, 27, 0.12));
}

* { box-sizing: border-box; }

body {
  margin: 0;
  font-family: var(--font-display, system-ui, sans-serif);
  background: var(--proto-paper);
  color: var(--proto-ink);
  line-height: 1.5;
}

.prototype-controls {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  align-items: center;
  padding: 0.5rem 1rem;
  border-bottom: 1px dashed var(--proto-hairline);
  font-size: 0.875rem;
}

.prototype-controls button[aria-pressed="true"] { outline: 2px solid var(--proto-accent); }

.surface {
  max-width: 40rem;
  margin: 2rem auto;
  padding: 1.5rem;
  background: var(--proto-surface);
  border: 1px solid var(--proto-hairline);
  border-radius: 0.5rem;
}

.error { color: var(--proto-danger); }

a { color: var(--proto-accent); }

:focus-visible { outline: 3px solid var(--proto-accent); outline-offset: 2px; }

@media (max-width: 480px) {
  .surface { margin: 0; border-radius: 0; border-left: 0; border-right: 0; }
}
"##;

/// REQ-cli-064: CI runs once a day on main as well as on pushes and pull requests, so
/// the flake window (REQ-cli-062) sees macOS and Linux every day; a scheduled run has
/// no pull request base and no previous commit, so the SpecSync Archive jobs fall back
/// to the run's own commit instead of handing the inspector an empty base.
#[test]
fn ci_runs_once_a_day_on_main_with_a_real_archive_base() {
    let workflow = fs::read_to_string(manifest_dir().join(".github/workflows/ci.yml")).unwrap();
    let triggers: Vec<&str> = workflow
        .split("\non:\n")
        .nth(1)
        .and_then(|rest| rest.split("\n\n").next())
        .expect("ci.yml has an `on:` block")
        .lines()
        .collect();
    for trigger in ["  push:", "  pull_request:", "  schedule:"] {
        assert!(
            triggers.contains(&trigger),
            "ci.yml's `on:` block lacks `{}`: {triggers:#?}",
            trigger.trim()
        );
    }
    let crons: Vec<&str> = triggers
        .iter()
        .filter_map(|line| line.trim().strip_prefix("- cron: "))
        .collect();
    assert_eq!(crons.len(), 1, "one daily cron entry: {triggers:#?}");
    let fields: Vec<&str> = crons[0].trim_matches('\'').split_whitespace().collect();
    assert_eq!(fields.len(), 5, "a five-field cron: {}", crons[0]);
    assert!(
        fields[0].parse::<u8>().is_ok() && fields[1].parse::<u8>().is_ok(),
        "the cron names one minute and one hour: {}",
        crons[0]
    );
    assert_eq!(
        &fields[2..],
        ["*", "*", "*"],
        "the cron runs every day: {}",
        crons[0]
    );
    let bases: Vec<&str> = workflow
        .lines()
        .filter(|line| line.trim_start().starts_with("ARCHIVE_BASE_SHA:"))
        .collect();
    assert!(
        !bases.is_empty(),
        "the SpecSync Archive jobs set ARCHIVE_BASE_SHA"
    );
    for base in bases {
        assert!(
            base.trim_end().ends_with(
                "github.event.pull_request.base.sha || github.event.before || github.sha }}"
            ),
            "the SpecSync Archive base falls back to github.sha: {base}"
        );
    }
}
