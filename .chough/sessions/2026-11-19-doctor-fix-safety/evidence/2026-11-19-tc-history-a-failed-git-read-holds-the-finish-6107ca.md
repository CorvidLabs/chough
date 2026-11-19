---
kind: evidence
id: 2026-11-19-tc-history-a-failed-git-read-holds-the-finish-6107ca
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
duration_seconds: 4
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:e55580cb7d0e0ea71f9549673d01e891eb01ae8abb15af68f639e5c7579cdc18
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:11:14Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so for TC-history-a-failed-git-read-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:11:14Z. The raw log stays in the machine-local store.
