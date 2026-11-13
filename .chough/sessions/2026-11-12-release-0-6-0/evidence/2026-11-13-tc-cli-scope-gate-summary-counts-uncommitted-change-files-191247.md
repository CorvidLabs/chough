---
kind: evidence
id: 2026-11-13-tc-cli-scope-gate-summary-counts-uncommitted-change-files-191247
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:5e0c230a2d05ab8f458fea3fcfc36b240d231c937db12ad8b7bc25eec5108ad9
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:20:51Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_counts_uncommitted_and_untracked_change_files for TC-cli-scope-gate-summary-counts-uncommitted-change-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:20:51Z. The raw log stays in the machine-local store.
