---
kind: evidence
id: 2026-11-13-tc-cli-scope-gate-summary-names-each-delta-first
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:808602ad34fddce25de1b910b34012771884c6d3a292baf4a92ecfa8c7c41acf
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:23:42Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_each_delta_first_with_its_requirements for TC-cli-scope-gate-summary-names-each-delta-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:23:42Z. The raw log stays in the machine-local store.
