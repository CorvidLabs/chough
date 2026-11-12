---
kind: evidence
id: 2026-11-12-tc-history-archived-story-says-archived
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_archived_story_says_archived_and_keeps_no_next for TC-history-archived-story-says-archived: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: 804bdb506371f0f50b97e76f0c09cc33daf439af
content: 431b305478c9131e54c07bc6bd895e4d5380b9af25bb28dad9e6056aed6a4f09
environment: local-macos-arm64
fingerprint: sha256:c6a653a2f565d822c8b60bae944f7a98bbde4baf7bd87c24283c6fbd4eb086fa
criteria: [DRIVE-13, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:06:03Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact an_archived_story_says_archived_and_keeps_no_next for TC-history-archived-story-says-archived: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 804bdb506371, observed 2026-11-12T21:06:03Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- an_archived_story_says_archived_and_keeps_no_next stdout ----
- thread 'an_archived_story_says_archived_and_keeps_no_next' (924236624) panicked at tests/sessions.rs:628:9:
- alpha: [
-     "kind: session",
-     "id: 2026-11-12-alpha",
-     "aliases: [alpha]",
-     "objective: Ship alpha",
-     "intent_scope: \"\"",
-     "modules: []",
-     "stage: intent",
-     "stage_status: active",
-     "iteration: 1",
-     "callers: []",
-     "native_change: \"\"",
-     "pr: \"\"",
-     "criteria: []",
-     "requirements: []",
-     "cases: []",
-     "design: []",
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 29 filtered out; finished in 0.84s
