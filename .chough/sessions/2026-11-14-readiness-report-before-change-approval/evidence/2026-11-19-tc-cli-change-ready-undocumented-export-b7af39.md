---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-undocumented-export-b7af39
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
revision: 240d7dafabeea57a669447149f515f63f6d3e15a
content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
environment: local-macos-arm64
fingerprint: sha256:420761d6b3ef12908aa18faf0bfb23531827a1cc4eaeca33c532fc123845cccc
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:13:54Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 240d7dafabee, observed 2026-11-19T03:13:54Z. The raw log stays in the machine-local store.
