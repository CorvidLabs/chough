---
kind: evidence
id: 2026-11-20-tc-records-tabs-in-values-read-back-867742
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
revision: 92b89b970299939dd38635eab1c13fb1952b539a
content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
environment: local-macos-arm64
fingerprint: sha256:ca05ea93fa44a9046c143213e20e36da866f635a85a7d758176c4390d0226121
criteria: [DRIVE-25, DRIVE-26, KNOW-32, LIFE-3]
requirements: [REQ-records-001, REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:24:18Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 92b89b970299, observed 2026-11-20T03:24:18Z. The raw log stays in the machine-local store.
