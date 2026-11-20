---
kind: evidence
id: 2026-11-20-tc-records-migrate-a-special-target-refuses-before-writing-aef616
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
duration_seconds: 11
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:03b22611829a5a73703ad17193fab24969774ce3ce1365e7ff46ae24cb9c0a3b
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:59:15Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting for TC-records-migrate-a-special-target-refuses-before-writing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T03:59:15Z. The raw log stays in the machine-local store.
