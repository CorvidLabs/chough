---
change: split-the-cli-spec-move-the-retention-and-transition-history-requirements-req-cli-018-and-req-cli-025-to-req-cli-027-to
artifact: design
---

# Design

No visual or interaction design. Module boundary only:

| Module | Owns files | Requirements |
|---|---|---|
| cli | every current file except the three below | REQ-cli-001 to 017, 019 to 024, 028 to 032 |
| history | src/history.rs, tests/history.rs, tests/evidence.rs | REQ-history-001 to 004 |

Requirement mapping: REQ-cli-018 -> REQ-history-001, REQ-cli-025 -> REQ-history-002,
REQ-cli-026 -> REQ-history-003, REQ-cli-027 -> REQ-history-004.

Case mapping: `TC-cli-<rest>` -> `TC-history-<rest>`, and
`TC-cli-history-<rest>` -> `TC-history-<rest>`.
