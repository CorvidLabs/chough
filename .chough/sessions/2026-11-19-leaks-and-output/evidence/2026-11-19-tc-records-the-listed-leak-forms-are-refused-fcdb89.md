---
kind: evidence
id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-fcdb89
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
duration_seconds: 0
revision: 402a3a5d4ae91b332f3bfbc86fd0661c5860a997
content: 265ddf43d047aa35119f882a9ef474d35d96af50d2b5a4a0230489d2076c36b9
environment: local-macos-arm64
fingerprint: sha256:e0f53f084e9dab20103fb332857ace9c1a65518ad3565c2dcfc39522bbff270a
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:44:25Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 402a3a5d4ae9, observed 2026-11-19T02:44:25Z. The raw log stays in the machine-local store.
