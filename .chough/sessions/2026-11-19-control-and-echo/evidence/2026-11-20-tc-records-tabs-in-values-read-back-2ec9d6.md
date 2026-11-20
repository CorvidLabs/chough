---
kind: evidence
id: 2026-11-20-tc-records-tabs-in-values-read-back-2ec9d6
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
duration_seconds: 2
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:0b3f67e20fa1dfa81248bc9c34d9f351d48ee86d048d60b3ed73f373840b9098
criteria: [DRIVE-25, DRIVE-26, KNOW-32, LIFE-3]
requirements: [REQ-records-001, REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:42:42Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:42:42Z. The raw log stays in the machine-local store.
