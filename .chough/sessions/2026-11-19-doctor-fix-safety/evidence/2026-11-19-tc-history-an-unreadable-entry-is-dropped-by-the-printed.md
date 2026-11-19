---
kind: evidence
id: 2026-11-19-tc-history-an-unreadable-entry-is-dropped-by-the-printed
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
duration_seconds: 6
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:dbe517cb475b98fc099f5fb870bc4272b04eb76da013de2a2570abb9566c3597
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:14:55Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands for TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:14:55Z. The raw log stays in the machine-local store.
