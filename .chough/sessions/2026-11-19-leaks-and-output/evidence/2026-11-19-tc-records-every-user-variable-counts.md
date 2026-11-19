---
kind: evidence
id: 2026-11-19-tc-records-every-user-variable-counts
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
revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
environment: local-macos-arm64
fingerprint: sha256:2d18a3ec3f0a1e749bd59b19a29147b90e46525824d13208000edb3d1feca402
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:39:57Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dfc30a73ea6b, observed 2026-11-19T04:39:57Z. The raw log stays in the machine-local store.
