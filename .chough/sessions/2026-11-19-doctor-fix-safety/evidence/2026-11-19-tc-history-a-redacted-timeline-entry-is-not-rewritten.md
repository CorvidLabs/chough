---
kind: evidence
id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten
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
duration_seconds: 9
revision: 73cf8c7aaca29e4495ad0d6461db9f7b7fd64fb5
content: e0e1ce89a4414c7225189af8ad1965a6d132cfc4115a1598607a2421e13c2113
environment: local-macos-arm64
fingerprint: sha256:c847cba0de4a43dc4dac70fd779458d5f82e090b1515211eaacfa9a124bea268
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:34:33Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 73cf8c7aaca2, observed 2026-11-19T05:34:33Z. The raw log stays in the machine-local store.
