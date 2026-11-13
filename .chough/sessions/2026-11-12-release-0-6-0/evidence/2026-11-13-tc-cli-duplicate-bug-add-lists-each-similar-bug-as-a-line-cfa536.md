---
kind: evidence
id: 2026-11-13-tc-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line-cfa536
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:9a157b5cae358eb9ce1f683a181ceffb5bbda56be2195f8a821c34a7ce1ab456
criteria: [FLOW-8]
requirements: [REQ-cli-054]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:16:56Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_duplicate_bug_add_lists_each_similar_bug_as_a_line for TC-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:16:56Z. The raw log stays in the machine-local store.
