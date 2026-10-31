---
change: bound-external-commands-in-time-and-memory-and-honor-validation-stop-requests-after-waiting
artifact: plan
---

# Plan

## Approach

Introduce the bounded runner in `src/records.rs`, move `cases::execute`, `native` and
`native_json` onto it, and fix the validation loop order.

## Out of scope

Stage gate rules (separate change), SpecSync behavior, Windows job objects, changing
case binding schema or the 600 s default case timeout.

## Steps

1. Add the runner, limits and platform tree stop, with unit tests for the bounded reader.
2. Use it for case execution; add per-stream truncation notes and limitations.
3. Use it for native reads with the deadline and `timed-out` state.
4. Reorder the validation loop and slice the interval; add `paused`.
5. Integration tests per platform; rerun the recorded reproductions; docs.

## Risks

Platform differences in process trees (mitigated by per-platform tests in hosted CI);
an abandoned reader thread lives until the descendant dies (bounded memory, reported);
slow native tools exceeding 120 s (override variable).

## Constraints consulted

specs/cli/context.md lessons on not-run results, checkbox-free gates and disposable
Git identity; Invariants 5 and 7.
