---
kind: evidence
id: 2026-11-13-tc-cli-gate-close-records-direct-writes-first-94c8f0
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:a616e2fb8b0a79b9ee22ca7b4e6691e0d18d7b2d5d8a3818d3ce98ca31d43427
criteria: [DRIVE-21]
requirements: [REQ-cli-060]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:46:32Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact a_direct_write_before_a_gate_close_is_recorded_before_its_entry for TC-cli-gate-close-records-direct-writes-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:46:32Z. The raw log stays in the machine-local store.
