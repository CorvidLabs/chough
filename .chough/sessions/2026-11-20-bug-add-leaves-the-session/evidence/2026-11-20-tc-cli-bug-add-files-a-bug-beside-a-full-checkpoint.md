---
kind: evidence
id: 2026-11-20-tc-cli-bug-add-files-a-bug-beside-a-full-checkpoint
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
revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
environment: local-macos-arm64
fingerprint: sha256:1a5c070da9b8a67bfb064629edc9ad6fb56e34b9446e0987cc195b2dcca6df48
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T15:48:28Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs for TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 2f99dcfb42f1, observed 2026-11-20T15:48:28Z. The raw log stays in the machine-local store.
