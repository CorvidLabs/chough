---
kind: evidence
id: 2026-11-12-tc-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_duplicate_bug_add_lists_each_similar_bug_as_a_line for TC-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
content: dcb4f087dd1af97765710e6f3754c942490fbfd0fd5d504ff31fe35cbbd50233
environment: local-macos-arm64
fingerprint: sha256:ebcc5e6b8ea8bc5d668b28c98dea49593390299bbc73511239426ff61fbfd271
criteria: [FLOW-8]
requirements: [REQ-cli-054]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:55:43Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_duplicate_bug_add_lists_each_similar_bug_as_a_line for TC-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 7fb91195b8f2, observed 2026-11-12T22:55:43Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_duplicate_bug_add_lists_each_similar_bug_as_a_line stdout ----
- thread 'a_duplicate_bug_add_lists_each_similar_bug_as_a_line' (927165065) panicked at tests/bugs.rs:2243:5:
- Error: similar bugs exist: [{"id":"2026-11-12-shipping-is-charged-at-exactly-the-free-threshold","score":1.25,"status":"open","title":"Shipping is charged at exactly the free threshold"}]. Append evidence with `chough bug update <id> --observation <id> --note <text>`, or add with --distinct <why this is a different defect>
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 31 filtered out; finished in 0.68s
