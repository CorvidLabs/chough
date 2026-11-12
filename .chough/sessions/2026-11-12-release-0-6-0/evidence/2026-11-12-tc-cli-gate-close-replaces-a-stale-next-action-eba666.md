---
kind: evidence
id: 2026-11-12-tc-cli-gate-close-replaces-a-stale-next-action-eba666
aliases: []
claim: "Ran cargo test --locked --test humans -- approving_the_scope_gate_replaces_the_next_action_it_left_stale a_gate_close_replaces_the_next_action_only_when_no_gate_holds_the_stage for TC-cli-gate-close-replaces-a-stale-next-action: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-close-replaces-a-stale-next-action]
case_digest: [9af14e57fece01a17a4bb234e49002ff3d55fbc0b9e73db6f35c23486e153398]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - approving_the_scope_gate_replaces_the_next_action_it_left_stale
  - a_gate_close_replaces_the_next_action_only_when_no_gate_holds_the_stage
exit_code: 0
duration_seconds: 2
revision: cca98758c52eb3f7b42d1f1caf751fc0e5a500c1
content: 94d89ecbe5a0f2d293255d074ccc5b065282faf8c8e453fc4e4ec5f4f093b88f
environment: local-macos-arm64
fingerprint: sha256:48ee5583ff3647566bd2203706c9b374e56b71c084c35dc4240232385f1e73f5
criteria: [FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T21:03:47Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- approving_the_scope_gate_replaces_the_next_action_it_left_stale a_gate_close_replaces_the_next_action_only_when_no_gate_holds_the_stage for TC-cli-gate-close-replaces-a-stale-next-action: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision cca98758c52e, observed 2026-11-12T21:03:47Z. The raw log stays in the machine-local store.
