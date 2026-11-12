---
kind: evidence
id: 2026-11-12-tc-cli-gate-close-records-direct-writes-first-def0b9
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
revision: 06407b98e650d73a951f505369cb7c694765ffde
content: 20c9710c0555251958526a8d2c4296aeac3f42743b807b3fecd9dd54614bf1da
environment: local-macos-arm64
fingerprint: sha256:88b25aaa998a4f092610450faa7d00daf198824dbfc09ed6d497e796f091d080
criteria: [DRIVE-21]
requirements: [REQ-cli-060]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:20:51Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact a_direct_write_before_a_gate_close_is_recorded_before_its_entry for TC-cli-gate-close-records-direct-writes-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 06407b98e650, observed 2026-11-12T21:20:51Z. The raw log stays in the machine-local store.
