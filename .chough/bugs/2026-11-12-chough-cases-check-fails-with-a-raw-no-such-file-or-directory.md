---
kind: bug
id: 2026-11-12-chough-cases-check-fails-with-a-raw-no-such-file-or-directory
aliases: []
title: chough cases check fails with a raw No such file or directory when the module has no requirements.md yet, though the approved delta adds the requirement
status: verified
classification: product-defect
scope: blocking
impact: Blocks failing-first through Chough on the first change to a new module; the workaround is specsync change check to materialize first
criteria: []
requirements: []
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-cases-check-a-module-without-requirements-md
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-cases-check-a-module-without-requirements-md-acf483
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:39Z
updated: 2026-11-12T17:45:15Z
---
# chough cases check fails with a raw No such file or directory when the module has no requirements.md yet, though the approved delta adds the requirement

**What happened:** After the scope approval of a change whose delta adds REQ-tally-001 to a module with only tally.spec.md, chough cases check fails: contract requires its native requirements.md companion: No such file or directory (os error 2)
Actual: A raw IO error; with an empty requirements.md the same command lists both links under approved_delta_requirements

**Steps:** The first change to a module scaffolded without requirements.md, with a case linking a requirement only the approved delta adds

**Expected:** The link is accepted under approved_delta_requirements, as the REQ-cli acceptance criterion and docs/cli.md (Approved delta links) say

**Why it matters:** Blocks failing-first through Chough on the first change to a new module; the workaround is specsync change check to materialize first

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:39Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
- 2026-11-12T17:43:16Z · status: open -> investigating; scope: in-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-cases-check-a-module-without-requirements-md. Reproduced: t37f's red test (2fdf5300, cherry-picked as b4c10774) failed at dd3e2ab5: cases check refused the case with the missing requirements.md companion · by claude-code (hc t382)
- 2026-11-12T17:45:15Z · status: investigating -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-cases-check-a-module-without-requirements-md-acf483. Green after t37f's fix ad859114 (cherry-picked as cee8d481): a missing requirements.md holds no requirement, so the approved delta resolves the link; a link it does not add is refused as before · by claude-code (hc t382)
