---
kind: evidence
id: 2026-11-20-tc-records-no-command-echoes-a-given-value-d1cdc9
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
duration_seconds: 9
revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
environment: local-macos-arm64
fingerprint: sha256:362e1d6e99bd6d31a9f4c67837269663457558e9a676939a68baf95cee454026
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:39:07Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6816b8fe5e23, observed 2026-11-20T06:39:07Z. The raw log stays in the machine-local store.
