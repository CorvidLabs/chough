---
kind: bug
id: 2026-11-12-gate-record-on-a-gate-that-drives-approval-reports-only-closed
aliases: []
title: gate record on a gate that drives approval reports only closed, and the SpecSync approval note names chough checkpoint --close-gate
status: verified
classification: product-defect
scope: blocking
impact: The record misstates how the approval ran
criteria: []
requirements: []
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-record-reports-the-approval-it-ran
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-record-reports-the-approval-it-ran-a49bfe
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:40Z
updated: 2026-11-12T17:41:51Z
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
- 2026-11-12T17:37:49Z · status: open -> investigating; scope: in-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-record-reports-the-approval-it-ran. Reproduced: t38b's red test (799bb15e, cherry-picked as 0b3cf9f0) failed at c7cefe40: closed[0].native_step was null where the approval it ran was expected · by claude-code (hc t382)
- 2026-11-12T17:41:51Z · status: investigating -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-record-reports-the-approval-it-ran-a49bfe. Green after t38b's fix 620fcc52 (cherry-picked as 37ebc0fa): gate record's closed entries carry each native_step, and the approval note names chough gate record <list> when the relay came through one · by claude-code (hc t382)
