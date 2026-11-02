---
change: bound-transition-history-and-preserve-referenced-proof
artifact: context
---

# Context

Issue #31 asks for bounded history without losing resumability, independent branch events or proof. 0xLeif said "Ok continue" after the recommendation to reconcile stale tracking, prepare this definition and present scope approval before implementation. That is authorization to prepare this package, not approval of its proposed persisted format or policy.

Baseline: origin/main 825a3805530456005e361dfa6d96d93eee52cd09, merged PR #36. Main CI and Push on main both completed successfully. There were no open PRs at pickup. This definition is on 0xleif/feat/bounded-history-definition.

Existing intent: LIFE-2, LIFE-3, LIFE-6, LIFE-7, LIFE-8 and LIFE-10; FLOW-2, FLOW-7 and FLOW-13. No HI criterion is added or reworded. The single checkpoint and its current-cycle merge group from PR #29 remain authoritative. SpecSync owns approval, verification, review and archives. Current-agent-first remains selected. Rune, APS and a second writable checkpoint remain outside scope.

Prior acceptance remains unchanged: PR #35's cold-start proof failed; PR #36's Kimi baseline, change and real-repository runs were not run by 0xLeif's choice. This package requests no substitute grade and changes no recorded evidence or frozen archive.

Read before drafting: AGENTS.md, INTENT.md, hi/life.md, docs/FRD.md, specs/cli/cli.spec.md and companions, the workflow stage, plan and checkpoint references, and specs/cli/context.md lessons. The latest recorded failure is OBS-product-test-bfc159865a39075249fb6e40, linked to the verified identity-main librarian bug. Source lessons require bounded fixtures, explicit unknowns, source-linked proof and closeout before native verification.

## Implementation observations

2026-11-02, materialization order (observation): SpecSync 6.1.2 refused `change check` while implementation tasks were open, before adding the new requirement IDs that bound case validation needs. The drafted last task mixed those prerequisites with post-materialization case/lane execution. Resequence the task checklist honestly: finish implementation and Rust regression checks, materialize through the native owner, then execute all bound cases and the mandatory verify lane as explicit remaining obligations. No check is marked passed before it runs, and no acceptance criterion or approved delta is changed. Source: the native refusal and the existing specs/cli/context.md closeout-order lesson. Scope: this native workflow version; revisit if SpecSync adds separate materialization.

2026-11-02, cleanup ordering (observation): an intermediate development fixture reported one removal where two eligible removals were expected because deleting a newer cycle first hid an ancestor needed by fresh protection checks. Source: .chough/artifacts/history-retention-ordering-development.log. Limits: that intermediate source identity was not retained, so this log is development evidence, not historical native qualification. Current code orders candidates by selected ancestry from oldest to newest; the real Git and retention fixtures assert retained hashes and explicit checkpoint selection. Future action: preserve causal links needed by revalidation and keep timestamp ties protected; lexical/random identity order is not chronology.

Integration update: approved main d839fef (#47) arrived after the9d02 qualification. Its unchanged setup/hint inputs are imported under the archived upstream change. Earlier exact-source results remain retained; final qualification must cover this new combined source before closeout. No new product policy or human decision is inferred.
