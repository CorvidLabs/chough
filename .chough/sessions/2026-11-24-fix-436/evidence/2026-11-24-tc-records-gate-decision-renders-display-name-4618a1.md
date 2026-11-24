---
kind: evidence
id: 2026-11-24-tc-records-gate-decision-renders-display-name-4618a1
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact an_alias_close_renders_the_configured_display_name_in_the_decision for TC-records-gate-decision-renders-display-name: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-gate-decision-renders-display-name]
case_digest: [a7101f4d393d057270fb23d2cd79b0387a0df409f6db1d5a63e4b409da838ba2]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - an_alias_close_renders_the_configured_display_name_in_the_decision
exit_code: 0
duration_seconds: 1
revision: 16230c5626ce08ab2e9ffdf0447b9e40b67e3c17
content: 703787281d7e1c5367cb914878749d367144e87ca0b4b399bec417985d838868
environment: local-macos-arm64
fingerprint: sha256:a07f16acd2d3f3856b21d122f9b71289dc0f853775773c38fa703c1ad8003b79
criteria: [FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-24T04:51:43Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test humans -- --exact an_alias_close_renders_the_configured_display_name_in_the_decision for TC-records-gate-decision-renders-display-name: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 16230c5626ce, observed 2026-11-24T04:51:43Z. The raw log stays in the machine-local store.
