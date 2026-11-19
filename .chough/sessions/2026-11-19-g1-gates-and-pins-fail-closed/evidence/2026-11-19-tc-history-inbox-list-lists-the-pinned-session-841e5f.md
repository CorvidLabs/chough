---
kind: evidence
id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-841e5f
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
revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
environment: local-macos-arm64
fingerprint: sha256:fda4fed78f0a77b633c0ff7f7dde2b9f4d3f2cd3ec2afe2e3e8b1ed82a72f77e
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-history-009]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:25:38Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0c31209bb80, observed 2026-11-19T15:25:38Z. The raw log stays in the machine-local store.
