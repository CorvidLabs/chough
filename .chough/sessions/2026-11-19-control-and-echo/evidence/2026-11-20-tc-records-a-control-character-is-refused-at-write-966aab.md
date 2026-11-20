---
kind: evidence
id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-966aab
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
duration_seconds: 25
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:47b4dba3423981298ecab0c9f774e61001de1b33e9615b19f9e28a3008cb2c36
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:11:56Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:11:56Z. The raw log stays in the machine-local store.
