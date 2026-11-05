---
id: keep-vault-copies-another-writer-changed-when-a-refused-trim-rolls-back-110
state: approved
type: bug_fix
base_commit: 99c02a57e1279237e6e2f80099d8bab1d6074dc4
---

# Keep vault copies another writer changed when a refused trim rolls back (110)

## Intent

Keep vault copies another writer changed when a refused trim rolls back (110)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A refused trim apply removes only the vault copies it created that still hold the bytes and mode it wrote; a copy another writer changed is kept as written and named in the refusal; the source is restored either way. The new test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within REQ-cli-046 and KNOW-10: a restore must not delete another writer's bytes; no requirement text changes.
