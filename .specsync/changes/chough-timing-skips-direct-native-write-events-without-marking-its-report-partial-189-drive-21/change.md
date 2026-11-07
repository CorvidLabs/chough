---
id: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
state: draft
type: bug_fix
base_commit: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
---

# Chough timing skips direct-native-write events without marking its report partial (189, DRIVE-21)

## Intent

chough timing skips direct-native-write events without marking its report partial (189, DRIVE-21)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- REQ-history-013 (modified): chough timing reads a history holding a direct-native-write event as complete, skipping it as part of no stage segment, with no unrecognised kind and no partial mark; truly unknown kinds still mark the report partial. The new test in tests/direct_writes.rs fails first, a mutation that counts the kind again fails it, and fledge run linux-verify passes.

## No-spec Rationale

Not applicable
