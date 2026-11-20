---
kind: evidence
id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-04b819
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
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:79af7f62340babf794ad338fca13b53251042634ec1bd7720991f6418186d98c
criteria: [LIFE-18, LIFE-7]
requirements: [REQ-history-015]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:25:56Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:25:56Z. The raw log stays in the machine-local store.
