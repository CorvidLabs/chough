---
kind: evidence
id: 2026-11-20-tc-cli-ready-a-whole-project-scope-leaves-nothing-out
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact another_change_declaring_the_whole_project_leaves_nothing_out for TC-cli-ready-a-whole-project-scope-leaves-nothing-out: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ready-a-whole-project-scope-leaves-nothing-out]
case_digest: [f8b38c4cd90c2d2f8487170f1f86d0fdd68fc869ca3cebc704dc21f11a4dfb29]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - another_change_declaring_the_whole_project_leaves_nothing_out
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 6571ed05beecf7230bf1ca054a511498a03a3569
content: 3b0af0eee772b0e16f075859206fe29bfddd0832bdb3bdb8464467cea6477800
environment: local-macos-arm64
fingerprint: sha256:9a8bcce97cec6c0c96ea9150714b727a611000f83f311a1916e224230a3508a0
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T22:23:16Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact another_change_declaring_the_whole_project_leaves_nothing_out for TC-cli-ready-a-whole-project-scope-leaves-nothing-out: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6571ed05beec, observed 2026-11-20T22:23:16Z. The raw log stays in the machine-local store.
