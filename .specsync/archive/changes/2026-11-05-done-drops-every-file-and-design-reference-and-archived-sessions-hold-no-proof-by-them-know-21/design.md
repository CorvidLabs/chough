---
change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
artifact: design
---

# Design

## Approach

1. **Drop at done.** `drop_closed_cycle_references` in src/stage.rs keeps its observation rule. It now also takes every file and design reference.
   - It saves once and reports the dropped `dropped_files` and `dropped_design` lists.
   - A missing reset time skips only the observation part.
   - An unreadable record still refuses the whole step, so nothing is dropped.
   - A failed save restores all three lists.
2. **Ignore archived sessions in retention.** `file_references` in src/evidence.rs skips checkpoints labeled `archived session <key>`, matching the label `crate::sessions::every_checkpoint` gives an archived session. Archived session files are never rewritten.

## Risks

- Display: the checkpoint at done no longer lists file and design references. The native archive keeps the change's paths, and the history keeps its transitions.
- Tests: the #109 folder-reference protection test used the done trim. Since the done move now drops the reference first, it is exercised by `library archive --apply` before done, with every assertion kept.

## Constraints consulted

- REQ-cli-041, REQ-cli-045, REQ-cli-046.
- KNOW-13, KNOW-16, KNOW-18.
- The specs/cli/context.md lessons on retention and the #108 review: an unreadable record refuses.
