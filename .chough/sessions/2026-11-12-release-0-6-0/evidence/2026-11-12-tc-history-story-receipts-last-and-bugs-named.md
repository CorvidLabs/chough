---
kind: evidence
id: 2026-11-12-tc-history-story-receipts-last-and-bugs-named
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_story_puts_its_receipts_last_and_names_its_bugs for TC-history-story-receipts-last-and-bugs-named: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-story-receipts-last-and-bugs-named]
case_digest: [bcd5cb75ed64eb357408b3541926b37728275d7998b0c7a4fb696fbc519cfba7]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_story_puts_its_receipts_last_and_names_its_bugs
exit_code: 101
tests_run: 1
duration_seconds: 3
revision: 4e245263fe4e35b757d9b0531a0992c7cf0f884d
content: 073d7330cf258fdd25b33d439e35f9344842f8fdfa2a1cd81c388ab4af69e3ca
environment: local-macos-arm64
fingerprint: sha256:1ea9d9b523572aad53615be480791d1dec5c465519fd060ed37b8478e2c26acf
criteria: [LIFE-7, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:04:28Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact a_story_puts_its_receipts_last_and_names_its_bugs for TC-history-story-receipts-last-and-bugs-named: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 4e245263fe4e, observed 2026-11-12T23:04:28Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_story_puts_its_receipts_last_and_names_its_bugs stdout ----
- thread 'a_story_puts_its_receipts_last_and_names_its_bugs' (927439901) panicked at tests/sessions.rs:1958:5:
- the front matter holds receipts: [
-     "kind: session",
-     "id: 2026-11-12-ship-a-focused-change",
-     "aliases: [ship-a-focused-change]",
-     "objective: Ship a focused change",
-     "intent_scope: LIFE",
-     "modules: [cli]",
-     "stage: intent",
-     "stage_status: archived",
-     "iteration: 1",
-     "callers: []",
-     "native_change: \"\"",
-     "pr: \"\"",
-     "criteria: []",
-     "requirements: []",
-     "cases: []",
-     "design: []",
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 31 filtered out; finished in 2.79s
