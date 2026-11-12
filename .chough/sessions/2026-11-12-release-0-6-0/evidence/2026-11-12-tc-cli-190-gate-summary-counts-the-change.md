---
kind: evidence
id: 2026-11-12-tc-cli-190-gate-summary-counts-the-change
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact gate_summary_counts_only_the_changes_own_files for TC-cli-190-gate-summary-counts-the-change: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-190-gate-summary-counts-the-change]
case_digest: [087d32b333cbca67a875dfa17244ee8a3a29082ac164da481a66c35093023750]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - gate_summary_counts_only_the_changes_own_files
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: d5381eef975bfee77aa224bfad4a5181742dab09
content: c5a6d9ab3697a6f65dc74ee237f5fab28786947d3b062f6996e0bccb06c9a9e0
environment: local-macos-arm64
fingerprint: sha256:9788c10f05036fed0bf1674ad8405df165001c7e51313ddba7c20c753640faff
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:28:27Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact gate_summary_counts_only_the_changes_own_files for TC-cli-190-gate-summary-counts-the-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d5381eef975b, observed 2026-11-12T15:28:27Z. The raw log stays in the machine-local store.
