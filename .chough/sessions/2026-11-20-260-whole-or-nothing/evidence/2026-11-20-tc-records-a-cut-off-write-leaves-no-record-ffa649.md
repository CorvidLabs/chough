---
kind: evidence
id: 2026-11-20-tc-records-a-cut-off-write-leaves-no-record-ffa649
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
revision: b5e461265cb8c859ece2aefd1f75db8441b213cd
content: 47f48f674af2b26873198e18d643cde1bc9388765919f10320e0d91364a769a5
environment: local-macos-arm64
fingerprint: sha256:e4ebc789c36a292fe92d12379b34c91a203291eaa0eee1c60e807002e68a80a8
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T18:49:14Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record for TC-records-a-cut-off-write-leaves-no-record: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b5e461265cb8, observed 2026-11-20T18:49:14Z. The raw log stays in the machine-local store.
