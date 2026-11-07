---
change: after-the-move-into-done-chough-library-index-check-passes-with-no-command-in-between-session-notes-link-archive-notes
artifact: requirements
---

# Requirements

- MODIFIED REQ-cli-043 (`deltas/cli.md`): after a refreshed first pass, when the ledger rotation rotates or the active session archives itself, the move into done regenerates once more after the session's archive step. `library index --check` then passes right after the move, and `applied.librarian_index` reports the last regeneration. A new acceptance criterion covers an archive that lists the self-archiving session file.
- MODIFIED REQ-cli-049 (`deltas/cli.md`): the self-archive is the last step of the move into done that changes a record, and only the final knowledge regeneration follows it.
- MODIFIED REQ-cli-051 (`deltas/cli.md`): a session note links its native change's archive note when the same build writes that note, decided only from sources and never from notes already in the directory. A new acceptance criterion covers a roll-up into a project with no knowledge directory. KNOW-4 is added to its trace.

Trace: KNOW-4, KNOW-29, KNOW-23 (kept: rotation still after the refresh).
