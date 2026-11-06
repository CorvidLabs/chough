---
change: chough-doctor-finds-and-fixes-the-chough-debt-that-stops-a-fresh-worktree-from-entering-chough-drive-17
artifact: design
---

# Design

## Approach

- **`src/doctor.rs`** collects the findings in the REQ-history-012 order. Each finding records its kind, target, fix command and decision flag, plus an apply function.
- **Report and dry run:** report mode and `--dry-run` print the list and touch nothing. They take no lock and write no `.gitignore`.
- **`--fix`** takes the project lock once (as the session commands do), recomputes the findings under the lock, and applies them in order:
  1. **Legacy checkpoint:** run history adoption when the checkpoint has no cursor, then the session adoption. The key is the native change if valid, else `--adopt-as`; otherwise the finding is skipped.
  2. **Stale pointer:** remove the pointer file.
  3. **Stale gates,** only with `--reason`: pin to that session and clear `awaiting_human` through the checkpoint save path, so history records a gate withdrawal and no decision. Then archive it.
  4. **Finished sessions:** the same no-replace rename as `archive_done`, with the unfolded-inbox rule.
  5. **`--close <key>`,** only with `--reason`: #166's `close`.
- **Exit code:** the command exits 1 if any finding is still `found` or `skipped`.
- **No native read:** findings come from `.chough/` files and `.specsync/archive/changes/*/finalization.json`, through `library::finalized_at`.

- **Shared code, not copies.** `history::adopt` and `checkpoint::withdraw_gates` are extracted from `history adopt --apply` and `checkpoint --clear-awaiting-human`. Those commands keep their exact output and behavior. The doctor calls the same functions, so the gate-closed events and the history cursor are written one way.

## Risks

- **Ordering:** adopting the legacy checkpoint makes it the active session, so its pointer is valid. If its change is archived, step 4 archives it and removes the pointer in the same run.
- **Gate withdrawal on a session that is not active** needs the checkpoint save path for another file. Pinning per command already supports one resolved session per run, and the doctor saves at most one session at a time under the lock.
- **Builds on #166.** This branch starts from #166's head and merges main after #166 lands.

## Constraints consulted

DRIVE-17, DRIVE-16, REQ-history-006, REQ-cli-037, REQ-cli-055 (new), REQ-history-012 (new).
