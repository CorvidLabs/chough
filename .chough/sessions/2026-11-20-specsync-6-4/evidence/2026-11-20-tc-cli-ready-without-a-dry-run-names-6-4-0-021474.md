---
kind: evidence
id: 2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0-021474
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
revision: 6571ed05beecf7230bf1ca054a511498a03a3569
content: 3b0af0eee772b0e16f075859206fe29bfddd0832bdb3bdb8464467cea6477800
environment: local-macos-arm64
fingerprint: sha256:9fbf35e6e9e8dbaa9e2ee565c0e8ef308c09c52b722e0a26c5c505d9a829c347
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T22:23:04Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_specsync_without_the_dry_run_names_the_install_command for TC-cli-ready-without-a-dry-run-names-6-4-0: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6571ed05beec, observed 2026-11-20T22:23:04Z. The raw log stays in the machine-local store.
