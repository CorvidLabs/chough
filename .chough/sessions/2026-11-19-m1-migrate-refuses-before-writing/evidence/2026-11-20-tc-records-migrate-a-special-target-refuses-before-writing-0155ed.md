---
kind: evidence
id: 2026-11-20-tc-records-migrate-a-special-target-refuses-before-writing-0155ed
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting for TC-records-migrate-a-special-target-refuses-before-writing: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-a-special-target-refuses-before-writing]
case_digest: [f9a9b4bcd8ec15bd9a2d3f85e58c0e1b2c75880af7fac9fb1de4103e61dea597]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:702b22f99192a887589adeb9f22f436b73c81eccb71f757018c36b4d7391abfc
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:36:02Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting for TC-records-migrate-a-special-target-refuses-before-writing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:36:02Z. The raw log stays in the machine-local store.
