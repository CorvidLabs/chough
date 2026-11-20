---
change: fix-m1-migrate-refuses-before-writing
artifact: tasks
---

# Tasks

- [x] Failing tests first, one per issue, each red on main.
- [x] #376: the after-fix pointer check in doctor resolves aliases and bare slugs.
- [x] #337: one no-follow check of every target and receipt name before any write.
- [x] #370: refuse a returning record whose ID is already held; rebuild the index
      and feed in place.
- [x] The change-open refusal, when built: plural wording, one change id per line,
      and the command that archives or finishes a change named (no sentence changes).
- [x] #378: take the writer lock before the plan is read.
- [x] #338: write a case-colliding target under a temporary name and rename it into
      place before any removal; remove no source until every planned file is on disk.
- [x] #375: remove a folder left holding only macOS metadata, and name it in the
      apply's output.
- [x] `docs/cli.md`, case definitions and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
