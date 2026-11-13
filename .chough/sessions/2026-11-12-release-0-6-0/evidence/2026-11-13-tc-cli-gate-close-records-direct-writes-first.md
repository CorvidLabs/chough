---
kind: evidence
id: 2026-11-13-tc-cli-gate-close-records-direct-writes-first
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:656a72416a949f45b0533012440e74cddb656f4e8c9d09480c29222a4279cdf6
criteria: [DRIVE-21]
requirements: [REQ-cli-060]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:20:42Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact a_direct_write_before_a_gate_close_is_recorded_before_its_entry for TC-cli-gate-close-records-direct-writes-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:20:42Z. The raw log stays in the machine-local store.
