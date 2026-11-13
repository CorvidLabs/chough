---
kind: evidence
id: 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-f7c2df
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
duration_seconds: 3
revision: f9b22f2215f9f61de538bbc17c45829385438cba
content: 84a021f93c1361a396a48300717a2b72419f9657749cffc84f701402a1bc2d9a
environment: local-macos-arm64
fingerprint: sha256:b4f400ef7ad19209b51935cf759fd62c2bbf5d09b81b17cf5b5b4548b26e19f7
criteria: [LIFE-2, LIFE-7, KNOW-30]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:30:18Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f9b22f2215f9, observed 2026-11-13T03:30:18Z. The raw log stays in the machine-local store.
