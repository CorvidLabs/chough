---
change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
artifact: tasks
---

# Tasks

- [x] Add the integration test `at_most_eight_validity_reads_run_at_once` in tests/reduce.rs: 12 cold archives, each listing a superseded run, so the trim reads 12 validities; at most 8 overlap and at least 2 do.
- [x] Move the reader count into `validity_readers(available, archives)` in src/evidence.rs, with `VALIDITY_READERS = 8` and no behavior change, and add the unit test `at_most_eight_validity_readers_on_any_host`.
- [x] Add the REQ-cli-033 criterion for the ceiling, and a CHANGELOG line.

After `specsync change check` materializes the delta, not a task box: rerun both tests with the ceiling raised to 128, keep the failing logs, restore the ceiling, record the cases, and run `fledge lanes run verify` on Linux.
