---
kind: evidence
id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-723c21
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
duration_seconds: 1
revision: 908d3ace4545d270125d13706f6b012cdc276e21
content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
environment: local-macos-arm64
fingerprint: sha256:c1faeb0317875664f13bd0bb020f7ac085bb038ff4275d542021bc59dd12e778
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:11:27Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 908d3ace4545, observed 2026-11-19T05:11:27Z. The raw log stays in the machine-local store.
