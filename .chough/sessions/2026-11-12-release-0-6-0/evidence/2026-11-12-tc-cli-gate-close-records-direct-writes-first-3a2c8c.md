---
kind: evidence
id: 2026-11-12-tc-cli-gate-close-records-direct-writes-first-3a2c8c
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
revision: 91c3824f94a9742f127a480d297fc7f2175e631c
content: c2aa3b745dd7a94452e0b4feee5950b8b356056ae629fdf516c4dcf5c7d1e6fd
environment: local-macos-arm64
fingerprint: sha256:c740886ed93fff853948f4cada6657dd71b87f1b29de1f87a5c032035ae80cf4
criteria: [DRIVE-21]
requirements: [REQ-cli-060]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:07:10Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact a_direct_write_before_a_gate_close_is_recorded_before_its_entry for TC-cli-gate-close-records-direct-writes-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 91c3824f94a9, observed 2026-11-12T21:07:10Z. The raw log stays in the machine-local store.
