---
change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
artifact: testing
---

# Testing

- tests/reduce.rs `at_most_eight_validity_reads_run_at_once` sets up 15 archived changes. The 12 cold ones each list two passing runs of their own case, and the older run is superseded. The stub `specsync` sleeps 1 s per status read and logs how many run at once. `stage advance` into done trims all 12 older runs and keeps the 12 newer ones, and the most reads seen at once is between 2 and 8. With the ceiling raised to 128 it fails on a host that reports more than 8 cores. On this Mac (20 cores) it sees 12 at once.
- src/evidence.rs `at_most_eight_validity_readers_on_any_host` (unit): `validity_readers(128, 20) == 8`, `(9, 9) == 8`, `(1, 20) == 2`, `(20, 3) == 3`, `(4, 0) == 1`. With the ceiling raised to 128 it fails on every host.
- Unchanged and still passing: `native_validity_reads_run_in_parallel` (two cold archives overlap).
- Mutation check: rerun both tests with `VALIDITY_READERS` (or the clamp) at 128. Both must fail. Then restore and rerun; both pass. Keep both logs.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-033 | tests/reduce.rs `at_most_eight_validity_reads_run_at_once`, `native_validity_reads_run_in_parallel`; src/evidence.rs `at_most_eight_validity_readers_on_any_host` |

## Lanes

`fledge lanes run verify` passes on Linux.
