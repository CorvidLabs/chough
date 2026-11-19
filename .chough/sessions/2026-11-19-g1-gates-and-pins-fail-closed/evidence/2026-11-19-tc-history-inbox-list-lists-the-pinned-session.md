---
kind: evidence
id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session
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
duration_seconds: 1
revision: 8482f2025741f35d4b56afae8df77248c26f55f3
content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
environment: local-macos-arm64
fingerprint: sha256:f8ee68803b7a13e326bc1a21f256a1826c0d0ae97204031ec518ae772a538e0e
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-history-009]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:12:33Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8482f2025741, observed 2026-11-19T15:12:33Z. The raw log stays in the machine-local store.
