---
kind: evidence
id: 2026-11-18-tc-cli-change-ready-undocumented-export
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-undocumented-export]
case_digest: [0cb0b17d7d5e65710413d8d3c217bfb814383aaf357ecae7b605b5a430e486fa]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - an_undocumented_export_in_a_declared_file_is_named
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
environment: local-macos-arm64
fingerprint: sha256:11c87c7c87b9df1cef57c962125b82968ab0b358f744fa96d383932ca22068de
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T14:59:04Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c817d3d44418, observed 2026-11-18T14:59:04Z. The raw log stays in the machine-local store.
