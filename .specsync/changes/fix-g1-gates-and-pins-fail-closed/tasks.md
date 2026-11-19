---
change: fix-g1-gates-and-pins-fail-closed
artifact: tasks
---

# Tasks

- [x] Failing tests first, one per issue, each red on main.
- [x] #401: the blank, placeholder and both-fields checks move into the shared session
      validation that runs on read and on save.
- [x] #393: `inbox list` resolves the pinned session as `inbox fold` does.
- [x] #372: `session use` and `session new` say when a pin still names another
      session; `session shelve` resolves a folder name, alias or bare slug.
- [x] #369: `change ship` passes its resolved session to the gate close and the move.
- [x] #400: the history check covers the whole range and refuses when Git cannot
      list or read it.
- [ ] After #407 merges: `chough change` arguments are checked before SpecSync runs.
- [x] `docs/cli.md`, case definitions and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
