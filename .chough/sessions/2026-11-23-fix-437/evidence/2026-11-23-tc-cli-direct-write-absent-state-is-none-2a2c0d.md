---
kind: evidence
id: 2026-11-23-tc-cli-direct-write-absent-state-is-none-2a2c0d
aliases: []
claim: "Ran cargo test --locked --bin chough -- --exact direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none for TC-cli-direct-write-absent-state-is-none: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-direct-write-absent-state-is-none]
case_digest: [2b41086e0ce034184f0b3bd465021396266360db840495286bfb41313092095e]
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
revision: 4fbb7766ad13298491229d97ade73bf9515af354
content: 0a54c448ee7bde06381c56621371511f6af1bb835b431c6446501107a46bbbf5
environment: local-macos-arm64
fingerprint: sha256:87359ca164502f7bb3fac8ee07852256891bc5f8bf4d4f68b068462516c46452
criteria: [FLOW-2]
requirements: [REQ-history-013]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-23T15:29:18Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --bin chough -- --exact direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none for TC-cli-direct-write-absent-state-is-none: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 4fbb7766ad13, observed 2026-11-23T15:29:18Z. The raw log stays in the machine-local store.
