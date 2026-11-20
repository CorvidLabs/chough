---
kind: evidence
id: 2026-11-20-tc-cli-g2-a-session-before-the-first-commit-1afd11
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-g2-a-session-before-the-first-commit]
case_digest: [6aac022451b58ebc4595b293330f52a91eeb8d0eefdcccbe621685e39891842d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_session_started_before_the_first_commit_counts_from_it
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: b0006dd3acf176c6e05b870c0213084d93116a71
content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
environment: local-macos-arm64
fingerprint: sha256:27bedee362587b1a42d5f000a6690b800ffd1ee18433f09f6126da3dcb4d48ee
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:57:01Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0006dd3acf1, observed 2026-11-20T19:57:01Z. The raw log stays in the machine-local store.
