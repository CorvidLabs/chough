---
kind: evidence
id: 2026-11-19-tc-history-an-ignored-journal-post-does-not-hold-the-finish-82b94f
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_does_not_hold_the_finish for TC-history-an-ignored-journal-post-does-not-hold-the-finish: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-ignored-journal-post-does-not-hold-the-finish]
case_digest: [31fbceaff5b2eb87e4aacd537d6ff532ff62df280919eabd04c48a6756d5671a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_ignored_journal_post_does_not_hold_the_finish
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:ddc29da8a641ab2b4092c5532c07ea241aba52ba5d0382725db06dd8cd0e6b23
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:18Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_does_not_hold_the_finish for TC-history-an-ignored-journal-post-does-not-hold-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:18Z. The raw log stays in the machine-local store.
