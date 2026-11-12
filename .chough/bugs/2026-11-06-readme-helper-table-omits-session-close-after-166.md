---
kind: bug
id: 2026-11-06-readme-helper-table-omits-session-close-after-166
aliases: [BUG-readme-helper-table-omits-session-close-after-166-bffb0c93c65a7cdeee281a27]
title: "README helper table omits session close after #166"
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: []
requirements: []
cases: [TC-cli-readme-helper-table-covers-help]
evidence:
  - OBS-tc-cli-readme-helper-table-covers-help-7733ef9562e8a9db29ba3033
  - OBS-tc-cli-readme-helper-table-covers-help-4738d41898b5a2e4375fa215
receipts:
  - id: 2026-11-06-tc-cli-readme-helper-table-covers-help
    aliases: OBS-tc-cli-readme-helper-table-covers-help-7733ef9562e8a9db29ba3033
    claim: "Ran cargo test --locked --test docs -- --exact readme_helper_table_names_every_command_the_helper_lists for TC-cli-readme-helper-table-covers-help: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact readme_helper_table_names_every_command_the_helper_lists
    revision: 15cdaef2c48f5ce02f9fd5caaa6fdf5dc246f8d4
    environment: local-macos-arm64
    content: 73253d5af58765136022d15e928e585c2cba71e74e844e4b188c55cac2e649f1
    fingerprint: sha256:88f94b568003554b8de575687a322cc5a7967c7612c16c5f783d7e661fdcd530
    case: TC-cli-readme-helper-table-covers-help
    case_digest: ceb5661bfa42aba6427816c6570519bcee4122e58423ab49691845f07f6726e0
    observed: 2026-11-06T20:13:17Z
    duration_seconds: 2
  - id: 2026-11-06-tc-cli-readme-helper-table-covers-help-ad1da4
    aliases: OBS-tc-cli-readme-helper-table-covers-help-4738d41898b5a2e4375fa215
    claim: "Ran cargo test --locked --test docs -- --exact readme_helper_table_names_every_command_the_helper_lists for TC-cli-readme-helper-table-covers-help: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact readme_helper_table_names_every_command_the_helper_lists
    revision: 15cdaef2c48f5ce02f9fd5caaa6fdf5dc246f8d4
    environment: local-macos-arm64
    content: 6b366c3cab90c60b951094200913f5d555255de95af301430420cd512fde8095
    fingerprint: sha256:73196d7e95418d8cad14ea2bd0a3823dafda76ee53b17d74dac07d0c612350f3
    case: TC-cli-readme-helper-table-covers-help
    case_digest: ceb5661bfa42aba6427816c6570519bcee4122e58423ab49691845f07f6726e0
    observed: 2026-11-06T20:14:01Z
    duration_seconds: 2
environment: local-macos-arm64
revision: 15cdaef2c48f5ce02f9fd5caaa6fdf5dc246f8d4
found_by: manual report
related: []
created: 2026-11-06T20:13:34Z
updated: 2026-11-06T20:14:14Z
---
# README helper table omits session close after #166

**What happened:** tests/docs.rs readme_helper_table_names_every_command_the_helper_lists fails on #164 merged with main: '`session` lists subcommands but not close'
Actual: README.md:136 names session new/adopt/use/list/shelve

**Steps:** Merging main at ef2dae8e (#166 adds chough session close <key> --reason) into the FLOW-18 branch

**Expected:** README.md's helper row names every session subcommand chough session --help lists, including close

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-06T20:13:34Z · Reported (manual report); evidence: OBS-tc-cli-readme-helper-table-covers-help-7733ef9562e8a9db29ba3033
- 2026-11-06T20:14:14Z · status: open -> verified; evidence: OBS-tc-cli-readme-helper-table-covers-help-4738d41898b5a2e4375fa215
