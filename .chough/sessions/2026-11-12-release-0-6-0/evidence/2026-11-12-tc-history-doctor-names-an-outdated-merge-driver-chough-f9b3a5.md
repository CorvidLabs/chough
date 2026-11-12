---
kind: evidence
id: 2026-11-12-tc-history-doctor-names-an-outdated-merge-driver-chough-f9b3a5
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
revision: 91c3824f94a9742f127a480d297fc7f2175e631c
content: c2aa3b745dd7a94452e0b4feee5950b8b356056ae629fdf516c4dcf5c7d1e6fd
environment: local-macos-arm64
fingerprint: sha256:34db0e1d5d987bc9596d59a778363a3234a431a99f1cca022457e9614404827f
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:07:21Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test doctor -- --exact doctor_reports_merge_drivers_that_would_run_an_older_chough for TC-history-doctor-names-an-outdated-merge-driver-chough: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 91c3824f94a9, observed 2026-11-12T21:07:21Z. The raw log stays in the machine-local store.
