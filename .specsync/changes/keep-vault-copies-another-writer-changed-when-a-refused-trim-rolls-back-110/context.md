---
change: keep-vault-copies-another-writer-changed-when-a-refused-trim-rolls-back-110
artifact: context
---

# Context

Issue #110 (P1, reproduced by Leif's review agent on merged main 2587fe0): when a trim's apply is refused, its rollback deletes every vault copy the apply created, by path only (`copies` holds path strings). If another writer changed one of those attachments after the apply created it, the rollback deletes that writer's bytes. Restoring the original `.chough/` source doesn't recover them. Leif's decision through the orc (2026-11-05 09:43 MST): Chough 0.4.0 is released after #112, #114 and #110, and this issue comes next in order.
