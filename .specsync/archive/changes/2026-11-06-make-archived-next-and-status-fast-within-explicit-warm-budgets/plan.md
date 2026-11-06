---
change: make-archived-next-and-status-fast-within-explicit-warm-budgets
artifact: plan
---

# Plan

## Approach

Implement a strictly invalidated, read-only cache of SpecSync-authenticated archived status for `next`/`status`, and run the two exact snapshot digest passes concurrently. Keep every lifecycle mutation and non-report native read live. Add failing-first controlled tests and measure a freshly built release binary against the approved warm budget.

## Out of scope

- Checkpoint capacity, session storage/locking/merging, and their implementation tests.
- Cached or reconstructed stage-gate decisions; cached active-change status.
- Changes to `timing`, retention, native verification, native gate predicates, deadlines, output caps or evidence freshness semantics.
- A cold-cache latency promise, an active-change latency promise, SpecSync internals, or a new upstream fix for spec-sync#831.
- Batched `stage advance --to`: the coordinated PR #69 split assigns that behavior to a separate small PR. The archive cache belongs to this #32 change, and #69 closes pointing to both replacements.
- Release, deployment or automatic merge.

## Steps

1. Obtain the combined intent/scope decision on the numeric budget, read-only cache and snapshot worker design. Capture the agreed HI criterion as DRIVE-15; the registry audit of origin/main `260ecc949ea01214f4bec43b30ea4226ce8fae59` and every remote branch found DRIVE-14 highest and REQ-cli-051 highest at that point. This change then took REQ-cli-053 after ch-pr69 claimed REQ-cli-052 for the split stage-advance PR.
2. Merge current origin/main without rebasing, preserve PR #133 and other integrated edits, and refresh the native delta/scope if the integration changes the definition.
3. Add controlled failing tests for cold recording, warm zero-status reuse, invalidation/fallback, anti-spoofing, and live stage/retention reads. Run them against unmodified product code and retain the failure logs.
4. Implement `src/native.rs` and the report-only call seam. Keep live failure/timeout behavior and native output parsing unchanged.
5. Add exact fingerprint known-answer and freshness regressions, then implement scoped snapshot workers. Confirm identical legacy/content digests and errors remain visible.
6. Run focused tests and mutation checks, then the complete Fledge verification lane. Add per-change Chough case definitions after the scoped native check materializes REQ-cli-053.
7. Build release Chough and record ten warm archived `next` and `status` runs after one successful priming run on the same 121-archive inputs. Record every run, p95, maximum, native call counts, phase traces and limitations. If priming times out, record no cache write and no warm-budget claim; never leave native descendants running or extend the timeout to obtain a prime. Repeat if the budget fails; do not relax it.
8. Merge origin/main again before review/merge, rebuild the knowledge index if needed, and leave a clean pinned head for human review.

## Risks

- An omitted Git/native input could make a cached archived verdict stale. Mitigate with the explicit identity contract, fail-live checks, history-state regressions and no cached lifecycle mutations.
- Parallel filesystem reads may not improve wall time under all storage/load conditions. Mitigate with exact-digest tests and the real budget; keep the simpler sequential code if the measured worker design cannot meet the approved scope without a semantic change, and return to the human gate rather than silently choosing a weaker cache.
- Shared files may conflict with PR #133 or other siblings. Merge main normally and preserve their edits; never rebase or force push.

## Constraints consulted

DRIVE-2; REQ-cli-003/004/021/022/029; `specs/cli/context.md` archived-status, parallel-stub, merge and bounds lessons; PR #69 review history-invalidation findings; the one-issue/one-change/one-PR rule. Existing tests, gates, bounds and timeouts are minimums, not tuning knobs.
