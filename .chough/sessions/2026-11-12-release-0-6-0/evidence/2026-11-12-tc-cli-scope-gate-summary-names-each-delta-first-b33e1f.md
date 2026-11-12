---
kind: evidence
id: 2026-11-12-tc-cli-scope-gate-summary-names-each-delta-first-b33e1f
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_each_delta_first_with_its_requirements for TC-cli-scope-gate-summary-names-each-delta-first: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-scope-gate-summary-names-each-delta-first]
case_digest: [4a2d1c45218f0e0dfb72c5e4f6488080990be96638ee6943e2b77d4fdb233e4b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_scope_gate_summary_names_each_delta_first_with_its_requirements
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:eeb87ebec2bd3b326a30ec391a2b8861e69fca80d8b54679aa6ea461953bcbe9
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:26:02Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_each_delta_first_with_its_requirements for TC-cli-scope-gate-summary-names-each-delta-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:26:02Z. The raw log stays in the machine-local store.
