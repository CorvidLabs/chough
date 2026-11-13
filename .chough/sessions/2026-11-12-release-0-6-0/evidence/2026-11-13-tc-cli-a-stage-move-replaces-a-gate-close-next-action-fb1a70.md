---
kind: evidence
id: 2026-11-13-tc-cli-a-stage-move-replaces-a-gate-close-next-action-fb1a70
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact a_stage_move_replaces_the_next_action_a_gate_close_left for TC-cli-a-stage-move-replaces-a-gate-close-next-action: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:af4357c9902e10ced11b020b6ff0b56759e1b179d8ba674655f50608786de8cb
criteria: [FLOW-13, FLOW-2]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:52:36Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_stage_move_replaces_the_next_action_a_gate_close_left for TC-cli-a-stage-move-replaces-a-gate-close-next-action: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:52:36Z. The raw log stays in the machine-local store.
