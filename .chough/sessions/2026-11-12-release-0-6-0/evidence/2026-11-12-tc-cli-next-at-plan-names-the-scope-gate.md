---
kind: evidence
id: 2026-11-12-tc-cli-next-at-plan-names-the-scope-gate
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_plan_next_names_the_scope_gate_and_never_specsyncs_approve for TC-cli-next-at-plan-names-the-scope-gate: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-at-plan-names-the-scope-gate]
case_digest: [75777266c77d6f9a658a56e102c54b2e702c75a7001270f5c9dd49664f21d4bd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_plan_next_names_the_scope_gate_and_never_specsyncs_approve
exit_code: 101
tests_run: 1
duration_seconds: 10
revision: da98f439e0a8db2a2c9cde127a83426d96730f49
content: 936e291c15b0e4e6a4c5e2b1f42d6ba1a17ecca8018baad29bbbaba7196270f8
environment: local-macos-arm64
fingerprint: sha256:0f4046950d47945fb8d7b322a7e665752ace153f88e56fba76f4866d72c0d614
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:47:26Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_plan_next_names_the_scope_gate_and_never_specsyncs_approve for TC-cli-next-at-plan-names-the-scope-gate: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision da98f439e0a8, observed 2026-11-12T16:47:26Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- at_plan_next_names_the_scope_gate_and_never_specsyncs_approve stdout ----
- thread 'at_plan_next_names_the_scope_gate_and_never_specsyncs_approve' (919484998) panicked at tests/next.rs:2398:9:
- missing chough checkpoint --awaiting-human: Gate not open (refused:native-draft): Reaching build needs native state approved, implementing or verifying; it is draft.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 24 filtered out; finished in 7.62s
