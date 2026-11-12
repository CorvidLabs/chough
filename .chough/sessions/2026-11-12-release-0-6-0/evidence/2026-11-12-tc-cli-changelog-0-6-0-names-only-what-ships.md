---
kind: evidence
id: 2026-11-12-tc-cli-changelog-0-6-0-names-only-what-ships
aliases: []
claim: "Ran cargo test --locked --test changelog -- --exact the_0_6_0_also_list_names_nothing_0_6_0_removed for TC-cli-changelog-0-6-0-names-only-what-ships: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-changelog-0-6-0-names-only-what-ships]
case_digest: [9cfdfa019b484f365a9eaad6a8cf06597ee715aaf04b8a7883186168259ead0a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - changelog
  - "--"
  - "--exact"
  - the_0_6_0_also_list_names_nothing_0_6_0_removed
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: e005da2e408386cbc35a21498f87135c42952257
content: ea3944c1edfdad8431229957114e4716be4f731c5c63f406c4ae3b3d649b5470
environment: local-macos-arm64
fingerprint: sha256:1520673c7623beceb69bb24e89e3c2ade0202070bbdc7e506ed11c04952d5d55
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:33:16Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test changelog -- --exact the_0_6_0_also_list_names_nothing_0_6_0_removed for TC-cli-changelog-0-6-0-names-only-what-ships: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision e005da2e4083, observed 2026-11-12T16:33:16Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_0_6_0_also_list_names_nothing_0_6_0_removed stdout ----
- thread 'the_0_6_0_also_list_names_nothing_0_6_0_removed' (919219937) panicked at tests/changelog.rs:181:9:
- Also in 0.6.0 still describes "evidence prune", which 0.6.0 removed
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s
