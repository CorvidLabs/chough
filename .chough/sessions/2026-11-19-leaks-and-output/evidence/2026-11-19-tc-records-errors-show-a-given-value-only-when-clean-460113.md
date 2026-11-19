---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-460113
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-errors-show-a-given-value-only-when-clean]
case_digest: [1b9e60bd395240e7bada8ab87a5255a98f5c52063ce7aa6466b737437d87f5e4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - errors_show_a_given_value_only_when_clean
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
environment: local-macos-arm64
fingerprint: sha256:a3173fe28f5ad795e039a319a51c596fa1b057c84a3928a8c5c17277588a5d37
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:39:43Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dfc30a73ea6b, observed 2026-11-19T04:39:43Z. The raw log stays in the machine-local store.
