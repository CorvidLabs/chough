---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-undocumented-export-b98e9d
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
duration_seconds: 1
revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
environment: local-macos-arm64
fingerprint: sha256:371b83a920363fa8232ecf2af3baff693dabb94f6f4ea632975ee59a1fa6470b
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:11:05Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4f0412cee9a8, observed 2026-11-19T05:11:05Z. The raw log stays in the machine-local store.
