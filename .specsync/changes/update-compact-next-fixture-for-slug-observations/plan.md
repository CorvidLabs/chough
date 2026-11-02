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

The actual fixture failure is retained in `OBS-tc-cli-0120-76a960d09d62f9e548d773cf` and linked to local bug `BUG-compact-next-fixture-writes-an-obsolete-observation-id-d23860cbf663336b9f83ae75`. This definition remains unapproved; no tests/next.rs edit has been made.
