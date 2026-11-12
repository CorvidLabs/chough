---
kind: evidence
id: 2026-11-12-tc-cli-executed-case-run-provenance-0e0ebc
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
duration_seconds: 2
revision: 34b7cbb75288a57e7249c1bc50eb3354441840b6
content: 1b9f7fae45c3596bc36a084cc2c4b391a1bb119b03fd75d07f671e11dedfaf1d
environment: local-macos-arm64
fingerprint: sha256:f6f805a0869316dca24cca374c812f2e25b779570c9105d0c7029aa577be72da
criteria: [FLOW-7]
requirements: [REQ-cli-011, REQ-cli-005]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:27:08Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact an_executed_case_runs_provenance_does_not_deny_the_execution for TC-cli-executed-case-run-provenance: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 34b7cbb75288, observed 2026-11-12T20:27:08Z. The raw log stays in the machine-local store.
