---
kind: evidence
id: 2026-11-15-tc-cli-knowledge-index-check
aliases: []
claim: "Ran cargo test --locked --test vault -- --exact index_check_names_stale_files_and_passes_after_a_rebuild for TC-cli-knowledge-index-check: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-knowledge-index-check]
case_digest: [9d8e03aa6c11246bac9437a73345439438d54d93a358854132ac25fb362ea7cf]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - vault
  - "--"
  - "--exact"
  - index_check_names_stale_files_and_passes_after_a_rebuild
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:994617393d3485bbece3f12af78004c985935c80865ad81f2300dc0f2f8576b9
criteria: [DRIVE-12]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:04Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test vault -- --exact index_check_names_stale_files_and_passes_after_a_rebuild for TC-cli-knowledge-index-check: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:04Z. The raw log stays in the machine-local store.
