---
kind: bug
id: 2026-11-12-a-closed-gate-that-drives-nothing-reads-closed-closed
aliases: []
title: A closed gate that drives nothing reads closed (closed)
status: open
classification: product-defect
scope: in-scope
impact: Output polish a first-time user stumbles on (t38k F12)
criteria: []
requirements: [REQ-cli-050]
cases: [TC-cli-a-close-that-runs-no-native-step-says-closed-once]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-a-close-that-runs-no-native-step-says-closed-once
receipts: []
environment: local-macos-arm64
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T21:02:30Z
updated: 2026-11-12T21:02:30Z
---
# A closed gate that drives nothing reads closed (closed)

**What happened:** Closing a gate that runs no native step writes the next action 'Gate "<gate>" closed (closed); …'
Actual: closed (closed)

**Steps:** Trigger: Close a gate that drives nothing
1. Open a gate held at intent
2. Close it with the relay
3. Read the checkpoint's next action

**Expected:** The decision word once

**Why it matters:** Output polish a first-time user stumbles on (t38k F12)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T21:02:30Z · Reported (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-a-close-that-runs-no-native-step-says-closed-once · by claude-code (hc t382)
