---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-no-spec-change-473e45
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
duration_seconds: 1
revision: 240d7dafabeea57a669447149f515f63f6d3e15a
content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
environment: local-macos-arm64
fingerprint: sha256:2a05c0dbe98cda9df7562ce5c080d23d7d8c85aa9eaf833efcd44f1bc2fd5fe1
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:13:42Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 240d7dafabee, observed 2026-11-19T03:13:42Z. The raw log stays in the machine-local store.
