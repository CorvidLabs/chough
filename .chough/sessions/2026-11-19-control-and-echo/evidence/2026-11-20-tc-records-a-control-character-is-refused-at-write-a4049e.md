---
kind: evidence
id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-a4049e
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
duration_seconds: 20
revision: cdc0d2e2f85a6d09567b8780662656ccec551526
content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
environment: local-macos-arm64
fingerprint: sha256:d008952dd4fa1fc4ea1dabb35f6ae814bd13d1d3036dde26c3945531539c920c
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:33:50Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cdc0d2e2f85a, observed 2026-11-20T04:33:50Z. The raw log stays in the machine-local store.
