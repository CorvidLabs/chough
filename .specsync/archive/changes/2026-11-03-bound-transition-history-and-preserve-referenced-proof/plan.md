---
change: bound-transition-history-and-preserve-referenced-proof
artifact: plan
---

# Plan

## Approach

One definition and implementation change for #31, integrating the already-merged checkpoint and identity work. The agent owns the declared source/test/docs paths on this branch. No additional agents are dispatched. Sequential product, QA, engineering and architecture findings are in scope-review.md.

## Steps

1. Reconcile current roadmap and issue mappings, preserve the historical baseline, finish this definition, and obtain one native scope decision.
2. Record failing regressions for legacy adoption, branch-lineage integration and retention changes in disposable repositories. Link real defects to local bugs before fixing them.
3. Implement immutable events and optional checkpoint cursor, explicit adoption, bounded readers and atomic current-cycle merge handling. Preserve native gate calls.
4. Implement cycle preview/authorized apply and conservative protection revalidation; integrate timing, next, context and stage views without joining unrelated timelines.
5. Add real Git integration and interruption fixtures; add curated case IDs and required docs/bundle predecessor registration. Keep installed edited copies safe and leave historical fixtures unchanged.
6. Run affected cases and the required Fledge lane, record scoped observations, close out lessons inside the open native change, then follow genuine human acceptance, review and archive order. No merge, release or live pruning is automatic.

## Relative estimate

Large persisted-data change. Main uncertainty is lineage selection and interrupted retention across current-cycle merges, not rendering or native tool startup. The planning timing command returned 12 plan-stage samples: min 3 s, median 953 s, max 6,323 s. These are elapsed wall-clock history, not effort, a deadline or a prediction for this change. The sample includes unrelated scopes; use relative size for this package.

## Risks

An optional cursor changes old-reader compatibility. Explicit adoption, actionable old-reader refusal, legacy regression tests and preserving schema-1 migration bound that risk. A new cycle may preserve proof indefinitely; preview must say protected growth prevents reclamation. Filesystem deletion has irreversible effects on uncommitted data, so default is preview, apply is authorized and revalidated, and this repository's old proof is never used as a cleanup fixture.

## Out of scope

Cold-start reruns or new grades; release/publication; changing proof in .specsync/archive or docs/exercises/handoff-cold-start; live retention apply; automatic agent coordination; additional stores for current state; sweeping backlog closure; rewriting numeric IDs; caches that substitute for native lifecycle checks; broad performance work from #32.

## Constraints consulted

AGENTS.md, existing HI/FRD, specs/cli requirements and lessons, PR #29 current-cycle merge semantics, PR #33 record identities, PR #34 timing/native boundaries, and the explicit human decision to proceed without Kimi.
