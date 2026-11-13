---
kind: evidence
id: 2026-11-13-tc-history-archived-story-says-archived-a9e734
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:f4573af2cc470559c519df68e94ffda9f1b09b97fbb483294ade192c317432cc
criteria: [DRIVE-13, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:47:30Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact an_archived_story_says_archived_and_keeps_no_next for TC-history-archived-story-says-archived: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:47:30Z. The raw log stays in the machine-local store.
