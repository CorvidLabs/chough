---
kind: bug
id: 2026-11-11-verify-lane-test-step-fails-intermittently-under-load-green-on
aliases:
  - BUG-verify-lane-test-step-fails-intermittently-under-load-green-on-isolated-30184d3be250a048bc85cc08
title: Verify lane test step fails intermittently under load, green on isolated rerun
status: verified
classification: unclassified
scope: in-scope
impact: "Merge path for PR #212 needs lane retries; reported on issue #132; 0.6.0 must fix it (zero open bugs at tag time)"
criteria: [DRIVE-19]
requirements: []
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-red-rerun-of-the-verify-lane-flake-at-85dd828c-6-of-12-loaded
  - 2026-11-12-release-0-6-0/2026-11-12-green-rerun-of-the-verify-lane-flake-at-b6fcd0e9-t38b-s
receipts: []
environment: local macOS arm64, loaded machine
revision: 956acbe2adb5c8f25852a0b979507c18f89faa18
found_by: manual report
related: []
created: 2026-11-11T04:21:21Z
updated: 2026-11-12T17:09:42Z
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
- 2026-11-12T17:09:42Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-red-rerun-of-the-verify-lane-flake-at-85dd828c-6-of-12-loaded, 2026-11-12-release-0-6-0/2026-11-12-green-rerun-of-the-verify-lane-flake-at-b6fcd0e9-t38b-s. Root cause (t38b, measured): macOS checks every newly written executable on its first exec, one at a time across the machine; under load a fresh stand-in's first exec took 3.9 s at the median, inside the tests' time bounds (stop: the command never started; cases: returned after 12 to 31 s); and the specsync stubs wrote their PID file before its content, so a polling test could read it empty. Fixes: install_executable warms each stand-in once at setup (1888f4f5) and the stubs publish their PID by rename (2ee84412); no assertion or bound changed. Rerun by t382 with t38b's scripts: red at 85dd828c 6/12 runs failed, green at b6fcd0e9 0/12. t38b is investigating a different, rarer flake (HUP survivor) separately · by claude-code (hc t382)
