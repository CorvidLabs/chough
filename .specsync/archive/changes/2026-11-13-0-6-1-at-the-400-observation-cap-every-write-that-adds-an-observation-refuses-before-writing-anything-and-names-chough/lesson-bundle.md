# Lesson bundle: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: 0.6.1: at the 400-observation cap, every write that adds an observation refuses before writing anything and names chough session shelve
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: CHANGELOG.md, docs/cli.md, src/cases.rs, src/checkpoint.rs, src/evidence.rs, src/stage.rs, tests/next.rs, tests/stages.rs
- **Acceptance**: At the 400-observation cap, chough cases run, cases record, evidence add and checkpoint --observation each refuse before running or writing anything, naming the bound and chough session shelve <id>, with every file unchanged; and at accept next takes a case latest run only from the observations the checkpoint references, so an unreferenced newer entry does not hide a stale run (red then green)

## Evidence

- Verification commit: `751561b67b4da4995466e91c54734434c8f2c0b2`
- Base commit: `9d53604071df50b537adccad0ccf5cac9a2fc32b`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Preparing Chough 0.6.0's review, the release session reached REQ-cli-032's bound of 400
observation references. From there `chough cases run` executed each case, wrote its
evidence entry, then refused with `too many observation (limit 400)`: 30 passing
entries were left unreferenced, and the message named no recovery, though REQ-cli-032
says such a refusal leaves every record byte-identical and names `chough session shelve
<id>`. `chough next` at accept then counted the unreferenced newer entries as the
cases' latest runs. The bug is recorded as
`.chough/bugs/2026-11-13-at-the-400-observation-cap-cases-run-writes-its-evidence-entry.md`,
deferred to 0.6.1 by Leif. On 2026-11-13T02:03Z he chose "Yes, start (Recommended)"
for 0.6.1 as six small PRs; this one and the leak check go first.

## From the change's design.md

# Design

Every write that adds an observation reference (`cases run`, `cases record`, `evidence
add`, `checkpoint --observation`) checks the cap before it runs a case or writes an
entry, and refuses naming the bound and `chough session shelve <id>`. `next` takes a
case's latest run only from the observations the checkpoint references. No contract
text changes: REQ-cli-032 and REQ-cli-021 already say this.

## From the change's testing.md

# Testing

A fixture session holding 400 observation references: `chough cases run`, `cases
record`, `evidence add` and `checkpoint --observation` each refuse naming the bound and
`chough session shelve <id>`, with every file byte-identical and no case run. At accept,
an unreferenced entry newer than a case's referenced run does not count as its latest
run. Red at the test commit, green at the fix.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-032 | `tests/cases.rs`: at 400 references, observation writes refuse before writing, naming `chough session shelve` |
| REQ-cli-021 | `tests/next.rs`: an unreferenced newer entry is not a case's latest run |

## Where these lessons go

- `specs/cli/cli.3md`
