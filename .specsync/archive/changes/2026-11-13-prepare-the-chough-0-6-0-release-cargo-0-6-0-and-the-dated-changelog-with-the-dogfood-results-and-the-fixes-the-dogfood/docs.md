---
change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
artifact: docs
---

# Docs

- docs/cli.md, gate lists: "what changed" counts from the change's own base, as
  REQ-cli-061 now says; the note that this was planned after 0.6.0 comes out.
- docs/cli.md, migration: the preview names the open native changes that make
  `--apply` refuse.
- CHANGELOG.md: the dated [0.6.0] section with the dogfood results, and these two
  fixes under it.

- docs/cli.md, checkpoint and journal: `--title` and `--summary`, how the post at the
  move into done uses them and what it derives without them, and `journal add
  --record` for a backfill.
- docs/cli.md, leaks: a temporary-root path in a case definition is not a leak.
- The workflow skill (skills/chough-workflow, and the installed copy under
  .agents/skills): set the post's title and summary before the move into done.
- CHANGELOG.md: the journal options, the backfill, the lessons and the leak exception
  under [0.6.0].

- README.md, docs/cli.md and the workflow skill: a new-project path, `specsync change
  adopt` to turn SpecSync's change gates on, and the delta format with an example.
- docs/cli.md: `next` at plan, the scope-gate summary, labelled Timeline fields, `bug
  update --fix`, and the outputs the fresh-eyes report found undocumented.
