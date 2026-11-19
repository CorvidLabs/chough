---
change: fix-349
artifact: tasks
---

# Tasks

- [x] Extend the receipt test so it fails on main: the What was verified line shows limitations.
- [x] The line gains `; limits: <limitations>` when the receipt has any.
- [x] Show it in the line format in `docs/cli.md`; add the case definition and the changelog fragment.

- [x] Bound what a person gives with `--limitation`: 8 per entry, 512 bytes each, in `evidence add` and `cases record`.

Human gates (not checkboxes): scope approval, implementation review, merge.
