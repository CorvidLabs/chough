---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-draft-fixes-6dee3c
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
duration_seconds: 1
revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
environment: local-macos-arm64
fingerprint: sha256:2b61a21b91f2daa0a277fc24f5f199f1fd69edba11d1a848603cdc5592953c56
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:38:12Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e2c643fed4b, observed 2026-11-19T04:38:12Z. The raw log stays in the machine-local store.
