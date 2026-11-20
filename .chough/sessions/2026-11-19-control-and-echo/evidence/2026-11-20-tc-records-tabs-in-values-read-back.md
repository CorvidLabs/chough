---
kind: evidence
id: 2026-11-20-tc-records-tabs-in-values-read-back
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
revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
environment: local-macos-arm64
fingerprint: sha256:157512629af7fa82c9110b0ee54aeef14d6deee95558737bf29aa0a8ae69f696
criteria: [DRIVE-25, DRIVE-26, KNOW-32, LIFE-3]
requirements: [REQ-records-001, REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:21:52Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d8aa8a63acd5, observed 2026-11-20T03:21:52Z. The raw log stays in the machine-local store.
