---
kind: evidence
id: 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph]
case_digest: [5ce9702c9e43233de2be292ee011650bc5c4eea3e9e59ccabb6ebfdcd4d9cff4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - a_bug_body_gives_its_actual_line_its_own_paragraph
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: f4cfb57bd145d15ca613b2747d73c5372137538d
content: 7b5512beec4b9a2f80ee505be47bcd1cb09ac81b65b09d909d878177e022d9ed
environment: local-macos-arm64
fingerprint: sha256:2d82781ce02d8c09b03583986568094edbe249c2ae156d8bfc6c13d333c4b3d6
criteria: [LIFE-2, LIFE-7, KNOW-30]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:11:28Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision f4cfb57bd145, observed 2026-11-13T03:11:28Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_bug_body_gives_its_actual_line_its_own_paragraph stdout ----
- thread 'a_bug_body_gives_its_actual_line_its_own_paragraph' (932806334) panicked at tests/bugs.rs:2273:5:
- ---
- kind: bug
- id: 2026-11-13-shipping-is-charged-at-exactly-the-free-threshold
- aliases: []
- title: Shipping is charged at exactly the free threshold
- status: open
- classification: unclassified
- scope: in-scope
- impact: Not yet assessed.
- criteria: []
- requirements: [REQ-cli-001]
- cases: []
- evidence: []
- receipts: []
- environment: test-local
- revision: 4b50cd91818441016fb136b92cf1d6282383e64c
- found_by: manual report
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.99s
