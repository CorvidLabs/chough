---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-carried-delta-c74f5b
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
revision: 240d7dafabeea57a669447149f515f63f6d3e15a
content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
environment: local-macos-arm64
fingerprint: sha256:82de58d3fc0a18cf46adecb55112d58418bde3862052fc10de3b3b1e0e8fd1ff
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:13:22Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 240d7dafabee, observed 2026-11-19T03:13:22Z. The raw log stays in the machine-local store.
