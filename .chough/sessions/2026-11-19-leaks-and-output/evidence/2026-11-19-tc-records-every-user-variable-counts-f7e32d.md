---
kind: evidence
id: 2026-11-19-tc-records-every-user-variable-counts-f7e32d
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-every-user-variable-counts]
case_digest: [01512ff4c0f2dbaa33be4cbaaddd3968f785c41c406aaf70e1db62e130091dc2]
command: [cargo, test, --locked, --test, leaks, --, --exact, every_user_variable_that_is_set_counts]
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
environment: local-macos-arm64
fingerprint: sha256:3aa5b12bd6054f2b8094be29daffe2cbd26a4037f57986dbc6ec711784a681d3
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:54:18Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7db0083b5b70, observed 2026-11-19T04:54:18Z. The raw log stays in the machine-local store.
