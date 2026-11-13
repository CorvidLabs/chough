---
kind: evidence
id: 2026-11-13-tc-cli-scope-gate-summary-counts-uncommitted-change-files-ae678a
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:af1bd586725ed8eabe638174107fe51b1f14bc7790ec5c300a32f2dbaf278948
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:52:51Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_counts_uncommitted_and_untracked_change_files for TC-cli-scope-gate-summary-counts-uncommitted-change-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:52:51Z. The raw log stays in the machine-local store.
