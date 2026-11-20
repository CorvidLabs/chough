---
kind: evidence
id: 2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0-4a584e
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
revision: 600a34b4a7212970c8fa4d39958da116e4f2b9b8
content: f288da34b6490daffc348266c23831006988e58d530541024eaaa57cc05f99ec
environment: local-macos-arm64
fingerprint: sha256:f92d94d654bcdf7eabfd8ab4c3b0cf53f7e5e9d9b8ca5e7645d89076da813427
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T23:41:32Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_specsync_without_the_dry_run_names_the_install_command for TC-cli-ready-without-a-dry-run-names-6-4-0: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 600a34b4a721, observed 2026-11-20T23:41:32Z. The raw log stays in the machine-local store.
