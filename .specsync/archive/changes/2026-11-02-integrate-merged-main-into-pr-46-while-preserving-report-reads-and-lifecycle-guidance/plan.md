---
change: integrate-merged-main-into-pr-46-while-preserving-report-reads-and-lifecycle-guidance
artifact: plan
---

# Plan

## Approach
Merge current origin/main into the existing PR #46 branch. Reconcile only overlapping integration material. Preserve all previously approved semantic requirements and their implementations. Advance canonical version metadata beyond main and retain both changelog histories.

## Out of scope
New public behavior, persistent caches, new runtimes, archive edits, proof rewrites, force pushes, rebases, PR merge, release and deployment.

## Steps
Preserve original merge operands under /tmp. Merge main. Use Chough's checkpoint merger; if the current-cycle group diverges, keep PR #46's complete group as explicitly selected by the checkpoint owner and preserve additive collections. Preserve every appended transition occurrence. Resolve canonical metadata and package compatibility additively. Prepare existing cases and verification. Record current local qualification and native verification, commit and push the existing branch. Root performs native review and ship later.

## Risks
A textual merge may lose a requirement, case, digest or proof reference. Compare both parents and retain exact old proof bytes. Different archived changes can share a canonical version; reconcile monotonic version metadata without editing archives. Report diagnostics must preserve typed cause chains. This is a small bounded integration with conflict preservation as its main uncertainty; no wall-clock deadline is inferred.

## Constraints consulted
AGENTS.md; skills/chough-workflow/SKILL.md and lifecycle guidance; INTENT.md; LIFE-1, LIFE-2, LIFE-3, LIFE-6, LIFE-7 and FLOW-13; specs/cli/context.md diagnostic and native-ordering lessons. Root owns PR #37; other workers own PR #44 and Grok changes. This worker owns only this isolated PR #46 integration.
