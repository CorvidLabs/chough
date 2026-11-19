---
kind: evidence
id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-d07c05
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
revision: 908d3ace4545d270125d13706f6b012cdc276e21
content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
environment: local-macos-arm64
fingerprint: sha256:e320605e92f0514f9cbe3185694ec7422601c8765b2a855f1dcf7b8a1c4c0af4
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:11:30Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 908d3ace4545, observed 2026-11-19T05:11:30Z. The raw log stays in the machine-local store.
