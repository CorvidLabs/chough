---
kind: bug
id: 2026-11-12-chough-status-cuts-its-next-action-mid-command-and-the
aliases: []
title: chough status cuts its next action mid-command, and the checkpoint's next action goes stale after a gate closes
status: verified
classification: product-defect
scope: blocking
impact: Misleading next steps
criteria: []
requirements: []
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-status-gives-the-next-action-whole
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-close-replaces-a-stale-next-action
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-status-gives-the-next-action-whole-6c7235
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-close-replaces-a-stale-next-action-188881
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:41Z
updated: 2026-11-12T17:45:15Z
---
# chough status cuts its next action mid-command, and the checkpoint's next action goes stale after a gate closes

**What happened:** chough status summary.next_valid_action.action ends in the middle of a command (chough stage  [cut]) in the full view; after a gate closes, the recorded next action still says to present the scope
Actual: A cut action, and a stale next action

**Steps:** chough status during a cycle; closing a gate

**Expected:** The full view gives the whole action, and closing a gate prompts or updates the next action

**Why it matters:** Misleading next steps

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:41Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
- 2026-11-12T17:43:16Z · status: open -> investigating; scope: in-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-status-gives-the-next-action-whole, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-close-replaces-a-stale-next-action. Reproduced: t37f's red tests (07584d94, cherry-picked as 92e68394) failed at dd3e2ab5: status cut the next action, and a gate close left the next action the agent set while the gate was open · by claude-code (hc t382)
- 2026-11-12T17:45:15Z · status: investigating -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-status-gives-the-next-action-whole-6c7235, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-close-replaces-a-stale-next-action-188881. Green after t37f's fix dabc933f (cherry-picked as 7be760cc): status gives its next action whole; closing the last gate holding the stage, with no --next-action in the same call, replaces the next action with the gate's outcome and chough next · by claude-code (hc t382)
