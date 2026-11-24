---
kind: evidence
id: 2026-11-24-tc-records-gate-decision-exact-name-unchanged
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact an_exact_name_close_renders_the_display_name_unchanged for TC-records-gate-decision-exact-name-unchanged: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-gate-decision-exact-name-unchanged]
case_digest: [46f61526a4e222805cf4ddf1409cdfca50187ac8f73d3ee14ddf502b1a64c2a9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - an_exact_name_close_renders_the_display_name_unchanged
exit_code: 0
duration_seconds: 0
revision: 8c8acfcab211a820125d7570d9acebb7386730bd
content: 7531917fec1a0fb21b4b6bdf0ccf0beec1686ef2bb1b7bc8dec8ab824b8d837c
environment: local-macos-arm64
fingerprint: sha256:e0656edeaedc8dc72be2b76ca132b69de710df1c08ace5677915eaf7d8ca6b2f
criteria: [FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-24T04:47:03Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test humans -- --exact an_exact_name_close_renders_the_display_name_unchanged for TC-records-gate-decision-exact-name-unchanged: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 8c8acfcab211, observed 2026-11-24T04:47:03Z. The raw log stays in the machine-local store.
