---
kind: evidence
id: 2026-11-12-tc-cli-docs-name-what-cases-run-returns-f3b95c
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
duration_seconds: 17
revision: 5f1c57c81296da2f1afd3180266844051680dcb3
content: d147d2faee47f0dbef9a0b63332105852ce094a35699f89beebee474e82c10c9
environment: local-macos-arm64
fingerprint: sha256:874734dfcc2a9763e1ff86748179fccb359ae0a07db258da7567c5b240989e8b
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:36:32Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact the_cli_reference_names_what_cases_run_returns for TC-cli-docs-name-what-cases-run-returns: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5f1c57c81296, observed 2026-11-12T16:36:32Z. The raw log stays in the machine-local store.
