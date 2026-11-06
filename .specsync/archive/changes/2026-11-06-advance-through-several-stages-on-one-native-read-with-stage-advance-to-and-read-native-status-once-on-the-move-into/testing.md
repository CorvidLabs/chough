---
change: advance-through-several-stages-on-one-native-read-with-stage-advance-to-and-read-native-status-once-on-the-move-into
artifact: testing
---

# Testing

## tests/advance_to.rs (new, unix)

- `a_batched_advance_reads_native_status_once_in_a_session`: under `session new`, plan to accept on one read, three events, steps reported.
- `a_batched_advance_in_a_single_checkpoint_project`: the same, in `.chough/transitions.jsonl`.
- `a_batch_with_no_native_prerequisite_reads_nothing`: intent to plan, zero reads, two events.
- `a_refused_step_refuses_the_whole_batch_with_nothing_recorded`: a blocking bug, draft state, a target that is not ahead, a side or unknown target, and an open human gate. Each leaves the session file and history byte-identical, with at most one read.
- `advance_into_done_reads_native_status_once`: from accept, and with `--to done` from test, on an archived change with an acceptance manifest. Each makes exactly one read and reports `librarian_trim`.
- `a_plain_advance_is_unchanged`: one transition, no `steps`.

All but the last fail first against main.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-052 | tests/advance_to.rs, all six tests |

## Lanes

`fledge lanes run verify` passes.
