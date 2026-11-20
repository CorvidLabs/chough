---
kind: evidence
id: 2026-11-20-tc-records-no-command-echoes-a-given-value-741e03
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
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:912a8f5c63dd62c230f87f03ee616297ae2576ac87c8b7feb9c95b7205150caa
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:12:35Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:12:35Z. The raw log stays in the machine-local store.
