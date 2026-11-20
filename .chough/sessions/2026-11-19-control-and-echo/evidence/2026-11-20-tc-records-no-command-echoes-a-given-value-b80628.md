---
kind: evidence
id: 2026-11-20-tc-records-no-command-echoes-a-given-value-b80628
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
duration_seconds: 12
revision: cdc0d2e2f85a6d09567b8780662656ccec551526
content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
environment: local-macos-arm64
fingerprint: sha256:f5c4ab0fddd2034904a17d39c4bc5cbf48c3bf2bce4d504947a8f907d3bd31a1
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:34:28Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cdc0d2e2f85a, observed 2026-11-20T04:34:28Z. The raw log stays in the machine-local store.
