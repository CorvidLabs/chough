---
kind: evidence
id: 2026-11-20-tc-history-a-folded-inbox-entry-does-not-hold-the-finish
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_folded_inbox_entry_does_not_hold_the_finish for TC-history-a-folded-inbox-entry-does-not-hold-the-finish: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-folded-inbox-entry-does-not-hold-the-finish]
case_digest: [54c0d8ad5f8d90c7aa0cf321207f18f9d244df51bc2ca4448b69c80ea673c296]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_folded_inbox_entry_does_not_hold_the_finish
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:d6c0cff8b9d51b2e3e5a76dea66405395e2093a8e4b730da300f751e72406e2e
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:49:42Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_folded_inbox_entry_does_not_hold_the_finish for TC-history-a-folded-inbox-entry-does-not-hold-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:49:42Z. The raw log stays in the machine-local store.
