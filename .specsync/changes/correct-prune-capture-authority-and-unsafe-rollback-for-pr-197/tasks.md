---
change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
artifact: tasks
---

# Tasks

- [x] After scope approval, land and commit three deterministic failure-first tests plus the test-only capture seam; run the exact capture and unsafe-restore filters before changing production behavior.
- [x] Reject captured artifact bytes that do not match the selected eligible digest, restoring any earlier removals.
- [x] Contain unsafe restoration-path errors per file, preserve original recovery bytes and second-writer symlinks, continue earlier restores and append honest accounting for all ledgered sets.
- [x] Materialize only the CLI Purpose/history Error Cases section deltas, update history testing/context and add the new per-change case file without editing prior assertions or case definitions.

Verification and case recording are specified in testing.md, not task checkboxes. Scope approval was recorded by orc-t19f at c763d13d41edae6ec09c76b10333189118e1e668; only the two approved section deltas are left for native materialization. Fresh exact-head human implementation review and native archive are separate later gates; no old acceptance is reused for corrected code.
