---
change: integrate-merged-main-134ee19-pr-46-report-reads-into-pr-48-identity-acceptance-evidence
artifact: plan
---

# Plan

## Approach

Merge main 134ee19 into PR #48 without a rebase, using a true three-way checkpoint merge against base 16fe55d. Resolve the text conflicts additively and update the one test call that the PR #46 rename broke. Then requalify the PR #48 cases at the combined source.

## Out of scope

- New behavior or changed criteria.
- Changing the 200-pointer or 65536-byte limits.
- Pruning, or rewriting old records or evidence.
- Cold-start results.
- Merging, closing or releasing the PR.
- Closing the #30 criteria that are still open.

## Steps

1. Save the raw base, ours and theirs operands. Run the three-way merge with ours as the cycle owner.
2. Union transitions without reordering main.
3. Renumber the version-history row and resolve CHANGELOG.
4. Update the test call.
5. Rerun the PR #48 cases, then verify, commit, push and observe CI.

## Risks

The checkpoint has about 870 bytes and 3 pointer slots of headroom, so checkpoint writes are kept short. A raw union would bring back main's 24 removals.

## Constraints consulted

- AGENTS.md
- skills/chough-workflow (lifecycle and accept references)
- PR #46 archived context
- issuecomment-6595229360
