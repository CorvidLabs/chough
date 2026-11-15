---
kind: evidence
id: 2026-11-15-tc-cli-doc-links-resolve-a61080
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
revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
environment: local-macos-arm64
fingerprint: sha256:f553635b07439395390b28dfc44172e6e00029514e59db033c231e83f842b645
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T21:09:07Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8b5bd53cab53, observed 2026-11-15T21:09:07Z. The raw log stays in the machine-local store.
