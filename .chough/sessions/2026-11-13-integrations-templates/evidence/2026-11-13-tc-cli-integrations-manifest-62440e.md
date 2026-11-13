---
kind: evidence
id: 2026-11-13-tc-cli-integrations-manifest-62440e
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
revision: a442a2acc4a23948c031c36675861ba57f04c980
content: 2fb922c6ab66ac421f2cbd88feaa0ec2fd87004f296b827945dfb087f7231437
environment: local-macos-arm64
fingerprint: sha256:f831f20d1c54af1064ed6aa9a10531c17819946697f70ee4015e489812d8f32e
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:54:36Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a442a2acc4a2, observed 2026-11-13T04:54:36Z. The raw log stays in the machine-local store.
