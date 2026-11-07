---
change: after-the-move-into-done-chough-library-index-check-passes-with-no-command-in-between-session-notes-link-archive-notes
artifact: tasks
---

# Tasks

- [x] Add the two tests in `tests/reduce.rs` and their cases. They fail first on main f41e7ab4.
- [x] Link session notes from the build's archive set (`src/vault.rs`, `src/library.rs`).
- [x] Regenerate after the session archives itself when a later step changed a source (`src/stage.rs`).
- [x] Update `docs/cli.md` and the CHANGELOG.
- [x] Modify REQ-cli-043, REQ-cli-049 and REQ-cli-051 (materialized from the deltas).

After `specsync change check` materializes the deltas, not task boxes:
- record the cases' failing-first and passing observations;
- run the mutation round;
- run `fledge lanes run verify`.
