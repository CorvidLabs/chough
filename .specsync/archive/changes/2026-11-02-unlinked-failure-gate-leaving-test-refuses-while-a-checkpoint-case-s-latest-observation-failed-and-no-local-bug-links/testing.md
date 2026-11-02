---
change: unlinked-failure-gate-leaving-test-refuses-while-a-checkpoint-case-s-latest-observation-failed-and-no-local-bug-links
artifact: testing
---

# Testing

## Cases

| Case | Requirement | Criteria | Method | What it shows |
|---|---|---|---|---|
| TC-cli-leaving-test-refuses-an-unlinked-failed-case | REQ-cli-008 | FLOW-7, LIFE-5, FLOW-2 | automated | The tests/stages.rs gate tests pass: refusal, both remedies, the unaffected cases, blocking-bug precedence and the next action |
| TC-cli-a-linking-bug-lifts-the-unlinked-failure-refusal | REQ-cli-008 | FLOW-7, LIFE-5, FLOW-8 | automated | A linking bug of any status lifts it; blocking bugs refuse first; unreferenced and passed-latest cases do not refuse; equal times order by ID; back is unaffected |

The cases were added after the scoped check materializes the delta (case recording is
not a tasks.md checkbox); TC-cli-0032 (blocking bugs) was rerun as well.

## Reproduction

A disposable copy of `examples/shipping` with SDD off: advance to test, reference
TC-shipping-0002, record a failed observation without a bug, then `stage advance`.
Expected before the fix: exit 0 and stage accept. After: refusal naming
`refused:unlinked-failure`, the case and the observation. The log is kept as an
artifact linked to the local bug.

## Regression

The full suite, including the existing blocking-bug and stage tests unchanged,
`chough cases check` and `fledge lanes run verify` on Linux with Fledge 1.8.0,
SpecSync 6.1.2 and hi 0.8.0 (its exercise step links its failure to a bug before
leaving test). Hosted CI is reported only as observed.
