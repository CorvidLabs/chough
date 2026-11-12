---
kind: evidence
id: 2026-11-12-tc-cli-a-stage-move-replaces-a-gate-close-next-action
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact a_stage_move_replaces_the_next_action_a_gate_close_left for TC-cli-a-stage-move-replaces-a-gate-close-next-action: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-stage-move-replaces-a-gate-close-next-action]
case_digest: [5e112f26227d20b7ab0a21131e542487b0f42e79e3eceab952db32d2fdc543ff]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - a_stage_move_replaces_the_next_action_a_gate_close_left
exit_code: 101
tests_run: 1
duration_seconds: 5
revision: 00f8f8d8956ffdc798e2ca66854ec876cb035cc9
content: 38ed197a47305c10d2dcc1fea8ddf9ddf393e12075d20eeda5c78ac6761b39eb
environment: local-macos-arm64
fingerprint: sha256:363848ccf96db57868e112b64d804ed86c241a99de593d35724004fc3b523423
criteria: [FLOW-13, FLOW-2]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:38:57Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_stage_move_replaces_the_next_action_a_gate_close_left for TC-cli-a-stage-move-replaces-a-gate-close-next-action: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 00f8f8d8956f, observed 2026-11-12T23:38:57Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_stage_move_replaces_the_next_action_a_gate_close_left stdout ----
- thread 'a_stage_move_replaces_the_next_action_a_gate_close_left' (928091493) panicked at tests/humans.rs:761:5:
- assertion `left == right` failed
-   left: "Gate \"Scope approval\" closed (approved); run `chough stage advance` to move on."
-  right: "Continue build"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 16 filtered out; finished in 3.62s
