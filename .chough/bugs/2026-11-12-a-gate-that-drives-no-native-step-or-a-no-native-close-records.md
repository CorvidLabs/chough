---
kind: bug
id: 2026-11-12-a-gate-that-drives-no-native-step-or-a-no-native-close-records
aliases: []
title: A gate that drives no native step, or a --no-native close, records a decision in any name
status: verified
classification: product-defect
scope: blocking
impact: An intent or design approval can be recorded in any person's name, the most serious finding of the second fresh-eyes round (t38k F1)
criteria: []
requirements: [REQ-records-008]
cases:
  - TC-records-every-gate-close-names-a-configured-human
  - TC-records-ship-names-a-configured-human-at-any-gate
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-every-gate-close-names-a-configured-human
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-ship-names-a-configured-human-at-any-gate
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-every-gate-close-names-a-configured-human-f93c42
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-ship-names-a-configured-human-at-any-gate-79d253
receipts: []
environment: local-macos-arm64
revision: ab5426a506a54d3ceb2b6594e6a223761200676a
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report, F1 (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T19:57:49Z
updated: 2026-11-12T20:06:22Z
---
# A gate that drives no native step, or a --no-native close, records a decision in any name

**What happened:** chough checkpoint --close-gate, gate record and change ship --close-gate resolve --decided-by against the identity file only when the gate drives a native step and the close runs it; an intent or design gate, or any close with --no-native, accepts any name, though README and docs/cli.md say Chough refuses an unknown human
Actual: The gate closes and the decision is recorded in the unconfigured name; a ship at such a gate runs SpecSync's ship first

**Steps:** Trigger: Close a gate that drives nothing (an intent gate) with --decided-by naming no configured human
1. Open an intent gate with chough checkpoint --awaiting-human
2. Close it with --decided-by 'someone not configured' and the other three relay values
3. Or close a driving gate with --no-native, reply through gate record, or ship at a gate that drives nothing

**Expected:** refused:human-identity-unknown before anything runs, with every file unchanged (REQ-records-008 as re-scoped)

**Why it matters:** An intent or design approval can be recorded in any person's name, the most serious finding of the second fresh-eyes round (t38k F1)

**Fix:** f5d8ed79: checkpoint --close-gate, gate record and change ship resolve --decided-by against the identity file for every close, whatever the gate drives and with or without --no-native, before anything runs (REQ-records-008)

## Log

- 2026-11-12T19:57:49Z · Reported (t38k's second fresh-eyes round report, F1 (relayed by t370)); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-records-every-gate-close-names-a-configured-human, 2026-11-12-release-0-6-0/2026-11-12-tc-records-ship-names-a-configured-human-at-any-gate · by claude-code (hc t382)
- 2026-11-12T20:06:22Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-records-every-gate-close-names-a-configured-human-f93c42, 2026-11-12-release-0-6-0/2026-11-12-tc-records-ship-names-a-configured-human-at-any-gate-79d253; fix: f5d8ed79: checkpoint --close-gate, gate record and change ship resolve --decided-by against the identity file for every close, whatever the gate drives and with or without --no-native, before anything runs (REQ-records-008). Red at ab5426a5 (both cases failed: the intent gate closed in an unconfigured name; the ship took any name), green at f5d8ed79 (both passed) · by claude-code (hc t382)
