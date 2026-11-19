---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-carried-delta-4a3b65
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-carried-delta]
case_digest: [06fd1cbe67c7f4336b5dbca85000d41baa9312cade6bbb57a4d42a22724f8493]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_delta_that_already_carries_the_canonical_move_is_not_named
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
environment: local-macos-arm64
fingerprint: sha256:7175794a4519fc4ea25a20cb78fcd28aee28156aa762197bb08b6934af78c812
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:14:22Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5b9c1b4ad6f2, observed 2026-11-19T15:14:22Z. The raw log stays in the machine-local store.
