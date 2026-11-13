//! Real upgrades of exact workflow guidance from the inspected main revisions.
//! This checks installer safety, not fresh-agent behavior or human authorization.

mod common;

use common::Fixture;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

type PriorFile = (&'static str, &'static [u8], &'static str, &'static [u8]);

const PLAN_AT_825A380: &str = r###"# Technical design and plan

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
"###;

const ACCEPT_AT_825A380: &str = r###"# Acceptance and native closeout

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
"###;

const PRIOR_FILES: [PriorFile; 4] = [
    (
        "SKILL.md",
        include_bytes!("fixtures/workflow-at-825a380-SKILL.md"),
        "6b81882bc2e8305dc1ae0181b1eb52ad9d586c0a9a6990e03e865d0de4149198",
        include_bytes!("../skills/chough-workflow/SKILL.md"),
    ),
    (
        "references/lifecycle.md",
        include_bytes!("fixtures/workflow-at-825a380-lifecycle.md"),
        "14fd2277d5e3f97f24fe6b995b1815eff1cfcd7b49d03669919bdcc7290c2200",
        include_bytes!("../skills/chough-workflow/references/lifecycle.md"),
    ),
    (
        "references/plan.md",
        PLAN_AT_825A380.as_bytes(),
        "1022db57d0c114effc8669ede9ebb453f932109f608f883712725e57ba53551d",
        include_bytes!("../skills/chough-workflow/references/plan.md"),
    ),
    (
        "references/accept.md",
        ACCEPT_AT_825A380.as_bytes(),
        "80f71366ad81e1720bdc5f91d51c5428cd6a60729c9d1675977e284f7b08e398",
        include_bytes!("../skills/chough-workflow/references/accept.md"),
    ),
];

const PLAN_AT_3D00E44: &str = r###"# Technical design and plan

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
has unchecked boxes, so list human gates, and running or recording Chough cases, as
plain text rather than checkboxes. A requirement the delta adds reaches the
companion only when `change check` materializes it, and `chough cases check` refuses
a case linking it until then, so a box for those cases would block the very check
that unblocks them.

## What needs reapproval

A change to approved intent, contract, acceptance criteria or affected scope needs
native reapproval. Routine implementation, tests, evidence, delta materialization
and lifecycle metadata do not.
"###;

const ACCEPT_AT_3D00E44: &str = r###"# Acceptance and native closeout

Acceptance is where the human decides whether the implementation is what they
wanted. Agent review, a passing command, scope approval or an actor label never
stands in for that decision.

## Prepare the review

Follow this order, so that one verification covers both the code and Chough's
records. Follow native status where it names a different next action.

1. When the delta adds or modifies requirements, run `specsync change check <id>`
   first, so the requirement text that cases link is materialized. It refuses while
   `tasks.md` has an unchecked box, so the cases in step 2 are never a box there.
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
"###;

const INTEGRATED_MAIN_FILES: [PriorFile; 2] = [
    (
        "references/plan.md",
        PLAN_AT_3D00E44.as_bytes(),
        "5fd009510e2ef60d726417a11a2ac3f3a258826ad80d0be5d8b8001c035c0784",
        include_bytes!("../skills/chough-workflow/references/plan.md"),
    ),
    (
        "references/accept.md",
        ACCEPT_AT_3D00E44.as_bytes(),
        "0d44a367560c8a342c288116e904ecd9d3dcc0a4c0fa702acfbe83ff3b5ceb40",
        include_bytes!("../skills/chough-workflow/references/accept.md"),
    ),
];

fn file_state<'a>(report: &'a Value, name: &str) -> &'a Value {
    report["hosts"][0]["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|file| file["file"] == name)
        .unwrap_or_else(|| panic!("{name} missing from {report}"))
}

#[test]
fn workflow_guidance_from_main_825a380_updates_and_preserves_edits() {
    for prior_files in [&PRIOR_FILES[..], &INTEGRATED_MAIN_FILES[..]] {
        // Git may convert text fixtures on checkout. Reconstruct the exact Git blob's
        // LF bytes before checking its independently recorded identity.
        let previous_files: Vec<_> = prior_files
            .iter()
            .map(|(name, previous, digest, current)| {
                let previous = String::from_utf8(previous.to_vec())
                    .unwrap()
                    .replace("\r\n", "\n")
                    .into_bytes();
                assert_eq!(format!("{:x}", Sha256::digest(&previous)), *digest);
                assert_ne!(previous, *current, "{name} must exercise an older bundle");
                (*name, previous, *current)
            })
            .collect();

        for edited in
            std::iter::once(None).chain(previous_files.iter().map(|(name, _, _)| Some(*name)))
        {
            let fixture = Fixture::new();
            fixture.init();
            let installed = fixture.root().join(".agents/skills/chough-workflow");
            let checkpoint = fixture.read(&fixture.session_file());
            let mut edited_bytes = None;
            for (name, previous, _) in &previous_files {
                let name = *name;
                let mut bytes = previous.to_vec();
                if edited == Some(name) {
                    bytes.extend_from_slice(b"\nHuman project guidance must stay.\n");
                    edited_bytes = Some(bytes.clone());
                }
                fs::write(installed.join(name), bytes).unwrap();
            }

            let status = fixture.ok(&["skill", "status", "--host", "openai"]);
            for (name, _, _) in &previous_files {
                let name = *name;
                assert_eq!(
                    file_state(&status, name)["state"],
                    if edited == Some(name) {
                        "edited"
                    } else {
                        "known-outdated"
                    },
                    "{name}: {status}"
                );
            }

            let update = fixture.ok(&["skill", "update", "--host", "openai"]);
            let after = fixture.ok(&["skill", "status", "--host", "openai"]);
            for (name, _, current) in &previous_files {
                let name = *name;
                let bytes = fs::read(installed.join(name)).unwrap();
                if edited == Some(name) {
                    assert_eq!(bytes, *edited_bytes.as_ref().unwrap(), "{name}");
                    assert_eq!(file_state(&update, name)["action"], "preserved-edit");
                    assert_eq!(file_state(&after, name)["state"], "edited");
                } else {
                    assert_eq!(bytes, *current, "{name}");
                    assert_eq!(file_state(&update, name)["action"], "updated");
                    assert_eq!(file_state(&after, name)["state"], "current");
                }
            }
            assert_eq!(fixture.read(&fixture.session_file()), checkpoint);
        }
    }
}

/// Extract inline and fenced examples without treating narrative as shell code.
fn code_examples(markdown: &str) -> Vec<String> {
    let mut examples = Vec::new();
    let mut remaining = markdown;
    while let Some(start) = remaining.find('`') {
        remaining = &remaining[start..];
        let width = remaining.bytes().take_while(|byte| *byte == b'`').count();
        let delimiter = "`".repeat(width);
        remaining = &remaining[width..];
        let Some(end) = remaining.find(&delimiter) else {
            break;
        };
        let code = remaining[..end].replace("\\\r\n", " ").replace("\\\n", " ");
        if width == 1 {
            examples.push(code);
        } else {
            examples.extend(code.lines().map(str::to_owned));
        }
        remaining = &remaining[end + width..];
    }
    examples
}

/// The examples use ordinary shell words and quoted account placeholders.
fn example_words(example: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut quote = None;
    for character in example.chars() {
        if quote == Some(character) {
            quote = None;
        } else if quote.is_none() && matches!(character, '\'' | '"') {
            quote = Some(character);
        } else if quote.is_none() && character.is_whitespace() {
            if !word.is_empty() {
                words.push(std::mem::take(&mut word));
            }
        } else {
            word.push(character);
        }
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
}

fn native_writer_arguments(markdown: &str) -> Vec<(String, Option<String>)> {
    let mut writers = Vec::new();
    for example in code_examples(markdown) {
        let words = example_words(&example);
        for (index, prefix) in words.windows(3).enumerate() {
            if prefix[0] != "specsync" || prefix[1] != "change" {
                continue;
            }
            // A Timeline entry's `native:` field names the command a gate close ran; it
            // is a recorded value, not an example to run.
            if index > 0 && words[index - 1] == "native:" {
                continue;
            }
            let option = match prefix[2].as_str() {
                "approve" => "--actor",
                "review" => "--reviewer",
                _ => continue,
            };
            let args = &words[index + 3..];
            // A bare command name is a reference, not a runnable example.
            if args.is_empty() {
                continue;
            }
            let end = args
                .iter()
                .position(|word| matches!(word.as_str(), ";" | "&&" | "||" | "#"))
                .unwrap_or(args.len());
            let args = &args[..end];
            let actor = args.iter().enumerate().find_map(|(index, word)| {
                if word == option {
                    args.get(index + 1).cloned()
                } else {
                    word.strip_prefix(&format!("{option}=")).map(str::to_owned)
                }
            });
            writers.push((prefix[2].clone(), actor));
        }
    }
    writers
}

fn markdown_files(base: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    let mut pending = vec![base.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "md") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

#[test]
fn native_writer_examples_and_installed_workflows_are_consistent() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let canonical = root.join("skills/chough-workflow");
    let mut sources = markdown_files(&canonical);
    sources.extend(
        ["README.md", "docs/cli.md", "docs/walkthrough.md"]
            .into_iter()
            .map(|file| root.join(file)),
    );
    let mut found = BTreeSet::new();
    let mut problems = Vec::new();
    for path in sources {
        let relative = path
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        for (operation, actor) in native_writer_arguments(&fs::read_to_string(&path).unwrap()) {
            found.insert((relative.to_string(), operation.clone()));
            if !matches!(
                actor.as_deref(),
                Some(
                    "<agent>-<terminal>"
                        | "<agent-account>"
                        | "<project-agent-account>"
                        | "corvid-agent"
                        | "corvid"
                )
            ) {
                problems.push(format!(
                    "{relative}: native {operation} must name the acting agent account, got {actor:?}"
                ));
            }
        }
    }
    for (file, operation) in [
        ("skills/chough-workflow/references/plan.md", "approve"),
        ("skills/chough-workflow/references/accept.md", "review"),
        ("skills/chough-workflow/references/lifecycle.md", "approve"),
        ("skills/chough-workflow/references/lifecycle.md", "review"),
        ("docs/cli.md", "approve"),
        ("docs/walkthrough.md", "approve"),
    ] {
        if !found.contains(&(file.into(), operation.into())) {
            problems.push(format!(
                "{file}: missing native {operation} command example"
            ));
        }
    }

    // LIFE-14: this repository carries one shared `.agents` copy and no `.claude` copy.
    if root.join(".claude").exists() {
        problems.push(".claude: this repository carries no .claude skill copy (LIFE-14)".into());
    }
    for host in [".agents"] {
        for (file, _, _, _) in PRIOR_FILES {
            let source = fs::read(canonical.join(file)).unwrap();
            let installed_path = root.join(host).join("skills/chough-workflow").join(file);
            match fs::read(&installed_path) {
                Ok(installed) if installed == source => {}
                Ok(installed) => problems.push(format!(
                    "{host}/skills/chough-workflow/{file}: differs from canonical bytes (installed {:x}, canonical {:x})",
                    Sha256::digest(installed),
                    Sha256::digest(source)
                )),
                Err(error) => problems.push(format!("{}: {error}", installed_path.display())),
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// v0.2.0's checkpoint reference (main edbc3c1) is a known earlier bundle file, so
/// `skill update` delivers the sessions guidance over an unedited copy.
#[test]
fn checkpoint_guidance_from_v0_2_0_updates_to_the_sessions_guidance() {
    let previous = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/workflow-at-edbc3c1-checkpoint.md"),
    )
    .unwrap()
    .replace("\r\n", "\n")
    .into_bytes();
    assert_eq!(
        format!("{:x}", Sha256::digest(&previous)),
        "00b3c3f4ffaaeeaaca6d829ff0277c9eb354ce0911f66f41a55639d27fc7d35d"
    );
    let current = include_bytes!("../skills/chough-workflow/references/checkpoint.md");
    assert_ne!(previous, current.to_vec());
    let fixture = Fixture::new();
    fixture.init();
    let installed = fixture
        .root()
        .join(".agents/skills/chough-workflow/references/checkpoint.md");
    fs::write(&installed, &previous).unwrap();
    let status = fixture.ok(&["skill", "status", "--host", "openai"]);
    assert_eq!(
        file_state(&status, "references/checkpoint.md")["state"],
        "known-outdated"
    );
    let update = fixture.ok(&["skill", "update", "--host", "openai"]);
    assert_eq!(
        file_state(&update, "references/checkpoint.md")["action"],
        "updated"
    );
    let delivered = fs::read_to_string(&installed).unwrap();
    assert!(delivered.contains("chough inbox fold"), "{delivered}");
}

/// v0.2.0's lifecycle reference (main edbc3c1) is a known earlier bundle file, so
/// `skill update` delivers the backfill guidance over an unedited copy (review of #73).
#[test]
fn lifecycle_guidance_from_v0_2_0_updates_to_the_backfill_guidance() {
    let previous = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/workflow-at-edbc3c1-lifecycle.md"),
    )
    .unwrap()
    .replace("\r\n", "\n")
    .into_bytes();
    assert_eq!(
        format!("{:x}", Sha256::digest(&previous)),
        "be351d5c3a8c65bd46f4784875b0118f82e5ba4a5b97a7d364d9856636d3b711"
    );
    let current = include_bytes!("../skills/chough-workflow/references/lifecycle.md");
    assert_ne!(previous, current.to_vec());
    let fixture = Fixture::new();
    fixture.init();
    let installed = fixture
        .root()
        .join(".agents/skills/chough-workflow/references/lifecycle.md");
    fs::write(&installed, &previous).unwrap();
    let status = fixture.ok(&["skill", "status", "--host", "openai"]);
    assert_eq!(
        file_state(&status, "references/lifecycle.md")["state"],
        "known-outdated"
    );
    let update = fixture.ok(&["skill", "update", "--host", "openai"]);
    assert_eq!(
        file_state(&update, "references/lifecycle.md")["action"],
        "updated"
    );
    let delivered = fs::read_to_string(&installed).unwrap();
    assert!(delivered.contains("chough stage backfill"), "{delivered}");
}

/// D5 (#122): closeout guidance is archive-first. The hosted Archive gate refuses while a
/// native change is open, so pushing for CI comes only after local verification, the
/// human's implementation review, the native review and finalize. In every place that
/// teaches the order, each instruction to push and observe CI comes after the first
/// mention of finalize.
#[test]
fn closeout_guidance_pushes_for_ci_only_after_finalize() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let pushes = [
        "push, ci",
        "push the branch",
        "push branch",
        "pushes the branch",
        "push and observe",
        "observe required ci",
        "observe hosted ci",
    ];
    for relative in [
        "skills/chough-workflow/references/lifecycle.md",
        "skills/chough-workflow/references/accept.md",
        ".agents/skills/chough-workflow/references/lifecycle.md",
        ".agents/skills/chough-workflow/references/accept.md",
        "docs/lifecycle.md",
        "docs/walkthrough.md",
        "docs/diagrams/lifecycle.mmd",
    ] {
        let text = fs::read_to_string(root.join(relative))
            .unwrap()
            .to_lowercase()
            .replace('\n', " ");
        let finalize = text
            .find("finalize")
            .unwrap_or_else(|| panic!("{relative} never names finalize"));
        for push in pushes {
            if let Some(at) = text.find(push) {
                assert!(
                    at > finalize,
                    "{relative}: \"{push}\" comes before finalize: …{}…",
                    &text[at.saturating_sub(80)..(at + 80).min(text.len())]
                );
            }
        }
    }
}

/// The text of a repository file with every run of whitespace (line ends included, so CRLF
/// and LF alike) collapsed to one space.
fn flat_text(path: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// REQ-cli-029, second criterion (#138, audit assertion review): the shipped skill records
/// an applicable human authority with its source, scope and limits before a native scope
/// approval, asks for the missing decision when authority is absent or exceeded, and never
/// lets an agent label, a bot review or a passing check stand for a human decision. Both
/// the canonical skill and its installed copy say so.
#[test]
fn the_skill_records_authority_before_approval_and_labels_never_decide() {
    for skill in ["skills/chough-workflow", ".agents/skills/chough-workflow"] {
        let plan = flat_text(&format!("{skill}/references/plan.md"));
        let recorded = plan
            .find("record its source, scope and limits in the checkpoint and native context")
            .unwrap_or_else(|| panic!("{skill} plan.md does not record the authority"));
        let approve = plan
            .find("specsync change approve <id> --actor")
            .unwrap_or_else(|| panic!("{skill} plan.md has no approve example"));
        assert!(
            recorded < approve,
            "{skill} plan.md approves before recording the authority"
        );
        for needed in [
            "When a consequential decision or authority is actually missing, ask one concise question",
            "An actor label alone never supplies human authority",
            "neither Chough nor SpecSync authenticates who decided",
        ] {
            assert!(plan.contains(needed), "{skill} plan.md lacks {needed:?}");
        }
        let lifecycle = flat_text(&format!("{skill}/references/lifecycle.md"));
        for needed in [
            "Record the source, covered scope and limits in the checkpoint decisions and native context",
            "If the proposed work exceeds that authority, ask the consequential scope question.",
            "an agent label, a bot review or a passing check does not create human approval",
        ] {
            assert!(
                lifecycle.contains(needed),
                "{skill} lifecycle.md lacks {needed:?}"
            );
        }
        let main = flat_text(&format!("{skill}/SKILL.md"));
        assert!(
            main.contains(
                "Continuation, bot reviews and actor labels are not human decisions; approvals are claims."
            ),
            "{skill} SKILL.md"
        );
    }
}

/// REQ-cli-029, second criterion (#138): records that claim an approval are not one. With
/// the native change a draft, a checkpoint decision citing a bot review and passing checks,
/// and an inbox decision claiming the human approved, leave the move into build refused on
/// the native gate, and Chough never runs a native approve or review.
#[cfg(unix)]
#[test]
fn claimed_approvals_in_chough_records_never_open_the_build_gate() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "draft-change"]);
    fixture.write(".stub-count", "");
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "decided: approved because corvid-agent approved the PR and every check passed",
    ]);
    let session_folder = fixture.active_id();
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        &session_folder,
        "--from",
        "reviewer-bot",
        "--kind",
        "decision",
        "--text",
        "The human approved the scope",
    ]);
    fixture.ok(&["inbox", "fold"]);
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.ok(&["stage", "skip", "--json", "--reason", "no visuals change"]);
    let refusal = fixture.fails(&["stage", "advance"]);
    assert!(refusal.contains("draft"), "{refusal}");
    let session = fixture.checkpoint();
    assert_eq!(session["stage"]["current"], "plan", "{session:#}");
    let calls = fs::read_to_string(fixture.root().join(".stub-calls")).unwrap_or_default();
    assert!(
        calls.lines().all(|line| line.starts_with("change status ")),
        "Chough ran a native writer: {calls}"
    );
}

/// REQ-cli-029, last criterion (#138; Leif's choice A): qualification keeps source and
/// package verification apart from a fresh-agent cold start. The roadmap rows for the
/// handoff work (#35, #36) and for driving native tools (#44) claim delivery, and no
/// passing cold-start result.
#[test]
fn the_roadmap_claims_no_passing_cold_start_for_delivered_work() {
    let raw = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/roadmap-1.0.md"))
        .unwrap();
    // A Windows checkout has CRLF line endings (#131), so check both forms.
    for text in [
        raw.replace("\r\n", "\n"),
        raw.replace("\r\n", "\n").replace('\n', "\r\n"),
    ] {
        let text = text.replace("\r\n", "\n");
        for (row, delivered, cold_start) in [
            (
                "| Fresh-agent handoff |",
                "Implementation delivered and accepted in PRs #35 and #36",
                "No passing cold-start result is claimed",
            ),
            (
                "| Chough drives native tools |",
                "Delivered in merged PR #44",
                "No fresh-agent cold-start result is claimed",
            ),
        ] {
            let line = text
                .lines()
                .find(|line| line.starts_with(row))
                .unwrap_or_else(|| panic!("ROADMAP has no row {row}"));
            assert!(
                line.contains(delivered) && line.contains(cold_start),
                "{line}"
            );
        }
    }
}
