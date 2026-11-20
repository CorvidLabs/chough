---
kind: evidence
id: 2026-11-20-tc-records-migrate-judges-returning-files-by-the-receipt-0e6c40
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
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:7d0900b007b0899578e9ecc222aaa8fafb8a00f6ec3d4be53fb5639e789f48ae
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:35:41Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_returning_edited_session_is_refused_naming_it_and_its_receipt for TC-records-migrate-judges-returning-files-by-the-receipt: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:35:41Z. The raw log stays in the machine-local store.
