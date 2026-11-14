---
kind: evidence
id: 2026-11-14-tc-cli-ship-relay-hyphen-values-join-their-flags
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_passes_a_relay_that_starts_with_a_hyphen_as_one_argument_per_field for TC-cli-ship-relay-hyphen-values-join-their-flags: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-relay-hyphen-values-join-their-flags]
case_digest: [ef91a8f05d187c930df04784ec31d54cd1d58bce61baa88ac0d221d896cbb16e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_passes_a_relay_that_starts_with_a_hyphen_as_one_argument_per_field
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 29ab84774fbabc23ced2eeae6940d4d35b0ce3af
content: c467396120570e9d41fddc76eb5d24498d6b4b88b6379fed3aae6584124655c3
environment: local-macos-arm64
fingerprint: sha256:8e4a87af26dc903238ff864ef50570da492f17288e0a81c556e41f664e54b23d
criteria: [FLOW-16]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-14T19:23:28Z
actor:
  - agent: claude-code
---
**Ran cargo test --locked --test change -- --exact ship_passes_a_relay_that_starts_with_a_hyphen_as_one_argument_per_field for TC-cli-ship-relay-hyphen-values-join-their-flags: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 29ab84774fba, observed 2026-11-14T19:23:28Z. The raw log stays in the machine-local store.
