---
kind: evidence
id: 2026-11-20-tc-history-doctor-reads-uncommitted-records-2c7a6f
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records for TC-history-doctor-reads-uncommitted-records: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-doctor-reads-uncommitted-records]
case_digest: [ffa17dc175342e495e3f67aee544085353a8ffc9b06e24f88e5838113f777372]
command: [cargo, test, --locked, --test, doctor, --, --exact, doctor_reads_uncommitted_records]
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
environment: local-macos-arm64
fingerprint: sha256:5a462589e6107c144a15a18e949a33497585229a283f8beba18d4be7efd10879
criteria: [DRIVE-17]
requirements: [REQ-history-016]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:36:01Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records for TC-history-doctor-reads-uncommitted-records: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cece9573c0c, observed 2026-11-20T14:36:01Z. The raw log stays in the machine-local store.
