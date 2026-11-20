---
kind: evidence
id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-f8ba89
aliases: []
claim: "Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-control-character-is-refused-at-write]
case_digest: [0d4b3f0897a1f6b85befdccd570871a59f03bbe3f14a853a018f3cc972d3b706]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - output_safety
  - "--"
  - "--exact"
  - a_control_character_is_refused_at_write_in_every_field
exit_code: 0
tests_run: 1
duration_seconds: 8
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:94ffd852a3778ea73dc29a6dd02dace2c7b66c76064e6636a9df80f8b6b5c9f6
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:26:06Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:26:06Z. The raw log stays in the machine-local store.
