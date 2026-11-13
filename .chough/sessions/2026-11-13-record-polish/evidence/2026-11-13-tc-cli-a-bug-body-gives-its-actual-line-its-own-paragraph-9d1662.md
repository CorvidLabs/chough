---
kind: evidence
id: 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-9d1662
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
duration_seconds: 4
revision: a1f4735a820dacdb102a4ef4d9c1bda53e31f7bb
content: 65b5bda886fd3fbadb3834aeb5a77124da7d7b9a0d3f00f12ba7d6040b8ce43b
environment: local-macos-arm64
fingerprint: sha256:eeccc88275c6346258883792d66382f979026c5f7e0827f147494273572eefcd
criteria: [LIFE-2, LIFE-7, KNOW-30]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:25:39Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a1f4735a820d, observed 2026-11-13T04:25:39Z. The raw log stays in the machine-local store.
