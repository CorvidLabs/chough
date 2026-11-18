---
kind: evidence
id: 2026-11-18-tc-history-a-session-whose-record-is-a-fifo-is-listed-1d0a81
aliases: []
claim: "Ran cargo test --locked --test special_files -- --exact a_session_whose_record_is_a_fifo_is_listed_with_the_reason_and_recovery for TC-history-a-session-whose-record-is-a-fifo-is-listed: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-session-whose-record-is-a-fifo-is-listed]
case_digest: [f0b56ea950f6845b1abf4846b61ff4f642e2d7a5c382cd07fd12337b2b73d699]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - special_files
  - "--"
  - "--exact"
  - a_session_whose_record_is_a_fifo_is_listed_with_the_reason_and_recovery
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: af8d04991c50e7ddae3616a645495c1f1b7c306b
content: b3158149ff06f564ddcb10a8acfe7e6c62b32c3f5e8e64c059b6d29279a32860
environment: local-macos-arm64
fingerprint: sha256:8d7f03dc9b04badd7e42d7b96108f73f5a74da333e89be28ba6de5f0d5fe1cee
criteria: [DRIVE-14]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T01:15:08Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test special_files -- --exact a_session_whose_record_is_a_fifo_is_listed_with_the_reason_and_recovery for TC-history-a-session-whose-record-is-a-fifo-is-listed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision af8d04991c50, observed 2026-11-18T01:15:08Z. The raw log stays in the machine-local store.
