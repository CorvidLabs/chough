---
kind: evidence
id: 2026-11-20-tc-history-an-ignored-story-holds-the-finish
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
duration_seconds: 1
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:8399fc34118565c76d5145a0ea0e51f398ec525b958be760b6007c0eba35387c
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:49:01Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:49:01Z. The raw log stays in the machine-local store.
