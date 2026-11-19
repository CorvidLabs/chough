---
kind: evidence
id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-the-listed-leak-forms-are-refused]
case_digest: [e3434ea14829075b3529ca6a1192d4ab045c58637f1109d22566bab2931531b1]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - the_listed_leak_forms_are_refused_and_counted
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: d7a716ff88a492e7d4c31e77a3ea828a5f5e00d5
content: ba0e70f36f347b58b2f49da39e0b55e9067112bffcd451b86d9c47ec3803a28c
environment: local-macos-arm64
fingerprint: sha256:06c6dc1bbb2226877eb148a9ad3cbf9a44eeee67f696bf87c3d08c21abf176a2
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:43:18Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d7a716ff88a4, observed 2026-11-19T02:43:18Z. The raw log stays in the machine-local store.
