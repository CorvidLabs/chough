---
kind: evidence
id: 2026-11-23-tc-cli-direct-write-absent-state-is-none
aliases: []
claim: "Ran cargo test --locked --lib -- --exact direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none for TC-cli-direct-write-absent-state-is-none: failed (exit 101)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-direct-write-absent-state-is-none]
case_digest: [e05adc756c610ef3d925f6f34b022a999d39f9354d5203e10bd2fffe4d8d629c]
command:
  - cargo
  - test
  - "--locked"
  - "--lib"
  - "--"
  - "--exact"
  - direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none
exit_code: 101
duration_seconds: 0
revision: 4fbb7766ad13298491229d97ade73bf9515af354
content: 0a54c448ee7bde06381c56621371511f6af1bb835b431c6446501107a46bbbf5
environment: local-macos-arm64
fingerprint: sha256:5e2d5d972d824a6d84cc2b9fe89cb5ea23574f06d208b305492c993ac5a94d0c
criteria: [FLOW-2]
requirements: [REQ-history-013]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"test direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none ... ok\", \"test result: ok. 1 passed\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-23T15:29:04Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --lib -- --exact direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none for TC-cli-direct-write-absent-state-is-none: failed (exit 101).**

Recorded as failed in local-macos-arm64 at revision 4fbb7766ad13, observed 2026-11-23T15:29:04Z. The raw log stays in the machine-local store.

## Why it failed

- The expected output was missing: "test direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none ... ok", "test result: ok. 1 passed".

## Key failing lines

- error: no library targets found in package `chough`
