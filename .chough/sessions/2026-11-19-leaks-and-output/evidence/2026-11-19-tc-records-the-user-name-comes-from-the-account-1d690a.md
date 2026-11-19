---
kind: evidence
id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-1d690a
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
duration_seconds: 0
revision: ccb57abff508d187593c07849872a17c37036014
content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
environment: local-macos-arm64
fingerprint: sha256:bbbca574df0a5b7bc27f415ff1ba7125116c1054d38ba0d7262b046e42398521
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:38:49Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ccb57abff508, observed 2026-11-19T03:38:49Z. The raw log stays in the machine-local store.
