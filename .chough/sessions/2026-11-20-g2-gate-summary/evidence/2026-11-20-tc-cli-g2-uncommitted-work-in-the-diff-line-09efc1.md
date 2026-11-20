---
kind: evidence
id: 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-09efc1
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
revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
environment: local-macos-arm64
fingerprint: sha256:b95afc473a9f4f1a719a3d5eff2fc994f655abc1b78f0f130f685967cc4e9cf7
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T21:09:41Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 01593390f15c, observed 2026-11-20T21:09:41Z. The raw log stays in the machine-local store.
