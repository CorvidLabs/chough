---
id: fix-329-check-first
state: archived
type: bug_fix
base_commit: 2e371c8855e4ea8ca965634b59c56c26b0329d06
---

# Fix 329 check first

## Intent

fix 329 check first

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With a session active that already holds its 200 bug links, chough bug add refuses before writing anything, naming the session and the limit, and no new .chough/bugs record exists afterwards; the session record is byte-identical. With room left, bug add writes the bug and links it as before.

## No-spec Rationale

REQ-cli-012 already says chough bug add while a session is active SHALL add the new bug to that session's checkpoint; a bug written but left out of a full session broke that, so bug add now checks that the session can take the link before writing anything and refuses otherwise, with only code, test and doc changes.
