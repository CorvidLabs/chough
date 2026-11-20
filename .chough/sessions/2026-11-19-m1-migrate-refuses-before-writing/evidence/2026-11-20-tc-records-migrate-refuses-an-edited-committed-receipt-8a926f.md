---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-an-edited-committed-receipt-8a926f
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored for TC-records-migrate-refuses-an-edited-committed-receipt: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-refuses-an-edited-committed-receipt]
case_digest: [365b8b4c3b4cd60cec0c86242516373eedbe64be6894863ce037fa128aa1e103]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - an_edited_committed_receipt_is_refused_until_committed_or_restored
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:c4b3e660274d2f29350b300ce510e0c6996a4fb21acaf32d3f08f58ab88170d5
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:58:41Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored for TC-records-migrate-refuses-an-edited-committed-receipt: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T03:58:41Z. The raw log stays in the machine-local store.
