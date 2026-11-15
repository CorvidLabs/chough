---
kind: evidence
id: 2026-11-15-tc-cli-vault-regenerates-identically
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
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:809e0849afb9283b286b73ccb4606ccde5ab446af7054aa59c15f9efff5410f8
criteria: [KNOW-4]
requirements: [REQ-cli-044]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:12Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test vault -- --exact the_vault_regenerates_identically for TC-cli-vault-regenerates-identically: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:12Z. The raw log stays in the machine-local store.
