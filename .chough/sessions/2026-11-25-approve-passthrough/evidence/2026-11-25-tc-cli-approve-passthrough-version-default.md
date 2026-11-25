---
kind: evidence
id: 2026-11-25-tc-cli-approve-passthrough-version-default
aliases: []
claim: "Ran cargo test --locked --test change -- --exact approve_defaults_the_recorder_to_the_chough_version for TC-cli-approve-passthrough-version-default: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-approve-passthrough-version-default]
case_digest: [852125c1fb7355f9d4cc558c9a9d9b43a08a60b367a142f7faa64bc643588a89]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - approve_defaults_the_recorder_to_the_chough_version
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: dad6694d96aa5447f9506a45c20934b06a89822d
content: 58390fe0acc2255869e6837eb2783c43ccfb9649a70dac3ff9d37fef353091e2
environment: local
fingerprint: sha256:83d0fae26e6a28b9dc613861015b75ef66f7f7a1f107443efe424c147008851f
criteria: [DRIVE-20]
requirements: [REQ-cli-071]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-25T04:12:31Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test change -- --exact approve_defaults_the_recorder_to_the_chough_version for TC-cli-approve-passthrough-version-default: passed (exit 0, tests run 1).**

Recorded as passed in local at revision dad6694d96aa, observed 2026-11-25T04:12:31Z. The raw log stays in the machine-local store.
