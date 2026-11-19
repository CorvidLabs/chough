---
kind: evidence
id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument
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
revision: 8482f2025741f35d4b56afae8df77248c26f55f3
content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
environment: local-macos-arm64
fingerprint: sha256:cf33b6772c173cbf5452a1f734ff96ae966ae87fae194485bd31f2b3ce806727
criteria: [DRIVE-3]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:12:30Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8482f2025741, observed 2026-11-19T15:12:30Z. The raw log stays in the machine-local store.
