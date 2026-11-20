---
kind: evidence
id: 2026-11-20-tc-records-migrate-judges-returning-files-by-the-receipt-f2391b
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_returning_edited_session_is_refused_naming_it_and_its_receipt for TC-records-migrate-judges-returning-files-by-the-receipt: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-judges-returning-files-by-the-receipt]
case_digest: [6c7cfd3ac789b843371f448c58173940d0035062ba7f60ac2f55b3018ca95734]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_returning_edited_session_is_refused_naming_it_and_its_receipt
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:39b6339377aadf988e593ec0e2eb5c1480df988ab54c43968cc158a51b156215
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:58:32Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_returning_edited_session_is_refused_naming_it_and_its_receipt for TC-records-migrate-judges-returning-files-by-the-receipt: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T03:58:32Z. The raw log stays in the machine-local store.
