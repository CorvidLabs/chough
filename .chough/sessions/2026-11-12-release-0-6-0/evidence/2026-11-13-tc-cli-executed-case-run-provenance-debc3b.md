---
kind: evidence
id: 2026-11-13-tc-cli-executed-case-run-provenance-debc3b
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:62b1a70dd5a71caa5f8c4ff1be80be2c9c4231ef496f696e53a302193a5e5e28
criteria: [FLOW-7]
requirements: [REQ-cli-011, REQ-cli-005]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:42:50Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact an_executed_case_runs_provenance_does_not_deny_the_execution for TC-cli-executed-case-run-provenance: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:42:50Z. The raw log stays in the machine-local store.
