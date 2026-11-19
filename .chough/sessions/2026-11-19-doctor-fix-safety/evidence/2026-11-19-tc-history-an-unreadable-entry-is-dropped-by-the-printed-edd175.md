---
kind: evidence
id: 2026-11-19-tc-history-an-unreadable-entry-is-dropped-by-the-printed-edd175
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
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:5939f5abb795b2b065e0cf522244fff3de6f60eea2ef76c5ef3b96d14f4a242b
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:25Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands for TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:25Z. The raw log stays in the machine-local store.
