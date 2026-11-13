---
change: 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each
artifact: context
---

# Context

docs/test-plan-1.0.md bar 2 already says what the flake tracker is: a GitHub `flake`
label plus one bug record per flake in `.chough/bugs/`. The label did not exist yet.
The flake-window check (REQ-cli-062, a separate PR) blocks the window while a `flake`
issue has no bug record naming it as `GitHub #<n>`. So seeding the tracker means
three things: create the label, label the known flake issues, and add a note to each
flake record naming DRIVE-19 and its issue.

This is the first PR in ch10-flake's #217 design, which Leif answered on 2026-11-13 at
03:30Z (relayed by t370). It adds no contract text.

What was found:
- #187 and #132 are closed. Their records are
  `2026-11-07-kill-zero-reports-a-killed-unreaped-zombie-alive-187` and
  `2026-11-11-verify-lane-test-step-fails-intermittently-under-load-green-on`.
- #169, closed, was the timing-trace flake that DRIVE-19 was drafted for. Its record
  is `2026-11-06-timing-trace-test-flakes-under-load-when-paired-runs-cross-a`.
- #247 is open: the exercise same-second race, which t37f is fixing. t382 added its
  `GitHub #247` note on the fix branch.
- Three flakes were only ever recorded locally:
  `2026-11-12-stopping-a-command-can-leave-a-child-it-forked-at-that-moment`,
  `2026-11-02-timing-native-budget-regression-sometimes-records-no-initial` and
  `2026-11-07-full-macos-lane-checkpoint-writer-lock-timeout`.

t382 owns the records. `bug update` has no criterion flag, so the link goes in a
`--note`.
