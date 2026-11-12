---
kind: evidence
id: 2026-11-12-tc-history-archived-story-says-archived-64087b
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
revision: 91c3824f94a9742f127a480d297fc7f2175e631c
content: c2aa3b745dd7a94452e0b4feee5950b8b356056ae629fdf516c4dcf5c7d1e6fd
environment: local-macos-arm64
fingerprint: sha256:e83a84b2997d970a38212b25a1c7a4265241b3628f6015ace7f1cc4a4bf574c3
criteria: [DRIVE-13, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:07:33Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact an_archived_story_says_archived_and_keeps_no_next for TC-history-archived-story-says-archived: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 91c3824f94a9, observed 2026-11-12T21:07:33Z. The raw log stays in the machine-local store.
