---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-cc18d8
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
duration_seconds: 1
revision: ccb57abff508d187593c07849872a17c37036014
content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
environment: local-macos-arm64
fingerprint: sha256:59487c7e7a7015bf725ebb609c40133f59b7c77dca05bf06f77e0496da6a88d5
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:38:57Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ccb57abff508, observed 2026-11-19T03:38:57Z. The raw log stays in the machine-local store.
