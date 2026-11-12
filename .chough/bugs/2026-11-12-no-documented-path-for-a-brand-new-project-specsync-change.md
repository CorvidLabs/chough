---
kind: bug
id: 2026-11-12-no-documented-path-for-a-brand-new-project-specsync-change
aliases: []
title: "No documented path for a brand-new project: specsync change adopt is buried and the delta format is documented nowhere"
status: open
classification: evidence-gap
scope: in-scope
impact: A new agent would likely approve a change with no delta
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
# No documented path for a brand-new project: specsync change adopt is buried and the delta format is documented nowhere

**What happened:** README Start here defers to walkthrough.md and its init example assumes HI criteria and contracts exist; nothing says specsync init leaves SDD off until specsync change adopt; the skill's plan.md asks for a semantic delta per module with no format
Actual: The tester read an archived delta in the chough repo to learn the format; SpecSync 6.3.0 called a change with no delta ready for approval

**Steps:** Starting Chough on a new project from README, CHANGELOG, docs/cli.md and the skill

**Expected:** A complete new-project path in those docs, including adopt and the delta format

**Why it matters:** A new agent would likely approve a change with no delta

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:39Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
