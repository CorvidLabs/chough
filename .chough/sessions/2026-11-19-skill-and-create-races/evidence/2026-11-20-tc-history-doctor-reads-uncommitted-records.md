---
kind: evidence
id: 2026-11-20-tc-history-doctor-reads-uncommitted-records
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
duration_seconds: 1
revision: 63057710d3223425b0db8d988717f2c657e5035b
content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
environment: local-macos-arm64
fingerprint: sha256:b01bdad875b83ebecab9403b3a6966cb98aa7151e7e65aa618858029aa4d830f
criteria: [DRIVE-17]
requirements: [REQ-history-016]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T09:20:46Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records for TC-history-doctor-reads-uncommitted-records: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 63057710d322, observed 2026-11-20T09:20:46Z. The raw log stays in the machine-local store.
