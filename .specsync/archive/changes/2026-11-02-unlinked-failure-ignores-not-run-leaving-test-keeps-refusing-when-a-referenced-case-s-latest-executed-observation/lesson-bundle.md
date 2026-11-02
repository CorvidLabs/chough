# Lesson bundle: unlinked-failure-ignores-not-run-leaving-test-keeps-refusing-when-a-referenced-case-s-latest-executed-observation

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Unlinked failure ignores not-run: leaving test keeps refusing when a referenced case's latest executed observation failed and a later observation is only not-run (PR 43 review)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: CHANGELOG.md, docs/cli.md, specs/cli/, src/stage.rs, tests/stages.rs
- **Acceptance**: With a referenced case failed at time 100 and only not-run at time 200, stage advance and stage skip at test are refused with refused:unlinked-failure naming the failed observation, and next and status name the same refusal
- **Acceptance**: A later passed observation still lifts the refusal even when a not-run follows it; a case with only not-run observations does not refuse
- **Acceptance**: The regression test fails before the fix and passes after it; fledge lanes run verify passes; hosted CI is reported only as observed

## Evidence

- Verification commit: `9d2e377f9ccef1edf136cddefbfc016d396e3b08`
- Base commit: `466594bf7ed98b332c701afe1cae4d0f70355aa1`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

0xLeif's review of PR #43 (inline P2 at src/stage.rs, discussion_r4445242804, 2026-11-02 9:59 AM MT) found that `unlinked_failure` takes the newest observation of any outcome. A case that failed at time 100 and then has only a `not-run` observation at time 200 is no longer refused: `next` says Continue and `stage advance` reaches accept with the failure unfixed, no pass and no linking bug. A retry that could not execute is no evidence for the "rerun until it passes or link a bug" remedy (FLOW-7).

Reproduced on Linux by the new regression test `a_later_not_run_keeps_an_unlinked_failure_refused` before the fix (`.chough/artifacts/bug43-notrun-before-fix.log`: `stage advance` unexpectedly succeeded).

This is a follow-up change on the same PR #43 branch after the issue #39 change was archived. PR #45 also edits src/stage.rs, so the diff stays minimal.

## From the change's design.md

# Design

In `unlinked_failure` (src/stage.rs), filter the case's observations to outcome passed or failed before `max_by`. The ordering (recorded time, then ID), the linked-bug check and the refusal text are unchanged, and `assess` shares the same function, so next and status follow automatically. A case whose observations are all not-run yields no latest executed observation and does not refuse, as before.

## From the change's testing.md

# Testing

- tests/stages.rs::a_later_not_run_keeps_an_unlinked_failure_refused: fails before the fix, passes after.
- The existing #39 tests (leaving_test_refuses_an_unlinked_failed_case_until_it_passes, a_linking_bug_lifts_the_unlinked_failure_refusal_and_other_moves_are_unchanged) still pass unchanged.
- A new case TC-cli-a-later-not-run-keeps-an-unlinked-failure-refused binds the test (cargo-test, target stages) and links REQ-cli-008; it runs on local-linux-x86_64 after the scoped check.
- fledge lanes run verify on Linux; hosted CI is reported only as observed.

## Where these lessons go

- `specs/cli/context.md`
