---
kind: evidence
id: 2026-11-21-cargo-test-locked-test-github-labels-passed-5-tests-including
aliases: []
claim: "cargo test --locked --test github_labels passed 5 tests, including the_script_ignores_a_symlinked_session and the_workflow_does_not_recreate_matching_labels. Those two have no case binding: adding one now would touch a case file outside the approved paths."
result: passed
evidence_kind: product-test
revision: 178b66a1607e3d830bbae4de41ce33996f057ef7
content: fa88d8509c33bb33f34fb1a4445868ab8afad994e194116155fdd1d326830d07
environment: local-macos-arm64
criteria: [LIFE-31]
requirements: [REQ-cli-070]
contracts: []
limitations:
  - The two new tests are not bound to case IDs. The three bound label cases were run separately and passed.
observed: 2026-11-21T17:36:05Z
actor:
  - terminal: t10j
---
**cargo test --locked --test github_labels passed 5 tests, including the_script_ignores_a_symlinked_session and the_workflow_does_not_recreate_matching_labels. Those two have no case binding: adding one now would touch a case file outside the approved paths.**

Recorded as passed in local-macos-arm64 at revision 178b66a1607e, observed 2026-11-21T17:36:05Z.
