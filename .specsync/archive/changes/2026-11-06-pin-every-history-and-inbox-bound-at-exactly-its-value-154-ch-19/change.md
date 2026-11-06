---
id: pin-every-history-and-inbox-bound-at-exactly-its-value-154-ch-19
state: archived
type: bug_fix
base_commit: 64aa77e01d2763395d8af7566e62743ede37cc32
---

# Pin every history and inbox bound at exactly its value (154, CH-19)

## Intent

Pin every history and inbox bound at exactly its value (154, CH-19)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- Each of the five unpinned history and inbox bounds has an end-to-end test at exactly its value and one past it, and a mutation that loosens any of the 11 history and session bounds by one is caught; no product, contract, assertion or bound changes; fledge lanes run verify passes on Linux.

## No-spec Rationale

Issue #154 (audit CH-19): a mutation check found five history and inbox bounds no test pins; tests only, no requirement text changes.
