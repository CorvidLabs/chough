---
kind: evidence
id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-e220ba
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
duration_seconds: 1
revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
environment: local-macos-arm64
fingerprint: sha256:cca777f8e53ab99c8290de98436f395df6d553df15727558ff09b1fdf2a7aa81
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:31:42Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 44aec05c95e2, observed 2026-11-19T14:31:42Z. The raw log stays in the machine-local store.
