# Lesson bundle: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Assert the eight-reader ceiling for archive validity reads on any host (CH-M11)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, tests/reduce.rs, CHANGELOG.md
- **Acceptance**: REQ-cli-033 gains a criterion: with twelve affected archives no more than 8 validity reads run at once, and the reader count is at most 8 whatever parallelism the host reports. The reader count is a pure function with no behavior change. A unit test holds the ceiling for a host reporting 128 cores, and a 12-cold-archive integration test observes at most 8 overlapping reads. Both fail with the ceiling raised to 128 (the audit's CH-M11 mutant) and pass restored; the existing overlap test still passes. fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `69a9cec01c391ec4fb2f2ca7c35e02a633d5fa70`
- Base commit: `73a7da5653312e907621c0165d0b5f903c22ff53`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Audit finding CH-M11, and MA-03 in the Chough assertion review. The audit was of main `700dad96`.

REQ-cli-033 says the trim reads the native validity of the affected archives "in parallel, with at most 8 at once". `src/evidence.rs` `validity()` starts `available_parallelism().clamp(2, 8).min(archives)` reader threads. The only test, `tests/reduce.rs` `native_validity_reads_run_in_parallel`, uses two cold archives and checks that at least two reads overlap. The audit raised `.clamp(2, 8)` to `.clamp(2, 128)`, and that test still passed: it proves overlap, and it would also pass with unbounded fan-out.

The host decides how many readers start, so an end-to-end test can tell 8 from 128 only on a host that reports more than 8 cores. This Mac reports 20; GitHub's ubuntu-latest reports 4, and there the mutant behaves exactly like the original. Measured before this change, on this Mac: a 12-cold-archive integration test fails with the mutant ("12 validity reads ran at once") and passes restored, while the existing test passes both ways.

## Leif's plan

The 95% plan, item C5 (2026-11-05): a test that asserts the eight-reader ceiling, which mutating 8 to 128 must fail. The ceiling stays 8 and the floor 2. Runtime behavior does not change.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
