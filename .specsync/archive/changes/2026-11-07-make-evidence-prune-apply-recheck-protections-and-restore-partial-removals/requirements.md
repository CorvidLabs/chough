---
change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
artifact: requirements
---

# Requirements

Modify REQ-history-004, Conservative evidence cleanup, as recorded in `deltas/history.md`.

The requirement continues to preserve the existing eligibility thresholds, protection semantics, ledger entry shape and bounded reads. It adds these observable failure guarantees:

- A protection fingerprint is rechecked immediately before each artifact and observation removal.
- Every removal uses the existing byte-checked `remove_verified` hold, with artifacts removed before the observation record.
- A failure after the set's ledger line restores every earlier removal with create-new writes and original permissions, never replacing a file recreated meanwhile.
- Original bytes that cannot be restored are retained under `.chough/removing/` and named in the refusal; Git HEAD is not assumed to contain them.
- A `restored` event uses the existing ledger shape for each ledgered set, so readers do not report the failed apply as completed pruning.
- Successful apply output and all existing protections, bounds and authorization remain unchanged.

The `cli` delta modifies only the module Purpose to acknowledge the failure-handling behavior implemented in `src/evidence.rs`; it adds no requirement ID and leaves every CLI requirement unchanged.

No new requirement ID is needed.
