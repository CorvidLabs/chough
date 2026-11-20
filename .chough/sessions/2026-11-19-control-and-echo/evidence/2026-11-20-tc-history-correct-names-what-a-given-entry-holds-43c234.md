---
kind: evidence
id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-43c234
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
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:b89a9c32aae4d4dbc642004cfa77d4f14be1fa016074b547e7fc94c04a5e4b16
criteria: [LIFE-18, LIFE-7]
requirements: [REQ-history-015]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:41:51Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:41:51Z. The raw log stays in the machine-local store.
