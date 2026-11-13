---
kind: evidence
id: 2026-11-13-tc-cli-integrations-digest-detects-change-0d3a56
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
revision: 5219fe8daacaa2baeb870782955b1f06090c072f
content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
environment: local-macos-arm64
fingerprint: sha256:b8a762f950c1b35eeee4db2f55e96cb2dbf1f025f08674b1d14488c2e2efd159
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T14:50:03Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5219fe8daaca, observed 2026-11-13T14:50:03Z. The raw log stays in the machine-local store.
