---
kind: evidence
id: 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-7d5867
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 3453e6e8a2edd34624c6c89b620ed8b262aa3a2c
content: 13e8fbe68ad3139b1c422a706d9abd7d321e4259afebf7c54f9ab96bea045d8e
environment: local-macos-arm64
fingerprint: sha256:378393db8bc823e293f43ca9bd65b88c20b38bfa3308d8a5497d04906a9ebeff
criteria: [LIFE-2, LIFE-7, KNOW-30]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:07:42Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3453e6e8a2ed, observed 2026-11-13T06:07:42Z. The raw log stays in the machine-local store.
