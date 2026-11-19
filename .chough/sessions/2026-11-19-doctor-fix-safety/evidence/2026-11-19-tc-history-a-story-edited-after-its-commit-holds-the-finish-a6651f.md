---
kind: evidence
id: 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish-a6651f
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
duration_seconds: 4
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:1cb3b526a3d2e852a7289da4dcfde454cf690dd5e3fecea5bb5d5802792386ad
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:49:37Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:49:37Z. The raw log stays in the machine-local store.
