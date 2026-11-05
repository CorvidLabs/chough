# Lesson bundle: keep-one-session-per-objective-or-native-change-under-chough-sessions-with-append-only-transitions-an-active-pointer

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep one session per objective or native change under .chough/sessions with append-only transitions, an active pointer, automatic archiving and an append-only subagent inbox (DRIVE-3)
- **Kind**: Feature
- **Specs**: cli, history
- **Paths**: src/sessions.rs, src/checkpoint.rs, src/main.rs, src/history.rs, tests/sessions.rs, docs/cli.md, CHANGELOG.md, .gitattributes, hi/drive.md, skills/chough-workflow/references/checkpoint.md, .agents/skills/chough-workflow/references/checkpoint.md, src/skill.rs, tests/workflow_authority.rs, tests/fixtures/workflow-at-edbc3c1-checkpoint.md, src/evidence.rs, tests/evidence.rs, tests/spec_split.rs
- **Acceptance**: Source: Leif's brief t811u item 3, "Want: sessions keyed by objective or native change id (for example `.chough/sessions/<change-id>.json` with append-only per-session transitions), a pointer to the active one, automatic archiving of a finished session, and a way for subagents to append observations to the parent session without editing the same file (an append-only inbox). Merging two branches with different sessions must not conflict. Keep `checkpoint --merge` working, or replace it.", quoted as DRIVE-3 and confirmed as his criteria at 5:24 PM MST (card t811s9271); the parent relayed at 6:37 PM MST to follow that stored preference. Outcomes: (1) `chough session new <key>` creates `.chough/sessions/<key>.json`, a schema 2 checkpoint with its own history cycle, and `chough session adopt <key>` moves an existing `.chough/session.json` with a history cursor there byte for byte; (2) with `.chough/sessions/` present every command reads and writes the active session, named by the untracked pointer `.chough/local/active-session` set by `session new` and `session use`, or the only open session when no pointer is set, and refuses when it cannot tell which session is active; (3) each session's transitions are its own append-only history cycle, so two branches that each record work in a different session merge with no conflict in real Git, and `checkpoint --merge` still merges two edits of the same session file; (4) `session new` and `session use` move every other session whose stage is done to `.chough/sessions/archive/<key>.json` unchanged, and `session list` reports each session's key, stage, status and whether it is active or archived; (5) `chough inbox add --session <key> --from <name> --kind observation|decision|completed|question --text <text>` creates a new entry file under `.chough/sessions/<key>.inbox/` and never edits the session file, and `chough inbox fold` adds the active session's unfolded entries to its lists without duplicates and moves them to `folded/`; (6) inbox entries carry no approval, review, consent, gate clear or stage move, and stage gates still come from native SpecSync state; (7) without `.chough/sessions/` behaviour and bytes are unchanged; (8) docs, CHANGELOG and the checkpoint skill reference describe it, the new tests fail before the change and fledge lanes run verify passes

## Evidence

- Verification commit: `ad5c48a454f5dff525a3891ad0e9565719f9c13d`
- Base commit: `bc83411c7e28872926f4f394d80fd81e9e422e94`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

On Raven on 2026-11-03 several objectives shared the one `.chough/session.json`, and
subagents had to edit that same file to report back, so branches with different work
conflicted on it. Leif's brief t811u item 3 (DRIVE-3) asks for: "sessions keyed by
objective or native change id (for example `.chough/sessions/<change-id>.json` with
append-only per-session transitions), a pointer to the active one, automatic
archiving of a finished session, and a way for subagents to append observations to
the parent session without editing the same file (an append-only inbox). Merging two
branches with different sessions must not conflict. Keep `checkpoint --merge`
working, or replace it." The parent relayed at 6:37 PM MST to follow that stored
preference with the hi-first lifecycle.

The hard rule holds: Chough records no human approval or review, and native SpecSync
state decides stage gates. Specs have no size cap (Leif, relayed 6:43 PM MST); the cli
change here is one short requirement and the sessions behaviour goes in the history
spec, so any later cli split is guidance only.

## From the change's design.md

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

## From the change's testing.md

# Testing

`tests/sessions.rs`: `session new` creates the file with its own cycle and points at
it, and stage moves write that session and its cycle only; `adopt` keeps the bytes and
removes `.chough/session.json`; resolution refuses with no session, several open
sessions and no pointer, or a pointer to an archived session, and uses the only open
session without a pointer; `session use` archives a done session unchanged and `list`
reports it; `inbox add` never changes the session file, `fold` adds each entry once,
moves it to `folded/` and a rerun adds nothing; an inbox entry with an approval field
is refused; a real-Git merge of two branches that each advance a different session
has no conflict and both sessions keep their history; a legacy project is unchanged.
`tests/workflow_authority.rs`: an unedited v0.2.0 checkpoint reference is
known-outdated and `skill update` delivers the sessions guidance. The tests fail
before the change. `fledge lanes run verify` runs the full suite.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-037 | `tests/sessions.rs`: `commands_use_the_active_session_and_refuse_when_unclear`, `adopt_moves_the_checkpoint_byte_for_byte`, `a_switch_partway_through_a_command_does_not_redirect_its_save`. |
| REQ-history-006 | `tests/sessions.rs`: `each_session_has_its_own_cycle`, `switching_archives_done_sessions`, `inbox_entries_fold_once_without_touching_the_session`, `branches_with_different_sessions_merge_without_conflict`, `a_done_session_with_unfolded_entries_stays_open_on_switch`, `fold_checks_its_targets_before_saving_the_session`, `fold_validates_entries_like_add`; `tests/evidence.rs`: `retention_protects_what_inactive_and_archived_sessions_reference`. |
| REQ-cli-007 | `tests/sessions.rs`: `the_repository_declares_the_merge_driver_for_every_session_file`, `branches_with_different_sessions_merge_without_conflict`, with the existing checkpoint merge tests. |
| REQ-history-002 | `tests/sessions.rs`: `each_session_has_its_own_cycle`, with the existing history tests; `tests/spec_split.rs` pins the modified wording. |

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
