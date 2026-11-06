# Lesson bundle: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Name bug merge in the CLI reference's command table, with a check that the table matches --help (#173, FLOW-18)
- **Kind**: Documentation
- **Specs**: cli
- **Paths**: docs/cli.md, tests/docs.rs, CHANGELOG.md, knowledge/
- **Acceptance**: tests/docs.rs gains a check that docs/cli.md's command table names every chough command and every subcommand chough --help lists; it fails on main at 69caa0e7 because the bug row omits merge (#173), and passes after the row reads bug add|update|list|search|show|report|merge with merge described as the bug-record merge driver. A chough-testing/1 case binds the check. No helper behavior or requirement text changes, and fledge lanes run verify passes locally.

## Evidence

- Verification commit: `77579661fb0d876418dc9b7b4468b10efb6f8546`
- Base commit: `69caa0e721d0a150652fd50b7104fcb6f4e444a7`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

docs/cli.md's Commands table lists the bug subcommands as
`bug add\|update\|list\|search\|show\|report`, but `chough bug --help` also lists
`merge`, the Git merge driver for bug records (`merge=chough-bug` in
.gitattributes). The FLOW-18 audit found this as F18b. #164 left the line to #158,
which was editing that row, and #158 merged without `merge`. The gap is filed as
#173. orc assigned it under FLOW-18 on 2026-11-06: "add 'bug merge' to docs/cli.md's
bug row and a check that cli.md's command table matches --help (failing first)".

tests/docs.rs already checks the README's helper table against `chough --help`
(readme_helper_table_names_every_command_the_helper_lists). It also checks every flag
on a line that starts with `chough `. The CLI reference's Commands table matches
neither: its cells have no `chough ` prefix, they escape pipes as `\|`, and options
carry brackets (`[--cycle current|all]`). So nothing checks that table.

The new check, cli_reference_command_table_matches_the_helpers_help, makes four
checks: every command in `chough --help` has a row; the rows name every subcommand of
each command; every subcommand a row names exists; and every flag a row gives exists
for that command or subcommand. A `|` inside brackets separates option values, not
subcommands. On main at 69caa0e7 the only problem it reports is
"`bug` rows do not name merge". Temporary mutations confirmed the other three
checks catch drift: an unknown flag, an unknown flag inside `[--dry-run\|--fix]`, and
a removed subcommand. Each was reported, and docs/cli.md was restored.

Constraints: failing check first; no helper behavior or requirement text changes;
`specsync change audit --strict` and pre-push before every push; no tag, release
or publication.

## Where these lessons go

- `specs/cli/context.md`
