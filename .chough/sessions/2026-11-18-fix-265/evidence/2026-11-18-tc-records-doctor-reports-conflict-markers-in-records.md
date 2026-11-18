---
kind: evidence
id: 2026-11-18-tc-records-doctor-reports-conflict-markers-in-records
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
revision: 0b7ac77e7acf916d40a464035b1fa22c0b503d29
content: ce9fcb1fb2e58cb8ec126573304156028c1da80195af64bd728d412e98f4c48a
environment: local-macos-arm64
fingerprint: sha256:b3796bfcf8bd897abd523de6bc8a8947815bad11985b1b25e924157659beba4f
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:15:12Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_reports_conflict_markers_in_committed_records_only for TC-records-doctor-reports-conflict-markers-in-records: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0b7ac77e7acf, observed 2026-11-18T19:15:12Z. The raw log stays in the machine-local store.
