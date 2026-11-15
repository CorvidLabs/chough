---
kind: evidence
id: 2026-11-15-tc-cli-integrations-digest-detects-change
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-digest-detects-change]
case_digest: [aadc547832dd06247ad6e92c9b62442095049f56faf523a6ecc0c319c57cb466]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - a_changed_template_fails_until_a_new_version_lists_its_digest
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:02625857e36bfe9c700886291a3baef511c8af5f7d1b958682f3481b37efcb21
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:11Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:11Z. The raw log stays in the machine-local store.
