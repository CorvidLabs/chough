---
kind: evidence
id: 2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file-23bdea
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
revision: 8fa6e1956af371763b64313cc1d7abe563a2b15f
content: 74a9c7428af47b1c7344d6ed34e2ac8f0d430be3c98a8091da80cf96eeb3c8c1
environment: local-macos-arm64
fingerprint: sha256:fb7af8c7806a37228991956d1087c68eb221efd25ea0cc586678b68938763bcf
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T02:06:30Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact a_bug_add_whose_write_fails_part_way_leaves_no_partial_record for TC-records-a-failed-new-record-write-leaves-no-file: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8fa6e1956af3, observed 2026-11-18T02:06:30Z. The raw log stays in the machine-local store.
