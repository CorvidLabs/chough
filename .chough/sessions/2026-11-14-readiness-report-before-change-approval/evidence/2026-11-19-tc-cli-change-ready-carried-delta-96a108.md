---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-carried-delta-96a108
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
revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
environment: local-macos-arm64
fingerprint: sha256:e602a9afc04bb08ead7a63df01edf5d2319f1d4d00fd8c2f026bb23c1942f081
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:53:31Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6ce76b8b0ab2, observed 2026-11-19T04:53:31Z. The raw log stays in the machine-local store.
