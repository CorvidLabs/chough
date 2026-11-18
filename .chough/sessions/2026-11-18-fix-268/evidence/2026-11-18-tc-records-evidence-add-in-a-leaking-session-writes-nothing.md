---
kind: evidence
id: 2026-11-18-tc-records-evidence-add-in-a-leaking-session-writes-nothing
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact evidence_add_in_a_session_holding_a_leak_writes_nothing for TC-records-evidence-add-in-a-leaking-session-writes-nothing: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-evidence-add-in-a-leaking-session-writes-nothing]
case_digest: [1f377c2c55944a6a9b764cb69d519db6deb1af6b64540e99b3cef5a9cadd3b4a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - evidence_add_in_a_session_holding_a_leak_writes_nothing
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 0ec498884fa5a68bf0c9528b4dc627a0fdf28751
content: 7433f506277b409e303be381d1401a0aba6095c5ff2812ba85b260fd89102238
environment: local-macos-arm64
fingerprint: sha256:fb3ae179306fbe2e4f56cc4f7055bcdb1744648b92165eab22179a37e30cb889
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T15:55:00Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4g
---
**Ran cargo test --locked --test leaks -- --exact evidence_add_in_a_session_holding_a_leak_writes_nothing for TC-records-evidence-add-in-a-leaking-session-writes-nothing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0ec498884fa5, observed 2026-11-18T15:55:00Z. The raw log stays in the machine-local store.
