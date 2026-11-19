---
kind: evidence
id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-8617d8
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
revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
environment: local-macos-arm64
fingerprint: sha256:0a2595f48dd6b114185b36d9d1112bdbbaef1a7ad803af202f19ddeafb02c041
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:53:36Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7db0083b5b70, observed 2026-11-19T04:53:36Z. The raw log stays in the machine-local store.
