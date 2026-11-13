---
kind: evidence
id: 2026-11-13-tc-cli-docs-name-pinned-versions
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact docs_state_only_the_pinned_tool_versions for TC-cli-docs-name-pinned-versions: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-name-pinned-versions]
case_digest: [404be58afd21d5cd21d3eb5f7faed4cf91ff6ab0d49075d4eac06f227fa89015]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - docs_state_only_the_pinned_tool_versions
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 443c63ab65b747f47248ad2166f8fb299da94a41
content: b056982fd067aff750b535081ae04bbaf4da1742935833f4a5a8559c6a739214
environment: local-macos-arm64
fingerprint: sha256:6758fc4563aab6689ce2460d80881ed08b50477bef200c06a302d6ef43996cc2
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:55:16Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact docs_state_only_the_pinned_tool_versions for TC-cli-docs-name-pinned-versions: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 443c63ab65b7, observed 2026-11-13T01:55:16Z. The raw log stays in the machine-local store.
