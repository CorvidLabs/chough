---
kind: evidence
id: 2026-11-12-tc-cli-docs-show-a-minimal-hi-file-and-requirements-md-b3ee78
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact the_new_project_section_shows_a_minimal_hi_file_and_requirements_md for TC-cli-docs-show-a-minimal-hi-file-and-requirements-md: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-show-a-minimal-hi-file-and-requirements-md]
case_digest: [b73653a5ad0ee4e11d5b83387ec8df64e8499937e370e4b686b238d60eb5540b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - the_new_project_section_shows_a_minimal_hi_file_and_requirements_md
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 34b7cbb75288a57e7249c1bc50eb3354441840b6
content: 1b9f7fae45c3596bc36a084cc2c4b391a1bb119b03fd75d07f671e11dedfaf1d
environment: local-macos-arm64
fingerprint: sha256:7cbd6cfdd8c81ce5d437fbada26b7bdf78d07bf6acbf45de72c0d94c162d8997
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:27:15Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_new_project_section_shows_a_minimal_hi_file_and_requirements_md for TC-cli-docs-show-a-minimal-hi-file-and-requirements-md: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 34b7cbb75288, observed 2026-11-12T20:27:15Z. The raw log stays in the machine-local store.
