---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-6992a9
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-errors-show-a-given-value-only-when-clean]
case_digest: [1b9e60bd395240e7bada8ab87a5255a98f5c52063ce7aa6466b737437d87f5e4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - errors_show_a_given_value_only_when_clean
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
environment: local-macos-arm64
fingerprint: sha256:35978fe020dea06e2d300b1bbf72a0dd7afc63f28b002eb14671f071e4b80d85
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:54:00Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7db0083b5b70, observed 2026-11-19T04:54:00Z. The raw log stays in the machine-local store.
