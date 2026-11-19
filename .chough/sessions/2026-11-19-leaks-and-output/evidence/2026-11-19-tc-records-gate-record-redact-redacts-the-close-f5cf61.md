---
kind: evidence
id: 2026-11-19-tc-records-gate-record-redact-redacts-the-close-f5cf61
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
revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
environment: local-macos-arm64
fingerprint: sha256:11046964b4867cf6ce7612c6d1a1e9b9106616e19e292cd334f35b67640e00de
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:31:45Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 135a7e22079e, observed 2026-11-19T04:31:45Z. The raw log stays in the machine-local store.
