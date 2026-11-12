---
kind: evidence
id: 2026-11-12-tc-cli-vault-links-and-copies-no-text
aliases: []
claim: "Ran cargo test --locked --test vault -- --exact the_index_links_lessons_bundles_and_the_journal_and_copies_no_text for TC-cli-vault-links-and-copies-no-text: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-vault-links-and-copies-no-text]
case_digest: [0281c3af3b26371115011cc9e5b3f1a3572a7bc05b7fc226a2c7ee5009ff08eb]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - vault
  - "--"
  - "--exact"
  - the_index_links_lessons_bundles_and_the_journal_and_copies_no_text
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e871d36fc1a39009bee4ea40456fe2ada51a00d2
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
fingerprint: sha256:a871b22a239307cb74f7376e5b8024b853d8a0a89b629c607c1e748b271e693b
criteria: [KNOW-3, KNOW-15]
requirements: [REQ-cli-043, REQ-cli-044]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T14:59:57Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test vault -- --exact the_index_links_lessons_bundles_and_the_journal_and_copies_no_text for TC-cli-vault-links-and-copies-no-text: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e871d36fc1a3, observed 2026-11-12T14:59:57Z. The raw log stays in the machine-local store.
