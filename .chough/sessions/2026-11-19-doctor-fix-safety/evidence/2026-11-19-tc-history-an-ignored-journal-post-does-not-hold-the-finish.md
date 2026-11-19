---
kind: evidence
id: 2026-11-19-tc-history-an-ignored-journal-post-does-not-hold-the-finish
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
duration_seconds: 5
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:b785e90ace39bd54f2f757f74c7192d9e8241f6cc3963a8df9c33c8b46748900
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:11:06Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_does_not_hold_the_finish for TC-history-an-ignored-journal-post-does-not-hold-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:11:06Z. The raw log stays in the machine-local store.
