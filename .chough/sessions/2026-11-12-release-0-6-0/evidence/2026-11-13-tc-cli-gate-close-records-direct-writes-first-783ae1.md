---
kind: evidence
id: 2026-11-13-tc-cli-gate-close-records-direct-writes-first-783ae1
aliases: []
claim: "Ran cargo test --locked --test direct_writes -- --exact a_direct_write_before_a_gate_close_is_recorded_before_its_entry for TC-cli-gate-close-records-direct-writes-first: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-close-records-direct-writes-first]
case_digest: [1c926056f121dd6cf2e03e4263b25c0e53e8771a1e147241f8ddb1b1df5268b4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - direct_writes
  - "--"
  - "--exact"
  - a_direct_write_before_a_gate_close_is_recorded_before_its_entry
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:0cda0314e8eaf045da8459dd92f7448a01385c5aeb29b67598dcc1acdbbd9e72
criteria: [DRIVE-21]
requirements: [REQ-cli-060]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:13:06Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact a_direct_write_before_a_gate_close_is_recorded_before_its_entry for TC-cli-gate-close-records-direct-writes-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:13:06Z. The raw log stays in the machine-local store.
