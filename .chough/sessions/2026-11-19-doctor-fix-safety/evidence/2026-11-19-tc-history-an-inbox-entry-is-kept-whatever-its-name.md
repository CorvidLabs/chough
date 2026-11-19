---
kind: evidence
id: 2026-11-19-tc-history-an-inbox-entry-is-kept-whatever-its-name
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_inbox_entry_is_kept_whatever_its_name for TC-history-an-inbox-entry-is-kept-whatever-its-name: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-inbox-entry-is-kept-whatever-its-name]
case_digest: [239e62b0874ea258aaed6e5e48acb7208fe7fe805f843405ce534ae27c6cf4b9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_inbox_entry_is_kept_whatever_its_name
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:4afd142d89b1ea728d28b1a0fc8c19a79b8cc4d718e094e1a70dceb2049bda84
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:48Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_inbox_entry_is_kept_whatever_its_name for TC-history-an-inbox-entry-is-kept-whatever-its-name: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:48Z. The raw log stays in the machine-local store.
