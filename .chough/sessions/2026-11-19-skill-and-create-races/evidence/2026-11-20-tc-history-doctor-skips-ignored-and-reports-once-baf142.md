---
kind: evidence
id: 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once-baf142
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
revision: 29fb39131c6435612182f52929b94e69ab0dcba3
content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
environment: local-macos-arm64
fingerprint: sha256:f41dc12f1d89ff99e3027ab59524cf3a0421c01637111104c6c728a7e623273a
criteria: [DRIVE-17]
requirements: [REQ-history-016]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:19:47Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once for TC-history-doctor-skips-ignored-and-reports-once: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 29fb39131c64, observed 2026-11-20T14:19:47Z. The raw log stays in the machine-local store.
