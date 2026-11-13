---
kind: evidence
id: 2026-11-13-tc-cli-integrations-digest-detects-change-75ba7d
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-digest-detects-change]
case_digest: [adaaf77380423e15ce74e2525868d5f59885efefd6aff95edb4a5c54c1fc2fbd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - a_changed_template_byte_no_longer_matches_its_listed_digest
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 12f2cafe777a48ef6e6cb92c9435997ab5b4e0d2
content: dd44a2f14e6358d057ef934649d20ff96745157b0280619cc43ef2f0e973b2c4
environment: local-macos-arm64
fingerprint: sha256:106eafea73112b45d19769a577c5c5314a64c34612a3399e58cbdbb2b0cafecc
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:14:08Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 12f2cafe777a, observed 2026-11-13T04:14:08Z. The raw log stays in the machine-local store.
