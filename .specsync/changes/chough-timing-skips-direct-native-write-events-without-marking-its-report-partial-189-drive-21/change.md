---
id: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
state: approved
type: bug_fix
base_commit: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
---

# chough timing skips direct-native-write events, and chough change new passes --no-spec-change (189, DRIVE-21, DRIVE-20)

## Intent

chough timing skips direct-native-write events, and chough change new passes --no-spec-change (189, DRIVE-21, DRIVE-20)

## Affected Canonical Specs

- `cli`
- `history`
- `reporting`

## Acceptance Criteria

- REQ-history-013 (modified): chough timing reads a history holding a direct-native-write event as complete, skipping it as part of no stage segment, with no unrecognised kind and no partial mark; truly unknown kinds still mark the report partial. The new test in tests/direct_writes.rs fails first, a mutation that counts the kind again fails it, and fledge run linux-verify passes.

## No-spec Rationale

Not applicable
