---
change: split-the-cli-spec-move-the-retention-and-transition-history-requirements-req-cli-018-and-req-cli-025-to-req-cli-027-to
artifact: testing
---

# Testing

- tests/spec_split.rs: each moved body's SHA-256 equals its digest at main 7bd1b62;
  every old ID is gone from specs/cli and its new ID is in specs/history; every
  REQ-cli ID from 001 to 032 is in cli or mapped; the cli companion stays under the
  record bound with headroom.
- A one-time before/after comparison of all 32 requirement blocks (old file at
  7bd1b62 against the two new companions) is recorded as an observation artifact.
- Every moved case (`chough cases run TC-history-...`) and the remaining cli cases
  touched by the module change run on local-linux-x86_64.
- chough next, status and bug list read the checkpoint and bug records that cite old
  case IDs without error.
- fmt, lint, the full tests and `fledge lanes run verify`.
