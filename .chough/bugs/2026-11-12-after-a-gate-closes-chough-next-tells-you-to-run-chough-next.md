---
kind: bug
id: 2026-11-12-after-a-gate-closes-chough-next-tells-you-to-run-chough-next
aliases: []
title: After a gate closes, chough next tells you to run chough next
status: verified
classification: product-defect
scope: in-scope
impact: A fresh agent loops instead of advancing (t38k F3)
criteria: []
requirements: [REQ-cli-021]
cases: [TC-cli-a-gate-close-names-the-step-its-decision-allows]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-a-gate-close-names-the-step-its-decision-allows
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-a-gate-close-names-the-step-its-decision-allows-3c3207
receipts: []
environment: local-macos-arm64
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T21:02:34Z
updated: 2026-11-12T21:04:01Z
---
# After a gate closes, chough next tells you to run chough next

**What happened:** A gate close sets the next action 'Gate … closed (…); run chough next for the next action.', and chough next then repeats it, a loop
Actual: run chough next for the next action

**Steps:** Trigger: Close the gate holding the stage, then run chough next
1. Close a scope gate with the human's approval
2. chough next

**Expected:** The step the decision allows: stage advance, change ship, stage back --to build, or checkpoint --next-action

**Why it matters:** A fresh agent loops instead of advancing (t38k F3)

**Fix:** bf257b07 (t37f's 78d0f1f7): a gate close sets the next action to the step its decision allows: stage advance, change ship, stage back --to build, or checkpoint --next-action (REQ-cli-021)

## Log

- 2026-11-12T21:02:34Z · Reported (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-a-gate-close-names-the-step-its-decision-allows · by claude-code (hc t382)
- 2026-11-12T21:04:01Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-a-gate-close-names-the-step-its-decision-allows-3c3207; fix: bf257b07 (t37f's 78d0f1f7): a gate close sets the next action to the step its decision allows: stage advance, change ship, stage back --to build, or checkpoint --next-action (REQ-cli-021). Red at 5195bd67, green at cca98758 · by claude-code (hc t382)
