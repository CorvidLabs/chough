---
change: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
artifact: docs
---

# Docs

- docs/cli.md: the Commands table's bug row names `merge`
  (`bug add\|update\|list\|search\|show\|report\|merge`), and its behavior cell says
  `merge` is the Git merge driver for bug records that `.gitattributes` declares
  (`merge=chough-bug`) (#173).
- tests/docs.rs: a check that the Commands table matches `chough --help`, bound by
  TC-cli-cli-command-table-matches-help.
- CHANGELOG.md: the FLOW-18 Unreleased entry also names the CLI reference's command
  table check.
