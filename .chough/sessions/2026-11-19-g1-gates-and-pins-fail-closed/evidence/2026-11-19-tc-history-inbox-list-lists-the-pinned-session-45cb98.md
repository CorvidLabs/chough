---
kind: evidence
id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-45cb98
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
revision: 753a022557083e0873d048ced9f5be2b40f686fd
content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
environment: local-macos-arm64
fingerprint: sha256:f9b9f852d3a29efad7394e0e0d979ed1d77a0a26e89b3160b00ea684e1d7a662
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-history-009]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:17:44Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 753a02255708, observed 2026-11-19T17:17:44Z. The raw log stays in the machine-local store.
