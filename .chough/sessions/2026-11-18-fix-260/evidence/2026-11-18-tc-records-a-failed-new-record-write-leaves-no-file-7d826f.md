---
kind: evidence
id: 2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file-7d826f
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_bug_add_whose_write_fails_part_way_leaves_no_partial_record for TC-records-a-failed-new-record-write-leaves-no-file: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-failed-new-record-write-leaves-no-file]
case_digest: [b5a032152aba26398884a7748273d75dc4c56b71f545dccafcdf338d2b5e24c2]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - a_bug_add_whose_write_fails_part_way_leaves_no_partial_record
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: e78f07ea45d7f47db8b1e920ce2b76d27c19cb4a
content: ff16085e338a870d33e41bf5407651c1bb47d18308d194e9c99df73634e60ee9
environment: local-macos-arm64
fingerprint: sha256:2b23e594a19216c07adc9828b5b2a1874c904b71e22340558ffd065a2cc46925
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T01:54:04Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact a_bug_add_whose_write_fails_part_way_leaves_no_partial_record for TC-records-a-failed-new-record-write-leaves-no-file: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e78f07ea45d7, observed 2026-11-18T01:54:04Z. The raw log stays in the machine-local store.
