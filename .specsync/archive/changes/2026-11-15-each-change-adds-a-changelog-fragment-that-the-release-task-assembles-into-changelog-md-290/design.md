---
change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
artifact: design
---

# Design

A change writes `changelog.d/<YYYY-MM-DD>-<change-id>.md` holding its bullets. `tools/changelog.py`, run as `fledge run changelog -- --release <X.Y.Z> --date <date>` (or `--release Unreleased`), reads the fragments in reverse name order, so newest first, appends their bullets after the section's existing entries, inserts a new `## [X.Y.Z] - <date>` heading right after Unreleased, and removes the fragments. It refuses, writing nothing, when a heading for that version exists with any date. The same fragments give the same bytes.

The 16 bullets under main's Unreleased became fragments named `NN-<date>-<id>.md`, ranked so that assembly gives main's text back byte for byte. A date-named fragment sorts after every ranked one, so new entries come first; the ranked names go away at the next release. `.gitattributes` keeps `CHANGELOG.md merge=union`, which is harmless and keeps `docs/cli.md`'s declared block equal to it.
