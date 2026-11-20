---
kind: evidence
id: 2026-11-20-tc-history-an-unreadable-entry-is-dropped-by-the-printed
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands for TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands]
case_digest: [6074ca970b1b27c49b2260941825108ef0fbbefc6353aa716e128bbbde123616]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_unreadable_entry_is_dropped_by_the_printed_commands
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:089692d5c1fbc24425ad4ddaf5add4d2fcbc5caff67ff49112e9e1a3948d9991
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:49:13Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands for TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:49:13Z. The raw log stays in the machine-local store.
