---
kind: evidence
id: 2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files
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
revision: e8d2ecc43843abdac87b9c439fcbd58f991d112b
content: 8ae2b83a6c451ac9ac8c50f3ac993bc64bf2201b907353f186b31caddaab6902
environment: local-macos-arm64
fingerprint: sha256:50b3d442a6a87b23a3c5961476fd6615f518c25f6936fae7b73d6ce055112667
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:12:50Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact specsync_s_own_untracked_files_are_not_outside_the_declared_paths for TC-cli-ready-leaves-out-specsync-s-own-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e8d2ecc43843, observed 2026-11-20T19:12:50Z. The raw log stays in the machine-local store.
