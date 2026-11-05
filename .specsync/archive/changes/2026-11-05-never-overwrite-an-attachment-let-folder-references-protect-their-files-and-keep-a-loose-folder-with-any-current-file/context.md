---
change: never-overwrite-an-attachment-let-folder-references-protect-their-files-and-keep-a-loose-folder-with-any-current-file
artifact: context
---

# Context

Leif's review agent left three P1 threads on #109 at 6e0a835 (2026-11-05), each reproduced:

1. **An attachment is overwritten.** The move uses `fs::write`, which truncates an existing destination and then registers it as a copy this apply made. A committed `knowledge/attachments/a1/evidence/m9.json` holding another agent's proof was replaced. A rollback would also delete an identical attachment that was already there.
2. **A folder fixture does not protect its files.** Bug fixtures and checkpoint references are matched only against exact file paths. An open bug whose fixture is the folder `.chough/artifacts/open-repro` lost `open-repro/out.log` to the trim. The observation-set rules match references the same way.
3. **A cold sibling decides the owner of a whole folder.** One cold-listed file made a loose folder cold, and the first-commit check was skipped for listed folders. So `reused-folder/current.log`, committed after the newest archive, was removed along with `a-old.log`.

Leif chose "Record both, then merge" (2026-11-05). The previous change, for the Windows paths and file modes, is archived, and the move into done waits for this one, so no trim runs with these bugs.
