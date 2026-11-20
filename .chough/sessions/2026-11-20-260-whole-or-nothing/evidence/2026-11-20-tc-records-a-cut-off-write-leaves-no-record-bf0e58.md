---
kind: evidence
id: 2026-11-20-tc-records-a-cut-off-write-leaves-no-record-bf0e58
aliases: []
claim: "Ran cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record for TC-records-a-cut-off-write-leaves-no-record: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-cut-off-write-leaves-no-record]
case_digest: [8958da7751a5e35a7379844b4271e8c9a6de8620daa8ced583ccfde854d50317]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - records
  - "--"
  - "--exact"
  - a_write_cut_off_by_a_size_limit_leaves_no_record
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 8418dff2f5fcfaff322fcee860159ad82e6a593d
content: 5ec53b8bda35b385cc4e52ed962166b271b604f4149eee7e0e597aa2f51a987c
environment: local-macos-arm64
fingerprint: sha256:f745f157aa1215c169ef9eba177c822901e1437159cb87455d5abff126f8efea
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:56:44Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record for TC-records-a-cut-off-write-leaves-no-record: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8418dff2f5fc, observed 2026-11-20T17:56:44Z. The raw log stays in the machine-local store.
