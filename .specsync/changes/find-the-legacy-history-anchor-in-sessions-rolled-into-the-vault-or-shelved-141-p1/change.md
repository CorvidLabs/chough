---
id: find-the-legacy-history-anchor-in-sessions-rolled-into-the-vault-or-shelved-141-p1
state: implementing
type: bug_fix
base_commit: 260ecc949ea01214f4bec43b30ea4226ce8fae59
---

# Find the legacy history anchor in sessions rolled into the vault or shelved (141, P1)

## Intent

Find the legacy history anchor in sessions rolled into the vault or shelved (141, P1)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- A session without its own legacy anchor keeps writing gate and stage events when the session holding the anchor has been rolled into the vault or shelved, finding the anchor there with the same exact match; a changed legacy log is still refused. The new test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Issue #141 P1 regression from KNOW-29 (#135): adopted_anchor scanned only open and archived sessions; the adopting session rolled into the vault and every other session refused writes. No requirement text changes.
