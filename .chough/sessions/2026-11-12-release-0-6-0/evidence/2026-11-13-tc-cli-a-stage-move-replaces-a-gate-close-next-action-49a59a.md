---
kind: evidence
id: 2026-11-13-tc-cli-a-stage-move-replaces-a-gate-close-next-action-49a59a
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:3110c9fd475c322a4ffa5ce3610cc27032818be3c71ddd476ede9f913edc0abe
criteria: [FLOW-13, FLOW-2]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:20:33Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_stage_move_replaces_the_next_action_a_gate_close_left for TC-cli-a-stage-move-replaces-a-gate-close-next-action: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:20:33Z. The raw log stays in the machine-local store.
