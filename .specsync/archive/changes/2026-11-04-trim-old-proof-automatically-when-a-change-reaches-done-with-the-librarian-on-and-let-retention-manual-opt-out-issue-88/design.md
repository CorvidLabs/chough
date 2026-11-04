---
change: trim-old-proof-automatically-when-a-change-reaches-done-with-the-librarian-on-and-let-retention-manual-opt-out-issue-88
artifact: design
---

# Design

- src/prefs.rs: `retention` accepts `manual` beside `off` (default) and `auto`.
- src/library.rs: the `library archive` body becomes `archive_value(root, dir,
  authority)`, used by the command unchanged. New `trim_at_done(root)` returns JSON and
  never fails: `skipped` when the librarian is off or `retention` is `manual`; otherwise
  it applies the post-archive summary with the recorded authority (a run consent or
  `retention auto`) or, failing that, "standing preference: librarian on (KNOW-13)", and
  returns `trimmed` with the summary, or `refused` with the reason (an open native
  change, an unreadable manifest, a validity change restored by KNOW-10).
- src/stage.rs: after `stage advance` into done is saved, `applied.librarian_trim`
  carries that result; the move itself never depends on it.
