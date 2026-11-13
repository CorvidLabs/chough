---
kind: evidence
id: 2026-11-13-tc-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_duplicate_bug_add_lists_each_similar_bug_as_a_line for TC-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line]
case_digest: [7c750b73092593f4ea8dcd8c63d70d2500f16ea14dc8c5659501be78ea478a6c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - a_duplicate_bug_add_lists_each_similar_bug_as_a_line
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:c4b38e7bf1b422486a90196e5c34f8669c0cc97290fb9d34d1778f7dd5ab6010
criteria: [FLOW-8]
requirements: [REQ-cli-054]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:23:07Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_duplicate_bug_add_lists_each_similar_bug_as_a_line for TC-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:23:07Z. The raw log stays in the machine-local store.
