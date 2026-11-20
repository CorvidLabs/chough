---
kind: evidence
id: 2026-11-20-tc-cli-g2-a-session-before-the-first-commit
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
revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
environment: local-macos-arm64
fingerprint: sha256:99994a7fd5001b996ce22c591c58fd25accadcfcd58f0fe36258648fa8066e83
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:43:20Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cad78fd4d15, observed 2026-11-20T17:43:20Z. The raw log stays in the machine-local store.
