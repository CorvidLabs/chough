---
kind: evidence
id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-b7d61f
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
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:794dfae31c6db759bc183b5585d2cefd56f8ea8a3636b89aa5eb01363f071ee0
criteria: [DRIVE-17]
requirements: [REQ-history-012, REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:48:58Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:48:58Z. The raw log stays in the machine-local store.
