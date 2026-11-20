---
kind: evidence
id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-7f3465
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
revision: 92b89b970299939dd38635eab1c13fb1952b539a
content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
environment: local-macos-arm64
fingerprint: sha256:eb626bbb221e3910801c406ca6207728659004af6f5e2a5b3dd74ad522f0f764
criteria: [LIFE-18, LIFE-7]
requirements: [REQ-history-015]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:23:21Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 92b89b970299, observed 2026-11-20T03:23:21Z. The raw log stays in the machine-local store.
