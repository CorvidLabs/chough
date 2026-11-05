---
change: keep-one-session-per-objective-or-native-change-under-chough-sessions-with-append-only-transitions-an-active-pointer
artifact: design
---

# Design

- Layout: `.chough/sessions/<key>.json` (schema 2 checkpoint, key is a valid change
  ID), `.chough/sessions/archive/<key>.json`, `.chough/sessions/<key>.inbox/INB-*.json`
  and `.chough/sessions/<key>.inbox/folded/`. The pointer is
  `.chough/local/active-session`; `.chough/local/.gitignore` holds `*` so the pointer
  is never committed and each worktree keeps its own.
- Resolution (src/sessions.rs `checkpoint_path`): no `.chough/sessions/` directory
  means `.chough/session.json`. Otherwise the pointer's key if its file exists, else
  the only open session; none, several, or a pointer to a missing or archived session
  refuses with the next command to run.
- `session new <key> [--objective] [--intent-scope] [--module]` creates a schema 2
  checkpoint with a fresh history cursor, create-only, and points at it. It refuses
  while an unadopted `.chough/session.json` exists. `session adopt <key>` copies an
  existing schema 2 checkpoint with a history cursor to the session file byte for
  byte, then removes `.chough/session.json`. `init` refuses in sessions mode.
- Archive: `session new` and `session use` move every other open session whose stage
  is `done` to `archive/` with a rename (bytes unchanged) and report the keys.
- Inbox: `inbox add --session <key> --from <name> --kind <kind> --text <text>` writes
  `{schema_version, id, session, from, kind, text, recorded_at_unix_seconds}` with a
  create-only write and a random `INB-` ID; the record denies unknown fields. `inbox
  fold` reads the active session's unfolded entries in ID order, appends their text to
  observations, decisions, completed or open questions without duplicates, saves the
  checkpoint within its bounds, then renames each entry into `folded/`. A rerun after
  an interrupted fold adds nothing twice.
- Merging: two sessions are two files with their own cycles, so Git merges them with
  no conflict. `.gitattributes` adds `.chough/sessions/*.json` and
  `.chough/sessions/archive/*.json` to the `chough-session` driver, so `checkpoint
  --merge` keeps merging two edits of the same session.
- History protection sources also walk `.chough/sessions`.
