---
kind: evidence
id: 2026-11-13-tc-cli-near-miss-gate-close-names-the-closest-open-gate-0f727a
aliases: []
claim: "Ran cargo test --locked --test relay -- --exact a_near_miss_gate_close_names_the_closest_open_gate for TC-cli-near-miss-gate-close-names-the-closest-open-gate: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-near-miss-gate-close-names-the-closest-open-gate]
case_digest: [006dc3d493a282c6a24f9210a48831f759e21c98f701b8a0a909efbed1c9f4a9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - relay
  - "--"
  - "--exact"
  - a_near_miss_gate_close_names_the_closest_open_gate
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:86f2de23fa1ae4a3db21055571c6596ca4ebce3985be56a8b2d376ea00b5a8bd
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:17:15Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test relay -- --exact a_near_miss_gate_close_names_the_closest_open_gate for TC-cli-near-miss-gate-close-names-the-closest-open-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:17:15Z. The raw log stays in the machine-local store.
