---
kind: evidence
id: 2026-11-20-tc-records-tabs-in-values-read-back-936d51
aliases: []
claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-tabs-in-values-read-back]
case_digest: [40343f67bca7e9c6c4a8e645d7362d1e3a4196492a31339d2db7fa1afb9d3d22]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - front_matter_writes
  - "--"
  - "--exact"
  - a_tab_in_a_given_value_is_refused_and_the_records_stay_readable
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:078a30edd49a46731d8f4d199ba6c8d3951621e8048455a23d33664b20f09785
criteria: [DRIVE-25, DRIVE-26, KNOW-32, LIFE-3]
requirements: [REQ-records-001, REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:27:09Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:27:09Z. The raw log stays in the machine-local store.
