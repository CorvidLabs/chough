---
kind: evidence
id: 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once for TC-history-doctor-skips-ignored-and-reports-once: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-doctor-skips-ignored-and-reports-once]
case_digest: [bca77d93afd1d748cd865bf4ee61a60f234c7d0465b9b3bbd91da7f9a523f336]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_skips_ignored_files_and_reports_a_committed_record_once
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 63057710d3223425b0db8d988717f2c657e5035b
content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
environment: local-macos-arm64
fingerprint: sha256:161d34f7d7b676b52736142598875000896c715df1e223ad55569f25fecb4ffc
criteria: [DRIVE-17]
requirements: [REQ-history-016]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T09:20:50Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once for TC-history-doctor-skips-ignored-and-reports-once: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 63057710d322, observed 2026-11-20T09:20:50Z. The raw log stays in the machine-local store.
