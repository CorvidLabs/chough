---
kind: evidence
id: 2026-11-17-tc-cli-integrations-digest-detects-change
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
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:888b4ebd89bd4cb4cb9bc4381c4f7625b5384c9d7c1cc5aa316d3f41b7be5364
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:34Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:34Z. The raw log stays in the machine-local store.
