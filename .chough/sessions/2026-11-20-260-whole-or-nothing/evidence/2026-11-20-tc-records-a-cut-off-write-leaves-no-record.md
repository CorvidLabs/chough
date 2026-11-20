---
kind: evidence
id: 2026-11-20-tc-records-a-cut-off-write-leaves-no-record
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
revision: fcba5f1c433bff099b257add834061ef20d128d1
content: 8b4498351d7422a99f5e3262da5e5b6b8289dda8cfd3d68393d7cf783bdbfaa1
environment: local-macos-arm64
fingerprint: sha256:09bf657c1cd96f6b737843a4621f1d2c58f01ba4be89e660fe6eb58195372c98
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:58:25Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record for TC-records-a-cut-off-write-leaves-no-record: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fcba5f1c433b, observed 2026-11-20T16:58:25Z. The raw log stays in the machine-local store.
