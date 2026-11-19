---
id: fix-262
state: archived
type: bug_fix
base_commit: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
---

# Fix 262

## Intent

fix 262

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- With a FIFO named like a bug record in .chough/bugs/, chough status, chough next and chough bug list each exit 0 without blocking. With a FIFO in place of an open session's session.md, chough session list reports that session as not readable with a reason and the recovery chough session shelve <id>, another session's commands still work, and chough session shelve <id> moves the folder without blocking.

## No-spec Rationale

REQ-history-006 already says a session that cannot be read is listed with the reason and the shelve recovery and stops no other command; only the readers and their tests change.
