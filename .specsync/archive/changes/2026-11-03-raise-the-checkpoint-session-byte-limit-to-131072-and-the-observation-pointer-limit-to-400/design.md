---
change: raise-the-checkpoint-session-byte-limit-to-131072-and-the-observation-pointer-limit-to-400
artifact: design
---

# Design

Add `SESSION_LIMIT = 131_072` and `MAX_OBSERVATIONS = 400` in `src/checkpoint.rs`. Add bounded variants `read_bounded_record(path, limit)` and `write_new_bounded` / `write_replace_bounded(..., limit)` in `src/records.rs`; the existing functions call them with `RECORD_LIMIT`, so every other record keeps 65,536 bytes. Checkpoint reads, validation, init, saves, migration and merge use `SESSION_LIMIT`. `ensure_ids` takes its limit, and only `observations` passes `MAX_OBSERVATIONS`.

128 KiB is twice the old bound and keeps the checkpoint well under the 1 MiB receipt bound that history protection already uses to read it. 400 pointers at the current ~90-byte observation IDs use about 36 KiB, within the new byte bound.
