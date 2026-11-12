---
kind: evidence
id: 2026-11-12-tc-cli-vault-rules-note
aliases: []
claim: "Ran cargo test --locked --test vault -- --exact the_rules_note_links_rules_skills_and_decisions_that_exist for TC-cli-vault-rules-note: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-vault-rules-note]
case_digest: [231e23ae7a26987bded0d01ad6f63e81785da54415d611e3b20bf2a8b2c33264]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - vault
  - "--"
  - "--exact"
  - the_rules_note_links_rules_skills_and_decisions_that_exist
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e871d36fc1a39009bee4ea40456fe2ada51a00d2
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
fingerprint: sha256:877b81d71afbc5fe32df6f35c4f84c24315e672a2566a5bf365a6903d04f95cc
criteria: [KNOW-15]
requirements: [REQ-cli-044]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:00:05Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test vault -- --exact the_rules_note_links_rules_skills_and_decisions_that_exist for TC-cli-vault-rules-note: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e871d36fc1a3, observed 2026-11-12T15:00:05Z. The raw log stays in the machine-local store.
