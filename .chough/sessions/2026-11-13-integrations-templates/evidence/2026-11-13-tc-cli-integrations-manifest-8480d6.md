---
kind: evidence
id: 2026-11-13-tc-cli-integrations-manifest-8480d6
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
revision: 12f2cafe777a48ef6e6cb92c9435997ab5b4e0d2
content: dd44a2f14e6358d057ef934649d20ff96745157b0280619cc43ef2f0e973b2c4
environment: local-macos-arm64
fingerprint: sha256:d7b652aded16e962b2e1342ee23b840cbae2278d262b4a6ed2d6ff1372514fc4
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:14:04Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 12f2cafe777a, observed 2026-11-13T04:14:04Z. The raw log stays in the machine-local store.
