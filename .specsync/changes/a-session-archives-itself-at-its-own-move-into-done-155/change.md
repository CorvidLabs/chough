---
id: a-session-archives-itself-at-its-own-move-into-done-155
state: draft
type: feature
base_commit: 615b587b33a1ef3a569f9557e2782a6be3df2f0f
---

# A session archives itself at its own move into done (155)

## Intent

A session archives itself at its own move into done (155)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- Issue #155 option D (Leif, relayed by orc 2026-11-06), under DRIVE-13 and DRIVE-3, in REQ-history-006: the move into done archives the active session itself in its own change and clears the active pointer (applied.session_archive archived); a session with unfolded inbox entries stays open (kept_open); without sessions it reports skipped; next then refuses naming chough session new <key>, and a following session new archives nothing. The one-time sweep moves the done sessions open on main into the archive in this change. The three new tests fail first, no existing assertion changes, docs and the skill say so, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
