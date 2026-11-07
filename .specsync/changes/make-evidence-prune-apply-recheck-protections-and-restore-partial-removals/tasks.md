---
change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
artifact: tasks
---

# Tasks

- [ ] Add the protection-race, same-set and earlier-set rollback, and recreated-file conflict regressions plus their per-change cases; run them before the fix and retain failure-first evidence.
- [ ] Implement stale-hold refusal, prepared byte-checked removals, immediate protection rechecks, rollback and restored ledger events in `src/evidence.rs`.
- [ ] Update REQ-history-004, testing and contract companions, FRD and CLI documentation, and the Unreleased changelog.
- [ ] Complete focused and full verification, preserve the resulting evidence and resolve any failure without weakening a test, gate, bound or timeout.
- [ ] Integrate origin/main, complete native verification and human implementation review, then archive and deliver the PR through the orc.

Human scope approval and implementation review are gates, not task checkboxes. Running and recording the Chough cases is specified in `testing.md`, not listed as task checkboxes here.
