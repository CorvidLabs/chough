---
kind: evidence
id: 2026-11-12-tc-cli-executed-case-run-provenance
aliases: []
claim: "Ran cargo test --locked --test cases -- --exact an_executed_case_runs_provenance_does_not_deny_the_execution for TC-cli-executed-case-run-provenance: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: a0152789eec8d548d98dbd11f86b0818ad98f494
content: 0ef70fa8fed5b907a0059b2d0f7320b32f187bc45fae18ee147c4deb03536246
environment: local-macos-arm64
fingerprint: sha256:9b791d36054287c17e24c577b67ff65201194d1db612c9413f92ad8aa6c1e5c0
criteria: [FLOW-7]
requirements: [REQ-cli-011, REQ-cli-005]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:25:34Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact an_executed_case_runs_provenance_does_not_deny_the_execution for TC-cli-executed-case-run-provenance: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision a0152789eec8, observed 2026-11-12T20:25:34Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- an_executed_case_runs_provenance_does_not_deny_the_execution stdout ----
- thread 'an_executed_case_runs_provenance_does_not_deny_the_execution' (922842350) panicked at tests/cases.rs:493:9:
- assertion `left == right` failed: TC-cli-0001: {
-   "case": "TC-cli-0001",
-   "observation": "2026-11-12-tc-cli-0001",
-   "outcome": "passed",
-   "provenance": "unauthenticated observation; not proof of human approval, test execution, or deployment",
-   "schema_version": 1,
-   "summary": "Ran cargo test --locked --test cli -- --exact passes for TC-cli-0001: passed (exit 0, tests run 1)."
- }
-   left: String("unauthenticated observation; not proof of human approval, test execution, or deployment")
-  right: "unauthenticated observation; not proof of human approval or deployment"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 35 filtered out; finished in 1.75s
