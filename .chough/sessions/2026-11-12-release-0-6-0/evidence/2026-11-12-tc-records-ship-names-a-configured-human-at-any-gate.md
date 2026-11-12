---
kind: evidence
id: 2026-11-12-tc-records-ship-names-a-configured-human-at-any-gate
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_refuses_an_unknown_human_at_a_gate_that_drives_nothing for TC-records-ship-names-a-configured-human-at-any-gate: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-ship-names-a-configured-human-at-any-gate]
case_digest: [acac730fbb7744c80420055278f73fe45ee3ee07babd08614a747617b70e2c7d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_refuses_an_unknown_human_at_a_gate_that_drives_nothing
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: ab5426a506a54d3ceb2b6594e6a223761200676a
content: 6ede595e9edcc24100106f9d816a4b060c3c173e358522c42397ec0ba84de954
environment: local-macos-arm64
fingerprint: sha256:3affd467efac65acf5b8535c9c2d3ddf11d1a0d3c344ecca98cc7e234cb67428
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T19:57:37Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test change -- --exact ship_refuses_an_unknown_human_at_a_gate_that_drives_nothing for TC-records-ship-names-a-configured-human-at-any-gate: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision ab5426a506a5, observed 2026-11-12T19:57:37Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- ship_refuses_an_unknown_human_at_a_gate_that_drives_nothing stdout ----
- thread 'ship_refuses_an_unknown_human_at_a_gate_that_drives_nothing' (921966147) panicked at tests/change.rs:678:5:
- the ship took any name
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out; finished in 1.21s
