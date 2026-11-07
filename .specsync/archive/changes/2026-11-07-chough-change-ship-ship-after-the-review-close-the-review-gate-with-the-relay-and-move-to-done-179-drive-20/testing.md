---
change: chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20
artifact: testing
---

# Testing

New tests in `tests/change.rs`. They use an archived session at accept (as in `tests/advance_to.rs`) and a stand-in `specsync`. For `change ship` the stand-in records its argv, sleeps for `.stub-ship-sleep` seconds when that file exists, prints `.stub-ship-out` and `.stub-ship-err`, and exits with `.stub-ship-exit`. For `change status` it reports archived, and with `.stub-status-fail` it exits 2.

- `ship_with_the_review_relay_ships_closes_the_gate_and_moves_to_done`: one `change ship <id>` call; the gate is closed with the four values in `relayed_by`; the stage is done.
- `a_specsync_ship_refusal_passes_through_and_records_nothing`: SpecSync's bytes and exit code pass through, and `.chough/` is unchanged.
- `ship_refuses_before_running_without_the_relay_or_outside_accept`: each refusal leaves SpecSync uncalled and `.chough/` unchanged, and each named command then succeeds. The cases: an open gate with no relay; an incomplete relay; a gate that is not open; another change; stage test.
- `a_refused_done_move_after_ship_names_the_remaining_step`: with status failing after ship, the exit is 1, and the output says shipped and names `chough stage advance --to done`. Without the failure, that command then reaches done.
- `ship_outlives_the_native_read_bound`: with `CHOUGH_NATIVE_TIMEOUT_SECONDS=1` and a 2 s ship, the ship completes and the session reaches done.
- `the_walkthrough_and_closeout_run_chough_change`: the walkthrough and the accept reference name `chough change new`, `check` and `ship`, and neither has a direct `specsync change new`, `check … --commit` or `ship`.

They fail first because `chough change ship` does not exist and the docs still name the direct calls.

Mutation round, each mutant failing at least one new test:
- skip the stage check;
- skip the gate check;
- close the gate before ship;
- skip the close;
- skip the done move;
- return 0 after a refused done move;
- apply the native read bound to ship.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-059 | tests/change.rs, the six tests above; cases in `.chough/cases/cli.change-ship.cases.toml` |

## Lanes

`fledge lanes run verify` passes on Linux, and the change's tests pass on macOS.
