---
change: update-compact-next-fixture-for-slug-observations
artifact: plan
---

# Plan

## Approach

Remove the obsolete explicit observation ID, parse the returned recorded path, load that record and use its actual ID in the checkpoint fixture. Preserve all existing assertions and legacy stale records.

## Out of scope

Production source, contract changes, assertion weakening, other fixtures, and human implementation acceptance. Do not amend the approved identity scope.

## Steps

1. Obtain the separate native definition approval for this one-file update.
2. Apply the prepared tests/next.rs patch and rerun the unchanged compact-next regression.
3. Run the full Rust suite and required Fledge checks with the identity implementation.
4. Present the fixture follow-up and identity implementation together for human review.

## Risks

Hardcoding another ID could hide automatic-allocation behavior. Reference the returned record instead. Preserve 500 stale records, 7 unreferenced records, 40 gates/blockers, the 16384-byte cap and one native read per form.

## Constraints consulted

FLOW-6, FLOW-7, LIFE-2, LIFE-7, REQ-cli-021 and the approved identity policy. This change depends natively on use-slug-identities-for-all-new-chough-records. Relative size: very small. No product choice changes.

The actual fixture failure is retained in `OBS-tc-cli-0120-76a960d09d62f9e548d773cf` and linked to local bug `BUG-compact-next-fixture-writes-an-obsolete-observation-id-d23860cbf663336b9f83ae75`. 0xLeif subsequently approved this exact one-file scope by replying "yes i approve" on 2026-11-01. Native approval was recorded before applying the prepared patch. The full tests/next.rs suite then passed all 3 tests with Rust 1.99.1, including the original failing regression; every existing assertion remains unchanged.

## Observed native ordering blocker

Both unfiltered Rust suites passed all 182 tests. Full verify nevertheless failed at audit, step 6: tests/next.rs is not counted as covered while this follow-up depends on the identity change being accepted first. `specsync change check update-compact-next-fixture-for-slug-observations` returned exit 1: the parent is verifying and must be accepted before this change can start. The accepted-first edge was an unsuitable planning choice for a co-reviewed fixture. The current CLI has no dependency removal/amend command; reopen applies to stale accepted evidence, correct supports only public_contract/architecture_risk, and supersede requires an accepted predecessor. Preserve the valid approval and failed command evidence; do not rewrite native state, fabricate parent acceptance or weaken the audit. Native closeout remains blocked pending a supported ordering resolution.
