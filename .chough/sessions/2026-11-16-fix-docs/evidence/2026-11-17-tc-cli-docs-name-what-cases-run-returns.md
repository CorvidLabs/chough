---
kind: evidence
id: 2026-11-17-tc-cli-docs-name-what-cases-run-returns
aliases: []
claim: "Ran cargo test --locked --test cases -- --exact the_cli_reference_names_what_cases_run_returns for TC-cli-docs-name-what-cases-run-returns: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-name-what-cases-run-returns]
case_digest: [3459bfcddf1b5e82fa632dea9e9a3cf99c4eb6153919a396d795e933f959a482]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cases
  - "--"
  - "--exact"
  - the_cli_reference_names_what_cases_run_returns
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:0ac524e25718d9c98f2202d2aedec12ae2495df4c8975876aadc7af097bec95a
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:29Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test cases -- --exact the_cli_reference_names_what_cases_run_returns for TC-cli-docs-name-what-cases-run-returns: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:29Z. The raw log stays in the machine-local store.
