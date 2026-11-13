---
kind: evidence
id: 2026-11-13-tc-cli-integrations-manifest-51d9b1
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
revision: c0e3c340ae4fce048f37442831d242d42eeeef28
content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
environment: local-macos-arm64
fingerprint: sha256:ac55340fc51d4758ffa33af57bcd1c7fc993c4a2c07ba9642543d3a06d2a033b
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:19:32Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c0e3c340ae4f, observed 2026-11-13T06:19:32Z. The raw log stays in the machine-local store.
