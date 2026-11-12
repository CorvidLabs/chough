---
kind: evidence
id: 2026-11-12-tc-records-the-migrate-preview-warns-of-open-changes-1c9708
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact the_preview_names_open_native_changes_that_make_apply_refuse for TC-records-the-migrate-preview-warns-of-open-changes: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-the-migrate-preview-warns-of-open-changes]
case_digest: [0fd96f550b0e9596f0d30e1a6af6367f05f965adde515adc9600f5fce621d5d8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - the_preview_names_open_native_changes_that_make_apply_refuse
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 8daaad2327e308118c1a4e4df41b9d210ffbd863
content: dc34ec880520da27b814ff6588c9356a5dc93e5d45309a1c09837613c59347ab
environment: local-macos-arm64
fingerprint: sha256:64f34628477a64ddbf68b4509edf7adbd0c872fc7c207ead6f7cb3a9d6123780
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:36:28Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_preview_names_open_native_changes_that_make_apply_refuse for TC-records-the-migrate-preview-warns-of-open-changes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8daaad2327e3, observed 2026-11-12T15:36:28Z. The raw log stays in the machine-local store.
