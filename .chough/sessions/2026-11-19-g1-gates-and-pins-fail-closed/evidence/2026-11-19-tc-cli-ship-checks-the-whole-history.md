---
kind: evidence
id: 2026-11-19-tc-cli-ship-checks-the-whole-history
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
duration_seconds: 2
revision: 8482f2025741f35d4b56afae8df77248c26f55f3
content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
environment: local-macos-arm64
fingerprint: sha256:9cccbfca744e7b5446d0f39d5666114d11c3060275c59ce331ff1202cc8bfd4d
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:12:15Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8482f2025741, observed 2026-11-19T15:12:15Z. The raw log stays in the machine-local store.
