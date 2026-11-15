---
kind: evidence
id: 2026-11-15-tc-cli-doc-links-resolve
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
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:bb4942c1e5abe34c4a37b140267c582a9c70678bec7d4ccd5089358e1ae72b5a
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:17Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:17Z. The raw log stays in the machine-local store.
