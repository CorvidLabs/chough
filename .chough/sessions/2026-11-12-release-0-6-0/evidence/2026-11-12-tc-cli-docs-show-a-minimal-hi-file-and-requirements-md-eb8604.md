---
kind: evidence
id: 2026-11-12-tc-cli-docs-show-a-minimal-hi-file-and-requirements-md-eb8604
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:7854d2f3ecca1124f612ad48bccffa0b5dde3f83f99d16e438ce93c8901e0398
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:21:50Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_new_project_section_shows_a_minimal_hi_file_and_requirements_md for TC-cli-docs-show-a-minimal-hi-file-and-requirements-md: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:21:50Z. The raw log stays in the machine-local store.
