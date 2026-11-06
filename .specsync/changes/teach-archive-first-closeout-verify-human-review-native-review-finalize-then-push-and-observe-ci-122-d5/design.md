---
change: teach-archive-first-closeout-verify-human-review-native-review-finalize-then-push-and-observe-ci-122-d5
artifact: design
---

# Design

## Approach

Every place that teaches the closeout gets one order:
1. local checks;
2. the awaiting-human gate;
3. commit with `.chough/`;
4. `check --commit`;
5. present the work;
6. the human's implementation review;
7. `specsync change review`;
8. `ship` or `finalize`, on the same branch with no commit in between;
9. clear the gate and run `stage advance` to done;
10. commit, then push;
11. observe CI at the real tip;
12. merge.

The Archive gate is red only until step 8, so CI is observed once, on an archived tip. The no-intervening-commit rule and FLOW-10 are unchanged. The installed `.agents` copies are synced. `KNOWN_PREVIOUS` gains the 0.4.0 digests of the two references. The diagram is re-rendered with the existing renderer.

## Risks

- **An earlier push is still allowed.** A team may still want an early CI run on a draft PR. The guidance says it may push earlier for visibility but must expect the archive gate to be red and must not wait on it.
- **A synthetic actor.** The exercise's simulated reviewer is labeled synthetic, exactly as the archive fixtures label theirs.

## Constraints consulted

- FLOW-10 and REQ-cli-031 (no commit between review and finalize).
- docs/ci.md (the archive gate).
- Issue #122 and the audit's expected recovery.
