---
kind: evidence
id: 2026-11-19-tc-history-an-unreadable-entry-is-dropped-by-the-printed-e874c0
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
duration_seconds: 13
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:4ab4d0e6a965019e714ae298a80bc58cd8eaef82a25833e67e7a30c302574cbd
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:11:31Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands for TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:11:31Z. The raw log stays in the machine-local store.
