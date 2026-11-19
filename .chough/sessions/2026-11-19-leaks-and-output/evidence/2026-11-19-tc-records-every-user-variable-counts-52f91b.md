---
kind: evidence
id: 2026-11-19-tc-records-every-user-variable-counts-52f91b
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
duration_seconds: 0
revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
environment: local-macos-arm64
fingerprint: sha256:589c49f93f8897575ee3fc361094ff2b95ffa14dc3dc3bb8b52244a66eb28840
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:23:33Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 75f43060e66b, observed 2026-11-19T14:23:33Z. The raw log stays in the machine-local store.
