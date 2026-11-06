# Lesson bundle: stop-tests-stall-on-a-per-fixture-token-so-concurrent-runs-never-take-or-kill-each-other-processes-144

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Stop tests stall on a per-fixture token so concurrent runs never take or kill each other processes (144)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/stop.rs
- **Acceptance**: Every stop test stalls on a unique per-fixture token and matches and reaps only it, so a concurrent run of the same test (simulated by sleep 641/642/643 decoys) is neither taken for a survivor nor killed; every existing stop assertion holds; the decoy test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `4999ba52714e22a66e51962912c14692d2d4bf03`
- Base commit: `c675c72e023fbdf4a80f762b8ef3f70330b4a83c`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #144, found by ch-pr69.

Most stop tests in tests/stop.rs stall on a bare `sleep <n>` (611, 621, 631, 641, 651, 661 and 671). They find that process with `pgrep -f "sleep <n>"` and clean it up with `pkill -f`, both across the whole machine. So when another worktree runs the same test at the same time:
- the test takes the other run's sleep for its own survivor ("the command chough started survived"; 1 in 4 reruns under load);
- its cleanup can kill the other run's stall.

The fix pattern already exists: the #95 review added `identity()`, a per-fixture token, but only some tests (691, 695 and 681) use it.

The new test `the_backfill_stop_test_leaves_another_runs_processes_alone` runs `sleep 641/642/643` decoys. It reproduces the failure exactly.

## From the change's design.md

# Design

## Approach

Each scenario computes `let token = identity(<n>)` and uses `sleep {token}` in its script, its `still_running` pattern and its `Reap`. `identity` makes the token unique by process ID, a counter and nanoseconds. `stall_git` and `hold_git` take the token, not a `u32`.

## Risks

None to the product: this changes only how tests isolate themselves.

## Constraints consulted

The #95 review's `identity()`. The rule against changing assertions or timeouts.

## From the change's testing.md

# Testing

- tests/stop.rs `the_backfill_stop_test_leaves_another_runs_processes_alone`: while `sleep 641/642/643` decoys run, the backfill stop scenario still passes, and every decoy survives. It fails first with "INT: the command chough started survived".
- `a_stop_test_leaves_another_runs_processes_alone` (691) still passes.
- Every stop test still asserts that its own child is stopped and that `.chough` is byte-identical.
- No assertion, gate or timeout changes.
- `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
