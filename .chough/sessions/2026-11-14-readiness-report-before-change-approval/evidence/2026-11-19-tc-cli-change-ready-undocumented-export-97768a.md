---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-undocumented-export-97768a
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
duration_seconds: 2
revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
environment: local-macos-arm64
fingerprint: sha256:333636b0c2c7caa6a602a85e35c7f049d3c87892baa483412d07cf18a69ac9eb
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:50:05Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f42ce614db0e, observed 2026-11-19T03:50:05Z. The raw log stays in the machine-local store.
