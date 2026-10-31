---
change: prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package
artifact: tasks
---

# Tasks

- [ ] Rebase onto main after PRs #3, #7 and #8 merge.
- [ ] Add failing retention and package tests; record the first failure as a local bug.
- [ ] Implement the classifier, preview, apply, ledger and retention preference.
- [ ] Label current, historical, pruned and unavailable references in status, next, bug and validation output.
- [ ] Add the package include list and the known previous skill digests.
- [ ] Correct README, ROADMAP and the docs/architecture.md records table.
- [ ] Run the disposable retention exercise, a fresh-context resumption and the package and install check.

Human gates and post-verification steps (not tasks): LIFE-8 and LIFE-9 were captured in
the human's wording on 2026-10-31; scope approval; push and CI; the human implementation review;
native review and ship; any release decision.
