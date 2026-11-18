---
kind: evidence
id: 2026-11-18-tc-records-a-conflict-marker-write-is-refused
aliases: []
claim: "Ran cargo test --locked --test records -- --exact a_record_body_with_conflict_markers_is_refused_and_unchanged for TC-records-a-conflict-marker-write-is-refused: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-conflict-marker-write-is-refused]
case_digest: [cadf546fa3b8ae8bb4efcbedcdb4a8d5585e68ebdf5a2a6249342c5ce913be03]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - records
  - "--"
  - "--exact"
  - a_record_body_with_conflict_markers_is_refused_and_unchanged
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 0b7ac77e7acf916d40a464035b1fa22c0b503d29
content: ce9fcb1fb2e58cb8ec126573304156028c1da80195af64bd728d412e98f4c48a
environment: local-macos-arm64
fingerprint: sha256:9824d52eba10e81331bbf42394620abfd0be8c4fda4e21ddca384eeb26f106d1
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:14:54Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_record_body_with_conflict_markers_is_refused_and_unchanged for TC-records-a-conflict-marker-write-is-refused: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0b7ac77e7acf, observed 2026-11-18T19:14:54Z. The raw log stays in the machine-local store.
