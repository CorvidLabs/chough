---
change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
artifact: tasks
---

# Tasks

- [x] Add the protection-race, same-set and earlier-set rollback, and recreated-file conflict regressions plus their per-change cases; run them before the fix and retain failure-first evidence.
- [x] Implement stale-hold refusal, prepared byte-checked removals, immediate protection rechecks, rollback and restored ledger events in `src/evidence.rs`.
- [x] Update REQ-history-004 and the CLI Purpose, testing and contract companions, FRD and CLI documentation, and the Unreleased changelog.

Verification: complete the focused tests and cases, full Fledge verify on macOS and Linux in Docker, and record only observed outcomes in Chough evidence and this change's native verification records.

Human gates: scope approval was recorded by the orc as 0xLeif at `80fbe1df895f78d8db8fa33daba54914bcad80aa`. Implementation review remains a human gate through the orc. Running and recording the Chough cases is specified in `testing.md`, not listed as task checkboxes here. Integrate origin/main before PR and merge, then archive and deliver through the orc.
