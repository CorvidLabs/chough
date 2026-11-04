---
id: keep-one-session-per-objective-or-native-change-under-chough-sessions-with-append-only-transitions-an-active-pointer
state: verifying
type: feature
base_commit: bc83411c7e28872926f4f394d80fd81e9e422e94
---

# Keep one session per objective or native change under .chough/sessions with append-only transitions, an active pointer, automatic archiving and an append-only subagent inbox (DRIVE-3)

## Intent

Keep one session per objective or native change under .chough/sessions with append-only transitions, an active pointer, automatic archiving and an append-only subagent inbox (DRIVE-3)

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- Source: Leif's brief t811u item 3, "Want: sessions keyed by objective or native change id (for example `.chough/sessions/<change-id>.json` with append-only per-session transitions), a pointer to the active one, automatic archiving of a finished session, and a way for subagents to append observations to the parent session without editing the same file (an append-only inbox). Merging two branches with different sessions must not conflict. Keep `checkpoint --merge` working, or replace it.", quoted as DRIVE-3 and confirmed as his criteria at 5:24 PM MST (card t811s9271); the parent relayed at 6:37 PM MST to follow that stored preference. Outcomes: (1) `chough session new <key>` creates `.chough/sessions/<key>.json`, a schema 2 checkpoint with its own history cycle, and `chough session adopt <key>` moves an existing `.chough/session.json` with a history cursor there byte for byte; (2) with `.chough/sessions/` present every command reads and writes the active session, named by the untracked pointer `.chough/local/active-session` set by `session new` and `session use`, or the only open session when no pointer is set, and refuses when it cannot tell which session is active; (3) each session's transitions are its own append-only history cycle, so two branches that each record work in a different session merge with no conflict in real Git, and `checkpoint --merge` still merges two edits of the same session file; (4) `session new` and `session use` move every other session whose stage is done to `.chough/sessions/archive/<key>.json` unchanged, and `session list` reports each session's key, stage, status and whether it is active or archived; (5) `chough inbox add --session <key> --from <name> --kind observation|decision|completed|question --text <text>` creates a new entry file under `.chough/sessions/<key>.inbox/` and never edits the session file, and `chough inbox fold` adds the active session's unfolded entries to its lists without duplicates and moves them to `folded/`; (6) inbox entries carry no approval, review, consent, gate clear or stage move, and stage gates still come from native SpecSync state; (7) without `.chough/sessions/` behaviour and bytes are unchanged; (8) docs, CHANGELOG and the checkpoint skill reference describe it, the new tests fail before the change and fledge lanes run verify passes

## No-spec Rationale

Not applicable
