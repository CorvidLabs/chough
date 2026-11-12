---
kind: bug
id: 2026-11-12-a-gate-close-that-runs-no-native-step-reports-native-step-with
aliases: []
title: A gate close that runs no native step reports native_step with ran and why both null
status: open
classification: product-defect
scope: in-scope
impact: An agent reading the output cannot tell whether a step was skipped or failed (t38k F12)
criteria: []
requirements: [REQ-cli-050]
cases: [TC-cli-close-running-no-native-step-says-why]
evidence: [2026-11-12-release-0-6-0/2026-11-12-tc-cli-close-running-no-native-step-says-why]
receipts: []
environment: local-macos-arm64
revision: 731aba24c9db412aaa2f1b570043ee4c1f16cc31
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report, F12 (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T20:09:16Z
updated: 2026-11-12T20:09:16Z
---
# A gate close that runs no native step reports native_step with ran and why both null

**What happened:** checkpoint --close-gate and gate record report native_step as {ran: null, why: null} for a gate that drives nothing, though docs/cli.md says native_step gives what ran or why nothing did
Actual: {"ran": null, "why": null}

**Steps:** Trigger: Close a gate that drives no native step and read relayed_decision.native_step
1. Open a gate held at intent (it drives none)
2. Close it with the four relay values

**Expected:** native_step says why nothing ran: the gate drives none

**Why it matters:** An agent reading the output cannot tell whether a step was skipped or failed (t38k F12)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T20:09:16Z · Reported (t38k's second fresh-eyes round report, F12 (relayed by t370)); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-close-running-no-native-step-says-why · by claude-code (hc t382)
