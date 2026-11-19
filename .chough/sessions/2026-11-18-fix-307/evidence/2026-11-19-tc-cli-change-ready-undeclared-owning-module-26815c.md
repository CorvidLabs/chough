---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-undeclared-owning-module-26815c
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
revision: bddf4cba4316a9d51ea1fd753f9667227a29bd6e
content: fbf0d9683d6bb5d2d5ba739ebac2259e309224bd81465414bc1494cf34cbbbc2
environment: local-macos-arm64
fingerprint: sha256:8f1e9bd9baa741f3b19e1a29396e49d62446568beae45476b163e820912e7bd5
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:16:51Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_path_whose_owning_module_is_not_declared_is_named for TC-cli-change-ready-undeclared-owning-module: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision bddf4cba4316, observed 2026-11-19T05:16:51Z. The raw log stays in the machine-local store.
