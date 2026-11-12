---
kind: evidence
id: 2026-11-12-tc-cli-scope-gate-summary-counts-uncommitted-change-files-e361cf
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
duration_seconds: 2
revision: f618819ba23c3f8e65889d8f2871ea26a7a1cf81
content: 685d17b46d297a43c78012dededcb09f0048647d30d26e9c935cb460fd549e1c
environment: local-macos-arm64
fingerprint: sha256:5f449006eb8fa9fd7cafd15d589e0c3c129005c4eec8d3975acd0a288fe995b7
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:50:09Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_counts_uncommitted_and_untracked_change_files for TC-cli-scope-gate-summary-counts-uncommitted-change-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f618819ba23c, observed 2026-11-12T23:50:09Z. The raw log stays in the machine-local store.
