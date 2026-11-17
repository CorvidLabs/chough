---
kind: evidence
id: 2026-11-17-tc-cli-integrations-manifest
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-manifest]
case_digest: [f2dc5b12c78e1adc195ab83332e1a2f823e4fd34da08456cd7f51f72af9bdf29]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - the_manifest_versions_every_template_with_its_digest
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:925687bc26e91fb09b88299f1df272d65f118d64156aad1c856abe9ddcd0fc13
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
**Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:34Z. The raw log stays in the machine-local store.
