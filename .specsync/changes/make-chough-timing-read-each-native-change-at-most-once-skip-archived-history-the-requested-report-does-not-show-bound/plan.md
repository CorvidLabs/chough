---
change: make-chough-timing-read-each-native-change-at-most-once-skip-archived-history-the-requested-report-does-not-show-bound
artifact: plan
---

# Plan

## Approach

Tests first against a counting, sleeping and failing SpecSync stub, then the smallest
change in `src/timing.rs` and `src/records.rs`. Reuse is request-local inside one
timing report. The native gate path (`stage::native_state`, `next`, `status`) is not
edited.

What this does and does not buy: timing drops from eleven sequential native reads
(one of them a duplicate) to one read for filtered reports, and to at most ten
deduplicated reads within a 120 s total budget for unfiltered reports. `chough next`
and `chough status` are unchanged. Their one read of an archived change still costs
whatever SpecSync takes; the single measured read took 49.954 s. Making that faster
is upstream work, or a later scoped change with evidence.

## Out of scope

- SpecSync's archived-status latency (an upstream reproduction is prepared separately; nothing is filed without the human).
- Local next/status cost (2–3 s, release build). Only the trace is added to measure it.
- Persistent caches, parallel native reads, evidence snapshot/hash reuse.
- Retention policy (#31), case/run identity and allocation (#30), checkpoint merge (PR #29, Kimi).
- Any new case number or identity scheme.

## Steps

1. Extend the existing cases TC-cli-0089 (native reads in timing) and TC-cli-0100 (timed-out or unavailable native reads) by adding REQ-cli-023, and extend their bound tests with the deduplication/selection and budget/failure assertions. Add named Rust regressions in tests/timing.rs for the trace and for gate separation. Refresh live `.chough/cases/` from main before editing; the file is shared with #30 and #31, and only these two entries change.
2. Record failure-first observations at the unfixed tip with explicit IDs: `tc-cli-0089-native-dedupe-b35acaf-pre-fix` and `tc-cli-0100-native-budget-b35acaf-pre-fix`.
3. Implement the memo, history selection, budget (`native_json_within`) and trace.
4. Record post-fix observations: the same IDs with a `-post-fix` suffix and the implementation revision. Then run fmt, Clippy, the full test suite and the verify lane.
5. Run before/after release measurements on a clone of the same base. Cover next, status, timing unfiltered and filtered, and the prune preview, cold and warm, at the default deadline under a whole-command outer bound. Preserve unknown and timed-out states.
6. Update docs/cli.md (selection semantics, null history with its reason, the new variables, estimates from transitions only, gates unaffected) and add a CHANGELOG entry.

## Risks

- JSON consumers that assume `native.history` is always a list. Mitigation: documented null value with `history_selection`. No release has a consumer contract beyond 0.1.0's docs.
- A 120 s total budget can truncate unfiltered history that would complete. Mitigation: truncated entries are explicit and the variable raises the budget.
- Stub timing flakiness on CI. Mitigation: generous margins and asserting states, not tight durations.
- `chough timing` history is censored by the very slowness this addresses, so the estimate is by relative size only: small to medium.

## Constraints consulted

AGENTS.md, INTENT.md, hi/life.md (LIFE-3, 6, 7, 10), docs/FRD.md, REQ-cli-003, 020 and 021, issue #32 and siblings #30 and #31, PR #29's paths (not touched), and the measurements in context.md.

Ownership: Claude owns every path listed in this change. `.chough/cases/cli.cases.toml` is shared; coordinate before integration.
