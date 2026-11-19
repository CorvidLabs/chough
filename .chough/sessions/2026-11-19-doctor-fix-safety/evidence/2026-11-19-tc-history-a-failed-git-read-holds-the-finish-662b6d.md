---
kind: evidence
id: 2026-11-19-tc-history-a-failed-git-read-holds-the-finish-662b6d
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
duration_seconds: 2
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:736e31e09685c5025db70f10ec835b374fa81e7004527be456929569b0b906fa
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:14:45Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so for TC-history-a-failed-git-read-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:14:45Z. The raw log stays in the machine-local store.
