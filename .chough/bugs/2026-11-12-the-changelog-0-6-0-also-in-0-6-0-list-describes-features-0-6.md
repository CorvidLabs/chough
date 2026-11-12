---
kind: bug
id: 2026-11-12-the-changelog-0-6-0-also-in-0-6-0-list-describes-features-0-6
aliases: []
title: The CHANGELOG 0.6.0 Also in 0.6.0 list describes features 0.6.0 removed
status: open
classification: product-defect
scope: in-scope
impact: Release notes mislead upgraders
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
# The CHANGELOG 0.6.0 Also in 0.6.0 list describes features 0.6.0 removed

**What happened:** Lines about evidence prune --apply, event writes with a frozen legacy log, direct-native-write events before 0.5.0 and a session rolled into the vault contradict the breaking section and the binary; the diagnostic-inventory line is unreadable to a user
Actual: Entries carried from the 0.5.x cycle describe removed features

**Steps:** Reading CHANGELOG.md [0.6.0]

**Expected:** Every line under [0.6.0] describes 0.6.0 as shipped, in plain words

**Why it matters:** Release notes mislead upgraders

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:39Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
