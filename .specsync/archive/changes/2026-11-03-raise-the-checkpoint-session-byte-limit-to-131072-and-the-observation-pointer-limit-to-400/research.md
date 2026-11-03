---
change: raise-the-checkpoint-session-byte-limit-to-131072-and-the-observation-pointer-limit-to-400
artifact: research
---

# Research

- `src/records.rs` `RECORD_LIMIT` (65,536) bounds every structured record read (`read_record`) and managed write (`write_new`, `write_replace`); the checkpoint shares it through `read_session`, `migrate`, `read_merge_operand`, `save_checkpoint`, `init` and the merge result check.
- `src/checkpoint.rs` `MAX_ITEMS` (200) bounds every checkpoint list, including `observations` through `ensure_ids`.
- History protection reads the checkpoint with the 1 MiB receipt bound, so it needs no change. History events (`EVENT_BYTES`) do not embed the checkpoint.
- Main 747a12c checkpoint: 64,866 bytes, 197 observations. PR #50's merge: 68,737 bytes, 200 observations.
