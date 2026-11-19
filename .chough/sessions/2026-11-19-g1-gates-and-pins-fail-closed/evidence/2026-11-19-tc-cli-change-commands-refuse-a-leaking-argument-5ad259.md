---
kind: evidence
id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-5ad259
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
revision: 08399615be5c89eeecf91690cad2bfa741c2685c
content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
environment: local-macos-arm64
fingerprint: sha256:c40d7c853035c2d8a511be6097dbf5e1440e8db84f96f9026ffbbbff61aa8df2
criteria: [DRIVE-3]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:04:24Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 08399615be5c, observed 2026-11-19T17:04:24Z. The raw log stays in the machine-local store.
