## ADDED

### REQUIREMENT REQ-history-005

Backfill on transition records. A transition MAY carry `backfill` with `recorded_at_unix_seconds`, the full `commit` SHA and the `quote` it rests on, meaning its `at_unix_seconds` is the original time given when it was recorded later (REQ-cli-036). The field SHALL be omitted when absent, so every other transition keeps its exact bytes, and SHALL be carried unchanged by the legacy transitions log and by immutable history events. Trace: DRIVE-5, LIFE-7.

Acceptance Criteria
- A transition without a backfill serializes byte-identically to before.
- A backfilled transition written with history adopted is read back from its event with the same backfill.
