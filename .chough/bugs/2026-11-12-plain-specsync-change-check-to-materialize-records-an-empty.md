---
kind: bug
id: 2026-11-12-plain-specsync-change-check-to-materialize-records-an-empty
aliases: []
title: Plain specsync change check to materialize records an empty direct-native-write Timeline entry, and Chough has no wrapper for the non-commit check
status: investigating
classification: product-defect
scope: blocking
impact: Noise in the Timeline for a documented step
criteria: []
requirements: []
cases: []
evidence: [2026-11-12-release-0-6-0/2026-11-12-tc-history-direct-write-entry-says-what-changed]
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:40Z
updated: 2026-11-12T17:37:49Z
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
- 2026-11-12T17:37:49Z · status: open -> investigating; scope: in-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-direct-write-entry-says-what-changed. Reproduced: t38b's red test (cdccd4c3, cherry-picked as c66a4b77) failed at c7cefe40: the direct-native-write entry after a plain change check had no text · by claude-code (hc t382)
