---
kind: evidence
id: 2026-11-20-tc-records-no-command-echoes-a-given-value-f0d727
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
duration_seconds: 11
revision: 8ead84204edc55fcf5c30db35fb0beb6b7290cb2
content: 6880a79a758fb134e06561e2e779a9d6a9025ddc5484976b625f5c52cc4b2dea
environment: local-macos-arm64
fingerprint: sha256:46398c1aea5b7d813383406ddc016c539a7c5061aafe6931f8cdf5aec31b53aa
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:34:16Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8ead84204edc, observed 2026-11-20T17:34:16Z. The raw log stays in the machine-local store.
