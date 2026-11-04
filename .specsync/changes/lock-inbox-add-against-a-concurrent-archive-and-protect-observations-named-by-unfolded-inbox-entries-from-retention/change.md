---
id: lock-inbox-add-against-a-concurrent-archive-and-protect-observations-named-by-unfolded-inbox-entries-from-retention
state: verifying
type: bug_fix
base_commit: 75a732d8243181c375ee3c4f2651408ff9b41ab2
---

# Lock inbox add against a concurrent archive and protect observations named by unfolded inbox entries from retention (issue 98)

## Intent

Lock inbox add against a concurrent archive and protect observations named by unfolded inbox entries from retention (issue 98)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- While another process holds the lock and archives the session inbox add waits and then refuses with no open session and writes no entry. An old observation named only by an unfolded inbox entry is protected and survives evidence prune apply. Each new test fails on 75a732d and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
