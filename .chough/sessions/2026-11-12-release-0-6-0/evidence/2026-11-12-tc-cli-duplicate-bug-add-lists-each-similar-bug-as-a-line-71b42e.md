---
kind: evidence
id: 2026-11-12-tc-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line-71b42e
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:5521ef5b5a455ead759dde24103b9448befb9813946d6e79c2ba7dc716719262
criteria: [FLOW-8]
requirements: [REQ-cli-054]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:25:36Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_duplicate_bug_add_lists_each_similar_bug_as_a_line for TC-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:25:36Z. The raw log stays in the machine-local store.
