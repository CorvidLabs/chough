---
kind: evidence
id: 2026-11-20-tc-records-tabs-in-values-read-back-de8a98
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
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:8e6548d37ce731009587b2ef4f97670feffb090421539d5b4e008f07ad4ccc30
criteria: [DRIVE-25, DRIVE-26, KNOW-32, LIFE-3]
requirements: [REQ-records-001, REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:12:51Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:12:51Z. The raw log stays in the machine-local store.
