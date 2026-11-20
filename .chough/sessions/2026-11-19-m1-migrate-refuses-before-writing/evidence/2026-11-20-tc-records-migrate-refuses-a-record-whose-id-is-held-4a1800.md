---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-a-record-whose-id-is-held-4a1800
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
duration_seconds: 5
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:f59f7c6a4298ef7e9a8a55d149b184706603260d6353e76d147abbd82d5928fd
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:36:33Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused for TC-records-migrate-refuses-a-record-whose-id-is-held: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:36:33Z. The raw log stays in the machine-local store.
