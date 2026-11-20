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
- #370 (`tests/migrate_bugs.rs`, `tests/migrate.rs`): a returning bug, open session
  file and history event, each with edited bytes, are refused naming the file, the
  receipt and the way on; each with the receipt's bytes is removed as already
  migrated; a new bug migrates with the index rebuilt.
- #378 (`tests/migrate.rs`): three applies of one digest at once leave one receipt
  with one consent, and no second `applied: true`.

- #338 (`tests/migrate.rs`): the index is spelled `index.md` with its planned bytes
  and `INDEX.md` is gone, on macOS and in the Linux image; a planned write made to
  fail leaves every source in place. The bind-mount run is tester2's container.
- #375 (`tests/migrate.rs`): the existing metadata test now expects each emptied
  folder gone with its metadata files and named in the output, the digest unchanged,
  and metadata beside kept records still there.

Then every verify step, and the cases run on macOS and in the Linux image.
