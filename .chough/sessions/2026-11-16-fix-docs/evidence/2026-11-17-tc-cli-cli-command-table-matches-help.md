---
kind: evidence
id: 2026-11-17-tc-cli-cli-command-table-matches-help
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact cli_reference_command_table_matches_the_helpers_help for TC-cli-cli-command-table-matches-help: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-cli-command-table-matches-help]
case_digest: [bdbec5b6aebc017d6974bcd537996a0cfbfc9645beb5d0b0b47ed4295db9ce4a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - cli_reference_command_table_matches_the_helpers_help
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:98207a8be0bd9eeca599d91a2445e4774179fe09b276cfb5e77776f0559832cd
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:27Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact cli_reference_command_table_matches_the_helpers_help for TC-cli-cli-command-table-matches-help: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:27Z. The raw log stays in the machine-local store.
