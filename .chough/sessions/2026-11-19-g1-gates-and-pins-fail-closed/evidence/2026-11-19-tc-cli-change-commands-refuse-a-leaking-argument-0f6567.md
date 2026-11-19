---
kind: evidence
id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-0f6567
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
duration_seconds: 1
revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
environment: local-macos-arm64
fingerprint: sha256:f1b4e74c69aed202529f63f9b6cae5453ffafc1485d395effb56a3b90cf16e02
criteria: [DRIVE-3]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:44:25Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 49401eb44ee9, observed 2026-11-19T16:44:25Z. The raw log stays in the machine-local store.
