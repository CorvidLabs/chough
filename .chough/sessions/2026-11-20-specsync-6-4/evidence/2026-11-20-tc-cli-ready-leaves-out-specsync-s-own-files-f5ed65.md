---
kind: evidence
id: 2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files-f5ed65
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
revision: 6571ed05beecf7230bf1ca054a511498a03a3569
content: 3b0af0eee772b0e16f075859206fe29bfddd0832bdb3bdb8464467cea6477800
environment: local-macos-arm64
fingerprint: sha256:5f61992d093d083fcd327e0b6ea78ab492722342fc8cd71f1c8199f3d761e7a1
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T22:23:06Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact specsync_s_own_untracked_files_are_not_outside_the_declared_paths for TC-cli-ready-leaves-out-specsync-s-own-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6571ed05beec, observed 2026-11-20T22:23:06Z. The raw log stays in the machine-local store.
