---
change: reduce-old-observation-proof-drop-closed-cycle-checkpoint-references-at-done-trim-superseded-runs-of-cold-changes
artifact: plan
---

# Plan

## Approach

- **Classification:** it also marks the newest run per case and environment. The summary
  then drops the "newest" protection from runs that a newer run of the same group
  supersedes.
- **Unlisted sets:** each is attributed to the first archive finalized at or after its
  observation. A set that falls in a cold archive's cycle is a candidate owned by that
  archive. Validity is checked only for archives that list a removed file, and is read in
  parallel.
- **Knowledge entries:** an entry lists its attributed sets with their outcome, or their
  pruned state.
- **Stage advance into done:** it drops closed-cycle references before the trim, under
  the same writer lock, and saves the checkpoint again.

## Out of scope

Loose artifacts and attachments (KNOW-18, KNOW-19); `chough evidence prune`, whose
protections are unchanged; the hot window of 3.

## Steps

1. Capture KNOW-20 in hi.
2. Add the failing tests in tests/reduce.rs.
3. Implement the classification, attribution, entries and done step.
4. Update the docs and skill reference.
5. Measure the trim on this repository, and commit what it removes.
6. Verify on Linux.

## Risks

- Removing proof: the existing ledger-first, restore and open-change checks apply
  unchanged, and only HEAD-identical files are removed.
- Time: parallel validity reads, with at most 8 at once.

## Constraints consulted

KNOW-8, KNOW-9, KNOW-10, KNOW-11 and REQ-history-001 protections other than those
narrowed here; LIFE-8 (missing proof reads as pruned, never passed).
