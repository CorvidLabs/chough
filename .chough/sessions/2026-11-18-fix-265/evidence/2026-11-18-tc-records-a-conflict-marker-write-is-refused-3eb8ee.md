---
kind: evidence
id: 2026-11-18-tc-records-a-conflict-marker-write-is-refused-3eb8ee
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
revision: 5ae9b83ea3adb45f8f4e52bf5a5c5c5867c316f4
content: c7f5ad5c8d56d0dd0fc2b0413efd1a1b0bdc0a82299c500ec12b7eccae5d5009
environment: local-macos-arm64
fingerprint: sha256:2b21249371ef2295f4869f9471b4979e3843bb2970befaa504420ff6d067b39d
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:27:11Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_record_body_with_conflict_markers_is_refused_and_unchanged for TC-records-a-conflict-marker-write-is-refused: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5ae9b83ea3ad, observed 2026-11-18T20:27:11Z. The raw log stays in the machine-local store.
