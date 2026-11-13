---
kind: evidence
id: 2026-11-13-tc-records-migrate-preview-names-what-doctor-reports-9609f3
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact the_preview_names_the_sessions_doctor_reports_after_the_apply for TC-records-migrate-preview-names-what-doctor-reports: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-preview-names-what-doctor-reports]
case_digest: [6c9a069cdb9cbfb5a53789f87c9a5b593cc427539e89a3364028853ea22c7af5]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - the_preview_names_the_sessions_doctor_reports_after_the_apply
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: ca5685cdd311a905b1747b8464b1abf881e62c07
content: 7a7aea84dc1e09e6ea513c87077793df46508d7c62b826b2330d1abcc6d7bc1c
environment: local-macos-arm64
fingerprint: sha256:b1b230bf6de63d0a93001212ace6fb39eded6a0a78bff6417b01d5b460ff87fa
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T07:46:07Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_preview_names_the_sessions_doctor_reports_after_the_apply for TC-records-migrate-preview-names-what-doctor-reports: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ca5685cdd311, observed 2026-11-13T07:46:07Z. The raw log stays in the machine-local store.
