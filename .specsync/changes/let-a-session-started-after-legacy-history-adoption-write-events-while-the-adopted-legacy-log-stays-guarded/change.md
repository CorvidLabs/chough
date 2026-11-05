---
id: let-a-session-started-after-legacy-history-adoption-write-events-while-the-adopted-legacy-log-stays-guarded
state: approved
type: bug_fix
base_commit: 88466daafc72145962cade5c7406a9fa84fe570f
---

# Let a session started after legacy history adoption write events while the adopted legacy log stays guarded

## Intent

Let a session started after legacy history adoption write events while the adopted legacy log stays guarded

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- In a project that adopted a legacy transition log, a session started with chough session new writes events to its own cycle; the legacy log keeps its bytes, stays out of the new session's history, and a change to it still refuses writes in every session. The new test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within REQ-history-006 and the history adoption requirement: a new session must write events, and unexpected legacy changes still refuse writes; no requirement text changes.
