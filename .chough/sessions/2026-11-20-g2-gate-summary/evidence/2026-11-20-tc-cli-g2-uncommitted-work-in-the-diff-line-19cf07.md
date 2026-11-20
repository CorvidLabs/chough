---
kind: evidence
id: 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-19cf07
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-g2-uncommitted-work-in-the-diff-line]
case_digest: [4b5b405a8ca5af0e5003477e3576a039bec7072d9d9ac8cb244ad7de9d4ff878]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - uncommitted_work_is_shown_by_the_diff_line
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: b0006dd3acf176c6e05b870c0213084d93116a71
content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
environment: local-macos-arm64
fingerprint: sha256:e2887ba56b8776981f5d60d7cac80b354dd83c56ed71e8b538cbff5b409764ab
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:56:59Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0006dd3acf1, observed 2026-11-20T19:56:59Z. The raw log stays in the machine-local store.
