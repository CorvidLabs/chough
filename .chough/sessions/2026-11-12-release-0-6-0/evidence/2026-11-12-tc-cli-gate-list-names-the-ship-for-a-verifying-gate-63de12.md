---
kind: evidence
id: 2026-11-12-tc-cli-gate-list-names-the-ship-for-a-verifying-gate-63de12
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why for TC-cli-gate-list-names-the-ship-for-a-verifying-gate: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 06407b98e650d73a951f505369cb7c694765ffde
content: 20c9710c0555251958526a8d2c4296aeac3f42743b807b3fecd9dd54614bf1da
environment: local-macos-arm64
fingerprint: sha256:e7a5f459923cd16d577ba8ce71a6b51210c4350a5296d4bad38b7aee46634a1e
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:20:39Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why for TC-cli-gate-list-names-the-ship-for-a-verifying-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 06407b98e650, observed 2026-11-12T21:20:39Z. The raw log stays in the machine-local store.
