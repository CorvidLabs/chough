---
kind: evidence
id: 2026-11-20-tc-cli-ready-a-whole-project-scope-leaves-nothing-out-8bfa41
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
duration_seconds: 1
revision: 600a34b4a7212970c8fa4d39958da116e4f2b9b8
content: f288da34b6490daffc348266c23831006988e58d530541024eaaa57cc05f99ec
environment: local-macos-arm64
fingerprint: sha256:10b935897489868958562bbc6fcb8a7bfab66e77b4c792648c569f0eaf70fb68
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T23:41:46Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact another_change_declaring_the_whole_project_leaves_nothing_out for TC-cli-ready-a-whole-project-scope-leaves-nothing-out: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 600a34b4a721, observed 2026-11-20T23:41:46Z. The raw log stays in the machine-local store.
