---
change: record-a-stage-gate-that-already-happened-with-its-original-time-quoted-evidence-and-a-backfilled-flag-that-timing
artifact: testing
---

# Testing

`tests/stages.rs` with the SpecSync stub: a backfilled plan to build move records the
given time, the full commit and the quote, and makes one native read; `timing --json`
lists it under `backfilled`, marks the bounded segments and leaves them out of
estimates; with history adopted the event carries the same field. Refusals with
nothing recorded: a draft change (native state does not support build), an open
awaiting-human gate, a future time, a time before the latest transition, an unknown or
unreachable commit, a blank quote, and a side stage. No approval, review, consent or
gate record appears. The tests fail before the change. `fledge lanes run verify` runs
the full suite.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-036 | `tests/stages.rs`: `a_backfilled_move_keeps_its_original_time_and_quoted_evidence` and `backfill_refuses_what_advance_or_its_evidence_does_not_support`. |
| REQ-history-005 | `tests/stages.rs`: `a_backfilled_move_keeps_its_original_time_and_quoted_evidence` (legacy log bytes) and `a_backfilled_move_is_carried_by_history_events`. |
