---
kind: evidence
id: 2026-11-19-tc-cli-ship-checks-the-whole-history-ab5769
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
duration_seconds: 6
revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
environment: local-macos-arm64
fingerprint: sha256:e44c11548fade8eca4601403103451fc630e83345bb93e9b68e324620807b36f
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:43:56Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 49401eb44ee9, observed 2026-11-19T16:43:56Z. The raw log stays in the machine-local store.
