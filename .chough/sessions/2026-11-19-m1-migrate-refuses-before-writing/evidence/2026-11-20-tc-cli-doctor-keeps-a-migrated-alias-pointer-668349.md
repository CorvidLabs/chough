---
kind: evidence
id: 2026-11-20-tc-cli-doctor-keeps-a-migrated-alias-pointer-668349
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
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:afbc08af109308d5e353efaa7cc3fa76393c100d39509ce254d1932f9ba3e5eb
criteria: [DRIVE-17]
requirements: [REQ-cli-055]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:00:09Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias for TC-cli-doctor-keeps-a-migrated-alias-pointer: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T04:00:09Z. The raw log stays in the machine-local store.
