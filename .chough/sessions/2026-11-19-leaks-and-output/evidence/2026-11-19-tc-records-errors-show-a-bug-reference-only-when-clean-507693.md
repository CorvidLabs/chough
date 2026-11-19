---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-507693
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
duration_seconds: 2
revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
environment: local-macos-arm64
fingerprint: sha256:386267b4d443bdd97ed0f684c43f90bbe9a5c10e291fbf459013ca87a3f8df9c
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:39:33Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dfc30a73ea6b, observed 2026-11-19T04:39:33Z. The raw log stays in the machine-local store.
