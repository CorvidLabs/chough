---
kind: bug
id: 2026-11-12-gate-record-on-a-gate-that-drives-approval-reports-only-closed
aliases: []
title: gate record on a gate that drives approval reports only closed, and the SpecSync approval note names chough checkpoint --close-gate
status: open
classification: product-defect
scope: in-scope
impact: The record misstates how the approval ran
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
created: 2026-11-12T16:21:40Z
updated: 2026-11-12T16:21:40Z
---
# gate record on a gate that drives approval reports only closed, and the SpecSync approval note names chough checkpoint --close-gate

**What happened:** chough gate record closing an approve-driving gate does not report the native step it ran, and the approval note says through chough checkpoint --close-gate
Actual: Only closed, and a note naming the other command

**Steps:** chough gate record <list> --gates <n> on a scope gate

**Expected:** It reports the native step it ran, and the note names gate record

**Why it matters:** The record misstates how the approval ran

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:40Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
