---
kind: evidence
id: 2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0-e43745
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_specsync_without_the_dry_run_names_the_install_command for TC-cli-ready-without-a-dry-run-names-6-4-0: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ready-without-a-dry-run-names-6-4-0]
case_digest: [39ac7efa62d44b4d17300d1bc9ad37e1cffd89e5d1d1dc9adeb330a9b4d6e187]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_specsync_without_the_dry_run_names_the_install_command
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e5571fd735bf8c4c575ef11a69a63a8e14915bad
content: b4f463f5579645db964f839158eda5550438456bd20e553a4249913a565ca384
environment: local-macos-arm64
fingerprint: sha256:3428ad1f43b7576da2b55d3b4b5ddd9bc7032c0fa3f8950a8cd089ac7074b316
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:40:26Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_specsync_without_the_dry_run_names_the_install_command for TC-cli-ready-without-a-dry-run-names-6-4-0: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e5571fd735bf, observed 2026-11-20T19:40:26Z. The raw log stays in the machine-local store.
