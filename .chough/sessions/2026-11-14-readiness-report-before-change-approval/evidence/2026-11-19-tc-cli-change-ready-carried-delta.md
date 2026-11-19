---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-carried-delta
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
revision: 24323c5c04434521dfff65da788125d02010f545
content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
environment: local-macos-arm64
fingerprint: sha256:358e67e49cce89f2c08dbe66af5e7eb62fc3116f20a19a63bcd53145ecc8699c
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:16:42Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 24323c5c0443, observed 2026-11-19T02:16:42Z. The raw log stays in the machine-local store.
