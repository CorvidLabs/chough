---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-no-spec-change-e0665b
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-no-spec-change]
case_digest: [9fa20b1437572a7de57f9ee3c22d6ec8e9012213e9a5e9f8d7df5337dbcae50c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_no_spec_change_names_every_delta_for_removal
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
environment: local-macos-arm64
fingerprint: sha256:36c7e7ec2ee70b4fef56b2ee7e9b2618e477fffb5b542ab050ebd767f7316364
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:49:45Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f42ce614db0e, observed 2026-11-19T03:49:45Z. The raw log stays in the machine-local store.
