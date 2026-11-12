---
kind: evidence
id: 2026-11-12-tc-cli-readme-helper-table-covers-help
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact readme_helper_table_names_every_command_the_helper_lists for TC-cli-readme-helper-table-covers-help: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-readme-helper-table-covers-help]
case_digest: [ceb5661bfa42aba6427816c6570519bcee4122e58423ab49691845f07f6726e0]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - readme_helper_table_names_every_command_the_helper_lists
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 0af25ea1480d1d0ce7eadd189f3a8b0c8b520fd2
content: 5511f9a2ec985a55c1f2ae5978bc45fe1b06f0386f884799ca8962145d552f86
environment: local-macos-arm64
fingerprint: sha256:dc129673bbebd721a13506be2a19c60f107c3f0f66e0ac50187f31d0b03bfc20
criteria: [FLOW-18, LIFE-4]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:19:58Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact readme_helper_table_names_every_command_the_helper_lists for TC-cli-readme-helper-table-covers-help: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0af25ea1480d, observed 2026-11-12T17:19:58Z. The raw log stays in the machine-local store.
