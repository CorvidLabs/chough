---
kind: evidence
id: 2026-11-20-tc-cli-bug-add-files-a-bug-beside-a-full-checkpoint-04e584
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs for TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint]
case_digest: [4191fe071fb4b202a3f94f260e5368f80bb5a7dfa484076b83de7def3e83fcfd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
environment: local-macos-arm64
fingerprint: sha256:f8dc9741aecc18843914b0994aa6dbdae882fabbc81a6a506c548ff9b33ccbb6
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:34:43Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs for TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 04c4b86f441c, observed 2026-11-20T16:34:43Z. The raw log stays in the machine-local store.
