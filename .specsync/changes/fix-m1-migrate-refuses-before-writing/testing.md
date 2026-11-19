---
change: fix-m1-migrate-refuses-before-writing
artifact: testing
---

# Testing

One test per issue, each failing on main before the fix:

- #376 (`tests/doctor.rs`): after an in-place migration with a kept 0.5.x pointer,
  `doctor --fix` reports no `stale_pointer`, the pointer is byte-identical and `next`
  shows the session.
- #337 (`tests/migrate.rs`, `tests/migrate_knowledge.rs`): a directory, a file with
  no read permission, a non-UTF-8 file and a FIFO at `knowledge/index.md` and at the
  next receipt name; the preview returns, nothing is written, every source stays,
  the path in the refusal is relative.
- #370 (`tests/migrate_bugs.rs`): a returning bug with an edited title is refused by
  name in preview and apply; the unchanged one still migrates; a new one migrates
  with the index rebuilt.
- #378 (`tests/migrate.rs`): three applies of one digest at once leave one receipt
  with one consent, and no second `applied: true`.

Then every verify step, and the cases run on macOS and in the Linux image.
