---
kind: evidence
id: 2026-11-18-tc-cli-change-ready-cross-module-case
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
revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
environment: local-macos-arm64
fingerprint: sha256:a35a1fc4e1582e146158c4fae21f76b12390c45bc2b0a18d39740dfaedbf1359
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T14:59:06Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c817d3d44418, observed 2026-11-18T14:59:06Z. The raw log stays in the machine-local store.
