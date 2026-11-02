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
