---
kind: evidence
id: 2026-11-13-tc-cli-executed-case-run-provenance
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:9e37b630291b4b066a7b21608cd10a38b1aef979da16dc8fab75d888352635a4
criteria: [FLOW-7]
requirements: [REQ-cli-011, REQ-cli-005]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:17:46Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact an_executed_case_runs_provenance_does_not_deny_the_execution for TC-cli-executed-case-run-provenance: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:17:46Z. The raw log stays in the machine-local store.
