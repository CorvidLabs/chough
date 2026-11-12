---
kind: evidence
id: 2026-11-12-tc-cli-vault-regenerates-identically
aliases: []
claim: "Ran cargo test --locked --test vault -- --exact the_vault_regenerates_identically for TC-cli-vault-regenerates-identically: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-vault-regenerates-identically]
case_digest: [ece2e4307cf72330cf0f9cc3aba08a82d2b687bf75be00165c37433dd4134360]
command: [cargo, test, --locked, --test, vault, --, --exact, the_vault_regenerates_identically]
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e871d36fc1a39009bee4ea40456fe2ada51a00d2
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
fingerprint: sha256:d2eafd20ebda733f98add86f8704f561228e285e3f388a971212a2df0aceaaa0
criteria: [KNOW-4]
requirements: [REQ-cli-044]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:00:03Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test vault -- --exact the_vault_regenerates_identically for TC-cli-vault-regenerates-identically: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e871d36fc1a3, observed 2026-11-12T15:00:03Z. The raw log stays in the machine-local store.
