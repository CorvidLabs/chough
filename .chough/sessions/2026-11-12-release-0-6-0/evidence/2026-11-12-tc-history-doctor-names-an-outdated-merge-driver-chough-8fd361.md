---
kind: evidence
id: 2026-11-12-tc-history-doctor-names-an-outdated-merge-driver-chough-8fd361
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:be5e1201af7adc6c7f4218a9edb794f5103c8e70b8105341d43084d280b77b77
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:24:05Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test doctor -- --exact doctor_reports_merge_drivers_that_would_run_an_older_chough for TC-history-doctor-names-an-outdated-merge-driver-chough: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:24:05Z. The raw log stays in the machine-local store.
