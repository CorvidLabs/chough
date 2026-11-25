---
kind: evidence
id: 2026-11-25-tc-cli-approve-passthrough-pass-through
aliases: []
claim: "Ran cargo test --locked --test change -- --exact approve_passes_specsync_output_and_exit_code_through for TC-cli-approve-passthrough-pass-through: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-approve-passthrough-pass-through]
case_digest: [2c0889e9950844e4d7e6e72d3ed4e8113a568a3ec574a934fb7fcdeed31da331]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - approve_passes_specsync_output_and_exit_code_through
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: dad6694d96aa5447f9506a45c20934b06a89822d
content: 58390fe0acc2255869e6837eb2783c43ccfb9649a70dac3ff9d37fef353091e2
environment: local
fingerprint: sha256:727a80c617f72bdc7a94aa98b282ab6ff688c4fb654f0e99225fcb55533b7de7
criteria: [DRIVE-20]
requirements: [REQ-cli-071]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-25T04:12:32Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test change -- --exact approve_passes_specsync_output_and_exit_code_through for TC-cli-approve-passthrough-pass-through: passed (exit 0, tests run 1).**

Recorded as passed in local at revision dad6694d96aa, observed 2026-11-25T04:12:32Z. The raw log stays in the machine-local store.
