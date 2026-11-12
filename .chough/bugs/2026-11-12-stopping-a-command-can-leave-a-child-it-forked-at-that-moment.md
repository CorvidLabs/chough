---
kind: bug
id: 2026-11-12-stopping-a-command-can-leave-a-child-it-forked-at-that-moment
aliases: []
title: Stopping a command can leave a child it forked at that moment running on macOS, because one SIGKILL to the process group misses a member the group gains while it is signalled
status: open
classification: product-defect
scope: blocking
impact: A stopped case run or native read can leave a process running, and the stop tests flake under load
criteria: [DRIVE-19]
requirements: [REQ-cli-011]
cases: []
evidence: []
receipts: []
environment: local-macos-arm64
revision: dd3e2ab5df329a8dc68f9f89725310b2b56e029f
found_by: claude-code (hc t382) (t38b (hc t38b), captured with a temporary diagnostic and reproduced outside chough (3 of 320 with one kill, 0 of 320 when repeated until the group is empty), 2026-11-12; Leif chose to fix it in 0.6.0 (17:42Z, relayed by t370); recorded by t382, rerun pending)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T17:43:16Z
updated: 2026-11-12T17:43:16Z
---
# Stopping a command can leave a child it forked at that moment running on macOS, because one SIGKILL to the process group misses a member the group gains while it is signalled

**What happened:** Under load, stop tests failed 2 of 24 with HUP or TERM: the command chough started survived; the survivor was the command's background sleep, in the signalled process group, re-parented to launchd and still running 5 s after chough exited
Actual: One SIGKILL to the group; on macOS a member joining during the signal can survive

**Steps:** chough stopping a command (SIGINT, SIGTERM or SIGHUP, or a deadline) whose shell forks a child as the group kill arrives, on macOS under load

**Expected:** A stop stops the command and every descendant (FLOW-14)

**Why it matters:** A stopped case run or native read can leave a process running, and the stop tests flake under load

**Fix:** Not fixed yet.

## Log

- 2026-11-12T17:43:16Z · Reported (t38b (hc t38b), captured with a temporary diagnostic and reproduced outside chough (3 of 320 with one kill, 0 of 320 when repeated until the group is empty), 2026-11-12; Leif chose to fix it in 0.6.0 (17:42Z, relayed by t370); recorded by t382, rerun pending) · by claude-code (hc t382)
