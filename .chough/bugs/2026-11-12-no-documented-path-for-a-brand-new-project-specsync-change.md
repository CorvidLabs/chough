---
kind: bug
id: 2026-11-12-no-documented-path-for-a-brand-new-project-specsync-change
aliases: []
title: "No documented path for a brand-new project: specsync change adopt is buried and the delta format is documented nowhere"
status: verified
classification: evidence-gap
scope: blocking
impact: A new agent would likely approve a change with no delta
criteria: []
requirements: []
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-docs-show-a-new-project-and-a-delta
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-docs-show-a-new-project-and-a-delta-a22092
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-docs-show-a-new-project-and-a-delta-342bc4
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:39Z
updated: 2026-11-12T17:32:50Z
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
- 2026-11-12T17:32:50Z · status: open -> verified; scope: in-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-docs-show-a-new-project-and-a-delta, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-docs-show-a-new-project-and-a-delta-a22092, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-docs-show-a-new-project-and-a-delta-342bc4. Red at 56c3f4d3 (before the docs, in a throwaway worktree): README named no specsync change adopt. A first run here at a87b54ec also failed, on a wrapped `## MODIFIED` code span and the check's own section cut at the example's fence, both fixed in f2635147. Green at f2635147: README, docs/cli.md (A new project and its first change, with the delta format) and the skill's plan reference give the path and the format · by claude-code (hc t382)
