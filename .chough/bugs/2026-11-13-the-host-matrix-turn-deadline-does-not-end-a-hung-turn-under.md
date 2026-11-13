---
kind: bug
id: 2026-11-13-the-host-matrix-turn-deadline-does-not-end-a-hung-turn-under
aliases: []
title: The host-matrix turn deadline does not end a hung turn under dash
status: verified
classification: product-defect
scope: blocking
impact: The Ubuntu self-test job cannot pass, and a hung real host would never be stopped on Linux, including the planned Docker legs (LIFE-28)
criteria: [FLOW-21]
requirements: [REQ-cli-066]
cases: [TC-cli-host-matrix-selftest]
evidence:
  - 2026-11-13-host-matrix-selftest/2026-11-13-reproduced-t384-s-finding-run-sh-s-with-deadline-at-2bce65a2
  - 2026-11-13-host-matrix-selftest/2026-11-13-tc-cli-host-matrix-selftest-349558
receipts: []
environment: local-macos-arm64
revision: 0886a48aaebf05878bdb15a7be1b6ae73e88abf9
found_by: "claude-code (claude-opus-5-5, hc t38q) (Independent review by t384 on PR #254, 2026-11-13T06:55Z)"
actor:
  - agent: claude-code
    model: claude-opus-5-5
    terminal: t38q
related: []
created: 2026-11-13T06:57:59Z
updated: 2026-11-13T07:33:11Z
---
# The host-matrix turn deadline does not end a hung turn under dash

**What happened:** Under dash, which is sh on Ubuntu, run.sh's with_deadline neither ends a turn at its time limit nor returns promptly when a turn finishes, because it relies on set -m job control, which dash turns off without a terminal
Actual: with_deadline 2 sleep 8 runs 8 s and returns 0; the process-group kills fail, so the timer subshell's own sleep also holds every turn's wait

**Steps:** Trigger: Running tools/host-matrix/run.sh or selftest.sh with sh as dash and stdin not a terminal, as GitHub Actions and fledge do on Linux
1. dash scratchpad/deadline-repro.sh </dev/null (with_deadline copied from run.sh at 2bce65a2)

**Expected:** A turn past HOST_TURN_TIMEOUT ends with its whole process group and fails (REQ-cli-066)

**Why it matters:** The Ubuntu self-test job cannot pass, and a hung real host would never be stopped on Linux, including the planned Docker legs (LIFE-28)

**Fix:** Verified: 2026-11-13-host-matrix-selftest/2026-11-13-tc-cli-host-matrix-selftest-349558 passed at 91b227be06a40a5fea5950bcf92c8a812820491b.

## Log

- 2026-11-13T06:57:59Z · Reported (Independent review by t384 on PR #254, 2026-11-13T06:55Z); evidence: 2026-11-13-host-matrix-selftest/2026-11-13-reproduced-t384-s-finding-run-sh-s-with-deadline-at-2bce65a2 · by claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T07:33:11Z · status: open -> verified; evidence: 2026-11-13-host-matrix-selftest/2026-11-13-tc-cli-host-matrix-selftest-349558. Fixed in 91b227be: each turn and its timer start as process groups through perl's setpgrp, which needs no job control. TC-cli-host-matrix-selftest passed with sh as dash (349558) and as bash (b12eb5); the hung turn now fails at its 3 s limit. · by claude-code (claude-opus-5-5, hc t38q)
