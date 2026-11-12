---
kind: evidence
id: 2026-11-12-tc-cli-gate-summary-says-when-there-is-no-base
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_summary_with_no_base_says_so_instead_of_a_broken_diff_command for TC-cli-gate-summary-says-when-there-is-no-base: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-summary-says-when-there-is-no-base]
case_digest: [e789971eda848287157badb8ea5dbb03c250941cd4a27deca441c954baf1abf3]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_summary_with_no_base_says_so_instead_of_a_broken_diff_command
exit_code: 0
tests_run: 1
duration_seconds: 13
revision: d5381eef975bfee77aa224bfad4a5181742dab09
content: c5a6d9ab3697a6f65dc74ee237f5fab28786947d3b062f6996e0bccb06c9a9e0
environment: local-macos-arm64
fingerprint: sha256:1863f6f0b4e86238d23f05cad8c00bdbbf2f61774301cb21410a718df404f82d
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:28:43Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_summary_with_no_base_says_so_instead_of_a_broken_diff_command for TC-cli-gate-summary-says-when-there-is-no-base: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d5381eef975b, observed 2026-11-12T15:28:43Z. The raw log stays in the machine-local store.
