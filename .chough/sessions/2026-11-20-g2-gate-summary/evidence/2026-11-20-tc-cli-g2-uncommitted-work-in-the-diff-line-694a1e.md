---
kind: evidence
id: 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-694a1e
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
revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
environment: local-macos-arm64
fingerprint: sha256:94f6917d5a3cbbda8f4854144330648ba3ba55b1968508cd6525955355d57154
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T18:43:44Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 35859e1516e1, observed 2026-11-20T18:43:44Z. The raw log stays in the machine-local store.
