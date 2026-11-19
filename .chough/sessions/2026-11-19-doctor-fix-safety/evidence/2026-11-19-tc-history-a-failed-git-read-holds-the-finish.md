---
kind: evidence
id: 2026-11-19-tc-history-a-failed-git-read-holds-the-finish
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
duration_seconds: 1
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:1186bc1bf2667cc81b1cf2280aa4f851b63584a2c280bf967e63cb82d63a6690
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:49:57Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so for TC-history-a-failed-git-read-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:49:57Z. The raw log stays in the machine-local store.
