---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-cross-module-case
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
revision: 24323c5c04434521dfff65da788125d02010f545
content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
environment: local-macos-arm64
fingerprint: sha256:979d5fe346af584bd9fd1257c158c35070d6ff5be6f8e167fecce62ea80361e7
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:16:47Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 24323c5c0443, observed 2026-11-19T02:16:47Z. The raw log stays in the machine-local store.
