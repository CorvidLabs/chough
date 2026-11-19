---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-fast-path-dd926a
aliases: []
claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-fast-path]
case_digest: [82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - package
  - "--"
  - "--exact"
  - the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 240d7dafabeea57a669447149f515f63f6d3e15a
content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
environment: local-macos-arm64
fingerprint: sha256:fdb3cc458a0f3e3cabca72ddffb6a67a22a69e1d41c97320a60982494f8678c8
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:13:34Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 240d7dafabee, observed 2026-11-19T03:13:34Z. The raw log stays in the machine-local store.
