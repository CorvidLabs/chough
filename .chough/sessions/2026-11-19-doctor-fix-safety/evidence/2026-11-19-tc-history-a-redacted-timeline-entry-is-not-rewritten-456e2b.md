---
kind: evidence
id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-456e2b
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
duration_seconds: 4
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:e34ea630df93d38143a6fabaf911be43546c974e0ecc2227ff71b06c4c6b3c4e
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:13:44Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:13:44Z. The raw log stays in the machine-local store.
