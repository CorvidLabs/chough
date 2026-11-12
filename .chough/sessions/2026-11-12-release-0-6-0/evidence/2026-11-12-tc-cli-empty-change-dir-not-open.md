---
kind: evidence
id: 2026-11-12-tc-cli-empty-change-dir-not-open
aliases: []
claim: "Red for #148: #157's regression test, applied to the code just before #157 (8f620c39, the first parent of its merge b0fa5b3d), fails: an empty change directory left by a branch switch counts as an open native change"
result: failed
evidence_kind: product-test
case: [TC-cli-empty-change-dir-not-open]
case_digest: [c778d32397bac0d8944cad3f34670c3290a4189e4b11c13eeb69aea13ad5d934]
revision: a57c30169272321d7cea083c2a3ee7885dadad72
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
fingerprint: sha256:cfb5631df25d83d01c78d656ff0a06e0d1efb727ee648b85528520c5fdc2cae8
external: ext-d3b290
criteria: [KNOW-1]
requirements: [REQ-cli-043]
contracts: [specs/cli/cli.spec.md]
limitations:
  - Run by hand in a scratch worktree at 8f620c39 with that one test added; the raw log is an external artifact kept on this machine
observed: 2026-11-12T15:01:57Z
actor:
  - agent: claude-code
    terminal: t382
---
**Red for #148: #157's regression test, applied to the code just before #157 (8f620c39, the first parent of its merge b0fa5b3d), fails: an empty change directory left by a branch switch counts as an open native change.**

Recorded as failed in local-macos-arm64 at revision a57c30169272, observed 2026-11-12T15:01:57Z.

## Key failing lines

- test an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change ... FAILED
- thread 'an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change' (917260746) panicked at tests/evidence.rs:1983:5:
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 28 filtered out; finished in 0.84s
