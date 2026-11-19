---
kind: evidence
id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-3a0423
aliases: []
claim: "Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-commands-refuse-a-leaking-argument]
case_digest: [5ef68d263d84f4d077ad328bbc007bf2239b4589c68f5d6ee91350d0b1fcbe2b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - change_commands_refuse_a_leaking_argument_before_specsync_runs
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
environment: local-macos-arm64
fingerprint: sha256:6781b1b8e120e677c2d84be4b56fe7c3dad846f763c40e6349232cfd3708266a
criteria: [DRIVE-3]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:33:44Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 659dcded2db9, observed 2026-11-19T17:33:44Z. The raw log stays in the machine-local store.
