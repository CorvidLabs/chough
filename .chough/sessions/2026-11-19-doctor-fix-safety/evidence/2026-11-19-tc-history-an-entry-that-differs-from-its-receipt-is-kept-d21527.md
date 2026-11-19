---
kind: evidence
id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-d21527
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-entry-that-differs-from-its-receipt-is-kept]
case_digest: [040df95e4eb215fb43ddf52a93138667b7245be119fa97244642d30cb7c6e83e]
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
duration_seconds: 10
revision: f09cf6ba0f1e40b8dab02f158f82c87adf20e886
content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
environment: local-macos-arm64
fingerprint: sha256:33ba80814cc380e115291a2ed3f7edcb9107a4e402e8a8130e60c1878306b79d
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:38:00Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f09cf6ba0f1e, observed 2026-11-19T05:38:00Z. The raw log stays in the machine-local store.
