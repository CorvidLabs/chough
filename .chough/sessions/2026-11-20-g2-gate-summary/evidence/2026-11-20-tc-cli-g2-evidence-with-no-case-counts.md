---
kind: evidence
id: 2026-11-20-tc-cli-g2-evidence-with-no-case-counts
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
revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
environment: local-macos-arm64
fingerprint: sha256:e5c8afb839e37ee31bbc9a899439281d10535b2d3d4cb36fb7e1f6b392338689
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:43:15Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cad78fd4d15, observed 2026-11-20T17:43:15Z. The raw log stays in the machine-local store.
