---
kind: evidence
id: 2026-11-12-tc-cli-a-gate-close-names-the-step-its-decision-allows
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact a_gate_close_names_the_step_its_decision_allows_never_chough_next for TC-cli-a-gate-close-names-the-step-its-decision-allows: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-gate-close-names-the-step-its-decision-allows]
case_digest: [d0909d9339d52326c1aa3c2a2c29154bfdf33f369b9d3c65572e2a1a2e85d19b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - a_gate_close_names_the_step_its_decision_allows_never_chough_next
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
content: ab5a1023f46b5eb925e66eb0451bfdf545f677a3592abbfceebad0d2f81c46c0
environment: local-macos-arm64
fingerprint: sha256:8394eaf5b57feab598d3e851c6c29b171cd60f9472180ed7950978c8f5c31dbe
criteria: [FLOW-2, FLOW-23]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:01:41Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_gate_close_names_the_step_its_decision_allows_never_chough_next for TC-cli-a-gate-close-names-the-step-its-decision-allows: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 5195bd677425, observed 2026-11-12T21:01:41Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_gate_close_names_the_step_its_decision_allows_never_chough_next stdout ----
- thread 'a_gate_close_names_the_step_its_decision_allows_never_chough_next' (924184094) panicked at tests/humans.rs:907:9:
- Scope approval: Gate "Scope approval" closed (approved); run `chough next` for the next action.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 2.18s
