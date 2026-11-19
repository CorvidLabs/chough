---
kind: evidence
id: 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish-970e31
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
duration_seconds: 8
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:b2834aff0acb18348a8ba1e07bb8b3ceba95405c4f598c69cabc31feacdb8aae
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:10:46Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:10:46Z. The raw log stays in the machine-local store.
