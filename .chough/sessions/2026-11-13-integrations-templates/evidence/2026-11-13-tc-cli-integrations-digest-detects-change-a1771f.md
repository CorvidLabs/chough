---
kind: evidence
id: 2026-11-13-tc-cli-integrations-digest-detects-change-a1771f
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
revision: c0e3c340ae4fce048f37442831d242d42eeeef28
content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
environment: local-macos-arm64
fingerprint: sha256:20c40b996156011ba721e05a91f8624b0e2acb9f93364802ec75c06d8a53bcb5
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:19:35Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c0e3c340ae4f, observed 2026-11-13T06:19:35Z. The raw log stays in the machine-local store.
