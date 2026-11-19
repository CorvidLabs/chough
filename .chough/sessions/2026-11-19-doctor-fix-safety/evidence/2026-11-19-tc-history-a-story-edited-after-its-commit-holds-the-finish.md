---
kind: evidence
id: 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-story-edited-after-its-commit-holds-the-finish]
case_digest: [2caa4b3395afbcb07ad09017b5940a9f729d0b486ef2cf2abe9933fad7c32711]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_story_edited_after_its_commit_holds_the_finish
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
environment: local-macos-arm64
fingerprint: sha256:4a813b3cd5a99e82df1544c60803bbc32e353e57dffbb7aa952d0a6eb685dc10
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:31:57Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 44aec05c95e2, observed 2026-11-19T14:31:57Z. The raw log stays in the machine-local store.
