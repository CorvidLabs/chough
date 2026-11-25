---
kind: evidence
id: 2026-11-25-tc-cli-approve-passthrough-refuses-without-relay
aliases: []
claim: "Ran cargo test --locked --test change -- --exact approve_refuses_before_running_without_the_relay for TC-cli-approve-passthrough-refuses-without-relay: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-approve-passthrough-refuses-without-relay]
case_digest: [b02edffafefebe30a051bd93b55399cc9460e27724eb89b057f9f53f399cdb34]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - approve_refuses_before_running_without_the_relay
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: dad6694d96aa5447f9506a45c20934b06a89822d
content: 58390fe0acc2255869e6837eb2783c43ccfb9649a70dac3ff9d37fef353091e2
environment: local
fingerprint: sha256:ef5d6b9785e28dec882c986c387b84528f23f832771016b0f93dacbd4917fea2
criteria: [DRIVE-20]
requirements: [REQ-cli-071]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-25T04:12:32Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test change -- --exact approve_refuses_before_running_without_the_relay for TC-cli-approve-passthrough-refuses-without-relay: passed (exit 0, tests run 1).**

Recorded as passed in local at revision dad6694d96aa, observed 2026-11-25T04:12:32Z. The raw log stays in the machine-local store.
