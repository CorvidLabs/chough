---
kind: bug
id: 2026-11-12-a-gate-close-s-next-action-outlives-the-stage-move-it-named
aliases: []
title: A gate close's next action outlives the stage move it named
status: verified
classification: product-defect
scope: in-scope
impact: An agent following it leaves build with nothing built (t38n F1, high)
criteria: []
requirements: [REQ-cli-021]
cases: [TC-cli-a-stage-move-replaces-a-gate-close-next-action]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-a-stage-move-replaces-a-gate-close-next-action
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-a-stage-move-replaces-a-gate-close-next-action-ee36e0
receipts: []
environment: local-macos-arm64
revision: 00f8f8d8956ffdc798e2ca66854ec876cb035cc9
found_by: claude-code (hc t382) (t38n's fourth fresh-eyes round report, F1 (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:39:09Z
updated: 2026-11-12T23:47:46Z
---
# A gate close's next action outlives the stage move it named

**What happened:** After the scope gate closes, the next action reads 'Gate … closed (approved); run chough stage advance to move on'; after that advance into build, chough next still says 'Continue build: … run chough stage advance', and the same happens at intent then design and after chough change ship into done
Actual: Build says to run chough stage advance

**Steps:** Trigger: Close a gate whose next action names a move, then make the move and run chough next
1. Close the scope gate with the human's approval
2. chough stage advance (plan to build)
3. chough next

**Expected:** The move replaces that next action with Continue build; next derives the step from build

**Why it matters:** An agent following it leaves build with nothing built (t38n F1, high)

**Fix:** 3f28a915: a move to another stage replaces a gate close's next action with Continue <stage>, and next shows Continue <stage> once (REQ-cli-021)

## Log

- 2026-11-12T23:39:09Z · Reported (t38n's fourth fresh-eyes round report, F1 (relayed by t370)); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-a-stage-move-replaces-a-gate-close-next-action · by claude-code (hc t382)
- 2026-11-12T23:39:09Z · Added despite similar reports: The verified bugs made a gate close set the step its decision allows; this one is that step outliving the move it named, so build says to advance with nothing built · by claude-code (hc t382)
- 2026-11-12T23:47:46Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-a-stage-move-replaces-a-gate-close-next-action-ee36e0; fix: 3f28a915: a move to another stage replaces a gate close's next action with Continue <stage>, and next shows Continue <stage> once (REQ-cli-021). Red at 00f8f8d8, green at 3f28a915 · by claude-code (hc t382)
