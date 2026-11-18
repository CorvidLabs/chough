---
kind: evidence
id: 2026-11-18-tc-records-doctor-reports-conflict-markers-in-records-fc3116
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_reports_conflict_markers_in_committed_records_only for TC-records-doctor-reports-conflict-markers-in-records: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-doctor-reports-conflict-markers-in-records]
case_digest: [0b90ed1c50eb9fec791cbac41f3696177242cfd54c7047ff6c04fabceaec9bf4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_reports_conflict_markers_in_committed_records_only
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5ae9b83ea3adb45f8f4e52bf5a5c5c5867c316f4
content: c7f5ad5c8d56d0dd0fc2b0413efd1a1b0bdc0a82299c500ec12b7eccae5d5009
environment: local-macos-arm64
fingerprint: sha256:46bf3b9fabc3410aebf59bceb5627af986bb3cba883fcba9a6419c8592158ec0
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:27:13Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_reports_conflict_markers_in_committed_records_only for TC-records-doctor-reports-conflict-markers-in-records: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5ae9b83ea3ad, observed 2026-11-18T20:27:13Z. The raw log stays in the machine-local store.
