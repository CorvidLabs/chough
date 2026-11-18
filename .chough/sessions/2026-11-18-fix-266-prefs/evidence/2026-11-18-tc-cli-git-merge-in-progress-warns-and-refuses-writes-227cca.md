---
kind: evidence
id: 2026-11-18-tc-cli-git-merge-in-progress-warns-and-refuses-writes-227cca
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
revision: 968172b73473b5f9149355b47bb9a59a7ea48d17
content: 845ad9bc3bc1f4da0e725592660cef43f4bc0a8388678ae77206256b820b9c22
environment: local-macos-arm64
fingerprint: sha256:400be492742ccf892028b04ae15b679c153b17828f79c441e90447548eac9275
criteria: [DRIVE-17]
requirements: [REQ-cli-069]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:36:12Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test git_merge -- --exact a_merge_conflict_in_chough_warns_and_refuses_writes for TC-cli-git-merge-in-progress-warns-and-refuses-writes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 968172b73473, observed 2026-11-18T19:36:12Z. The raw log stays in the machine-local store.
