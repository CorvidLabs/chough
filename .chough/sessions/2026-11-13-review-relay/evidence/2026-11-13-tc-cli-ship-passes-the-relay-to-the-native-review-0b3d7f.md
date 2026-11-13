---
kind: evidence
id: 2026-11-13-tc-cli-ship-passes-the-relay-to-the-native-review-0b3d7f
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_passes_the_relay_to_the_native_review for TC-cli-ship-passes-the-relay-to-the-native-review: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-passes-the-relay-to-the-native-review]
case_digest: [1f7ac0303b8fee361fdb17ae32edd9e1bd46009a8e170e774bb5e651e4044ea5]
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
revision: 75d26c2b22627cc200adc387135184a2b2ea6e9f
content: 938ce32ddca4a521c54a2a904c9e2a7c795174d309d73644b9b67c0fdfedd641
environment: local-macos-arm64
fingerprint: sha256:d44773f8817fb5494405a594b764c098642878362dcd0b25e297430ceaff3f53
criteria: [DRIVE-20, FLOW-23]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T07:59:01Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test change -- --exact ship_passes_the_relay_to_the_native_review for TC-cli-ship-passes-the-relay-to-the-native-review: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 75d26c2b2262, observed 2026-11-13T07:59:01Z. The raw log stays in the machine-local store.
