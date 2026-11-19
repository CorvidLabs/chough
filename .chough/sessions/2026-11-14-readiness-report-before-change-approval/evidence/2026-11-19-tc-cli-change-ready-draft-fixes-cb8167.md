---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-draft-fixes-cb8167
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-draft-fixes]
case_digest: [cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
environment: local-macos-arm64
fingerprint: sha256:663cd13911ec3c49f6124ce267d451280292124013f8c61b019cbcefb1fba3a4
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:10:42Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4f0412cee9a8, observed 2026-11-19T05:10:42Z. The raw log stays in the machine-local store.
