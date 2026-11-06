---
id: keep-the-five-most-recent-archived-sessions-and-roll-older-ones-into-the-knowledge-vault-know-29
state: archived
type: feature
base_commit: 73a7da5653312e907621c0165d0b5f903c22ff53
---

# Keep the five most recent archived sessions and roll older ones into the knowledge vault (KNOW-29)

## Intent

Keep the five most recent archived sessions and roll older ones into the knowledge vault (KNOW-29)

## Affected Canonical Specs

- `history`
- `cli`

## Acceptance Criteria

- At done with the librarian on, only the five most recent archived sessions (by their native change finalization, then key; none counts oldest) stay in .chough/sessions/archive; older ones move byte for byte, never replacing a file, into knowledge/attachments/<folder>/sessions/<key>.json, reported as applied.session_rollup; library index writes knowledge/sessions/<key>.md linking each, union-merged; with the librarian off nothing moves. The new tests fail first and fledge lanes run verify passes on Linux.

## No-spec Rationale

KNOW-29 (Leif, Capture as written, 2026-11-05 20:08 MST): only the five most recent done sessions stay in .chough/sessions/archive; older ones roll into the vault as a summary note with attachments.
