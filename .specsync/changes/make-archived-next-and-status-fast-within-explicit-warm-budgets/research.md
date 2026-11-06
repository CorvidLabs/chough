---
change: make-archived-next-and-status-fast-within-explicit-warm-budgets
artifact: research
---

# Research

## Current source paths

- `src/main.rs:317-399` performs the one native status read shared by `next` and `status` and derives report guidance from it.
- `src/stage.rs:215-237` owns live native state for lifecycle gates. `destination_gate` calls this path for every gated transition.
- `src/evidence.rs:302-380` enumerates Git inputs and computes separate legacy and content digests; `hash_input` reads ordinary files in each pass.
- `src/records.rs:1443-1549` owns bounded native JSON execution, parsing and explicit failure/timeout states.

## Measurements

See `.chough/artifacts/issue-32-next-status-latency/PROFILE.md` for binary identities, input counts, all runs and limitations. Archived native status is the largest single cost in the reported scenario (4.36-4.84 s and 261 Git children). Chough's own hashing is independently material, especially in debug and on release runs with colder filesystem data.

## Prior art

PR #69 stored SpecSync's `authenticated-history` result outside the worktree, keyed by archive/native inputs. Its review demonstrated a stale hit after changing the shallow boundary, and its scope included stage transitions and timing. This change must not port those behaviors unchanged. A read-only report cache keeps lifecycle mutation on SpecSync's live answer; complete history inputs and fail-live mismatch behavior are required.

## Alternatives

- Parse `state.json`, `finalization.json` and related archive records directly: rejected because Chough would reimplement native authentication and could disagree with SpecSync.
- Cache active-change status: rejected because active approval, correction and verification states change during ordinary work and stale guidance would be difficult to bound.
- Raise or shorten native timeouts: rejected; neither improves an authenticated warm response and both alter existing failure behavior.
- Optimize only local hashing: insufficient because the archived native read alone exceeds the proposed budget.
- Optimize only archived native reuse: insufficient because measured local work can itself exceed 2000 ms.

Relative size: medium. The main uncertainty is proving complete cache invalidation across Git history states while retaining a useful warm path. Current timing history has no directly comparable completed archived-report budget sample, so it does not support a narrower estimate.
