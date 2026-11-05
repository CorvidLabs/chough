---
change: adopt-event-history-and-per-change-sessions-in-this-repository-and-union-merge-the-retention-ledger-and-changelog
artifact: testing
---

# Testing

A scripted simulation on Linux runs in a scratch clone at two bases, main before this change and this branch. At each, two branches each:
- start their own work (on the adopted base, with `chough session new`);
- record a checkpoint update and three stage moves;
- append a ledger line;
- add a CHANGELOG bullet under Unreleased.

`git merge-tree --write-tree` must report conflicts on main, and none in `.chough/`, the ledger or CHANGELOG.md on this branch. The output is kept as a Chough observation. `fledge lanes run verify` passes on Linux, and `chough history show` and `chough session list` read the adopted state.
