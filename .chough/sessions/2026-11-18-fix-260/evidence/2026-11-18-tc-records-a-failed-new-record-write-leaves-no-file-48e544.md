---
kind: evidence
id: 2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file-48e544
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
duration_seconds: 0
revision: c27da8a11eb3437f7ea15ce7cd030ec4ca7deb2f
content: 8c0a66d0502634ece3c11548c15527a3a3e0b9f40720586fd5049c6ca00b3b2b
environment: local-macos-arm64
fingerprint: sha256:8ea5bf1ebd81043eb0d8dd4c672e6023b75d5f8a9956551a9de823cb28eede7a
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T14:09:11Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact a_bug_add_whose_write_fails_part_way_leaves_no_partial_record for TC-records-a-failed-new-record-write-leaves-no-file: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c27da8a11eb3, observed 2026-11-18T14:09:11Z. The raw log stays in the machine-local store.
