---
kind: evidence
id: 2026-11-12-tc-cli-gate-summary-counts-from-the-changes-own-base-afcf46
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_second_change_on_one_branch_counts_from_its_own_base_commit for TC-cli-gate-summary-counts-from-the-changes-own-base: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-summary-counts-from-the-changes-own-base]
case_digest: [96733866e58287900d082a4267d823512be8f83e3aad9c16f5f3885888e91ad8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_second_change_on_one_branch_counts_from_its_own_base_commit
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: d5381eef975bfee77aa224bfad4a5181742dab09
content: c5a6d9ab3697a6f65dc74ee237f5fab28786947d3b062f6996e0bccb06c9a9e0
environment: local-macos-arm64
fingerprint: sha256:a012081016eee4b8d7acb81c68fa8a45c0753ebf60838341729932eaa29390bb
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:28:20Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_second_change_on_one_branch_counts_from_its_own_base_commit for TC-cli-gate-summary-counts-from-the-changes-own-base: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d5381eef975b, observed 2026-11-12T15:28:20Z. The raw log stays in the machine-local store.
