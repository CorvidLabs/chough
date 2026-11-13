---
kind: evidence
id: 2026-11-13-tc-cli-integrations-manifest-1fbb4b
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
revision: 5219fe8daacaa2baeb870782955b1f06090c072f
content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
environment: local-macos-arm64
fingerprint: sha256:3a032147d36644f812486b419b20710a3749e377a8ba9599b43592d2ce2db937
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T14:50:01Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5219fe8daaca, observed 2026-11-13T14:50:01Z. The raw log stays in the machine-local store.
