---
kind: evidence
id: 2026-11-13-tc-cli-scope-gate-summary-names-each-delta-first-adbace
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_each_delta_first_with_its_requirements for TC-cli-scope-gate-summary-names-each-delta-first: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-scope-gate-summary-names-each-delta-first]
case_digest: [4a2d1c45218f0e0dfb72c5e4f6488080990be96638ee6943e2b77d4fdb233e4b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_scope_gate_summary_names_each_delta_first_with_its_requirements
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:53e7a32e86e1cc957c07b171817eb711a1591de75e7218b8409b72a4a0a705f8
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:17:50Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_each_delta_first_with_its_requirements for TC-cli-scope-gate-summary-names-each-delta-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:17:50Z. The raw log stays in the machine-local store.
