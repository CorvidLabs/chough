---
change: reuse-specsync-s-authentication-of-an-unchanged-committed-archive-so-next-status-stage-moves-and-timing-skip-the-slow
artifact: testing
---

# Testing

`tests/native.rs` with the counting SpecSync stub: a first read records and later next, status, stage advance and timing make zero reads with the cached label; changed archive byte, policy, stub binary, corrupt entry, active workspace and missing snapshot read live; stale and corrupt-history are not recorded; retention reads stay live; `stage advance --to accept` makes one read and three transitions, and a refused later step records nothing. The tests fail before the fix. `fledge lanes run verify` runs the full suite.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-035 | `tests/native.rs`: `an_authenticated_archive_is_read_once_and_reused_while_unchanged` (one live read records the verdict; `next`, `status`, `timing --json` and `stage advance` then make zero reads and show `native:archived-authenticated-cached`), `any_changed_input_reads_live_and_a_refused_read_still_refuses` (archive bytes uncommitted and committed, policy, legacy baseline, SpecSync binary, corrupt and mismatched entries, an active workspace, an unreachable recorded commit, and a failed live read still refused), `only_self_contained_authenticated_archives_are_recorded` (corrupt-history, exact, verifying, no snapshot, no manifest, no Git directory), `a_batched_advance_reads_native_status_once` and `a_refused_step_refuses_the_whole_batch_with_nothing_recorded`. |
| REQ-cli-024 | `tests/native.rs` above shows `next` and `status` make one read or none; the existing `tests/timing.rs` and `tests/stages.rs` single-read and budget tests still pass unchanged. |
| REQ-cli-028 | The only persistent result is the entry under the Git common directory that REQ-cli-035 allows; `tests/native.rs` counts its entries, and the full suite passes with no new dependency or runtime. |
