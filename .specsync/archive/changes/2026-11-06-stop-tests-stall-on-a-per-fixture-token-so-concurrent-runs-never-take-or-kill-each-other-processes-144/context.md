---
change: stop-tests-stall-on-a-per-fixture-token-so-concurrent-runs-never-take-or-kill-each-other-processes-144
artifact: context
---

# Context

Issue #144, found by ch-pr69.

Most stop tests in tests/stop.rs stall on a bare `sleep <n>` (611, 621, 631, 641, 651, 661 and 671). They find that process with `pgrep -f "sleep <n>"` and clean it up with `pkill -f`, both across the whole machine. So when another worktree runs the same test at the same time:
- the test takes the other run's sleep for its own survivor ("the command chough started survived"; 1 in 4 reruns under load);
- its cleanup can kill the other run's stall.

The fix pattern already exists: the #95 review added `identity()`, a per-fixture token, but only some tests (691, 695 and 681) use it.

The new test `the_backfill_stop_test_leaves_another_runs_processes_alone` runs `sleep 641/642/643` decoys. It reproduces the failure exactly.
