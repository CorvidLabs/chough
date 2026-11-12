---
kind: evidence
id: 2026-11-12-tc-history-relay-fields-are-labelled-lines-8fe7da
aliases: []
claim: "Ran cargo test --locked --test relay -- --exact a_relayed_decision_is_written_as_labelled_lines_and_reads_back for TC-history-relay-fields-are-labelled-lines: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-relay-fields-are-labelled-lines]
case_digest: [7debd07a0eb61ea6cdb4dfbad84fc037f44a3ecfa21f3452a633ae39ee4ccd17]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - relay
  - "--"
  - "--exact"
  - a_relayed_decision_is_written_as_labelled_lines_and_reads_back
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 0af25ea1480d1d0ce7eadd189f3a8b0c8b520fd2
content: 5511f9a2ec985a55c1f2ae5978bc45fe1b06f0386f884799ca8962145d552f86
environment: local-macos-arm64
fingerprint: sha256:d4c07aad7113000fbecd446a73a0d90e2732724bf30251498d134c5e70f0c7af
criteria: [LIFE-2]
requirements: [REQ-history-002]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:19:28Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test relay -- --exact a_relayed_decision_is_written_as_labelled_lines_and_reads_back for TC-history-relay-fields-are-labelled-lines: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0af25ea1480d, observed 2026-11-12T17:19:28Z. The raw log stays in the machine-local store.
