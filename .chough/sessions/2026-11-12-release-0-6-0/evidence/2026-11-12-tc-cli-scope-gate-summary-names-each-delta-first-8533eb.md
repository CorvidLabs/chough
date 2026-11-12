---
kind: evidence
id: 2026-11-12-tc-cli-scope-gate-summary-names-each-delta-first-8533eb
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
revision: e0131c098169579c13f2498db93514969f149f33
content: f90f0ac78534841119984ed8ae2779eb2acada66c1c412a9c38a7acfd38b6442
environment: local-macos-arm64
fingerprint: sha256:4cb0d93979f75145cdcfda88cf511b41b73c130d0f4cc5ba620bc9fb497f2b74
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:02:17Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_each_delta_first_with_its_requirements for TC-cli-scope-gate-summary-names-each-delta-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e0131c098169, observed 2026-11-12T23:02:17Z. The raw log stays in the machine-local store.
