## ADDED

### REQUIREMENT REQ-history-010

Relayed decision on gate records. A `gate-closed` transition MAY carry `relayed_by` with exactly the keys `decided_by`, `words`, `where` and `recorded_by`, each a non-blank string of at most 4096 bytes (REQ-cli-050). The field SHALL be omitted when absent, so every other transition keeps its exact bytes, and SHALL be carried unchanged by the legacy transitions log and by immutable history events. A `relayed_by` with a missing, blank, over-long or unknown key, or on a transition of any other kind, SHALL be refused as an invalid transition payload. The record is a relayed claim: it does not authenticate who decided. Trace: FLOW-16, FLOW-13.

Acceptance Criteria
- A transition without `relayed_by` serializes byte-identically to before.
- A relayed gate-closed transition written with history adopted, and one written to the legacy transitions log, are read back with the same four values.
- An event whose `relayed_by` lacks a key, has a blank or over-long value or an unknown key, or sits on a `gate-opened` or stage transition, is refused as an invalid transition payload.
