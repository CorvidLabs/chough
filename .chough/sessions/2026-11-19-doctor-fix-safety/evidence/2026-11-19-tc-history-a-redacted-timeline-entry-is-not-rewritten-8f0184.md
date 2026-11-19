---
kind: evidence
id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-8f0184
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
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:905472e9b4ab21c8dfc598274351cd4fff020a67d28f7935ede6f394edfa60c2
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:49:09Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:49:09Z. The raw log stays in the machine-local store.
