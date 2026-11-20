---
kind: evidence
id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds
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
duration_seconds: 0
revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
environment: local-macos-arm64
fingerprint: sha256:7fde0017927b7217104d7e61d68756e185d02b609e552d4e8fe5e30c8cff46cc
criteria: [LIFE-18, LIFE-7]
requirements: [REQ-history-015]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:21:06Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d8aa8a63acd5, observed 2026-11-20T03:21:06Z. The raw log stays in the machine-local store.
