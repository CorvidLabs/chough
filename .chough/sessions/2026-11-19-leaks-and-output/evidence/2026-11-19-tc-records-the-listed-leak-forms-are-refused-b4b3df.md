---
kind: evidence
id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-b4b3df
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
revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
environment: local-macos-arm64
fingerprint: sha256:ede5fb32f3b36819330e770c2716aca477e347af91388f79a8210c0180b6d2ee
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:31:25Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 135a7e22079e, observed 2026-11-19T04:31:25Z. The raw log stays in the machine-local store.
