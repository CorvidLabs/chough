---
kind: evidence
id: 2026-11-19-tc-history-an-ignored-story-holds-the-finish-2f1763
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-ignored-story-holds-the-finish]
case_digest: [86e02ab9da1d4f5559805d2d6126a8bc190ccf64d1956cc991f5101bbc274b87]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_ignored_story_holds_the_finish_and_names_git_add_f
exit_code: 0
tests_run: 1
duration_seconds: 4
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:e8cfbc931a0f40dab471fd228fb73644ebfc1792ac2a22795ae4efcbc2da17d4
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:14:33Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:14:33Z. The raw log stays in the machine-local store.
