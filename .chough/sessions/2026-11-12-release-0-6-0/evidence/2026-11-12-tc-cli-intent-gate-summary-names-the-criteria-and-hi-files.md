---
kind: evidence
id: 2026-11-12-tc-cli-intent-gate-summary-names-the-criteria-and-hi-files
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_names_the_criteria_and_their_hi_files for TC-cli-intent-gate-summary-names-the-criteria-and-hi-files: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-intent-gate-summary-names-the-criteria-and-hi-files]
case_digest: [375e7c6fe515b33e0429fe3791fad082f7f9a47425c818912f4881724ec67afa]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - an_intent_gate_summary_names_the_criteria_and_their_hi_files
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
content: dcb4f087dd1af97765710e6f3754c942490fbfd0fd5d504ff31fe35cbbd50233
environment: local-macos-arm64
fingerprint: sha256:cd8025f0c49d53ad7ce8b7d0dcfb3e9d5723bb61ac02fe64d9727c2441df7175
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:56:09Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_names_the_criteria_and_their_hi_files for TC-cli-intent-gate-summary-names-the-criteria-and-hi-files: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 7fb91195b8f2, observed 2026-11-12T22:56:09Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- an_intent_gate_summary_names_the_criteria_and_their_hi_files stdout ----
- thread 'an_intent_gate_summary_names_the_criteria_and_their_hi_files' (927173441) panicked at tests/gates.rs:786:5:
- Gate: Approve the intent
- Change: none recorded
- What changed: unknown (no base commit recorded)
- Tests: 0 passed, 0 failed, 0 not run
- Risks: none recorded
- Diff: unknown, with no merge base and no recorded base commit
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.78s
