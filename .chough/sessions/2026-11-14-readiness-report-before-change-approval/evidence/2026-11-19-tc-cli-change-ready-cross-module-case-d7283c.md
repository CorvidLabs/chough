---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-cross-module-case-d7283c
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
revision: e42c96fec77c56feff65256be9e1d5deb1f34742
content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
environment: local-macos-arm64
fingerprint: sha256:00094fcddd37bc4eaf8e02326ff05ac21e1e8d8aa73a899e79f0a940622e1270
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:12:25Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e42c96fec77c, observed 2026-11-19T04:12:25Z. The raw log stays in the machine-local store.
