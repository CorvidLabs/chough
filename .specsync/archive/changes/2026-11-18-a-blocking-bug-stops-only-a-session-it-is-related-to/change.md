---
id: a-blocking-bug-stops-only-a-session-it-is-related-to
state: archived
type: bug_fix
base_commit: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
---

# A blocking bug stops only a session it is related to

## Intent

A blocking bug stops only a session it is related to

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- An open blocking bug stops leaving test only for a session that lists it, has its evidence prefixed with that session id, or shares a criterion, requirement, or case. Another session advances, and chough next names the bug and says it does not block this session. A blocking bug that names nothing stops no session. A related blocking bug still refuses with refused:blocking-bugs. Closing it releases that session.

## No-spec Rationale

Not applicable
