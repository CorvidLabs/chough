---
change: stop-tests-stall-on-a-per-fixture-token-so-concurrent-runs-never-take-or-kill-each-other-processes-144
artifact: testing
---

# Testing

- tests/stop.rs `the_backfill_stop_test_leaves_another_runs_processes_alone`: while `sleep 641/642/643` decoys run, the backfill stop scenario still passes, and every decoy survives. It fails first with "INT: the command chough started survived".
- `a_stop_test_leaves_another_runs_processes_alone` (691) still passes.
- Every stop test still asserts that its own child is stopped and that `.chough` is byte-identical.
- No assertion, gate or timeout changes.
- `fledge lanes run verify` passes on Linux.
