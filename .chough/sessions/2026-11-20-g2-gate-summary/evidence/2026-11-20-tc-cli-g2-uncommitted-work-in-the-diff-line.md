---
kind: evidence
id: 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line
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
duration_seconds: 1
revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
environment: local-macos-arm64
fingerprint: sha256:ea3aa31c0e6195acbb212176525c296f0a6984e0193cdb2d18867dc4ab051289
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:43:18Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cad78fd4d15, observed 2026-11-20T17:43:18Z. The raw log stays in the machine-local store.
