---
kind: evidence
id: 2026-11-19-tc-history-a-failed-git-read-holds-the-finish-7038fa
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so for TC-history-a-failed-git-read-holds-the-finish: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-failed-git-read-holds-the-finish]
case_digest: [3f403dd3c9fe13b9d9d01caab5d6d7f1c05607d26d93e28fd6ecdbf8697cb5b8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_failed_git_read_holds_the_finish_and_says_so
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:54d53ce094e5a2484755b1624fec5dc3bb8db8a6a4c6d005dac561dee4c95d1e
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:21Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so for TC-history-a-failed-git-read-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:21Z. The raw log stays in the machine-local store.
