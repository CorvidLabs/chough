---
kind: evidence
id: 2026-11-12-tc-cli-a-stage-move-replaces-a-gate-close-next-action-ee36e0
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
duration_seconds: 5
revision: 3f28a915b7cfecea95299f267290b90e61d1e5a6
content: beae3ed2b362fe695ad56e6f03a3fc7db3fb1067ac054dcdaed43b8e91f71515
environment: local-macos-arm64
fingerprint: sha256:380231ce063954f16d882afc74529dd54a0119c35d166257a4b40aeac82be28a
criteria: [FLOW-13, FLOW-2]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:47:32Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_stage_move_replaces_the_next_action_a_gate_close_left for TC-cli-a-stage-move-replaces-a-gate-close-next-action: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f28a915b7cf, observed 2026-11-12T23:47:32Z. The raw log stays in the machine-local store.
