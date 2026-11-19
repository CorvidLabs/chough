---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-undeclared-owning-module-74e3ca
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
duration_seconds: 1
revision: cad241218bc0b965a7d3902d614e6d5fc18e149d
content: 622f6744d36049883230644a0a488bb3bc6459145b0e71011b8d9aef9007f533
environment: local-macos-arm64
fingerprint: sha256:4e408774405a9dd8d55e50ae6e7f08664bb273791268ac866c37ede916495ff4
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:10:33Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_path_whose_owning_module_is_not_declared_is_named for TC-cli-change-ready-undeclared-owning-module: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cad241218bc0, observed 2026-11-19T05:10:33Z. The raw log stays in the machine-local store.
