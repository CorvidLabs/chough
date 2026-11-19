---
kind: evidence
id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-f9f0f8
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
revision: 753a022557083e0873d048ced9f5be2b40f686fd
content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
environment: local-macos-arm64
fingerprint: sha256:672bf5bb6f1298ec78f0427625a7c2c33135882b34616fe46d9aeaf05aab13c3
criteria: [DRIVE-3]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:17:42Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 753a02255708, observed 2026-11-19T17:17:42Z. The raw log stays in the machine-local store.
