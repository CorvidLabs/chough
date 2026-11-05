---
id: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
state: archived
type: bug_fix
base_commit: a6cfd3298d70b8b3ce6e2de92a25c817c4cbb324
---

# Stage each vault copy privately so a failed copy never deletes another writer bytes (115 review)

## Intent

Stage each vault copy privately so a failed copy never deletes another writer bytes (115 review)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A vault copy is written and given its mode in a private staging file and then linked into place without replacing anything, so a failed write or mode change removes only the staging file and never another writer's bytes at the destination. The new unit test fails first, the evidence and reduce suites pass, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within REQ-cli-046 and KNOW-10, the same class as #110: a failed copy must not delete another writer's bytes; no requirement text changes.
