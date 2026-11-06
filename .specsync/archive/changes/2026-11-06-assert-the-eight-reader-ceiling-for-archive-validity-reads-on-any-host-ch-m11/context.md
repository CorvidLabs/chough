---
change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
artifact: context
---

# Context

Audit finding CH-M11, and MA-03 in the Chough assertion review. The audit was of main `700dad96`.

REQ-cli-033 says the trim reads the native validity of the affected archives "in parallel, with at most 8 at once". `src/evidence.rs` `validity()` starts `available_parallelism().clamp(2, 8).min(archives)` reader threads. The only test, `tests/reduce.rs` `native_validity_reads_run_in_parallel`, uses two cold archives and checks that at least two reads overlap. The audit raised `.clamp(2, 8)` to `.clamp(2, 128)`, and that test still passed: it proves overlap, and it would also pass with unbounded fan-out.

The host decides how many readers start, so an end-to-end test can tell 8 from 128 only on a host that reports more than 8 cores. This Mac reports 20; GitHub's ubuntu-latest reports 4, and there the mutant behaves exactly like the original. Measured before this change, on this Mac: a 12-cold-archive integration test fails with the mutant ("12 validity reads ran at once") and passes restored, while the existing test passes both ways.

## Leif's plan

The 95% plan, item C5 (2026-11-05): a test that asserts the eight-reader ceiling, which mutating 8 to 128 must fail. The ceiling stays 8 and the floor 2. Runtime behavior does not change.
