---
kind: evidence
id: 2026-11-18-tc-cli-git-merge-in-progress-warns-and-refuses-writes-882968
aliases: []
claim: "Ran cargo test --locked --test git_merge -- --exact a_merge_conflict_in_chough_warns_and_refuses_writes for TC-cli-git-merge-in-progress-warns-and-refuses-writes: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-git-merge-in-progress-warns-and-refuses-writes]
case_digest: [652ab0d5a28143f5a1b2eab2b38f12dfa4c6785d7a916b8f582dc468c00fb8ed]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - git_merge
  - "--"
  - "--exact"
  - a_merge_conflict_in_chough_warns_and_refuses_writes
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: eb8575a88ebf7b69d43a55ecf51357338536f9d7
content: 20d06a99a65f82e9328681d2aa1ccb4c0599bcf887a1d4b5948bb34e05d69043
environment: local-macos-arm64
fingerprint: sha256:9c0d963c20b325761b46a0ea06629cf827d3dbdc67e88b690f2d1c80ad05226d
criteria: [DRIVE-17]
requirements: [REQ-cli-069]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T18:53:14Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test git_merge -- --exact a_merge_conflict_in_chough_warns_and_refuses_writes for TC-cli-git-merge-in-progress-warns-and-refuses-writes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision eb8575a88ebf, observed 2026-11-18T18:53:14Z. The raw log stays in the machine-local store.
