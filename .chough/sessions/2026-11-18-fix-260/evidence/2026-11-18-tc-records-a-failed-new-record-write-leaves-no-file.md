---
kind: evidence
id: 2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file
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
revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
content: ddf5aec66a22c01afcc7aece6c3dffdc61fa0569294f54c7b11623283971879e
environment: local-macos-arm64
fingerprint: sha256:48929bbabf2804b2f2084ea46f027e3ee13ca079310363c6a35f2334ce86e1db
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T00:37:12Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact a_bug_add_whose_write_fails_part_way_leaves_no_partial_record for TC-records-a-failed-new-record-write-leaves-no-file: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8abb5bb89b52, observed 2026-11-18T00:37:12Z. The raw log stays in the machine-local store.
