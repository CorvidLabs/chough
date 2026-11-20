---
kind: evidence
id: 2026-11-20-tc-cli-doctor-keeps-a-migrated-alias-pointer-8e82d3
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias for TC-cli-doctor-keeps-a-migrated-alias-pointer: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 5
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:335bb1e921776b632c56c46ee4a93abb7cdf6c90a6c9c59ec57944e525ec6c68
criteria: [DRIVE-17]
requirements: [REQ-cli-055]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:36:51Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias for TC-cli-doctor-keeps-a-migrated-alias-pointer: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:36:51Z. The raw log stays in the machine-local store.
