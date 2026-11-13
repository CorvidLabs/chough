---
kind: evidence
id: 2026-11-13-tc-cli-docs-show-a-minimal-hi-file-and-requirements-md-c95d93
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:0d6c23060fee9d938723faaa62860d062fb4b658504500ad0865a0d494364f94
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:43:14Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_new_project_section_shows_a_minimal_hi_file_and_requirements_md for TC-cli-docs-show-a-minimal-hi-file-and-requirements-md: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:43:14Z. The raw log stays in the machine-local store.
