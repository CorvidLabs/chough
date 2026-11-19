---
kind: evidence
id: 2026-11-18-tc-cli-change-ready-carried-delta
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
duration_seconds: 0
revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
environment: local-macos-arm64
fingerprint: sha256:ea61034797f19a2e19988f482711cc447afdfca5ba96f664c002c2c914196670
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T14:59:02Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c817d3d44418, observed 2026-11-18T14:59:02Z. The raw log stays in the machine-local store.
