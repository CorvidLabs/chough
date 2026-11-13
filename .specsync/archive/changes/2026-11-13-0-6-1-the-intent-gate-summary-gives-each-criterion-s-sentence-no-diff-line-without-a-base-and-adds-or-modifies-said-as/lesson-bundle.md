# Lesson bundle: 0-6-1-the-intent-gate-summary-gives-each-criterion-s-sentence-no-diff-line-without-a-base-and-adds-or-modifies-said-as

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: 0.6.1: the intent gate summary gives each criterion's sentence, no diff line without a base, and adds or modifies said as such
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/gates.rs, tests/gates.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: At an intent gate with no native change, chough gate list gives each referenced criterion by ID with its sentence from the HI file that holds it and no diff line; a delta that only adds says adds, and one that adds and modifies names each (red then green)

## Evidence

- Verification commit: `a5202ffec0de488819b78ee79037bbdb8b363498`
- Base commit: `9d53604071df50b537adccad0ccf5cac9a2fc32b`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

In Chough 0.6.0's fourth fresh-eyes round (t38n, F7), the intent gate's summary gave
criterion IDs ("Intent: TALLY-2", "hi/tally.md: TALLY-2") without the sentences being
approved, printed "Diff: unknown, with no merge base and no recorded base commit", and a
delta that only adds said "adds or modifies". The bug is recorded as
`.chough/bugs/2026-11-12-the-intent-gate-summary-names-criterion-ids-without-their.md`,
deferred to 0.6.1 by Leif. On 2026-11-13T02:03Z he chose "Yes, start (Recommended)" for
0.6.1 as six small PRs; this is one.

## From the change's testing.md

# Testing

`tests/gates.rs`: an intent gate with no native change lists each referenced criterion's
ID and sentence and no diff line; a delta that only adds says adds, and one that adds
and modifies names each; the 24-line and 4,096-byte bounds hold and the digest is
stable. Red at the test commit, green at the fix.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-061 | `tests/gates.rs`: the intent gate summary gives each criterion's sentence and no diff line; a delta's requirements are said as added or modified |

## Where these lessons go

- `specs/cli/cli.3md`
