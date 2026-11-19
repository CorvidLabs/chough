---
kind: evidence
id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-716a68
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
duration_seconds: 2
revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
environment: local-macos-arm64
fingerprint: sha256:e204582c3e3ea28e34e3a208af1758e6378dcf995ee71ae2483ada67160fd727
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-history-009]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:44:30Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 49401eb44ee9, observed 2026-11-19T16:44:30Z. The raw log stays in the machine-local store.
