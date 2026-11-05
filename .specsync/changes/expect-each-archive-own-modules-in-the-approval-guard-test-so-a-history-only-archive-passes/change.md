---
id: expect-each-archive-own-modules-in-the-approval-guard-test-so-a-history-only-archive-passes
state: implementing
type: bug_fix
base_commit: 322a2e134fac0e4290b53b7dfa211fef6a6bde0c
---

# Expect each archive own modules in the approval guard test so a history-only archive passes

## Intent

Expect each archive own modules in the approval guard test so a history-only archive passes

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- the_guard_accepts_every_archived_approval_in_this_repository passes with the history-only #96 archive; it still requires unchanged deltas naming only modules in each archive approved scope, or no semantic deltas; it fails before the change on this branch; fledge lanes run verify passes on Linux.

## No-spec Rationale

Test-only: the guard test assumed every archived change touches cli; the history-only #96 archive reports unchanged: history. Product behavior is unchanged.
