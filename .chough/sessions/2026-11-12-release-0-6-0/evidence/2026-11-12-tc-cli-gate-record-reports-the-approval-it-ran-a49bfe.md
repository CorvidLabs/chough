---
kind: evidence
id: 2026-11-12-tc-cli-gate-record-reports-the-approval-it-ran-a49bfe
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact gate_record_reports_the_approval_it_ran_and_its_note_names_gate_record for TC-cli-gate-record-reports-the-approval-it-ran: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-record-reports-the-approval-it-ran]
case_digest: [48137d87ee867ae18dfda0f6199139e38ff84b74d0d103abb0760738adb4945c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - gate_record_reports_the_approval_it_ran_and_its_note_names_gate_record
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: e8bde99c60a7555434694e8ffadf1bb28fa25fda
content: 3ee3e75bcaf0a92f7bc5ca41f59bd421498d8a26342f52e2d390ef2dd4377b54
environment: local-macos-arm64
fingerprint: sha256:e73a52aa7c36505af60362479206a9ea59b496d85e09f50d1911857bfed6662e
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-history-014]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:39:31Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact gate_record_reports_the_approval_it_ran_and_its_note_names_gate_record for TC-cli-gate-record-reports-the-approval-it-ran: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e8bde99c60a7, observed 2026-11-12T17:39:31Z. The raw log stays in the machine-local store.
