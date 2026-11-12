---
kind: bug
id: 2026-11-11-verify-lane-test-step-fails-intermittently-under-load-green-on
aliases:
  - BUG-verify-lane-test-step-fails-intermittently-under-load-green-on-isolated-30184d3be250a048bc85cc08
title: Verify lane test step fails intermittently under load, green on isolated rerun
status: open
classification: unclassified
scope: in-scope
impact: "Merge path for PR #212 needs lane retries; reported on issue #132; 0.6.0 must fix it (zero open bugs at tag time)"
criteria: [DRIVE-19]
requirements: []
cases: []
evidence: []
receipts: []
environment: local macOS arm64, loaded machine
revision: 956acbe2adb5c8f25852a0b979507c18f89faa18
found_by: manual report
related: []
created: 2026-11-11T04:21:21Z
updated: 2026-11-11T04:21:21Z
---
# Verify lane test step fails intermittently under load, green on isolated rerun

**What happened:** fledge lanes run verify fails at step 3 (test) with a different test binary each time: 2026-11-10 saw '--test change' (1/14 failed), '--test stop', and '--test cases' fail in the lane; each passed fully on an isolated rerun right after (change 14/14, stop 10/10, cases 32/32)
Actual: Intermittent failures that pass on retry; matches the #132 fork/fd-inheritance pattern (ETXTBSY, held checkpoint lock)

**Steps:** Full verify lane on a heavily loaded <redacted:host-name> (several agents, load ~75); the --test change instance coincided with two lanes sharing one target dir

**Expected:** The lane's test step reflects real failures only (DRIVE-19)

**Why it matters:** Merge path for PR #212 needs lane retries; reported on issue #132; 0.6.0 must fix it (zero open bugs at tag time)

**Fix:** Not fixed yet.

## Log

- 2026-11-11T04:21:21Z · Reported (manual report)
