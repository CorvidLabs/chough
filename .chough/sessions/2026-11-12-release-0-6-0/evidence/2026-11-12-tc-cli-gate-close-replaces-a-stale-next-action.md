---
kind: evidence
id: 2026-11-12-tc-cli-gate-close-replaces-a-stale-next-action
aliases: []
claim: "Ran cargo test --locked --test humans -- approving_the_scope_gate_replaces_the_next_action_it_left_stale a_gate_close_replaces_the_next_action_only_when_no_gate_holds_the_stage for TC-cli-gate-close-replaces-a-stale-next-action: failed (exit 101)."
result: failed
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
exit_code: 101
duration_seconds: 2
revision: dd3e2ab5df329a8dc68f9f89725310b2b56e029f
content: 6287c845d27db39c417052a4608cf898a2c1017c06b5e7c461b069727288236f
environment: local-macos-arm64
fingerprint: sha256:eb6781b4a9906b4b60ec0e813ec51e6817b14c18143ff8c9dc68fcad9904cfa3
criteria: [FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"test result: ok. 2 passed\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T17:42:45Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- approving_the_scope_gate_replaces_the_next_action_it_left_stale a_gate_close_replaces_the_next_action_only_when_no_gate_holds_the_stage for TC-cli-gate-close-replaces-a-stale-next-action: failed (exit 101).**

Recorded as failed in local-macos-arm64 at revision dd3e2ab5df32, observed 2026-11-12T17:42:45Z. The raw log stays in the machine-local store.

## Key failing lines

-     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
-      Running tests/humans.rs (target/debug/deps/humans-3f880b949495cf2a)
- error: test failed, to rerun pass `--test humans`
