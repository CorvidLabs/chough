---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly-e3d8cf
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-names-a-missing-base-plainly]
case_digest: [d6851ef47f5c60f0fba435275e008a048d4eba48aa7aaef9def0095aff34494f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_base_commit_the_repository_lacks_is_named_plainly
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: ff13be893391d1866f1f881d014ec5d181c7e60f
content: 79d69754fcc4e759ac7714c82d28e0cf90a2841d03adf3a6e37f92b923396d4d
environment: local-macos-arm64
fingerprint: sha256:013aed0570a4e8338fb9fd4fedd939cfade1c059771c88d93ad6c63f6f2baf7e
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:14:34Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ff13be893391, observed 2026-11-20T14:14:34Z. The raw log stays in the machine-local store.
