---
kind: evidence
id: 2026-11-12-tc-history-doctor-names-an-outdated-merge-driver-chough-a5c00c
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
revision: 06407b98e650d73a951f505369cb7c694765ffde
content: 20c9710c0555251958526a8d2c4296aeac3f42743b807b3fecd9dd54614bf1da
environment: local-macos-arm64
fingerprint: sha256:aad42645e6177fbd1989ba8168b1d6dd415bbb389f40d4d69b99f3a2758a3d6e
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:21:03Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test doctor -- --exact doctor_reports_merge_drivers_that_would_run_an_older_chough for TC-history-doctor-names-an-outdated-merge-driver-chough: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 06407b98e650, observed 2026-11-12T21:21:03Z. The raw log stays in the machine-local store.
