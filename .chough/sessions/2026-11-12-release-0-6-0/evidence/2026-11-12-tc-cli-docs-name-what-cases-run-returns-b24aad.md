---
kind: evidence
id: 2026-11-12-tc-cli-docs-name-what-cases-run-returns-b24aad
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
duration_seconds: 3
revision: 0af25ea1480d1d0ce7eadd189f3a8b0c8b520fd2
content: 5511f9a2ec985a55c1f2ae5978bc45fe1b06f0386f884799ca8962145d552f86
environment: local-macos-arm64
fingerprint: sha256:e2eb0dbed6af5a057a54d0313e026bfecdf058404104825e4a018d56c3e62487
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:19:47Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact the_cli_reference_names_what_cases_run_returns for TC-cli-docs-name-what-cases-run-returns: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0af25ea1480d, observed 2026-11-12T17:19:47Z. The raw log stays in the machine-local store.
