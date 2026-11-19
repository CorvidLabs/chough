---
kind: evidence
id: 2026-11-19-tc-records-gate-record-redact-redacts-the-close-fbe4ff
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
duration_seconds: 2
revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
environment: local-macos-arm64
fingerprint: sha256:4ae6dc65a3f0b4fc0ff220b24dca803e18f9cda1fa8a73a37a63e20468d25747
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:54:14Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7db0083b5b70, observed 2026-11-19T04:54:14Z. The raw log stays in the machine-local store.
