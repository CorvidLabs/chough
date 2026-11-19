---
kind: evidence
id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-a6832a
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-redacted-timeline-entry-is-not-rewritten]
case_digest: [ecdb628002127dd0bac95a871c821db001b94787f8905b438849e19d8ab78eb4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:78bce2c8b942c76edc46b4cf7be8ae15825d00cc3ce2339cda8f18070e2a9db0
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:55Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:55Z. The raw log stays in the machine-local store.
