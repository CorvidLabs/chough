---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-83abb7
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
revision: b84728117b64a7ed3ac049540fde93c0228c58cb
content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
environment: local-macos-arm64
fingerprint: sha256:003d4a13a4fd3c7bd705a57d7da87554c01fbf04ca47e5e80e30ca93dbc5f81d
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:16:08Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b84728117b64, observed 2026-11-19T04:16:08Z. The raw log stays in the machine-local store.
