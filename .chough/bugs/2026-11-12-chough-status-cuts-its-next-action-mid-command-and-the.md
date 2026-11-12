---
kind: bug
id: 2026-11-12-chough-status-cuts-its-next-action-mid-command-and-the
aliases: []
title: chough status cuts its next action mid-command, and the checkpoint's next action goes stale after a gate closes
status: open
classification: product-defect
scope: in-scope
impact: Misleading next steps
criteria: []
requirements: []
cases: []
evidence: []
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:41Z
updated: 2026-11-12T16:21:41Z
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
