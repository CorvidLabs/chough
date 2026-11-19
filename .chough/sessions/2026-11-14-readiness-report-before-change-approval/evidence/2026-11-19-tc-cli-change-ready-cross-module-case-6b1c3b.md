---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-cross-module-case-6b1c3b
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
duration_seconds: 0
revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
environment: local-macos-arm64
fingerprint: sha256:4d77dc955cdfc159b01f9cc3c716ae1e51207f644f793d6bf8187f26bda34e14
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:10:40Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4f0412cee9a8, observed 2026-11-19T05:10:40Z. The raw log stays in the machine-local store.
