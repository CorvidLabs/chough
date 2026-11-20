---
kind: evidence
id: 2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files-6eb1dd
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact specsync_s_own_untracked_files_are_not_outside_the_declared_paths for TC-cli-ready-leaves-out-specsync-s-own-files: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ready-leaves-out-specsync-s-own-files]
case_digest: [258b68c689ad3d67b56ffa049fb7769d96682adde8dea691c7839d4b457a70df]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - specsync_s_own_untracked_files_are_not_outside_the_declared_paths
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 600a34b4a7212970c8fa4d39958da116e4f2b9b8
content: f288da34b6490daffc348266c23831006988e58d530541024eaaa57cc05f99ec
environment: local-macos-arm64
fingerprint: sha256:449e952e238da4ae0352c44198dc7ee9679e4276d44480ddfff5e5fadb183e63
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T23:41:35Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact specsync_s_own_untracked_files_are_not_outside_the_declared_paths for TC-cli-ready-leaves-out-specsync-s-own-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 600a34b4a721, observed 2026-11-20T23:41:35Z. The raw log stays in the machine-local store.
