---
kind: evidence
id: 2026-11-14-tc-cli-ship-relay-hyphen-values-join-their-flags-00f54b
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
revision: bcf921c319f4517914f551449d4d01c1dff2b1d7
content: 707c841c2633d7baa9e07e60c019a6bea208ce226103b37c6fcd2d9b5b77d2b1
environment: local-macos-arm64
fingerprint: sha256:fe1a69b7036cc7a2d768d203c162e4eb4f4aa554b157b3e31865eccd9cd306ef
criteria: [FLOW-16]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-14T19:25:22Z
actor:
  - agent: claude-code
---
**Ran cargo test --locked --test change -- --exact ship_passes_a_relay_that_starts_with_a_hyphen_as_one_argument_per_field for TC-cli-ship-relay-hyphen-values-join-their-flags: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision bcf921c319f4, observed 2026-11-14T19:25:22Z. The raw log stays in the machine-local store.
