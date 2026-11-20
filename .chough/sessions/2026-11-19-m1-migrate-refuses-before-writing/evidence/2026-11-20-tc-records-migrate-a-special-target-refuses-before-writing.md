---
kind: evidence
id: 2026-11-20-tc-records-migrate-a-special-target-refuses-before-writing
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
duration_seconds: 4
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:b4581ecd8d581f69482492a721e7cab8697bd42cc94f0d2e99c1587f1887f8fa
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:24:02Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting for TC-records-migrate-a-special-target-refuses-before-writing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:24:02Z. The raw log stays in the machine-local store.
