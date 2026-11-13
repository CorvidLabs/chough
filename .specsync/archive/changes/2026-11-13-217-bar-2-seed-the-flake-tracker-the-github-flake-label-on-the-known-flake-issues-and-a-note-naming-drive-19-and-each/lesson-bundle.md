# Lesson bundle: 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: #217 bar 2: seed the flake tracker: the GitHub flake label on the known flake issues, and a note naming DRIVE-19 and each GitHub issue on the flake bug records
- **Kind**: Documentation
- **Paths**: docs/test-plan-1.0.md
- **Acceptance**: The flake label exists on CorvidLabs/chough and is on #187, #132, #169 and #247; the bug records for #187, #132 and #169 carry a Log note naming DRIVE-19 and their issue as GitHub #<n>, and the three local-only flake records a note naming DRIVE-19 and that no issue was filed; docs/test-plan-1.0.md bar 2 names the seeded issues instead of 'Known today: #187 and #132'

## Evidence

- Verification commit: `7b88703dfac84cfc52a2e547494c1a4b6f3ac6e2`
- Base commit: `38fe24c063e3afde49a5be5093cd69791bd06126`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

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

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
