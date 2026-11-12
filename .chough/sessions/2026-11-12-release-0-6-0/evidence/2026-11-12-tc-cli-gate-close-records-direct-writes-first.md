---
kind: evidence
id: 2026-11-12-tc-cli-gate-close-records-direct-writes-first
aliases: []
claim: "Ran cargo test --locked --test direct_writes -- --exact a_direct_write_before_a_gate_close_is_recorded_before_its_entry for TC-cli-gate-close-records-direct-writes-first: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 804bdb506371f0f50b97e76f0c09cc33daf439af
content: 431b305478c9131e54c07bc6bd895e4d5380b9af25bb28dad9e6056aed6a4f09
environment: local-macos-arm64
fingerprint: sha256:ad79894254f45cdbb046039795a654b75f5209055cf99a5c6ccdc9c0bce93e87
criteria: [DRIVE-21]
requirements: [REQ-cli-060]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:05:41Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact a_direct_write_before_a_gate_close_is_recorded_before_its_entry for TC-cli-gate-close-records-direct-writes-first: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 804bdb506371, observed 2026-11-12T21:05:41Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_direct_write_before_a_gate_close_is_recorded_before_its_entry stdout ----
- thread 'a_direct_write_before_a_gate_close_is_recorded_before_its_entry' (924234089) panicked at tests/direct_writes.rs:523:5:
- assertion `left == right` failed: [
-     "iterate",
-     "gate-opened SYNTHETIC scope of d1",
-     "gate-closed SYNTHETIC scope of d1",
-     "gate-opened SYNTHETIC go-ahead",
-     "gate-closed SYNTHETIC go-ahead",
-     "direct-native-write added",
-     "direct-native-write changed",
-     "iterate",
- ]
-   left: ["gate-opened SYNTHETIC scope of d1", "gate-closed SYNTHETIC scope of d1", "gate-opened SYNTHETIC go-ahead", "gate-closed SYNTHETIC go-ahead", "direct-native-write added", "direct-native-write changed", "iterate"]
-  right: ["gate-opened SYNTHETIC scope of d1", "direct-native-write added", "gate-closed SYNTHETIC scope of d1", "gate-opened SYNTHETIC go-ahead", "direct-native-write changed", "gate-closed SYNTHETIC go-ahead", "iterate"]
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10 filtered out; finished in 1.21s
