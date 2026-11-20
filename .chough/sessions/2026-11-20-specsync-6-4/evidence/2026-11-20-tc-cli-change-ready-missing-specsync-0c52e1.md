---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-missing-specsync-0c52e1
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
revision: 6571ed05beecf7230bf1ca054a511498a03a3569
content: 3b0af0eee772b0e16f075859206fe29bfddd0832bdb3bdb8464467cea6477800
environment: local-macos-arm64
fingerprint: sha256:73acc8cbaf01bb2e59fea1d3c915a417aee2091674bfa2bd8fb46fb18e25f95c
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T22:23:23Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6571ed05beec, observed 2026-11-20T22:23:23Z. The raw log stays in the machine-local store.
