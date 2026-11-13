---
kind: evidence
id: 2026-11-13-tc-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line-7ab215
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:730e671432c484948f3a4446b53bcb658bb0ab7bcf50ceed8221167f0ad39dbf
criteria: [FLOW-8]
requirements: [REQ-cli-054]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:49:36Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_duplicate_bug_add_lists_each_similar_bug_as_a_line for TC-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:49:36Z. The raw log stays in the machine-local store.
