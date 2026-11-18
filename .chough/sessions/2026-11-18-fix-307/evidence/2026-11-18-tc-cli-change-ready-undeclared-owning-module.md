---
kind: evidence
id: 2026-11-18-tc-cli-change-ready-undeclared-owning-module
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_path_whose_owning_module_is_not_declared_is_named for TC-cli-change-ready-undeclared-owning-module: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-undeclared-owning-module]
case_digest: [29a9c2e4254764d0b622425dcee3fabdf84df8000f53fb71518036bf3089b674]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_path_whose_owning_module_is_not_declared_is_named
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 315aca55cae30ebf05f77d0af8dae048298e24ff
content: 6a4532d9dbc4960d8438a31425c57d1a6b56f73e4ccb57d4d9b21be11763afc6
environment: local-macos-arm64
fingerprint: sha256:1e112a526452d46c0181377f06e5fe822ddec2777b8093ca408905c5fdf4ce84
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T14:53:18Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_path_whose_owning_module_is_not_declared_is_named for TC-cli-change-ready-undeclared-owning-module: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 315aca55cae3, observed 2026-11-18T14:53:18Z. The raw log stays in the machine-local store.
