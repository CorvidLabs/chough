---
kind: evidence
id: 2026-11-12-tc-records-a-0-5-pointer-still-names-its-session-b81047
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_pointer_naming_a_migrated_key_still_names_its_session for TC-records-a-0-5-pointer-still-names-its-session: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-0-5-pointer-still-names-its-session]
case_digest: [a0cbce8aeeb365c49bbebe626796911f3c14835366c3baccc51931b8411aaf9e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_pointer_naming_a_migrated_key_still_names_its_session
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 8131d51125a2fd4a04848e360a2a4f24034c1de4
content: d33542bea92fc623c44de5e8b45aa6c2002c55e9dbae0443e88d6013988dd4c1
environment: local-macos-arm64
fingerprint: sha256:cfeb79ace7579e3563ed86221b560bf0542158de519a7e14214fe84886e165d2
criteria: [DRIVE-27, LIFE-20, LIFE-8, KNOW-30, LIFE-26]
requirements: [REQ-records-009, REQ-records-003]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T14:57:03Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact a_pointer_naming_a_migrated_key_still_names_its_session for TC-records-a-0-5-pointer-still-names-its-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8131d51125a2, observed 2026-11-12T14:57:03Z. The raw log stays in the machine-local store.
