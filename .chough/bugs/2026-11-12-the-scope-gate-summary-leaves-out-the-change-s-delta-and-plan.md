---
kind: bug
id: 2026-11-12-the-scope-gate-summary-leaves-out-the-change-s-delta-and-plan
aliases: []
title: The scope-gate summary leaves out the change's delta and plan
status: verified
classification: requirement-gap
scope: blocking
impact: The human approves a scope the summary does not show
criteria: []
requirements: [REQ-cli-061]
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-summary-shows-the-scope
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-summary-shows-the-scope-ad7787
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:40Z
updated: 2026-11-12T17:20:19Z
---
# The scope-gate summary leaves out the change's delta and plan

**What happened:** chough gate list at plan lists only docs/FRD.md under What changed; the delta and plan under .specsync/changes/, the scope being approved, are left out
Actual: REQ-cli-061 leaves out SpecSync's lifecycle bookkeeping, which takes the change folder with it

**Steps:** chough gate list with a scope gate open at plan

**Expected:** The summary shows what the human approves: the delta and the plan

**Why it matters:** The human approves a scope the summary does not show

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:40Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
- 2026-11-12T16:48:39Z · status: open -> investigating; scope: in-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-summary-shows-the-scope. Reproduced: red at da98f439, TC-cli-gate-summary-shows-the-scope failed; at a scope gate the summary named no file under the change's folder. In scope (Leif, 16:26Z, relayed by t370) · by claude-code (hc t382)
- 2026-11-12T17:20:19Z · status: investigating -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-summary-shows-the-scope-ad7787. Green at 56c3f4d3: TC-cli-gate-summary-shows-the-scope passed; at a scope gate the summary counts and names change.md, the deltas and plan first, without state.json or approvals.json · by claude-code (hc t382)
