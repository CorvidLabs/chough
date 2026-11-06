---
change: a-session-whose-native-change-is-archived-is-finished-and-abandoned-work-closes-with-a-reason-163-drive-16
artifact: design
---

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
