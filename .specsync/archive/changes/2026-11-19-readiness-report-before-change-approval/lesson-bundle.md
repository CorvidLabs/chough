# Lesson bundle: readiness-report-before-change-approval

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Readiness report before change approval
- **Kind**: Feature
- **Specs**: cli
- **Paths**: hi/drive.md, INTENT.md, src/main.rs, src/change.rs, src/ready.rs, tests/ready.rs, tests/package.rs, tests/skill_releases.rs, tests/fixtures/skill-releases/manifest.json, .chough/cases/cli.change-ready.cases.toml, CHANGELOG.md, docs/cli.md, README.md, specs/cli/requirements.md, specs/cli/cli.spec.md, skills/chough-workflow/references/stages.md, skills/chough-workflow/references/plan.md, .agents/skills/chough-workflow/references/stages.md, .agents/skills/chough-workflow/references/plan.md, src/skill.rs, changelog.d/2026-11-14-readiness-report-before-change-approval.md
- **Acceptance**: chough change ready exits 0 when nothing is missing and 1 when a finding remains, and each finding names the command that fixes it.

## Evidence

- Verification commit: `75b6bf6e3dc87965576e74a004fbfd5397588e2a`
- Base commit: `cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

On 2026-11-14, local-model agents wrote correct code fixes and then skipped the
session, the change, the delta or the commit. The rules they missed are ones
SpecSync states only when a later command refuses: a delta for every affected
spec, a delta still written against the canonical text it was approved on, every
export in a declared file documented in the spec, a case that links only its own
module's requirements, scope frozen at approval, and an interview or artifact
left unfinished. Change ids are long and must be typed exactly, so a second
attempt often repeats the same miss.

DRIVE-29, captured from Leif's choice "Capture as written" in chat on
2026-11-14, asks for one read-only command before approval. `chough change ready
<id>` prints every miss in one report and names the exact command that fixes
each one. It does not approve, check or rewrite anything.

The command is new. `chough change new`, `answer`, `check`, `status` and `ship`
stay SpecSync passthroughs (REQ-cli-057, REQ-cli-059). The cli spec's `files:`
list gains `src/ready.rs` and `tests/ready.rs`, which do not exist until the
build. README's helper table and `docs/cli.md`'s command table must name the
new subcommand, because those tables are checked against `chough --help`.

## From the change's design.md

# Design

Nothing visual changes. The command prints a text report to standard output and
a short refusal to standard error only when the change cannot be read. There is
no prompt, no colour contract and no second screen.

One finding is one paragraph or one bullet. It names what is missing, the
module or path, and one exact command or write that fixes it. The real change
id is filled in. The report ends with a count of findings, or with a line that
nothing is missing. Exit 0 means the count is zero.

`new`, `answer`, `check`, `status` and `ship` keep the output they have today.
`ready` is not one of those passthroughs.

## From the change's testing.md

# Testing

`tests/ready.rs` covers REQ-cli-067 before the implementation is treated as
done:

- A draft missing its delta, with an extra delta, an unanswered question and a
  TODO artifact exits 1, and the report names the fix for each. The same tree
  with those fixed exits 0, and no tracked file and no hash cache changes.
- A delta approved against a requirement, then left unchanged while that
  requirement's canonical text changes, is named. A delta rewritten onto the
  new text is not. A delta that already carries the canonical move is not.
- An undocumented export in a declared file is named as `specsync check --spec`
  names it. An export in a file the change does not declare is not.
- A changed case that links another module's requirement is named with the
  owning module. A case that links only its own module is not.
- A changed file outside the declared paths is named. A file under a declared
  prefix is not. A file under `verification_ignored_paths` is not. On a draft
  the fix is `chough change answer` with the full path list.
- An unknown change id exits 1 and names `chough change status`.

`.chough/cases/cli.change-ready.cases.toml` binds those tests to REQ-cli-067
under module `cli`. `tests/package.rs` asserts the fast path names
`chough change ready <id>` between the delta phrase and the scope-approval
phrase, and still fits in 32 lines.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-067 | `tests/ready.rs` as listed above, the case file `.chough/cases/cli.change-ready.cases.toml`, and the acceptance criteria on the added requirement |

## Where these lessons go

- `specs/cli/cli.3md`
