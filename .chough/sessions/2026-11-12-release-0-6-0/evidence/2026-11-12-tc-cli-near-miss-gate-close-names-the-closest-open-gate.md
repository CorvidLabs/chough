---
kind: evidence
id: 2026-11-12-tc-cli-near-miss-gate-close-names-the-closest-open-gate
aliases: []
claim: "Ran cargo test --locked --test relay -- --exact a_near_miss_gate_close_names_the_closest_open_gate for TC-cli-near-miss-gate-close-names-the-closest-open-gate: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-near-miss-gate-close-names-the-closest-open-gate]
case_digest: [006dc3d493a282c6a24f9210a48831f759e21c98f701b8a0a909efbed1c9f4a9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - relay
  - "--"
  - "--exact"
  - a_near_miss_gate_close_names_the_closest_open_gate
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
content: dcb4f087dd1af97765710e6f3754c942490fbfd0fd5d504ff31fe35cbbd50233
environment: local-macos-arm64
fingerprint: sha256:3e19fd214f57aab8a2eb111ce2fa7d88136e362150147a3f711d549c233660df
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:55:50Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test relay -- --exact a_near_miss_gate_close_names_the_closest_open_gate for TC-cli-near-miss-gate-close-names-the-closest-open-gate: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 7fb91195b8f2, observed 2026-11-12T22:55:50Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_near_miss_gate_close_names_the_closest_open_gate stdout ----
- thread 'a_near_miss_gate_close_names_the_closest_open_gate' (927165641) panicked at tests/relay.rs:505:27:
- Release aproval: Error: refused:gate-not-open: no open gate has the exact text "Release aproval", so nothing was saved.
- Run `chough next` to list the open gates, then close one by its exact text.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.65s
