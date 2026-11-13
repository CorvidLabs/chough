---
kind: evidence
id: 2026-11-13-tc-cli-doc-links-resolve
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-doc-links-resolve]
case_digest: [bc8019ca3cf835f594fa494d827d7c34b8f2e9a77993c9cf1c379afc62c4fef5]
command: [cargo, test, --locked, --test, docs, --, --exact, relative_links_and_anchors_resolve]
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 443c63ab65b747f47248ad2166f8fb299da94a41
content: b056982fd067aff750b535081ae04bbaf4da1742935833f4a5a8559c6a739214
environment: local-macos-arm64
fingerprint: sha256:ed078999e3cd9da2e2f63a35c288944bb5114ee96093bb445c23099feb48040d
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:55:14Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 443c63ab65b7, observed 2026-11-13T01:55:14Z. The raw log stays in the machine-local store.
