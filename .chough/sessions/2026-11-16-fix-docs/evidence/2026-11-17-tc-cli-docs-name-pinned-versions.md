---
kind: evidence
id: 2026-11-17-tc-cli-docs-name-pinned-versions
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact docs_state_only_the_pinned_tool_versions for TC-cli-docs-name-pinned-versions: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-name-pinned-versions]
case_digest: [404be58afd21d5cd21d3eb5f7faed4cf91ff6ab0d49075d4eac06f227fa89015]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - docs_state_only_the_pinned_tool_versions
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:427c376c298b3e04cd59cd4c9c0e55e4d67279013bf5034bb0c52d4418d34177
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:26Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact docs_state_only_the_pinned_tool_versions for TC-cli-docs-name-pinned-versions: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:26Z. The raw log stays in the machine-local store.
