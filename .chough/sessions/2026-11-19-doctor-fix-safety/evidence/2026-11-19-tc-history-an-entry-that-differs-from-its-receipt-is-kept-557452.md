---
kind: evidence
id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-557452
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
duration_seconds: 3
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:fc08af93dfabfa06d4018a95df19a00478eaed7844f4f86f4ec6e3458dcae1d2
criteria: [DRIVE-17]
requirements: [REQ-history-012, REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:13:21Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:13:21Z. The raw log stays in the machine-local store.
