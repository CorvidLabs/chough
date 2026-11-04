---
change: stop-descendants-that-left-the-process-group-on-linux-and-report-git-failures-and-grace-kills-of-chough-own-git
artifact: testing
---

# Testing

- tests/stop.rs (Linux):
  - `stopping_a_case_run_stops_a_descendant_that_left_the_group` (REQ-cli-011): a binding
    whose child runs `setsid sleep <n>` while the binding waits; after SIGTERM, no
    `sleep <n>` survives.
  - `stopping_a_case_run_stops_an_orphan_that_left_the_group` (REQ-cli-011): the binding
    double-forks a `setsid sleep <n>` whose parent exits at once; after SIGTERM it is gone.
  - `a_grace_kill_after_git_is_reported` (REQ-cli-011): a Git stand-in leaves a
    descendant holding its output; `stage advance` succeeds and stderr names the stop.
  - `a_failed_head_revision_is_reported` (REQ-cli-011): a Git stand-in fails
    `rev-parse HEAD` with exit 1; `stage advance` succeeds and stderr names the failure.
  - `a_stop_test_leaves_another_runs_processes_alone` (REQ-cli-011, second #95 review): a
    decoy `sleep 691` stands in for this suite running in another worktree; the setsid
    scenario must neither take it for a survivor nor kill it. Fails at ffb1c24, where the
    tests matched the fixed `sleep 691`. The setsid, orphan and grace-kill tests now use
    a `sleep` duration unique to each fixture, and `Reap` kills only that.
- Each of the first four fails on main 91a1fdd; `fledge lanes run verify` passes on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-011 | tests/stop.rs `stopping_a_case_run_stops_a_descendant_that_left_the_group`, `stopping_a_case_run_stops_an_orphan_that_left_the_group`, `a_grace_kill_after_git_is_reported`, `a_failed_head_revision_is_reported`, `a_stop_test_leaves_another_runs_processes_alone` |
