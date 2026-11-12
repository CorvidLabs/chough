---
kind: evidence
id: 2026-11-12-tc-cli-vault-readme-upgrade
aliases: []
claim: "Ran cargo test --locked --test vault -- --exact an_untouched_earlier_readme_is_upgraded_and_an_edited_one_kept for TC-cli-vault-readme-upgrade: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-vault-readme-upgrade]
case_digest: [a6e71643d31c94e4d1c5fb98a47b0e3ced781f291de2e4d7ac159b2a3c7be57c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - vault
  - "--"
  - "--exact"
  - an_untouched_earlier_readme_is_upgraded_and_an_edited_one_kept
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: e871d36fc1a39009bee4ea40456fe2ada51a00d2
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
fingerprint: sha256:d3bcd1355f0ce6c43f1fff7aca7ef65d7bbcf1919055467e9cdf65f26e76e6b7
criteria: [KNOW-14, KNOW-4]
requirements: [REQ-cli-044]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:00:00Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test vault -- --exact an_untouched_earlier_readme_is_upgraded_and_an_edited_one_kept for TC-cli-vault-readme-upgrade: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e871d36fc1a3, observed 2026-11-12T15:00:00Z. The raw log stays in the machine-local store.
