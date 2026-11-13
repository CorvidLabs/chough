---
kind: evidence
id: 2026-11-13-tc-cli-index-links-no-source-git-ignores
aliases: []
claim: "Ran cargo test --locked --test vault -- --exact the_index_links_no_source_git_ignores_so_a_fresh_clone_checks_clean for TC-cli-index-links-no-source-git-ignores: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 825447c42fc4a6e85dc6849af6f2384b9fdfc258
content: bf06ec3b82b3c4edb9e723e182d2ee3f1eeaf3e2e28dc42dab33deb5aaed774c
environment: local-macos-arm64
fingerprint: sha256:a215d0752b3f01a87fb577a880eae4a9b94b16a2acb603270f346121f59c96d9
criteria: [KNOW-14]
requirements: [REQ-cli-043, REQ-cli-044]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T14:49:55Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test vault -- --exact the_index_links_no_source_git_ignores_so_a_fresh_clone_checks_clean for TC-cli-index-links-no-source-git-ignores: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 825447c42fc4, observed 2026-11-13T14:49:55Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_index_links_no_source_git_ignores_so_a_fresh_clone_checks_clean stdout ----
- thread 'the_index_links_no_source_git_ignores_so_a_fresh_clone_checks_clean' (986435161) panicked at tests/vault.rs:738:5:
- a fresh clone's index check failed: Error: the knowledge directory is not what its sources generate; rebuild it with `chough library index` and commit it: knowledge/index.md
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 13 filtered out; finished in 1.56s
