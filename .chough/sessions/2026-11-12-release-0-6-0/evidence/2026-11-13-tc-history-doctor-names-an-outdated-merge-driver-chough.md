---
kind: evidence
id: 2026-11-13-tc-history-doctor-names-an-outdated-merge-driver-chough
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_reports_merge_drivers_that_would_run_an_older_chough for TC-history-doctor-names-an-outdated-merge-driver-chough: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-doctor-names-an-outdated-merge-driver-chough]
case_digest: [24ca88bc8d4fe7aac81f8e9663dee8ab55fa86abc92010a89fd999a0aab9e948]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_reports_merge_drivers_that_would_run_an_older_chough
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:97e6b86acfbe0861071c0927ce40be01e5a5f75e63d576040ef4dcfb8bcbbbbd
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:21:05Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test doctor -- --exact doctor_reports_merge_drivers_that_would_run_an_older_chough for TC-history-doctor-names-an-outdated-merge-driver-chough: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:21:05Z. The raw log stays in the machine-local store.
