---
kind: evidence
id: 2026-11-18-tc-cli-git-merge-in-progress-warns-and-refuses-writes
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
revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
content: a11510fbe57a829f1c9dee98c9d54a139e48ed67859b164d722739af774843ce
environment: local-macos-arm64
fingerprint: sha256:ecbaebfebf1f61a68d9db3da25fc3a54f06c118b71b3b91456128222abd91c2e
criteria: [DRIVE-17]
requirements: [REQ-cli-069]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T18:26:41Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test git_merge -- --exact a_merge_conflict_in_chough_warns_and_refuses_writes for TC-cli-git-merge-in-progress-warns-and-refuses-writes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cef86e2622a2, observed 2026-11-18T18:26:41Z. The raw log stays in the machine-local store.
