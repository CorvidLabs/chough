---
kind: evidence
id: 2026-11-19-tc-records-gate-record-redact-redacts-the-close
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
duration_seconds: 1
revision: b84728117b64a7ed3ac049540fde93c0228c58cb
content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
environment: local-macos-arm64
fingerprint: sha256:130c896e6d6eecefe18c38e5bf19d4cbb37f81d69041d1cfde5ae51d1018b19d
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:16:19Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b84728117b64, observed 2026-11-19T04:16:19Z. The raw log stays in the machine-local store.
