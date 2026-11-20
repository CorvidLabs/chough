---
kind: evidence
id: 2026-11-20-tc-cli-g2-evidence-with-no-case-counts-2af9e7
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-g2-evidence-with-no-case-counts]
case_digest: [951df83095a2697734c253fa89bb31caa1a5ed68454d932a225584d2218ac821]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - evidence_with_no_case_counts_in_the_tests_line
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: b0006dd3acf176c6e05b870c0213084d93116a71
content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
environment: local-macos-arm64
fingerprint: sha256:11bee782b9751c7d57d8b5ccbed4810123bd27f71ebfcca4d69ddf16e2abfab7
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:56:56Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0006dd3acf1, observed 2026-11-20T19:56:56Z. The raw log stays in the machine-local store.
