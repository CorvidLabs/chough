---
kind: evidence
id: 2026-11-20-tc-records-no-command-echoes-a-given-value-576667
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
revision: f34dcc8391b47cd2b32c04a89933af75497bc177
content: 9c46ada16719ee7f5bc1b93a1452cb6032aad8f285116a7b70082427b6284a88
environment: local-macos-arm64
fingerprint: sha256:a1140371f65deed1916f9b5384cecb9b6bccd61a0298db1496dea9bbde9153db
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T22:15:54Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f34dcc8391b4, observed 2026-11-20T22:15:54Z. The raw log stays in the machine-local store.
