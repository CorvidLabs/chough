---
change: adopt-event-history-and-per-change-sessions-in-this-repository-and-union-merge-the-retention-ledger-and-changelog
artifact: research
---

# Research

`git merge-tree --write-tree` replay of the 89 merges in the last 10 days (2026-11-05): 83 conflicted. Files and how many merges conflicted on each:

| File | Merges | Fixed by |
|---|---|---|
| `.chough/session.json` | 69 | this change: one session per change |
| `.chough/transitions.jsonl` | 60 | this change: event history (the legacy log is frozen, digest-anchored) |
| `specs/cli/cli.spec.md` | 51 | SpecSync, filed as CorvidLabs/spec-sync#862 |
| `CHANGELOG.md` | 44 | this change: `merge=union` |
| `.chough/cases/cli.cases.toml` | 31 | the DRIVE-12 product change: a case file per change |
| `specs/cli/requirements.md` | 21 | SpecSync, #862 |
| `knowledge/*` | 16 | the DRIVE-12 product change: rebuild after a merge, checked in CI |
| `.chough/pruned.jsonl` | (appends) | this change: `merge=union` |

The `history adopt` preview on main anchors the legacy log at 573 lines (208,803 bytes), with digest `fabbe382…`, and changes nothing else.
