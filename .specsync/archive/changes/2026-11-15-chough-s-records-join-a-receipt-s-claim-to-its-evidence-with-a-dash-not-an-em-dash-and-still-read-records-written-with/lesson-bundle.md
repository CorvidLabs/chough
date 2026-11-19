# Lesson bundle: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Chough's records join a receipt's claim to its evidence with a dash, not an em dash, and still read records written with the old join (#278)
- **Kind**: BugFix
- **Specs**: history, records, cli
- **Paths**: src/sessions.rs, src/journal.rs, src/gates.rs, tests/sessions.rs, tests/migrate_evidence.rs, tests/journal.rs, .chough/cases/history.archived-stories.cases.toml, CHANGELOG.md, docs/cli.md, specs/history/requirements.md, specs/records/requirements.md, specs/cli/requirements.md, specs/history/history.spec.md, specs/records/records.spec.md, specs/cli/cli.spec.md
- **Acceptance**: An archived or migrated story's What was verified line and a gate summary's Change line hold no em dash: each joins with ' - ',A journal post derived from a story archived with the earlier ' — ' join still gives the line's claim alone and copies no evidence ID

## Evidence

- Verification commit: `6e740be091d31ce05c70e6fcce098b808e7a8fb3`
- Base commit: `cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8`
- Verified by: `specsync check --spec cli --spec history --spec records`

## From the change's context.md

# Context

Issue #278: an archived story's What was verified lines joined each claim to its command and fingerprint digits with ` — ` (`src/sessions.rs`, `receipt_line`). raven's Trust lane lints every tracked Markdown file for em dashes in prose, so archiving a session with `chough doctor --fix` failed CorvidLabs/raven#721. It is the same class of problem as #13, but in generated records. The gate summary's `Change:` line used the same join.

The code change was written by Flint 40B (local, Ollama) through Pi in the #278 bake-off, commit `8d1b298` on `bake2/278-flint`; it was the only run that kept old stories readable. Claude Code brought it onto main, opened this session and native change, and adds the legacy-read test the bake-off grading asked for.

## From the change's design.md

# Design

`receipt_line` (`src/sessions.rs`) and the gate summary's `Change:` line (`src/gates.rs`) join with ` - `. `claim_of` (`src/journal.rs`) cuts a line at its join: the first ` - ` or ` — ` that is followed by the backticked command or the 12 fingerprint digits `receipt_line` writes after it. A journal post therefore reads a story archived before this change without rewriting it, and a ` - ` inside a claim stays in the claim. No visual or interaction change.

## From the change's testing.md

# Testing

`tests/sessions.rs` and `tests/migrate_evidence.rs` expect a What was verified line joined with ` - ` at archiving and after migration. A unit test in `src/journal.rs` gives `claim_of` a line joined with ` — ` and one joined with ` - ` and expects the same claim from both; it fails without the legacy read. A case in `.chough/cases/history.archived-stories.cases.toml` binds the archiving test to REQ-history-006. Checks: `fledge run fmt`, `fledge run test`, `fledge run lint`, `fledge lanes run verify`, and `chough change check`.

## Where these lessons go

- `specs/history/context.md`
- `specs/records/context.md`
- `specs/cli/cli.3md`
