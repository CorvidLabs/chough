---
change: after-the-move-into-done-chough-library-index-check-passes-with-no-command-in-between-session-notes-link-archive-notes
artifact: design
---

# Design

- **Session notes** (`src/vault.rs`). `session_notes(root, dir, archived)` takes the set of archive IDs that `library::build` writes notes for (`found[*].id`). It links `../archives/<change>.md` when the session's native change is in that set, and no longer calls `exists` on the output directory. `build` passes the set it already holds.
- **The move into done** (`src/stage.rs`). After `session_archive`, the helper regenerates again when both hold:
  - `librarian_index.status` is `refreshed`;
  - `ledger_rotation.status` is `rotated` or `session_archive.status` is `archived`.

  It calls `library::refresh_at_done` again and stores that result in `applied.librarian_index`. The earlier steps and their order are unchanged. A refusal of the second pass is reported the same way and never undoes the move.
- **No new output field.** `applied.librarian_index` keeps its shape (`refreshed` with the index path, `skipped` or `refused`).
