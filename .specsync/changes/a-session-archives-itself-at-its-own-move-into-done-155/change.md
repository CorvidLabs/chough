---
id: a-session-archives-itself-at-its-own-move-into-done-155
state: implementing
type: feature
base_commit: b09682e0cbb62f08e9bc1ab52f335311a3da4b8e
---

# A session archives itself at its own move into done (155)

## Intent

A session archives itself at its own move into done (155)

## Affected Canonical Specs

- `history`
- `cli`

## Acceptance Criteria

- Issue #155 option D (Leif, relayed by orc 2026-11-06), under DRIVE-13 and DRIVE-3, in REQ-history-006: the move into done archives the active session itself in its own change and clears the active pointer (applied.session_archive archived); a session with unfolded inbox entries stays open (kept_open); a session with release authorized at done stays open for the release and archives itself at the move from release into done; without sessions it reports skipped; next then refuses naming chough session new <key>, and a following session new archives nothing. The one-time sweep moves the done sessions open on main into the archive in this change. The new tests fail first; the switch tests in tests/sessions.rs model a done session left open by an older version, with no assertion change, docs and the skill say so, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
