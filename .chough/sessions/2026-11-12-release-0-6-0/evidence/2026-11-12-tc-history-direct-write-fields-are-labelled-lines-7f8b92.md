---
kind: evidence
id: 2026-11-12-tc-history-direct-write-fields-are-labelled-lines-7f8b92
aliases: []
claim: "Ran cargo test --locked --test direct_writes -- --exact a_direct_write_is_written_as_labelled_lines for TC-history-direct-write-fields-are-labelled-lines: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-direct-write-fields-are-labelled-lines]
case_digest: [ba6a5b94f5f8fca8e9c069f8b5c873a659ced3051402ef9cdb98b545a3a5e576]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - direct_writes
  - "--"
  - "--exact"
  - a_direct_write_is_written_as_labelled_lines
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 0af25ea1480d1d0ce7eadd189f3a8b0c8b520fd2
content: 5511f9a2ec985a55c1f2ae5978bc45fe1b06f0386f884799ca8962145d552f86
environment: local-macos-arm64
fingerprint: sha256:7497c179ef82e264cef689ea26b2d22ebd34c5c76f092783e764a2aa6538d6cf
criteria: [LIFE-2]
requirements: [REQ-history-002]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:19:33Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact a_direct_write_is_written_as_labelled_lines for TC-history-direct-write-fields-are-labelled-lines: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0af25ea1480d, observed 2026-11-12T17:19:33Z. The raw log stays in the machine-local store.
