---
kind: evidence
id: 2026-11-20-tc-cli-doctor-keeps-a-migrated-alias-pointer
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias for TC-cli-doctor-keeps-a-migrated-alias-pointer: not-run (exit 101, tests run 0)."
result: not-run
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-doctor-keeps-a-migrated-alias-pointer]
case_digest: [aaa3e31d7e306eb17a846963bba55d7497375e523063f99d8f280035a3dd741d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias
exit_code: 101
tests_run: 0
duration_seconds: 0
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:c6c8d1a534c9744884a60e85812e9fb7cfd602f47f45b1521c347567fc2ecbe1
criteria: [DRIVE-17]
requirements: [REQ-cli-055]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "cargo exited 101 before reporting any test result: a build or environment failure, not an assertion about the behavior"
  - inherited environment variables were not recorded
observed: 2026-11-20T02:24:26Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias for TC-cli-doctor-keeps-a-migrated-alias-pointer: not-run (exit 101, tests run 0).**

Recorded as not-run in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:24:26Z. The raw log stays in the machine-local store.
