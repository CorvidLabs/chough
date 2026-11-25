---
kind: evidence
id: 2026-11-25-tc-cli-approve-passthrough-leak-checks
aliases: []
claim: "Ran cargo test --locked --test change -- --exact approve_refuses_a_leaking_argument_before_specsync_runs for TC-cli-approve-passthrough-leak-checks: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-approve-passthrough-leak-checks]
case_digest: [735502bf5c1f24aca247921d64f052b82e9e90c2da6b2f0caed5275c4a24df37]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - approve_refuses_a_leaking_argument_before_specsync_runs
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: dad6694d96aa5447f9506a45c20934b06a89822d
content: 58390fe0acc2255869e6837eb2783c43ccfb9649a70dac3ff9d37fef353091e2
environment: local
fingerprint: sha256:e3a7e9f334d68e6a9600cb9a8bdb570cf2f642df0e97ce740fe98daea66b1264
criteria: [DRIVE-20]
requirements: [REQ-cli-071]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-25T04:12:33Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test change -- --exact approve_refuses_a_leaking_argument_before_specsync_runs for TC-cli-approve-passthrough-leak-checks: passed (exit 0, tests run 1).**

Recorded as passed in local at revision dad6694d96aa, observed 2026-11-25T04:12:33Z. The raw log stays in the machine-local store.
