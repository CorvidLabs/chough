---
kind: evidence
id: 2026-11-18-tc-cli-change-ready-missing-specsync
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-missing-specsync]
case_digest: [83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_missing_specsync_names_the_install_command
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
environment: local-macos-arm64
fingerprint: sha256:e129e03d8e0f59d6933a2d7c38f1a1dca04e82085e958dfc662af2b8e3d082b1
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T14:59:12Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c817d3d44418, observed 2026-11-18T14:59:12Z. The raw log stays in the machine-local store.
