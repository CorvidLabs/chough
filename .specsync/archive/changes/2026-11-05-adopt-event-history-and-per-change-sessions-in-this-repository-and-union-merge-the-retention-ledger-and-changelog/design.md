---
change: adopt-event-history-and-per-change-sessions-in-this-repository-and-union-merge-the-retention-ledger-and-changelog
artifact: design
---

# Design

- `chough history adopt --apply` adds a history cursor to the checkpoint. It keeps `.chough/transitions.jsonl` and the schema-1 migration backup byte for byte, anchored by digest. New transitions become immutable event files with unique paths.
- `chough session adopt <this change's ID>` moves the checkpoint to `.chough/sessions/<key>.json` byte for byte. Each later pull request starts with `chough session new <change-id>`, which archives finished sessions with identical renames, so parallel branches agree on them. The active-session pointer stays in `.chough/local/`, which Git ignores.
- `.gitattributes` gives `.chough/pruned.jsonl` (append-only JSON lines) and `CHANGELOG.md` (bullets added under Unreleased) Git's built-in `union` driver. Session files already declare `merge=chough-session`.
- AGENTS.md tells agents the routine: open a session per change, set up the merge driver once per clone, and what still conflicts and how to resolve it.
