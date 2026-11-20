---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-a-record-whose-id-is-held
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
duration_seconds: 2
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:571e314dc5e9cd51698bd1a9d3dc87d0f97ff88b0ad858ecf7e35de8db0d03a0
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:24:19Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused for TC-records-migrate-refuses-a-record-whose-id-is-held: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:24:19Z. The raw log stays in the machine-local store.
