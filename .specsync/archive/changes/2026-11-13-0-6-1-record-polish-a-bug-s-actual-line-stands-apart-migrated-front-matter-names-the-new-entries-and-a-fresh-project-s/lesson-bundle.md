# Lesson bundle: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: 0.6.1: record polish: a bug's Actual line stands apart, migrated front matter names the new entries, and a fresh project's records README and the README's init paragraph are trimmed
- **Kind**: BugFix
- **Specs**: cli, records, history
- **Paths**: CHANGELOG.md, README.md, docs/cli.md, src/bugs.rs, src/migrate.rs, src/sessions.rs, tests/bugs.rs, tests/docs.rs, tests/evidence.rs, tests/migrate_evidence.rs, tests/records.rs
- **Acceptance**: A bug body gives its Actual line as its own paragraph; a migrated session and bug name the carried entries in front matter, keeping each 0.5.x ID as an alias; a fresh project .chough/README.md names no 0.5.x folder; and the README init paragraph has no long parenthetical about hosts (red then green)

## Evidence

- Verification commit: `71b02c952046ad6739504ee0be42bd7a0e693bc1`
- Base commit: `9d53604071df50b537adccad0ccf5cac9a2fc32b`
- Verified by: `specsync check --spec cli --spec history --spec records`

## From the change's context.md

# Context

Chough 0.6.0's fourth fresh-eyes round (t38n) found three record polish items, deferred to
0.6.1 by Leif:
- F8: a bug body renders "**What happened:** <symptom>" with "Actual: <a>" on the next
  line, which Markdown joins into one paragraph.
- F9: migrated records keep 0.5.x observation IDs in front matter while the body links
  the new file.
- F11: a fresh project's `.chough/README.md` lists `artifacts/` as "kept from Chough
  0.5.x", and the README's init paragraph carries a 10-line parenthetical about hosts.

On 2026-11-13T02:03Z Leif chose "Yes, start (Recommended)" for 0.6.1 as six small PRs;
this is one. Bugs: `.chough/bugs/2026-11-12-a-bug-body-runs-its-symptom-and-its-actual-line-into-one.md`,
`.chough/bugs/2026-11-12-migrated-records-keep-old-observation-ids-in-their-front.md` and
`.chough/bugs/2026-11-12-a-fresh-project-s-records-readme-names-artifacts-as-kept-from.md`.

## From the change's design.md

# Design

The bug body gives `Actual:` its own paragraph. Migration writes the carried entries'
new names in a session's `observations` and a bug's `evidence`, with each 0.5.x ID kept
as the entry's alias. The records README a fresh project gets names no 0.5.x folder.
The README's init paragraph moves its host list to a short sentence. No contract text
changes.

## From the change's testing.md

# Testing

`tests/bugs.rs`: a bug's Actual line is its own paragraph. `tests/migrate_evidence.rs`: a
migrated session's `observations` and a bug's `evidence` name the new entries, the old
IDs resolving as aliases. `tests/records.rs`: a fresh project's reading guide names no 0.5.x folder, and
`tests/docs.rs`: the README's init paragraph is one sentence. Red at the test commits, green at the fixes.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-012 | `tests/bugs.rs`: a bug body's Actual line is its own paragraph |
| REQ-records-009 | `tests/migrate_evidence.rs`: migrated front matter names the new entries, keeping old IDs as aliases |
| REQ-records-002 | `tests/records.rs`: a fresh project's reading guide names no folder kept from 0.5.x |
| REQ-cli-016 | `tests/docs.rs`: the README's init paragraph names the host directories without a long parenthetical |

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/records/context.md`
- `specs/history/context.md`
