---
kind: evidence
id: 2026-11-20-tc-records-migrate-judges-returning-files-by-the-receipt
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
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:637f8e12bac3bc35a9dffccd0cb3447b6a24d591cac4945b3e3292ebf0e8431f
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:23:36Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_returning_edited_session_is_refused_naming_it_and_its_receipt for TC-records-migrate-judges-returning-files-by-the-receipt: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:23:36Z. The raw log stays in the machine-local store.
