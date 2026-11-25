---
kind: evidence
id: 2026-11-25-tc-cli-approve-passthrough-relay-note
aliases: []
claim: "Ran cargo test --locked --test change -- --exact approve_runs_specsync_with_the_relay_in_the_note for TC-cli-approve-passthrough-relay-note: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-approve-passthrough-relay-note]
case_digest: [86d76db76211358960099cc8509eb2765af72ef530aa417d2bcb8104bd7387e8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - approve_runs_specsync_with_the_relay_in_the_note
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: dad6694d96aa5447f9506a45c20934b06a89822d
content: 58390fe0acc2255869e6837eb2783c43ccfb9649a70dac3ff9d37fef353091e2
environment: local
fingerprint: sha256:f85c59ff5d51de62ff4b3c501d695141b19f8f67f5876834422052f689321681
criteria: [DRIVE-20]
requirements: [REQ-cli-071]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-25T04:12:26Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test change -- --exact approve_runs_specsync_with_the_relay_in_the_note for TC-cli-approve-passthrough-relay-note: passed (exit 0, tests run 1).**

Recorded as passed in local at revision dad6694d96aa, observed 2026-11-25T04:12:26Z. The raw log stays in the machine-local store.
