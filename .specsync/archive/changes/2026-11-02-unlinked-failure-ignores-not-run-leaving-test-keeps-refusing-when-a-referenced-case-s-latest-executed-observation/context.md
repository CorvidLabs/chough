---
change: unlinked-failure-ignores-not-run-leaving-test-keeps-refusing-when-a-referenced-case-s-latest-executed-observation
artifact: context
---

# Context

0xLeif's review of PR #43 (inline P2 at src/stage.rs, discussion_r4445242804, 2026-11-02 9:59 AM MT) found that `unlinked_failure` takes the newest observation of any outcome. A case that failed at time 100 and then has only a `not-run` observation at time 200 is no longer refused: `next` says Continue and `stage advance` reaches accept with the failure unfixed, no pass and no linking bug. A retry that could not execute is no evidence for the "rerun until it passes or link a bug" remedy (FLOW-7).

Reproduced on Linux by the new regression test `a_later_not_run_keeps_an_unlinked_failure_refused` before the fix (`.chough/artifacts/bug43-notrun-before-fix.log`: `stage advance` unexpectedly succeeded).

This is a follow-up change on the same PR #43 branch after the issue #39 change was archived. PR #45 also edits src/stage.rs, so the diff stays minimal.
