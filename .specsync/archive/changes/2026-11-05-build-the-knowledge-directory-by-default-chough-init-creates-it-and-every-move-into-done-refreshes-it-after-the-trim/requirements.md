---
change: build-the-knowledge-directory-by-default-chough-init-creates-it-and-every-move-into-done-refreshes-it-after-the-trim
artifact: requirements
---

# Requirements

ADDED REQ-cli-043 (see `deltas/cli.md`). With the librarian on, `chough init` creates
the knowledge directory create-only. Every move into done regenerates it after the trim
and reports `applied.librarian_index`; that report never undoes the move, and nothing is
built with the librarian off. Trace: KNOW-1, KNOW-4, KNOW-5, KNOW-13.
