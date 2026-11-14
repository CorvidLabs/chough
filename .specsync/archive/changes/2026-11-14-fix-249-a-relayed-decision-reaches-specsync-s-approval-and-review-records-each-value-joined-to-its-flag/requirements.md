---
change: fix-249-a-relayed-decision-reaches-specsync-s-approval-and-review-records-each-value-joined-to-its-flag
artifact: requirements
---

# Requirements

- REQ-records-008: a relayed gate close's native approval and review carry the relay's words, place and recording agent to SpecSync.
- REQ-cli-059: `chough change ship`'s review passes the relay the same way (one acceptance-criterion line; the rest of the requirement, including #284's cut-summary sentence, is unchanged).

See `deltas/cli.md` and `deltas/records.md`. Serves FLOW-16.
