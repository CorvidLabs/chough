---
kind: evidence
id: 2026-11-13-tc-cli-scope-gate-summary-counts-uncommitted-change-files
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_counts_uncommitted_and_untracked_change_files for TC-cli-scope-gate-summary-counts-uncommitted-change-files: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-scope-gate-summary-counts-uncommitted-change-files]
case_digest: [1718cddf6a47a27f123ce01376d2fed1c6c9fbc9475afbbc796b47ad52c01c07]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_scope_gate_summary_counts_uncommitted_and_untracked_change_files
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:43400cce8668f0370b4cc3c6fd23cbdaa69f52957fe03dbb2eddb5d5c61f5fdc
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:25:41Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_counts_uncommitted_and_untracked_change_files for TC-cli-scope-gate-summary-counts-uncommitted-change-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:25:41Z. The raw log stays in the machine-local store.
