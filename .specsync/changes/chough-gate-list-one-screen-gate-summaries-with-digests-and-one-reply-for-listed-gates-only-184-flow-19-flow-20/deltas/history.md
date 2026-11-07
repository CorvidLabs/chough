## ADDED

### REQUIREMENT REQ-history-014

Summary digests on relays. A `relayed_by` record (REQ-history-010) MAY carry `summary_digest`, 64 lowercase hex digits, and `list`, 12 lowercase hex digits, both or neither. A record with only one of them, or with a malformed value, SHALL be refused as an invalid payload. Records without them SHALL keep their bytes. Chough builds before this requirement refuse a relay holding them (unknown field), so the change log SHALL name the first release that contains it as the oldest Chough that reads it. Trace: FLOW-19, FLOW-20.

Acceptance Criteria
- A relay with both fields reads back; a relay with one of them, or with a malformed value, is refused as an invalid transition.
- The change log's entry for this change names `summary_digest` and the oldest Chough release that reads it.
