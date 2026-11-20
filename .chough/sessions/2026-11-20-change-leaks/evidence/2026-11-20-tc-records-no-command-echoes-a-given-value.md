---
kind: evidence
id: 2026-11-20-tc-records-no-command-echoes-a-given-value
aliases: []
claim: "Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-no-command-echoes-a-given-value]
case_digest: [76db36b53812aaa597a0496af0ccda5bd97b3a4831a688a83010c85202b2a4a9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - output_safety
  - "--"
  - "--exact"
  - no_command_echoes_a_value_given_on_the_command_line
exit_code: 0
tests_run: 1
duration_seconds: 10
revision: 38a01b22378e70a778b05f5da6891e6d6b41e13f
content: 6938808c10085e6558eb80e4f8523360fcb44ba436559dcd6187620cc9cd6a65
environment: local-macos-arm64
fingerprint: sha256:29d15eeccf9dc8819a3ac2135a53e236c14a348bb1b06a22442fb47f31d95ce2
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:06:58Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 38a01b22378e, observed 2026-11-20T17:06:58Z. The raw log stays in the machine-local store.
