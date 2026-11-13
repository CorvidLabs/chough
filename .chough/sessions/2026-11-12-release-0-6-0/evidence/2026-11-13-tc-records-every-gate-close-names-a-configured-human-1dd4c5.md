---
kind: evidence
id: 2026-11-13-tc-records-every-gate-close-names-a-configured-human-1dd4c5
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact every_gate_close_names_a_configured_human for TC-records-every-gate-close-names-a-configured-human: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-every-gate-close-names-a-configured-human]
case_digest: [0cc23c38572566605fb9a9bdc0daed35c6d6a3de5e4db2aac8e74afd4945bf42]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - every_gate_close_names_a_configured_human
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:6c304cdd6bc0eb25abe795d69d408e490a066101e40df96e05daa9225abd504e
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:41:56Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact every_gate_close_names_a_configured_human for TC-records-every-gate-close-names-a-configured-human: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:41:56Z. The raw log stays in the machine-local store.
