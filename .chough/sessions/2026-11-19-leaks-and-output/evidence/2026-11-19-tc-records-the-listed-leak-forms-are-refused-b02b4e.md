---
kind: evidence
id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-b02b4e
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
revision: dad03318e83ea9e93b14547cdf5f3dd1f4978ccd
content: 5beee9dcb4c059df64f32c6edb22a87fc208406ace0dc20c3d4567357dfd20ea
environment: local-macos-arm64
fingerprint: sha256:1e92929dc3f24dedfa2adee498427c4a86cd129508a9340c124c167fc9e08171
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:04:13Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dad03318e83e, observed 2026-11-19T03:04:13Z. The raw log stays in the machine-local store.
