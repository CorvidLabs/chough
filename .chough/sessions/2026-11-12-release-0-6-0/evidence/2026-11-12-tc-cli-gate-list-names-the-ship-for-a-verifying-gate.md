---
kind: evidence
id: 2026-11-12-tc-cli-gate-list-names-the-ship-for-a-verifying-gate
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why for TC-cli-gate-list-names-the-ship-for-a-verifying-gate: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-list-names-the-ship-for-a-verifying-gate]
case_digest: [7c6dcca7865896309780ac2c8587318876e678380ef07f5c99f1dff3a2743cfd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why
exit_code: 101
tests_run: 1
duration_seconds: 3
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
content: ab5a1023f46b5eb925e66eb0451bfdf545f677a3592abbfceebad0d2f81c46c0
environment: local-macos-arm64
fingerprint: sha256:d30146d1e8ad6c1dd958d08cba5c6d237d2e6c09fe397c23a6beead162c7b0a1
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:01:59Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why for TC-cli-gate-list-names-the-ship-for-a-verifying-gate: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 5195bd677425, observed 2026-11-12T21:01:59Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why stdout ----
- thread 'the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why' (924194738) panicked at tests/gates.rs:666:5:
- List 4e03c750376b. Record one reply for listed gates with `chough gate record 4e03c750376b --gates <n>[,<n>…] --decided-by "<who>" --words "<their words>" --where "<where>" --recorded-by "<agent>"`.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10 filtered out; finished in 1.95s
