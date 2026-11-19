---
kind: evidence
id: 2026-11-19-tc-history-an-ignored-journal-post-holds-the-finish
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_holds_the_finish for TC-history-an-ignored-journal-post-holds-the-finish: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-ignored-journal-post-holds-the-finish]
case_digest: [bcc54521c6af3a3d50e789eab08e36f18f5c3a2f05840ae8f81319eb81161d9a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_ignored_journal_post_holds_the_finish
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:1cb2b1398aa55239795d0b148fb3214ffbe9f4d6a7f7e3e15ce52552d023714d
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:49:53Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_holds_the_finish for TC-history-an-ignored-journal-post-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:49:53Z. The raw log stays in the machine-local store.
