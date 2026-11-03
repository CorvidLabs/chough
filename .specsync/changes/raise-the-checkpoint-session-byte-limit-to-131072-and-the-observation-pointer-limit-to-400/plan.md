---
change: raise-the-checkpoint-session-byte-limit-to-131072-and-the-observation-pointer-limit-to-400
artifact: plan
---

# Plan

## Approach

Give the checkpoint its own 131,072-byte bound and the observation list its own 400-item bound, leaving every other bound unchanged.

## Out of scope

Compaction of checkpoint entries, history pruning, any other record, list, history, case, output or context bound.

## Steps

1. Capture Leif's decision as HI LIFE-11.
2. Add REQ-cli-032 and update REQ-cli-007's acceptance bound.
3. Implement the bounded record helpers and checkpoint constants.
4. Update tests asserting 65536 for the checkpoint to 131072 and add boundary tests.
5. Docs, changelog, cases; run `fledge lanes run verify`.

## Risks

Older Chough binaries refuse a checkpoint over 64 KiB; documented in the changelog.

## Constraints consulted

Leif's 9:40 PM MT decision; never weaken tests or gates; no history pruning.
