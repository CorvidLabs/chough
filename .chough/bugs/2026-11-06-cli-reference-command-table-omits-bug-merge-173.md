---
kind: bug
id: 2026-11-06-cli-reference-command-table-omits-bug-merge-173
aliases: [BUG-cli-reference-command-table-omits-bug-merge-173-cfae88809770918250b18b65]
title: CLI reference command table omits bug merge (#173)
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: []
requirements: []
cases: [TC-cli-cli-command-table-matches-help]
evidence:
  - OBS-tc-cli-cli-command-table-matches-help-1921598bd0a22b35ad1a230f
  - OBS-tc-cli-cli-command-table-matches-help-4bb9fc6a4f11994c45c7d052
receipts:
  - id: 2026-11-06-tc-cli-cli-command-table-matches-help
    aliases: OBS-tc-cli-cli-command-table-matches-help-1921598bd0a22b35ad1a230f
    claim: "Ran cargo test --locked --test docs -- --exact cli_reference_command_table_matches_the_helpers_help for TC-cli-cli-command-table-matches-help: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact cli_reference_command_table_matches_the_helpers_help
    revision: 69caa0e721d0a150652fd50b7104fcb6f4e444a7
    environment: local-macos-arm64
    content: 6637fca2a903f628b1383d38b830c010128f74cdfcc5e81d386981a7e6d68002
    fingerprint: sha256:2c584b1940c17c6f3f754823f25e7128dd77b45e40bd70ccc1db9b83d64e6763
    case: TC-cli-cli-command-table-matches-help
    case_digest: bdbec5b6aebc017d6974bcd537996a0cfbfc9645beb5d0b0b47ed4295db9ce4a
    observed: 2026-11-06T21:00:09Z
    duration_seconds: 2
  - id: 2026-11-06-tc-cli-cli-command-table-matches-help-db67f0
    aliases: OBS-tc-cli-cli-command-table-matches-help-4bb9fc6a4f11994c45c7d052
    claim: "Ran cargo test --locked --test docs -- --exact cli_reference_command_table_matches_the_helpers_help for TC-cli-cli-command-table-matches-help: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact cli_reference_command_table_matches_the_helpers_help
    revision: 46ce740ac98c9448c5dcf6e1e2422bd5081fe28e
    environment: local-macos-arm64
    content: 3985793f09c9f3fd2a09911446ad1694293ede07cf9126b3b043e88863438829
    fingerprint: sha256:ecd1c8517b9ccec3eb295b5bb3d87b11ef3fb39fe94971efa6913d96cacf4f74
    case: TC-cli-cli-command-table-matches-help
    case_digest: bdbec5b6aebc017d6974bcd537996a0cfbfc9645beb5d0b0b47ed4295db9ce4a
    observed: 2026-11-06T21:03:57Z
    duration_seconds: 3
environment: local-macos-arm64
revision: 69caa0e721d0a150652fd50b7104fcb6f4e444a7
found_by: manual report
related: []
created: 2026-11-06T21:00:15Z
updated: 2026-11-06T21:03:57Z
---
# CLI reference command table omits bug merge (#173)

**What happened:** tests/docs.rs cli_reference_command_table_matches_the_helpers_help fails on main at 69caa0e7: 'docs/cli.md Commands: `bug` rows do not name merge'
Actual: docs/cli.md:25 lists bug add|update|list|search|show|report

**Steps:** Checking docs/cli.md's Commands table against chough --help; #158 rewrote the bug row without merge

**Expected:** The bug row names every bug subcommand chough bug --help lists, including merge

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-06T21:00:15Z · Reported (manual report); evidence: OBS-tc-cli-cli-command-table-matches-help-1921598bd0a22b35ad1a230f
- 2026-11-06T21:03:57Z · status: open -> verified; evidence: OBS-tc-cli-cli-command-table-matches-help-4bb9fc6a4f11994c45c7d052
