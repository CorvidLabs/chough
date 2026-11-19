---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-undeclared-owning-module
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
revision: 590fd061bc73e695e3f83185d527387315fafb78
content: b5a0a88abf009c244f3ec5fcb04584542b0cda76065cf74591f14fdedbf4b1fd
environment: local-macos-arm64
fingerprint: sha256:36228ea63a930f01e58d66a92e5f1347a574137224647f6f24c9b969708a8b61
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:31:02Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_path_whose_owning_module_is_not_declared_is_named for TC-cli-change-ready-undeclared-owning-module: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 590fd061bc73, observed 2026-11-19T04:31:02Z. The raw log stays in the machine-local store.
