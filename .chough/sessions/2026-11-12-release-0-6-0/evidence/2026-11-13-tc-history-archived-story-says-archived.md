---
kind: evidence
id: 2026-11-13-tc-history-archived-story-says-archived
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_archived_story_says_archived_and_keeps_no_next for TC-history-archived-story-says-archived: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-archived-story-says-archived]
case_digest: [f3d924fac5399a28a30d9e85c629aefa53c970e4783f6ce35f505a80d2083770]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_archived_story_says_archived_and_keeps_no_next
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:2e8d557787b1c2b8c7f227e20e2b5e1a763ae9e4bcf5e6380fb140a555dbbb4b
criteria: [DRIVE-13, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:21:29Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact an_archived_story_says_archived_and_keeps_no_next for TC-history-archived-story-says-archived: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:21:29Z. The raw log stays in the machine-local store.
