---
id: 2026-11-06-name-bug-merge-in-the-cli-reference-checked-against-help-173
title: Name bug merge in the CLI reference, checked against --help (#173)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T21:49:42Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-name-bug-merge-in-the-cli-reference-s-command-table-with-a.md
  - .specsync/archive/changes/2026-11-06-name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18/
record: .chough/archive/sessions/2026-11-06-name-bug-merge-in-the-cli-reference-s-command-table-with-a.md
actor:
  - agent: claude-code
    terminal: t382
---
# Name bug merge in the CLI reference, checked against --help (#173)

Issue #173: the CLI reference's command table did not name bug merge. Its bug row now does, and a new check under FLOW-18 compares the table with --help. The check is its own docs test because that table writes subcommands with escaped pipes and bracketed options, unlike the README's helper table.

The table check failed first and then passed. All eleven docs cases and the full cargo test run of 533 tests passed.

The maintainer approved the scope and passed the implementation review on PR #176. The change was archived and the story closed as done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-name-bug-merge-in-the-cli-reference-s-command-table-with-a.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18/)
