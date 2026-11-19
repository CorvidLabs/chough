---
kind: evidence
id: 2026-11-19-tc-records-gate-record-redact-redacts-the-close-be202e
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-gate-record-redact-redacts-the-close]
case_digest: [8d7158e0728006a4500fd90d93739207c92f10d0a7b4237b003814986f37dae2]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - gate_record_redact_redacts_in_the_close_it_runs
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
environment: local-macos-arm64
fingerprint: sha256:70d6b782f968119f0eae2cdc5797e8439979ae16db29bd685138054adab3b517
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:23:31Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 75f43060e66b, observed 2026-11-19T14:23:31Z. The raw log stays in the machine-local store.
