---
kind: evidence
id: 2026-11-12-tc-cli-empty-change-dir-not-open-483046
aliases: []
claim: "Ran cargo test --locked --test evidence -- --exact an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change for TC-cli-empty-change-dir-not-open: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-empty-change-dir-not-open]
case_digest: [c778d32397bac0d8944cad3f34670c3290a4189e4b11c13eeb69aea13ad5d934]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - evidence
  - "--"
  - "--exact"
  - an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: a57c30169272321d7cea083c2a3ee7885dadad72
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
fingerprint: sha256:4c1df2e1a9dddf872d5d465a7a8c51afe29565d59fe4bf16285aae428fac2cc2
criteria: [KNOW-1]
requirements: [REQ-cli-043]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:02:16Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test evidence -- --exact an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change for TC-cli-empty-change-dir-not-open: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a57c30169272, observed 2026-11-12T15:02:16Z. The raw log stays in the machine-local store.
