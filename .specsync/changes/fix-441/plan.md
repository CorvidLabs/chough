---
change: fix-441
artifact: plan
---

# Plan

## Approach

Reuse the existing checkpoint locking, render, record validation and single-file replacement. Share close preparation with the single-gate path so that decisions, next-action generation, direct-write history and recovery use the same rules. The command publishes either the complete group or no local update. Check the native step and retry guard through the same public gate command.

## Out of scope

The command acts on the selected open session. Cross-session selection keeps its documented refusal. Native approvals and reviews stay in SpecSync, and a native result is durable in SpecSync independently of the Chough write. The existing `chough change ship` route still owns verifying changes.

## Steps

1. Add public CLI failing-first regressions at both documented capacities and preserve the successful grouped control. Add a focused case definition linked to FLOW-20 and REQ-cli-061.
2. Prepare all closures on one locked snapshot. Render the entire update before running any applicable native step. Publish the prepared checkpoint once and settle the direct-write baseline as a single close does.
3. Qualify the same-session native success, refusal and safe retry paths with controlled native outcomes and call counts. Verify that two sessions remain unchanged when the existing cross-session guard refuses a mixed selection.
4. Clarify the gate-state guarantee and native recovery in `docs/cli.md`. Add the issue changelog fragment and run the focused checks and repository verification lane after scope approval.

## Risks

The largest uncertainty is a native action that succeeded before Chough could publish the checkpoint. A retry must inspect native evidence and refuse rather than duplicate an action whose result cannot safely be identified. A local capacity failure must be detected before calling SpecSync. A concurrent writer must see either the old session or the single committed result. Preserve direct-write order and the error recovery users already receive from the single-gate path. Implementation and mixed-native testing are the medium-sized increments. Docs and changelog are small. The current timing record contains planning only for this issue.

## Constraints consulted

FLOW-20, REQ-cli-061, REQ-cli-050, REQ-cli-040 and its writer lock, REQ-cli-060 and its direct-write baseline, REQ-history-003, REQ-records-001, REQ-records-008, `docs/cli.md` Gate lists, and the existing gate and checkpoint tests.
