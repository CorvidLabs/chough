---
kind: evidence
id: 2026-11-13-tc-cli-ship-passes-the-relay-to-the-native-review-567cea
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_passes_the_relay_to_the_native_review for TC-cli-ship-passes-the-relay-to-the-native-review: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-passes-the-relay-to-the-native-review]
case_digest: [f8447be060e01366a3d554586a1473896332970e01aa2d05783452c6ce17ef4c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_passes_the_relay_to_the_native_review
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: fd6efe7663471370b9119a34dc4d8f62f86aa487
content: e06fe9c67a7fccb02cb4bd517ea7fc260b3816a28367bcf2497ec25382c12c28
environment: local-macos-arm64
fingerprint: sha256:e4f9a4be5238d84dc51aa0d44dd815e19eb410bf4844baba41346b2f7a0af3ee
criteria: [DRIVE-20, FLOW-23]
requirements: [REQ-cli-059, REQ-records-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T07:44:58Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test change -- --exact ship_passes_the_relay_to_the_native_review for TC-cli-ship-passes-the-relay-to-the-native-review: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd6efe766347, observed 2026-11-13T07:44:58Z. The raw log stays in the machine-local store.
