---
kind: evidence
id: 2026-11-12-tc-cli-stage-move-authority-line-says-when-chough-runs-native
aliases: []
claim: "Ran cargo test --locked --test stages -- --exact every_stage_move_says_when_chough_runs_native_approval_or_review for TC-cli-stage-move-authority-line-says-when-chough-runs-native-steps: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-stage-move-authority-line-says-when-chough-runs-native-steps]
case_digest: [5096a003c3676001dd2b46920e1d507c97db01eac7c1547a58cc0495d60cf30e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - stages
  - "--"
  - "--exact"
  - every_stage_move_says_when_chough_runs_native_approval_or_review
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: c7cefe408f5e9b926ffab814452bb27ecf044d71
content: 689c6fe45ce152ebad43eda1f8c4b2b202ff0cf2434882b5ae05929bef8bbbb8
environment: local-macos-arm64
fingerprint: sha256:8b9490a217b86ce2dcb944ffb5297e3dac48d4807fe5a34617d0d5e88b7b160b
criteria: [LIFE-3]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:37:24Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test stages -- --exact every_stage_move_says_when_chough_runs_native_approval_or_review for TC-cli-stage-move-authority-line-says-when-chough-runs-native-steps: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision c7cefe408f5e, observed 2026-11-12T17:37:24Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- every_stage_move_says_when_chough_runs_native_approval_or_review stdout ----
- thread 'every_stage_move_says_when_chough_runs_native_approval_or_review' (920974025) panicked at tests/stages.rs:3242:9:
- assertion `left == right` failed: ["stage", "iterate", "--note", "SYNTHETIC another pass"]: {
-   "applied": {
-     "iteration": 2,
-     "kind": "iterate",
-     "stage": "intent"
-   },
-   "authority": "Native SpecSync status decides lifecycle gates; Chough never writes native approval or review, and a relayed decision it keeps is a claim, not authenticated.",
-   "awaiting_human": [],
-   "native_change": null,
-   "recent_transitions": [
-     {
-       "at_unix_seconds": 1794505042,
-       "from": "intent",
-       "gate": null,
-       "id": "t-065ac6",
-       "kind": "iterate",
-       "reason": "SYNTHETIC another pass",
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 48 filtered out; finished in 0.66s
