---
kind: evidence
id: 2026-11-18-tc-history-a-session-whose-record-is-a-fifo-is-listed
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
revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
content: e55fa100450dd1aeb9d67eddd7f012e4b6367cda9338a9a63fef31dbe2911481
environment: local-macos-arm64
fingerprint: sha256:c68339eea21eb1c3ec8aedbce4afef28588757cb2fa72f890afdb9e56fb7004f
criteria: [DRIVE-14]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T00:37:06Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test special_files -- --exact a_session_whose_record_is_a_fifo_is_listed_with_the_reason_and_recovery for TC-history-a-session-whose-record-is-a-fifo-is-listed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8abb5bb89b52, observed 2026-11-18T00:37:06Z. The raw log stays in the machine-local store.
