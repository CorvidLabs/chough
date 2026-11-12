---
kind: evidence
id: 2026-11-12-tc-cli-docs-show-a-minimal-hi-file-and-requirements-md
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact the_new_project_section_shows_a_minimal_hi_file_and_requirements_md for TC-cli-docs-show-a-minimal-hi-file-and-requirements-md: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: a0152789eec8d548d98dbd11f86b0818ad98f494
content: 0ef70fa8fed5b907a0059b2d0f7320b32f187bc45fae18ee147c4deb03536246
environment: local-macos-arm64
fingerprint: sha256:0a210f95a44d4ead3681e2622cfc4d0c76a6d2ff594e7b92e56541556dd03510
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:25:41Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_new_project_section_shows_a_minimal_hi_file_and_requirements_md for TC-cli-docs-show-a-minimal-hi-file-and-requirements-md: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision a0152789eec8, observed 2026-11-12T20:25:41Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_new_project_section_shows_a_minimal_hi_file_and_requirements_md stdout ----
- thread 'the_new_project_section_shows_a_minimal_hi_file_and_requirements_md' (922843911) panicked at tests/docs.rs:1712:9:
- the new-project section does not show "hi: 1"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s
