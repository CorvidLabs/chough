---
kind: evidence
id: 2026-11-15-tc-cli-readme-verify-list-matches-fledge
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step for TC-cli-readme-verify-list-matches-fledge: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-readme-verify-list-matches-fledge]
case_digest: [8053fe1bd76560e758c7d613849c909d72c620944f864dc5d9bfa8e51fdae130]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - readme_verify_lane_list_names_every_fledge_verify_step
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:049cdb6b7cf53281fcd3db7916bd1f8538eba43376ee6d8a94e927bf741db0bb
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:10Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step for TC-cli-readme-verify-list-matches-fledge: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:10Z. The raw log stays in the machine-local store.
