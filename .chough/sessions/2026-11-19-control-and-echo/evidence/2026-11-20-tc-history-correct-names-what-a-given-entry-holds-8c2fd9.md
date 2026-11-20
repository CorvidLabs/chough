---
kind: evidence
id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-8c2fd9
aliases: []
claim: "Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-correct-names-what-a-given-entry-holds]
case_digest: [2aec6d6f86db7893a2a401f6cd8f00e56e132fdd023a582f7807df86a5992adc]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - history
  - "--"
  - "--exact"
  - correct_names_what_a_given_entry_holds_and_never_echoes_it
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:dd6a311053dea7fe72b7721588ecd861bae1535d6c5d4a4e767d14936a1395b8
criteria: [LIFE-18, LIFE-7]
requirements: [REQ-history-015]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:11:28Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:11:28Z. The raw log stays in the machine-local store.
