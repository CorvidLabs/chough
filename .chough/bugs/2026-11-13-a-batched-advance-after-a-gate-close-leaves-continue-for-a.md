---
kind: bug
id: 2026-11-13-a-batched-advance-after-a-gate-close-leaves-continue-for-a
aliases: []
title: A batched advance after a gate close leaves Continue for a stage it passed through
status: verified
classification: product-defect
scope: in-scope
impact: A stale next action at the stage reached, in the round-4 F1 fix itself
criteria: []
requirements: [REQ-cli-021]
cases: [TC-cli-a-batched-advance-continues-in-the-stage-reached]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-13-tc-cli-a-batched-advance-continues-in-the-stage-reached
  - 2026-11-12-release-0-6-0/2026-11-13-tc-cli-a-batched-advance-continues-in-the-stage-reached-b5f4d8
receipts: []
environment: local-macos-arm64
revision: 11a54de9b19220c0ffe73a263de91f652ef11278
found_by: "claude-code (hc t382) (Found by t382 preparing Leif's review after round 4: chough stage advance --to accept printed 'Next action: Continue test')"
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-13T00:04:51Z
updated: 2026-11-13T00:07:56Z
---
# A batched advance after a gate close leaves Continue for a stage it passed through

**What happened:** After a gate close, chough stage advance --to accept made two steps; the first replaced the close's next action with Continue test, and the second kept it, so at accept the next action still read Continue test
Actual: Continue <the first stage passed through>

**Steps:** Trigger: Close a gate whose next action names a move, then advance several stages at once
1. Close the scope gate with the human's approval
2. chough stage advance --to test
3. Read the checkpoint's next action

**Expected:** Continue <the stage reached>

**Why it matters:** A stale next action at the stage reached, in the round-4 F1 fix itself

**Fix:** e1ada30f: each step of a batched advance replaces the Continue <stage> the step before it left, so the next action names the stage reached (REQ-cli-021)

## Log

- 2026-11-13T00:04:51Z · Reported (Found by t382 preparing Leif's review after round 4: chough stage advance --to accept printed 'Next action: Continue test'); evidence: 2026-11-12-release-0-6-0/2026-11-13-tc-cli-a-batched-advance-continues-in-the-stage-reached · by claude-code (hc t382)
- 2026-11-13T00:04:51Z · Added despite similar reports: The verified round-4 F1 bug replaced a gate close's next action at the move it named; this one is a batched advance, whose later steps kept the first step's Continue <stage> · by claude-code (hc t382)
- 2026-11-13T00:07:56Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-13-tc-cli-a-batched-advance-continues-in-the-stage-reached-b5f4d8; fix: e1ada30f: each step of a batched advance replaces the Continue <stage> the step before it left, so the next action names the stage reached (REQ-cli-021). Red at 11a54de9, green at e1ada30f · by claude-code (hc t382)
