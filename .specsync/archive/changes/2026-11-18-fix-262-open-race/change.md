---
id: fix-262-open-race
state: archived
type: bug_fix
base_commit: caa6ee65a46ec3f4798e0a64c22bb263d062814a
---

# Fix 262 open race

## Intent

fix 262 open race

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- The bounded record reader opens a path without blocking and checks on the opened handle that it is a regular file, so a FIFO at the path at the moment of the open is refused at once; chough session shelve reads session.md and each listed file through the same non-blocking open.

## No-spec Rationale

REQ-history-006 already says a session that cannot be read stops no other command; this closes a window in the reader added for #262. Only the readers and their tests change.
