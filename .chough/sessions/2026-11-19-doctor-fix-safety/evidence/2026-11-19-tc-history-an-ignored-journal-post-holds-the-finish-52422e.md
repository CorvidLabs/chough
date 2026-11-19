---
kind: evidence
id: 2026-11-19-tc-history-an-ignored-journal-post-holds-the-finish-52422e
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
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:719b5267c466071dfecf7fceaffad92c21a7af06c954843188711d598d529db1
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:14:40Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_holds_the_finish for TC-history-an-ignored-journal-post-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:14:40Z. The raw log stays in the machine-local store.
