# Lesson bundle: expect-each-archive-own-modules-in-the-approval-guard-test-so-a-history-only-archive-passes

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Expect each archive own modules in the approval guard test so a history-only archive passes
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/stages.rs
- **Acceptance**: the_guard_accepts_every_archived_approval_in_this_repository passes with the history-only #96 archive; it still requires unchanged deltas naming only modules in each archive approved scope, or no semantic deltas; it fails before the change on this branch; fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `537e9a00ed220a7dcd2cfe6aff99d6aff5519c89`
- Base commit: `322a2e134fac0e4290b53b7dfa211fef6a6bde0c`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

`the_guard_accepts_every_archived_approval_in_this_repository` in tests/stages.rs
replays every archived approval ledger in this repository through `chough next`. It
asserted that `approved_deltas` starts with `unchanged: cli`, because every archive
until now had a cli delta. #96's change touches only the history module, so once it was
archived the test read `unchanged: history`, and hosted CI on #96 failed. #99's
history-only change would fail the same way.

The assertion now requires `unchanged:` followed only by modules in that archive's
own approved scope, or exactly `no semantic deltas (no_spec_change)`. That is stricter
per archive than before, and no module outside the approved scope is accepted. Leif
(0xLeif) said "Yes, record and merge" for PRs #90–#97 and #99 on 2026-11-04.

## From the change's testing.md

# Testing

On this branch, before the change,
`cargo test --locked --test stages the_guard_accepts_every_archived_approval_in_this_repository`
fails with "remove-the-history-events-…: unchanged: history", as hosted CI did on #96.
After the change it passes. `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
