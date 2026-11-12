---
kind: bug
id: 2026-11-12-chough-cases-check-fails-with-a-raw-no-such-file-or-directory
aliases: []
title: chough cases check fails with a raw No such file or directory when the module has no requirements.md yet, though the approved delta adds the requirement
status: open
classification: product-defect
scope: in-scope
impact: Blocks failing-first through Chough on the first change to a new module; the workaround is specsync change check to materialize first
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
created: 2026-11-12T16:21:39Z
updated: 2026-11-12T16:21:39Z
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
