---
kind: evidence
id: 2026-11-17-tc-cli-doc-links-resolve
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
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:e67fe95b03e3571104cf21317a479b80e5bf6751901767fd982fe8c847a91308
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:25Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:25Z. The raw log stays in the machine-local store.
