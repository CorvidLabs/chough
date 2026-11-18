---
kind: evidence
id: 2026-11-18-tc-cli-git-merge-in-progress-warns-and-refuses-writes-437438
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
duration_seconds: 2
revision: 566fecda6a272a61ef8a823c2acffb03594495c1
content: 3c5f94eb7678f52ee96ca07fba132b041f4ce23980991bf846bb36ba9427a713
environment: local-macos-arm64
fingerprint: sha256:713a88edb38fe1f3153d22aca23ae7e3d4e2e8613a4fc9b16271f2160616e3d4
criteria: [DRIVE-17]
requirements: [REQ-cli-069]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:19:09Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test git_merge -- --exact a_merge_conflict_in_chough_warns_and_refuses_writes for TC-cli-git-merge-in-progress-warns-and-refuses-writes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 566fecda6a27, observed 2026-11-18T19:19:09Z. The raw log stays in the machine-local store.
