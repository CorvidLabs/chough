---
change: require-native-lifecycle-closure-before-chough-pull-requests-merge
artifact: plan
---

# Plan

## Approach

Use the published native SpecSync lifecycle readers for current health, inventory, and authenticated archive history. Add a small deterministic adapter only for closure policy, protocol validation, Git snapshot inventory, and error reporting. Preserve the current product gates, keep this branch independent of runtime PR #2, and implement the explicitly authorized CI work under the actual optional policy without changing that policy or fabricating native approval.

## Out of scope

Product/runtime changes; Claude's checkout or branch; approvals/reviews/archives fabricated by CI or this agent; repository-wide branch protection changes; deployment/release; V migration; broad TODO scanning; semantic human acceptance inferred from text; feature checks for unmerged interfaces; external-service or global-tool changes.

## Steps

1. Prepare this concrete scope and regression matrix; implement the explicitly authorized work under the current optional policy. Preserve the native draft and do not manufacture approval. A genuine native definition decision remains necessary for native verification/finalization of this change.
2. Verify published action commits and release assets, then implement workflow/helper/fixtures/docs within the approved paths. Preserve the existing gates and explicit version consistency.
3. Exercise all fixture cases with SpecSync 6.1.1, including authenticated archived success and missing/corrupt/omitted history failures; verify read-only behavior by hashes.
4. Run the unchanged product/platform-compatible local lane, HI, strict SpecSync, and the new fixture task. Validate workflow structure and job names. Local macOS testing does not substitute for hosted Ubuntu/Windows jobs.
5. Push the independent implementation tip and wait for actual hosted product/native/fixture checks. Expect Archive red while this real native change remains active.
6. Obtain genuine scoped human implementation review, run native verification/finalize in its required order, push the archive tip, and wait for all six checks on that exact tip. Do not merge before archive. Do not force green by omitting this scope.
7. Propose requiring the six exact contexts after this check exists on main; re-read actual branch protection/rulesets at that time. Do not alter rules here.

## Risks

SpecSync audit skips archived history, so audit alone cannot satisfy Archive. Native status can return exit zero with unhealthy summary values, so parse and require its integrity result. Bootstrap has SDD disabled, so closure cannot depend only on `report.enabled`. A shallow checkout breaks native history; require full base/history. An omitted scope can disappear from the final diff, so committed snapshot inventory is mandatory. Concurrent PR #2 changes workflow/audit tasks and will need deliberate integration after this isolated CI change.

## Constraints consulted

`AGENTS.md`, `INTENT.md`, LIFE-6/LIFE-7, the current CLI contract/companions, current workflow/Fledge configuration, published SpecSync v6.1.1 CLI help and source, its upstream Archive script, release metadata, and read-only GitHub branch/ruleset responses. Native tools own lifecycle judgments; structural passes do not prove product or human acceptance.
