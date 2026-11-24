---
kind: evidence
id: 2026-11-24-tc-records-gate-decision-renders-display-name
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact an_alias_close_renders_the_configured_display_name_in_the_decision for TC-records-gate-decision-renders-display-name: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-gate-decision-renders-display-name]
case_digest: [d698604f56c129116d44677322ab204278977908327180ac45d164bfc39b38f5]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - an_alias_close_renders_the_configured_display_name_in_the_decision
exit_code: 0
duration_seconds: 0
revision: 8c8acfcab211a820125d7570d9acebb7386730bd
content: 7531917fec1a0fb21b4b6bdf0ccf0beec1686ef2bb1b7bc8dec8ab824b8d837c
environment: local-macos-arm64
fingerprint: sha256:7efa55e49236b75ee93e85f6fa9477785b1a774ee8a9045ea6a6844143fdafb4
criteria: [FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-24T04:46:59Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test humans -- --exact an_alias_close_renders_the_configured_display_name_in_the_decision for TC-records-gate-decision-renders-display-name: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 8c8acfcab211, observed 2026-11-24T04:46:59Z. The raw log stays in the machine-local store.
