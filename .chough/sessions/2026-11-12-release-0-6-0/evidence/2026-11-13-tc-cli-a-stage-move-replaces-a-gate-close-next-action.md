---
kind: evidence
id: 2026-11-13-tc-cli-a-stage-move-replaces-a-gate-close-next-action
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
duration_seconds: 1
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:994c6e94a5926187b809b133de7d5b4ee51aea87957cbc541160fdeaca598085
criteria: [FLOW-13, FLOW-2]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:25:28Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_stage_move_replaces_the_next_action_a_gate_close_left for TC-cli-a-stage-move-replaces-a-gate-close-next-action: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:25:28Z. The raw log stays in the machine-local store.
