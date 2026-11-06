---
change: make-archived-next-and-status-fast-within-explicit-warm-budgets
artifact: design
---

# Design

## Native archive verdict reuse

Add a crate-private `src/native.rs` component used by the report-only native read in `src/main.rs`.

1. On a live response, accept for storage only an archived response whose change ID matches and whose terminal validity is `authenticated-history`. Require a single self-contained archive and its native acceptance records. Strip any cache marker supplied by the live process.
2. Store the native JSON and proof metadata under the Git common directory, outside tracked inputs and outside `.chough` session/history records. Use create-only/atomic bounded writes and reject symlinks, malformed entries and oversized records.
3. Key reuse to the change ID, archive tree and complete file bytes, archive-carrying commit and relevant ancestry/object availability, resolved SpecSync executable bytes and recorded version, native policy/configuration/baseline bytes, and shallow/replace/graft/alternates state. Recompute the cheap identity checks on every report. A mismatch or inability to prove identity invokes the existing live bounded read.
4. Reuse only the exact native JSON previously supplied by SpecSync. Mark the Chough read state `cached`; never accept that marker from a live response. Report output keeps its existing size and field rules.
5. Do not call this component from `stage`, `timing`, retention or verification. Their existing `native_json` calls remain live, including after a warm report. Existing native timeouts and output bounds stay unchanged.

The cache is local trusted state like the Git directory, not human approval, a second lifecycle ledger, or proof that a later live check succeeded. Documentation will explain removal and cold fallback.

## Exact snapshot work reduction

Run the existing ordered legacy and content fingerprint passes as independent scoped workers. Each worker retains its current iteration order, `hash_input` transformations and digest bytes. Join both workers before constructing the snapshot; propagate failures visibly and preserve the existing Git status check. This changes no fingerprint algorithm, freshness predicate, persistent record or bound. Measure this implementation against the sequential baseline; do not replace it with an unbounded in-memory file cache or metadata-only reuse.

## Ownership

This change owns `src/native.rs`, the report call seam in `src/main.rs`, minimal live-path assertions around `src/stage.rs`, snapshot workers in `src/evidence.rs`, and focused native/next/evidence tests and docs. It does not edit checkpoint/session implementation or tests. Merge main after PR #133 and retain its overlapping evidence, next-test, common-fixture, documentation and HI changes.
