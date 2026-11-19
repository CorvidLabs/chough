---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-96cdb0
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-errors-show-a-bug-reference-only-when-clean]
case_digest: [b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - a_bug_reference_is_shown_only_when_clean
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
environment: local-macos-arm64
fingerprint: sha256:246cce0b2e3a09c7f41e9826ef647fa146cdeaa20b441720a918680b20c9f742
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:31:35Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 135a7e22079e, observed 2026-11-19T04:31:35Z. The raw log stays in the machine-local store.
