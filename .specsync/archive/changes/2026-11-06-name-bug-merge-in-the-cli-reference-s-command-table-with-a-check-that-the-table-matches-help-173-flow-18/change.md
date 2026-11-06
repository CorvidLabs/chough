---
id: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
state: archived
type: documentation
base_commit: 69caa0e721d0a150652fd50b7104fcb6f4e444a7
---

# Name bug merge in the CLI reference's command table, with a check that the table matches --help (#173, FLOW-18)

## Intent

Name bug merge in the CLI reference's command table, with a check that the table matches --help (#173, FLOW-18)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- tests/docs.rs gains a check that docs/cli.md's command table names every chough command and every subcommand chough --help lists; it fails on main at 69caa0e7 because the bug row omits merge (#173), and passes after the row reads bug add|update|list|search|show|report|merge with merge described as the bug-record merge driver. A chough-testing/1 case binds the check. No helper behavior or requirement text changes, and fledge lanes run verify passes locally.

## No-spec Rationale

No requirement text changes: docs/cli.md's command table gains the bug merge subcommand main already ships (REQ-cli-019's existing rule that the docs say what the helper does), and tests/docs.rs checks that table against chough --help as it already checks the README's helper table. No helper behavior changes.
