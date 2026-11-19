---
kind: evidence
id: 2026-11-19-tc-cli-ship-checks-the-whole-history-effff9
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-checks-the-whole-history]
case_digest: [bfea68e596414a467d4884dc903a45af60f1029df4eeebaabbae5d1ddd968304]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_checks_the_whole_history_and_goes_on_without_a_base
exit_code: 0
tests_run: 1
duration_seconds: 4
revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
environment: local-macos-arm64
fingerprint: sha256:370007a3cc5b30fe505f4140459325ad3a7f171739f7a7bb514600cfdc3c1dde
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:25:19Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0c31209bb80, observed 2026-11-19T15:25:19Z. The raw log stays in the machine-local store.
