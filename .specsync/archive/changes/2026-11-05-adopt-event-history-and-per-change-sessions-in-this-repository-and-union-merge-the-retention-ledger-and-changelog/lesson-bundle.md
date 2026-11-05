# Lesson bundle: adopt-event-history-and-per-change-sessions-in-this-repository-and-union-merge-the-retention-ledger-and-changelog

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Adopt event history and per-change sessions in this repository and union-merge the retention ledger and changelog
- **Kind**: Migration
- **Specs**: history
- **Paths**: .gitattributes, AGENTS.md
- **Acceptance**: This repository runs Chough with event history and one session per change: the legacy transition log and migration backup stay byte for byte, the checkpoint moves to .chough/sessions/ byte for byte, and AGENTS.md tells agents to start each pull request with chough session new. The retention ledger and CHANGELOG.md merge with Git union. A simulated pair of parallel pull requests conflicts in .chough/ and CHANGELOG.md on main and merges with none of those conflicts on this branch; fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `6c6608863cf4bdf4efad9eb7a24515935c970b48`
- Base commit: `b9b27c2191835b0185ad449f3cac95bc7be013dd`
- Verified by: `specsync check --spec history`

## From the change's context.md

# Context

Leif asked for unique sessions so pull requests stop conflicting (2026-11-04), and chose "Adopt + new criterion" (2026-11-05). DRIVE-12 was captured from that choice. This repository still runs Chough in legacy mode: one `.chough/session.json` checkpoint and one appended `.chough/transitions.jsonl`. Every pull request rewrites both, so parallel branches conflict on them almost every time. Chough already ships per-change sessions (REQ-history-006, DRIVE-3) and immutable event history (REQ-history-007). This change adopts both here, on main before other branches diverge, so later pull requests each write their own files.

## From the change's design.md

# Design

- `chough history adopt --apply` adds a history cursor to the checkpoint. It keeps `.chough/transitions.jsonl` and the schema-1 migration backup byte for byte, anchored by digest. New transitions become immutable event files with unique paths.
- `chough session adopt <this change's ID>` moves the checkpoint to `.chough/sessions/<key>.json` byte for byte. Each later pull request starts with `chough session new <change-id>`, which archives finished sessions with identical renames, so parallel branches agree on them. The active-session pointer stays in `.chough/local/`, which Git ignores.
- `.gitattributes` gives `.chough/pruned.jsonl` (append-only JSON lines) and `CHANGELOG.md` (bullets added under Unreleased) Git's built-in `union` driver. Session files already declare `merge=chough-session`.
- AGENTS.md tells agents the routine: open a session per change, set up the merge driver once per clone, and what still conflicts and how to resolve it.

## From the change's testing.md

# Testing

A scripted simulation on Linux runs in a scratch clone at two bases, main before this change and this branch. At each, two branches each:
- start their own work (on the adopted base, with `chough session new`);
- record a checkpoint update and three stage moves;
- append a ledger line;
- add a CHANGELOG bullet under Unreleased.

`git merge-tree --write-tree` must report conflicts on main, and none in `.chough/`, the ledger or CHANGELOG.md on this branch. The output is kept as a Chough observation. `fledge lanes run verify` passes on Linux, and `chough history show` and `chough session list` read the adopted state.

## Where these lessons go

- `specs/history/context.md`
