---
kind: evidence
id: 2026-11-12-tc-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line-0396f2
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
revision: e0131c098169579c13f2498db93514969f149f33
content: f90f0ac78534841119984ed8ae2779eb2acada66c1c412a9c38a7acfd38b6442
environment: local-macos-arm64
fingerprint: sha256:e92c9496493374e9403250f3408aec327e815690909fec54b439df04c77133c7
criteria: [FLOW-8]
requirements: [REQ-cli-054]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:01:59Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_duplicate_bug_add_lists_each_similar_bug_as_a_line for TC-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e0131c098169, observed 2026-11-12T23:01:59Z. The raw log stays in the machine-local store.
