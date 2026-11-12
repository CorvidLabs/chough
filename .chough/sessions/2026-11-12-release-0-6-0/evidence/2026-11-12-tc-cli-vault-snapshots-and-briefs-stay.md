---
kind: evidence
id: 2026-11-12-tc-cli-vault-snapshots-and-briefs-stay
aliases: []
claim: "Ran cargo test --locked --test vault -- --exact snapshots_and_briefs_stay_as_they_are_and_snapshot_and_archive_are_gone for TC-cli-vault-snapshots-and-briefs-stay: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-vault-snapshots-and-briefs-stay]
case_digest: [178b710ec504bbfe7b344c4b89d20cdc83ad022cd417141652a87569d3c4e817]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - vault
  - "--"
  - "--exact"
  - snapshots_and_briefs_stay_as_they_are_and_snapshot_and_archive_are_gone
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e871d36fc1a39009bee4ea40456fe2ada51a00d2
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
fingerprint: sha256:ce4d9609eac8b7a07a4346045b92ea231f9d895c8ed12716f19cf93c70a7fb2d
criteria: [KNOW-6]
requirements: [REQ-cli-044]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:00:08Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test vault -- --exact snapshots_and_briefs_stay_as_they_are_and_snapshot_and_archive_are_gone for TC-cli-vault-snapshots-and-briefs-stay: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e871d36fc1a3, observed 2026-11-12T15:00:08Z. The raw log stays in the machine-local store.
