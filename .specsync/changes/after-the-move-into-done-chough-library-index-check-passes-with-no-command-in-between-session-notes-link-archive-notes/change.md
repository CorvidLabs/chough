---
id: after-the-move-into-done-chough-library-index-check-passes-with-no-command-in-between-session-notes-link-archive-notes
state: approved
type: bug_fix
base_commit: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
---

# After the move into done, chough library index --check passes with no command in between: session notes link archive notes from their sources, and the knowledge directory is regenerated after the session archives itself (#191, KNOW-4)

## Intent

After the move into done, chough library index --check passes with no command in between: session notes link archive notes from their sources, and the knowledge directory is regenerated after the session archives itself (#191, KNOW-4)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- REQ-cli-043, REQ-cli-049 and REQ-cli-051 (modified): right after chough stage advance into done, with no command in between, chough library index --check passes. (1) A session rolled into a vault that held no archive note for its change before that move links its change's archive note, because whether the link is written depends only on the archive notes the same build writes, never on notes already in the directory (KNOW-4). (2) When the change's archive lists the session file that archives itself at done, the directory is regenerated once more after that self-archive, so the archive note places the file in Git; applied.librarian_index reports the last regeneration. Two tests in tests/reduce.rs fail first on main f41e7ab4 and pass after; the spec-sync reproduction in #191 passes the check; docs/cli.md describes the order and the change log names #191.

## No-spec Rationale

Not applicable
