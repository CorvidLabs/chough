---
kind: evidence
id: 2026-11-20-tc-records-migrate-an-uncommitted-receipt-judges-nothing-f08ebb
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does for TC-records-migrate-an-uncommitted-receipt-judges-nothing: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-an-uncommitted-receipt-judges-nothing]
case_digest: [b0e42920b60fac10644ef01752dd88c198e7f0b3a4c97573934f7846643694ca]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - an_uncommitted_receipt_judges_nothing_and_a_committed_one_does
exit_code: 0
tests_run: 1
duration_seconds: 7
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:db814052503edca0603aba01405b3d64ca6fe1ceb8eab8c0c69ba76fdd672dac
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:58:50Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does for TC-records-migrate-an-uncommitted-receipt-judges-nothing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T03:58:50Z. The raw log stays in the machine-local store.
