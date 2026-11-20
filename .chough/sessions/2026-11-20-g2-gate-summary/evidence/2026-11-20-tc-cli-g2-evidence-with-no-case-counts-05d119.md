---
kind: evidence
id: 2026-11-20-tc-cli-g2-evidence-with-no-case-counts-05d119
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
revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
environment: local-macos-arm64
fingerprint: sha256:ac2b8de6df8ffa77ae680622ddf041ad5fb5c6ea142a0575fccef88b9a9b33e0
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T18:43:42Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 35859e1516e1, observed 2026-11-20T18:43:42Z. The raw log stays in the machine-local store.
