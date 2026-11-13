---
kind: evidence
id: 2026-11-13-tc-cli-executed-case-run-provenance-cc66ad
aliases: []
claim: "Ran cargo test --locked --test cases -- --exact an_executed_case_runs_provenance_does_not_deny_the_execution for TC-cli-executed-case-run-provenance: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-executed-case-run-provenance]
case_digest: [cbe21fe31425e47212042f41586cfee40f753a1337404bc0c24fd4f861c81d76]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cases
  - "--"
  - "--exact"
  - an_executed_case_runs_provenance_does_not_deny_the_execution
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:abf2aa77b2e576888ca8974e5dc4bbfaae48869f9989f4917d908b9b6675061e
criteria: [FLOW-7]
requirements: [REQ-cli-011, REQ-cli-005]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:08:28Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact an_executed_case_runs_provenance_does_not_deny_the_execution for TC-cli-executed-case-run-provenance: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:08:28Z. The raw log stays in the machine-local store.
