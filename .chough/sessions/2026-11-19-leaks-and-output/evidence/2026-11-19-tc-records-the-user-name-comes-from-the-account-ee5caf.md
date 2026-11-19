---
kind: evidence
id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-ee5caf
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-the-user-name-comes-from-the-account]
case_digest: [8973a6a08e273e42441c965a9310563cfcf78774ec6d7521d01cf833c4ea58f3]
command: [cargo, test, --locked, --test, leaks, --, --exact, the_user_name_comes_from_the_account]
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
environment: local-macos-arm64
fingerprint: sha256:73a52a2865b176896b15352db591a422cb4ab122930b1f3b98c7df087d3d4bbb
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:39:20Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dfc30a73ea6b, observed 2026-11-19T04:39:20Z. The raw log stays in the machine-local store.
