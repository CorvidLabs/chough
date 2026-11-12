---
kind: bug
id: 2026-11-12-every-stage-move-prints-an-authority-line-saying-chough-never
aliases: []
title: Every stage move prints an authority line saying Chough never writes native approval or review, which 0.6.0 gates do
status: verified
classification: product-defect
scope: blocking
impact: Misleads agents and humans about who approves
criteria: []
requirements: []
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-stage-move-authority-line-says-when-chough-runs-native
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-stage-move-authority-line-says-when-chough-runs-native-42c872
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
# Every stage move prints an authority line saying Chough never writes native approval or review, which 0.6.0 gates do

**What happened:** stage advance output carries authority: Native SpecSync status decides lifecycle gates; Chough never writes native approval or review
Actual: An untrue statement since #236

**Steps:** Any chough stage move

**Expected:** The line says that a gate decision runs native approve or review as the configured human, a relayed claim

**Why it matters:** Misleads agents and humans about who approves

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:40Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
- 2026-11-12T17:37:49Z · status: open -> investigating; scope: in-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-stage-move-authority-line-says-when-chough-runs-native. Reproduced: t38b's red test (0b46193b, cherry-picked as c69e1985) failed at c7cefe40: the authority line still said Chough never writes native approval or review · by claude-code (hc t382)
- 2026-11-12T17:41:51Z · status: investigating -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-stage-move-authority-line-says-when-chough-runs-native-42c872. Green after t38b's fix 552ab143 (cherry-picked as dd927968): the authority line says Chough runs a native approval or review only when a human's decision closes a gate that drives one, as that configured human · by claude-code (hc t382)
