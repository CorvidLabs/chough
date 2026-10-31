---
change: prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package
artifact: tasks
---

# Tasks

- [x] Bring main into this branch after PRs #3, #7 and #8 merged (merge 16f752b).
- [x] Add failing retention tests first (d761ff2: 0 passed, 8 failed against the unchanged code).
- [x] Implement the classifier, preview, apply, ledger and retention preference.
- [x] Label current, historical, pruned and unavailable references in status, next, bug and validation output.
- [x] Add the package include list and the known previous skill digests.
- [x] Correct README, ROADMAP and the docs/architecture.md records table.

Post-verification steps (not tasks): record the first failure as a local bug through case runs at d761ff2 and verify it at the fixed tip; run the disposable retention exercise, a fresh-context resumption and the package and install check; then re-run `change check`.

Human gates (not tasks): LIFE-8 and LIFE-9 were captured in
the human's wording on 2026-10-31; scope approval; push and CI; the human implementation review;
native review and ship; any release decision.
