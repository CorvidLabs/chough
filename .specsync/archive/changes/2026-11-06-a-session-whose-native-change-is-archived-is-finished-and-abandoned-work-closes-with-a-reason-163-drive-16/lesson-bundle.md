# Lesson bundle: a-session-whose-native-change-is-archived-is-finished-and-abandoned-work-closes-with-a-reason-163-drive-16

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: A session whose native change is archived is finished, and abandoned work closes with a reason (163, DRIVE-16)
- **Kind**: Feature
- **Specs**: history, cli
- **Paths**: src/sessions.rs, tests/sessions.rs, hi/drive.md, docs/cli.md, skills/chough-workflow/references/checkpoint.md, .agents/skills/chough-workflow/references/checkpoint.md, CHANGELOG.md
- **Acceptance**: DRIVE-16 (Leif, relayed by orc 2026-11-06, A1 + B for chough#163), in REQ-history-006 and REQ-cli-037: session new and session use archive, bytes unchanged, every other session whose native change is archived on HEAD (finalization.json, no native read) at any stage, keeping one with unfolded inbox entries open; with no pointer such a finished session does not count toward several open sessions, so the only unfinished one is active; chough session close <key> --reason <text> archives an open session for abandoned work with the reason recorded as a decision and no native read, clears the pointer when it named it, and refuses a blank reason, an archived or missing session and unfolded inbox entries with nothing changed. The four new tests in tests/sessions.rs fail first; docs, the skill reference and CHANGELOG say so; fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `ede9c096e9194c484970f002510c2e0022465a9e`
- Base commit: `dab18909368bb3c2d1a4521b4f82efca4ec1e58a`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue #163, filed at the orc's request on 2026-11-06: `chough next` refuses on several open sessions when their native changes are already archived. A move into done that timed out after SpecSync finalized its change leaves the session open at accept. Every other worktree without a pointer then refuses with "several open sessions". The session can't be archived, because `archive_done` takes only done sessions.

Leif chose "A1 + B" and captured DRIVE-16 ("Capture both"):
- A1: a session whose native change is archived on HEAD is finished, is archived by `session new`/`use`, and does not count toward several open. This reads `finalization.json`, with no native read.
- B: `chough session close <key> --reason` closes abandoned work, with no native read.

## From the change's design.md

# Design

## Approach

- **`finished(root, session)`** is true when `stage.current == "done"`, or when `native_change` names a change for which `library::finalized_at(root, change)` finds an archive whose `finalization.json` names it. It reads only `.specsync/archive/changes/`. SpecSync is never called.
- **`archive_done`** (`session new` and `use`) archives every other finished session, with the same no-replace rename. The kept-open rule for unfolded inbox entries is unchanged.
- **`active()`, with no pointer:** one open session is still active, as before; the timed-out session's own worktree keeps its pointer, so it is unaffected. With several open, it reads them and keeps the unfinished ones. Exactly one is active. With none left it refuses with "no open session (finished: ...); run `chough session new <key>`". With several it refuses as today. The read happens only when there are several sessions and no pointer.
- **`session close <key> --reason <text>`:**
  - It holds the project lock.
  - It refuses a blank reason, a missing session ("no session <key>") and an archived one ("session <key> is archived"). It refuses unfolded inbox entries, naming `chough session use <key>` then `chough inbox fold`.
  - It reads the session; an unreadable one gets the shelve hint.
  - It adds the decision `Closed without done: <reason>` and validates the session. It writes `archive/<key>.json` with `write_new_bounded`, which never replaces a file, and then removes the open file.
  - It clears the pointer if it names the key and unpins. It reports `{"closed","archived","reason"}`.
  - There is no native read and no history event: the reason lives in the archived session, which the roll-up later carries into the library.

## Risks

- **A finalized change whose session is still mid-work** (for example a reused change ID) would be archived on a switch. A change archived on HEAD is final, so this matches DRIVE-16.
- **Cost:** `finalized_at` lists the archive directory once per open session, only on `session new`/`use` and on a pointerless resolve with several open sessions.
- **#162 amends REQ-history-006 too.** This delta carries #162's text, so this branch merges main after #162 lands and before verification.

## Constraints consulted

DRIVE-16, DRIVE-13, DRIVE-3, REQ-history-006, REQ-cli-037.

## From the change's testing.md

# Testing

New tests in tests/sessions.rs. A `specsync` stub on PATH records any call and fails, so each test also asserts there was no native read.
- `a_session_whose_native_change_is_archived_is_finished_without_a_native_read`:
  - alpha's native change is archived and beta's is open, with no pointer;
  - `checkpoint --decision` lands in beta, and alpha is unchanged;
  - `session new gamma` archives alpha byte for byte and leaves beta open.
- `session_use_archives_a_finalized_session_unless_its_inbox_has_entries`: `session use beta` archives the finalized alpha and keeps the finalized gamma, which has an unfolded entry, open under `kept_open`.
- `session_close_archives_abandoned_work_with_its_reason`:
  - a blank reason refuses with nothing changed;
  - closing alpha archives it with the reason in its decisions and leaves beta active;
  - closing it again refuses as archived, and a missing key refuses;
  - closing the active beta clears the pointer, and `session list` shows both archived.
- `session_close_refuses_a_session_with_unfolded_inbox_entries`: it names `chough session use alpha` and `chough inbox fold`, and nothing moves.

All four fail first: an unknown `close` subcommand, nothing archived, and "several open sessions (alpha, beta)". The existing session tests stay unchanged. A case binds the tests. `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/history/context.md`
- `specs/cli/context.md`
