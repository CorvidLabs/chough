---
kind: evidence
id: 2026-11-19-tc-history-the-way-out-names-the-open-sessions-4107d9
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active for TC-history-the-way-out-names-the-open-sessions: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-the-way-out-names-the-open-sessions]
case_digest: [ea61f8b1731f5ac470892706e868693c5ee7e9fbc9d462a5748f39380f4be02f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - the_way_out_names_the_open_sessions_when_none_is_active
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:a14da88c2b4a894a81af9e21f88d221f5f2a494521a350eea9f365f3b988b23f
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:40Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active for TC-history-the-way-out-names-the-open-sessions: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:40Z. The raw log stays in the machine-local store.
