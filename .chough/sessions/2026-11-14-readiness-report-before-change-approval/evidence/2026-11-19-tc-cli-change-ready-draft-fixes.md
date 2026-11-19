---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-draft-fixes
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
duration_seconds: 2
revision: 24323c5c04434521dfff65da788125d02010f545
content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
environment: local-macos-arm64
fingerprint: sha256:6d0e963ac291e99c37d00d112be21d65e66ae4c9f0fef4e6a345de54f2ac75ce
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:16:51Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 24323c5c0443, observed 2026-11-19T02:16:51Z. The raw log stays in the machine-local store.
