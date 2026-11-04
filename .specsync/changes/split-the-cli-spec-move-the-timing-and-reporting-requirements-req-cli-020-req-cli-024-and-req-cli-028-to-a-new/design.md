---
change: split-the-cli-spec-move-the-timing-and-reporting-requirements-req-cli-020-req-cli-024-and-req-cli-028-to-a-new
artifact: design
---

# Design

No visual or interaction design. Module boundary only:

| Module | Owns files | Requirements |
|---|---|---|
| cli | every current file except the two below and history's | REQ-cli-001 to 017, 019, 021 to 023, 029 to 034, 036 |
| history | unchanged | REQ-history-001 to 005 |
| reporting | src/timing.rs, tests/timing.rs | REQ-reporting-001 to 003 |

Requirement mapping: REQ-cli-020 -> REQ-reporting-001, REQ-cli-024 ->
REQ-reporting-002, REQ-cli-028 -> REQ-reporting-003.

Case mapping: `TC-cli-<rest>` -> `TC-reporting-<rest>`, and `TC-cli-report-<rest>`
-> `TC-reporting-<rest>` (the repeated `report` is dropped). Moved: TC-cli-0081,
0083 to 0087, report-case-read-budget, report-read-invalidation and
report-trace-parity. Twinned (the cli case keeps its other links, the twin carries
the moved requirement): TC-cli-0080, 0082, 0088, 0089, 0100 and 0104.
