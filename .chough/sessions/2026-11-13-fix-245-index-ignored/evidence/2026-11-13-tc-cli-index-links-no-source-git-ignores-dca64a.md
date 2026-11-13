---
kind: evidence
id: 2026-11-13-tc-cli-index-links-no-source-git-ignores-dca64a
aliases: []
claim: "Ran cargo test --locked --test vault -- --exact the_index_links_no_source_git_ignores_so_a_fresh_clone_checks_clean for TC-cli-index-links-no-source-git-ignores: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-index-links-no-source-git-ignores]
case_digest: [fff580cb6d5d423c82fd71fc5f2bee98d484c004da58800b6bbdab8b8d532a63]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - vault
  - "--"
  - "--exact"
  - the_index_links_no_source_git_ignores_so_a_fresh_clone_checks_clean
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: f50a25cb2399fb57bf8074046d2662fb73e7d7ae
content: 3cb27a62edcb796a79c7ecb4f269521748f6e6695e5062059c4c07123f22118b
environment: local-macos-arm64
fingerprint: sha256:690005445b914ba742271197009766fda866bf5d211800680d517ef84ec8147c
criteria: [KNOW-14]
requirements: [REQ-cli-043, REQ-cli-044]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T14:59:24Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test vault -- --exact the_index_links_no_source_git_ignores_so_a_fresh_clone_checks_clean for TC-cli-index-links-no-source-git-ignores: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f50a25cb2399, observed 2026-11-13T14:59:24Z. The raw log stays in the machine-local store.
