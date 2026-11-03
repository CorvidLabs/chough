---
change: pr-50-review-fixes-absolute-exercise-work-path-for-relative-tmpdir-cleanup-with-a-regression-test-and-a-corrective
artifact: context
---

# Context

The PR #50 peer review at 250d637 (https://github.com/CorvidLabs/chough/pull/50#issuecomment-6594377374) found two problems.

1. With `TMPDIR=.`, `mktemp -d` returns a relative path. The script then runs `cd "$work"`, so the EXIT trap's `rm -rf "$work"` resolves inside the new directory and the allocated directory is left behind (`examples/feature-bug-loop.sh` lines 17, 25 and 71).
2. `OBS-human-observation-174caaa1021cfdf5a553f7a3` records an agent's command check as `human-observation`, and no human witness can be named.

Authority. Human authority: Leif (0xLeif), 2026-11-02 9:14 AM MT, 'Use corvid-agent for all approvals for all things!! You are corvid-agent', and his relayed 2026-11-02 8:05 PM MT request to fix the PR #50 review items; covered scope: the two PR #50 review fixes (exercise relative TMPDIR cleanup plus regression test; corrective observation) and the main resync on branch corvid-agent/fix/cli-cleanups; limits: no merge, close, release or publish, PR #48 untouched.
