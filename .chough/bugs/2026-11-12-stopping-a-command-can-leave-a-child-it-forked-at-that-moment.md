---
kind: bug
id: 2026-11-12-stopping-a-command-can-leave-a-child-it-forked-at-that-moment
aliases: []
title: Stopping a command can leave a child it forked at that moment running on macOS, because one SIGKILL to the process group misses a member the group gains while it is signalled
status: verified
classification: product-defect
scope: blocking
impact: A stopped case run or native read can leave a process running, and the stop tests flake under load
criteria: [DRIVE-19]
requirements: [REQ-cli-011]
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-red-rerun-of-the-stop-survivor-at-048a7c84-without-the-fix-7
  - 2026-11-12-release-0-6-0/2026-11-12-green-rerun-of-the-stop-survivor-at-36ae763d-t38b-s-fix
receipts: []
environment: local-macos-arm64
revision: dd3e2ab5df329a8dc68f9f89725310b2b56e029f
found_by: claude-code (hc t382) (t38b (hc t38b), captured with a temporary diagnostic and reproduced outside chough (3 of 320 with one kill, 0 of 320 when repeated until the group is empty), 2026-11-12; Leif chose to fix it in 0.6.0 (17:42Z, relayed by t370); recorded by t382, rerun pending)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T17:43:16Z
updated: 2026-11-12T18:07:18Z
---
# Stopping a command can leave a child it forked at that moment running on macOS, because one SIGKILL to the process group misses a member the group gains while it is signalled

**What happened:** Under load, stop tests failed 2 of 24 with HUP or TERM: the command chough started survived; the survivor was the command's background sleep, in the signalled process group, re-parented to launchd and still running 5 s after chough exited
Actual: One SIGKILL to the group; on macOS a member joining during the signal can survive

**Steps:** chough stopping a command (SIGINT, SIGTERM or SIGHUP, or a deadline) whose shell forks a child as the group kill arrives, on macOS under load

**Expected:** A stop stops the command and every descendant (FLOW-14)

**Why it matters:** A stopped case run or native read can leave a process running, and the stop tests flake under load

**Fix:** Verified: 2026-11-12-release-0-6-0/2026-11-12-green-rerun-of-the-stop-survivor-at-36ae763d-t38b-s-fix passed at f69889a9a33445be9e5a4e17566768172a12f543.

## Log

- 2026-11-12T17:43:16Z · Reported (t38b (hc t38b), captured with a temporary diagnostic and reproduced outside chough (3 of 320 with one kill, 0 of 320 when repeated until the group is empty), 2026-11-12; Leif chose to fix it in 0.6.0 (17:42Z, relayed by t370); recorded by t382, rerun pending) · by claude-code (hc t382)
- 2026-11-12T18:07:18Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-red-rerun-of-the-stop-survivor-at-048a7c84-without-the-fix-7, 2026-11-12-release-0-6-0/2026-11-12-green-rerun-of-the-stop-survivor-at-36ae763d-t38b-s-fix. Fix 16afb63a (t38b, cherry-picked as 36ae763d): after the first group kill, Chough repeats SIGKILL to the group every 10 ms until kill(2) finds no live member (ESRCH, or EPERM when only zombies remain), for at most 500 ms, without reaping the leader inside the loop; Linux keeps its descendant sweep. Rerun by t382 with t38b's scripts: red at 048a7c84 7 of 48 runs left a survivor, green at 36ae763d 0 of 48 · by claude-code (hc t382)
