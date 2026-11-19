---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-draft-fixes-b7fa59
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
duration_seconds: 4
revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
environment: local-macos-arm64
fingerprint: sha256:fd34beb67ace23ba1d1ee27dafe6b668ee9de198e51fecd4780fe29c123b1f5d
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:49:33Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f42ce614db0e, observed 2026-11-19T03:49:33Z. The raw log stays in the machine-local store.
