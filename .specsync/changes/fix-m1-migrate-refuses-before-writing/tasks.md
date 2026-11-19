---
change: fix-m1-migrate-refuses-before-writing
artifact: tasks
---

# Tasks

- [ ] Failing tests first, one per issue, each red on main.
- [ ] #376: the after-fix pointer check in doctor resolves aliases and bare slugs.
- [ ] #337: one no-follow check of every target and receipt name before any write.
- [ ] #370: refuse a returning record whose ID is already held; rebuild the index
      and feed in place.
- [ ] #378: take the writer lock before the plan is read.
- [ ] `docs/cli.md`, case definitions and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
