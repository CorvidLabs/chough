---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-a-record-whose-id-is-held-d53d68
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused for TC-records-migrate-refuses-a-record-whose-id-is-held: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-refuses-a-record-whose-id-is-held]
case_digest: [6f451587dac58b87c23df4915220cba3f6c0ebf8f489832ca9128dbb1b79f435]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused
exit_code: 0
tests_run: 1
duration_seconds: 4
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:9b40c331cfbbc19658dabd7e0756860e37977e7881116f6f5fef4c404c3b6995
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:59:50Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused for TC-records-migrate-refuses-a-record-whose-id-is-held: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T03:59:50Z. The raw log stays in the machine-local store.
