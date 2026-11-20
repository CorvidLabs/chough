---
kind: evidence
id: 2026-11-20-tc-records-a-fragment-of-a-stored-control-line-is-refused
aliases: []
claim: "Ran cargo test --locked --test output_safety -- --exact a_fragment_of_a_stored_control_line_is_still_refused for TC-records-a-fragment-of-a-stored-control-line-is-refused: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-fragment-of-a-stored-control-line-is-refused]
case_digest: [4e563d17db1240c7916c6bdaedeb23d6066dd601b8d7be50c3774d89b75f7cf1]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - output_safety
  - "--"
  - "--exact"
  - a_fragment_of_a_stored_control_line_is_still_refused
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:5337c160ae7746ace5935d31bcae91e2a3f3ab19e81e3b0b7bd7050845d5a50e
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:27:00Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_fragment_of_a_stored_control_line_is_still_refused for TC-records-a-fragment-of-a-stored-control-line-is-refused: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:27:00Z. The raw log stays in the machine-local store.
