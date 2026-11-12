---
kind: evidence
id: 2026-11-12-tc-cli-close-running-no-native-step-says-why
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_why for TC-cli-close-running-no-native-step-says-why: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-close-running-no-native-step-says-why]
case_digest: [f562db109f888aef651e02e3a46dadf0f472f59740bf8326177e02bb0661701c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - a_close_that_runs_no_native_step_says_why
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 731aba24c9db412aaa2f1b570043ee4c1f16cc31
content: 09ee87f20f595f31728e706969a0b939c75f822cf7699e157fdb40dda5f964d0
environment: local-macos-arm64
fingerprint: sha256:e6b34ab45f58a230679292be476a8abe984bbbe1cd6eef16b274babe33495f7f
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:08:41Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_why for TC-cli-close-running-no-native-step-says-why: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 731aba24c9db, observed 2026-11-12T20:08:41Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_close_that_runs_no_native_step_says_why stdout ----
- thread 'a_close_that_runs_no_native_step_says_why' (922357552) panicked at tests/humans.rs:497:5:
- assertion `left == right` failed: {
-   "checkpoint": {
-     "awaiting_human": [
-       "Scope approval"
-     ],
-     "blockers": [],
-     "bugs": [],
-     "completed": [],
-     "decisions": [
-       "SYNTHETIC-human decided \"SYNTHETIC go-ahead\": \"yes, go\" (SYNTHETIC chat, 2026-11-11); recorded by SYNTHETIC-agent; a relayed claim, not authenticated"
-     ],
-     "intent_scope": "LIFE",
-     "modules": [
-       "cli"
-     ],
-     "native_change": "human-change",
-     "next_action": "Gate \"SYNTHETIC go-ahead\" closed (closed); run `chough next` for the next action.",
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 13 filtered out; finished in 1.22s
