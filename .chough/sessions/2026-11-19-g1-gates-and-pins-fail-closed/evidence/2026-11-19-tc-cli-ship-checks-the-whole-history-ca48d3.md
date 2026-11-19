---
kind: evidence
id: 2026-11-19-tc-cli-ship-checks-the-whole-history-ca48d3
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
duration_seconds: 1
revision: 753a022557083e0873d048ced9f5be2b40f686fd
content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
environment: local-macos-arm64
fingerprint: sha256:7ef0e1e84756ab7c02d21e4b2798261c20f305774ca42c814e7874eb17aded19
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:17:31Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 753a02255708, observed 2026-11-19T17:17:31Z. The raw log stays in the machine-local store.
