---
kind: evidence
id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-ea5b56
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
duration_seconds: 6
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:459a88b381aa289f559feff1e4a3777e4dfa8c87da00ceb6ab75188895749e1d
criteria: [DRIVE-17]
requirements: [REQ-history-012, REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:09:18Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:09:18Z. The raw log stays in the machine-local store.
