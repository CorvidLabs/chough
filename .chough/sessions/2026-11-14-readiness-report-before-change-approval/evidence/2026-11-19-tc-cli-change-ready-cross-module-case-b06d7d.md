---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-cross-module-case-b06d7d
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-cross-module-case]
case_digest: [9f35cd45d6d08282e4d49df0bf8f98767346d5c83ae6e91e0f2c5be79e097b75]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_case_that_links_another_modules_requirement_is_named
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 240d7dafabeea57a669447149f515f63f6d3e15a
content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
environment: local-macos-arm64
fingerprint: sha256:29ec2589b6df021f6ded849b199106bbb7e89bcb14543b249c4137ebba52eb12
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:13:26Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 240d7dafabee, observed 2026-11-19T03:13:26Z. The raw log stays in the machine-local store.
