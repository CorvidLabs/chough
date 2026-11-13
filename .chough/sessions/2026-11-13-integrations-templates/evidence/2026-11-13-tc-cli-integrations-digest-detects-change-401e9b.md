---
kind: evidence
id: 2026-11-13-tc-cli-integrations-digest-detects-change-401e9b
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
revision: 2cf5facb7c792081f8f73104eb458f4cda4408b9
content: e87d689143e165f16ea94afa6e1b6bb1edd9f5a40749321661f272dfb48435ef
environment: local-macos-arm64
fingerprint: sha256:628af92c583d8de7465d575ed820d58bb2b9dcf92fe57adea6bbc42ada44870a
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:07:13Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 2cf5facb7c79, observed 2026-11-13T06:07:13Z. The raw log stays in the machine-local store.
