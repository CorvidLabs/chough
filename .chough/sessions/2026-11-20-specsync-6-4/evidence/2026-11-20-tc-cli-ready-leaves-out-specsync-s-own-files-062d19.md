---
kind: evidence
id: 2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files-062d19
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
revision: e5571fd735bf8c4c575ef11a69a63a8e14915bad
content: b4f463f5579645db964f839158eda5550438456bd20e553a4249913a565ca384
environment: local-macos-arm64
fingerprint: sha256:e3fd91b1517dc906d9fb8b566c207fdcb41377249ec857c57f3996307a6f325e
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:40:28Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact specsync_s_own_untracked_files_are_not_outside_the_declared_paths for TC-cli-ready-leaves-out-specsync-s-own-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e5571fd735bf, observed 2026-11-20T19:40:28Z. The raw log stays in the machine-local store.
