---
change: split-the-cli-spec-move-the-timing-and-reporting-requirements-req-cli-020-req-cli-024-and-req-cli-028-to-a-new
artifact: testing
---

# Testing

- tests/spec_split.rs: each moved body's SHA-256 equals its digest at main a33c747;
  every old ID is gone from specs/cli and its new ID is in specs/reporting; every
  REQ-cli ID from 001 to 034 and 036 is in cli or mapped; every companion, reporting
  included, stays under the record bound.
- A one-time before/after comparison of every cli requirement block (old file at
  70009d6 against the three companions) is recorded as an observation artifact.
- Every moved and twinned case (`chough cases run TC-reporting-...`) and the six cli
  cases that lost a moved link run on local-linux-x86_64.
- chough next, status and bug list read the checkpoint and bug records that cite old
  requirement and case IDs without error.
- fmt, lint, the full tests and `fledge lanes run verify`.
