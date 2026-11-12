---
kind: evidence
id: 2026-11-12-tc-cli-gate-close-replaces-a-stale-next-action-188881
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
duration_seconds: 1
revision: 7be760ccbc7da0fbb0ebe1a63ff562e5782ae91b
content: 2bc6f78c658c710d8247ff73e0cc5320123d1398568de992f6a8852e275b18e1
environment: local-macos-arm64
fingerprint: sha256:b2cb5315e6c3ee1810204812b84c444153decbdeeda63e89d0d13b7eb3797417
criteria: [FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T17:45:06Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- approving_the_scope_gate_replaces_the_next_action_it_left_stale a_gate_close_replaces_the_next_action_only_when_no_gate_holds_the_stage for TC-cli-gate-close-replaces-a-stale-next-action: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 7be760ccbc7d, observed 2026-11-12T17:45:06Z. The raw log stays in the machine-local store.
