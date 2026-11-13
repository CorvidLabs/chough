---
kind: evidence
id: 2026-11-13-tc-history-archived-story-says-archived-ffd8b3
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:eda508e3a5888a2c0bd95a1931e1d803a069271e897e9da56f3d751143285cdb
criteria: [DRIVE-13, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:14:19Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact an_archived_story_says_archived_and_keeps_no_next for TC-history-archived-story-says-archived: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:14:19Z. The raw log stays in the machine-local store.
