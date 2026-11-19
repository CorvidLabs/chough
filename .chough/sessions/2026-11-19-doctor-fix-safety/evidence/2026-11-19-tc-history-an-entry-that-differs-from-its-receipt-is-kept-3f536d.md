---
kind: evidence
id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-3f536d
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-entry-that-differs-from-its-receipt-is-kept]
case_digest: [6b4fe2900d7a4a2f73e65527f91b42c9950373e5e9a1e32fd0643f55501b7d93]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_entry_that_differs_from_its_receipt_is_kept
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:a4afb6b4dacea0dbf6846f8c1e4bea6816e5817dd00cbd320e0d173fda8be5fe
criteria: [DRIVE-17]
requirements: [REQ-history-012, REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:43Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:43Z. The raw log stays in the machine-local store.
