---
change: prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package
artifact: plan
---

# Plan

## Approach

Add the prune classifier and ledger in `src/evidence.rs`, the `retention` preference
in `src/prefs.rs`, the freshness labels in status/next/bug/validation output, the
package include list and known digests, then correct the guidance.

## Out of scope

Publishing to crates.io, CI artifact upload, ignore-rule changes, compaction of the
ledger, SpecSync changes, new integrations or hosts.

## Steps

1. Rebase on main after PRs #3, #7 and #8 merge.
2. Write failing tests first; record the first failure as a local bug.
3. Classifier, preview, apply, ledger, labels; package and skill digests; docs.
4. Disposable retention exercise; fresh-context resumption; package and install check.

## Risks

Deleting evidence that something still needs (mitigated: protect on any reference,
fail closed when native reads fail, preview by default); confusing pruned with missing
(separate labels and a ledger).

## Constraints consulted

specs/cli/context.md lessons and the 2026-10-31 lessons recorded in this change's
constraints answer.
