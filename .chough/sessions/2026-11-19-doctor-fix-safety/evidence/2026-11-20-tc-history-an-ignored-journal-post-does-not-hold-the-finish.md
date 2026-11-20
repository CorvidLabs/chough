---
kind: evidence
id: 2026-11-20-tc-history-an-ignored-journal-post-does-not-hold-the-finish
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
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:fc0874804be1aa7e144045ea4d4d9c050b80a596e16792d50adc7448b5708c47
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:49:05Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_does_not_hold_the_finish for TC-history-an-ignored-journal-post-does-not-hold-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:49:05Z. The raw log stays in the machine-local store.
