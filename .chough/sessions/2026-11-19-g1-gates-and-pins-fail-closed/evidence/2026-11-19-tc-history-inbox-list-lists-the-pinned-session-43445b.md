---
kind: evidence
id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-43445b
aliases: []
claim: "Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-inbox-list-lists-the-pinned-session]
case_digest: [b29b0dd986237b002d1a95dee4c6fb8871381d4383e9eaf94c1f8636674bc357]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - session_pin
  - "--"
  - "--exact"
  - inbox_list_lists_the_pinned_session
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
environment: local-macos-arm64
fingerprint: sha256:2b8a585fc8405baafb23b6013704af58c2dc08011def2b0b712f39ffdedaa854
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-history-009]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:33:46Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 659dcded2db9, observed 2026-11-19T17:33:46Z. The raw log stays in the machine-local store.
