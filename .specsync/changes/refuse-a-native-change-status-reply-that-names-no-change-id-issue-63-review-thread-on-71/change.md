---
id: refuse-a-native-change-status-reply-that-names-no-change-id-issue-63-review-thread-on-71
state: approved
type: bug_fix
base_commit: efa397da2f4c196d5e5120e87b7125dd25a106c1
---

# Refuse a native change status reply that names no change ID (issue 63, review thread on #71)

## Intent

Refuse a native change status reply that names no change ID (issue 63, review thread on #71)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With SpecSync configured chough checkpoint --native-change refuses as refused:native-change-unknown when the status reply is JSON that names neither summary.id nor change.id (for example [] or {}) and .chough/session.json stays byte-identical even with other updates in the same call; a reply that names the requested ID still saves; a reply naming another ID still refuses as refused:native-change-mismatch; docs/cli.md states it; the new test fails on the branch before the fix and fledge lanes run verify passes

## No-spec Rationale

Not applicable
