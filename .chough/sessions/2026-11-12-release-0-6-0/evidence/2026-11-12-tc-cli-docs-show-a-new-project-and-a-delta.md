---
kind: evidence
id: 2026-11-12-tc-cli-docs-show-a-new-project-and-a-delta
aliases: []
claim: "Red: cargo test --locked --test docs -- --exact a_new_project_path_and_the_delta_format_are_documented, run in a throwaway worktree at 56c3f4d3 (before the new-project docs) with only tests/docs.rs from a87b54ec added, failed (exit 101): README.md does not name specsync change adopt"
result: failed
evidence_kind: product-test
method: automated
case: [TC-cli-docs-show-a-new-project-and-a-delta]
case_digest: [893a075c612c13f8c8075047dd64d476c55bb967f8eded8fd8205aee7c193b9a]
revision: 71c823542fe95bc9aa2c6ad1ca3a5eb053175dfe
content: 054b67d651194d55acebf36cc770e6c8aeda5ee26e6c2bf34033671f0eb0f917
environment: local-macos-arm64
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations:
  - Run at 56c3f4d3 in a throwaway worktree with this commit's test, not at this entry's recorded revision
observed: 2026-11-12T17:27:54Z
actor:
  - agent: claude-code
    terminal: t382
---
**Red: cargo test --locked --test docs -- --exact a_new_project_path_and_the_delta_format_are_documented, run in a throwaway worktree at 56c3f4d3 (before the new-project docs) with only tests/docs.rs from a87b54ec added, failed (exit 101): README.md does not name specsync change adopt.**

Recorded as failed in local-macos-arm64 at revision 71c823542fe9, observed 2026-11-12T17:27:54Z.

## Key failing lines

- panicked at tests/docs.rs:1609:9: README.md does not name "specsync change adopt"
