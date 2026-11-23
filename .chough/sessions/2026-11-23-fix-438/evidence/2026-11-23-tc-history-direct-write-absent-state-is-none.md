---
kind: evidence
id: 2026-11-23-tc-history-direct-write-absent-state-is-none
aliases: []
claim: "Ran cargo test --locked --bin chough -- --exact direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none for TC-history-direct-write-absent-state-is-none: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-direct-write-absent-state-is-none]
case_digest: [f4f20334bb575e7f99f86b7766ffe29e363a5e5f1058d7982b75e49cfd949bcc]
command:
  - cargo
  - test
  - "--locked"
  - "--bin"
  - chough
  - "--"
  - "--exact"
  - direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none
exit_code: 0
duration_seconds: 0
revision: 039363a822d1908c5e6c11609c4e4bb743757b8a
content: 81904c057a9c9a851321f19cfcf6e327a282cd47dbf255f2d957e585d10bd651
environment: local-macos-arm64
fingerprint: sha256:4d4fadaae3ddc432e2722d2025a7aa335bcb47b8f79b8c467efcd9ccccd0fc4a
criteria: [FLOW-2]
requirements: [REQ-history-013]
contracts: [specs/history/history.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-23T18:08:47Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --bin chough -- --exact direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none for TC-history-direct-write-absent-state-is-none: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 039363a822d1, observed 2026-11-23T18:08:47Z. The raw log stays in the machine-local store.
