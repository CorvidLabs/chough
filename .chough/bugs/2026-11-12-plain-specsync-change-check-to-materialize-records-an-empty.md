---
kind: bug
id: 2026-11-12-plain-specsync-change-check-to-materialize-records-an-empty
aliases: []
title: Plain specsync change check to materialize records an empty direct-native-write Timeline entry, and Chough has no wrapper for the non-commit check
status: open
classification: product-defect
scope: in-scope
impact: Noise in the Timeline for a documented step
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
# Plain specsync change check to materialize records an empty direct-native-write Timeline entry, and Chough has no wrapper for the non-commit check

**What happened:** Running specsync change check <id> as docs/cli.md says, to materialize an approved delta, leaves a direct-native-write entry with empty text
Actual: An empty direct-native-write entry

**Steps:** specsync change check <id> without --commit, then any Chough save

**Expected:** A documented step that records a meaningful entry, or a Chough command for it

**Why it matters:** Noise in the Timeline for a documented step

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:40Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
