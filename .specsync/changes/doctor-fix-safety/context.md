---
change: doctor-fix-safety
artifact: context
---

# Context

Batch D1 of the 0.6.2 bug round: `chough doctor --fix` must neither destroy work nor hang. From the 0.6.2 candidate swarm and the agent reviewer:
- #381 (data loss, 62-F7): a branch adds an evidence entry to a session that main archives; after the merge keeps main's archive, `doctor --fix` finishes the archive and deletes the entry, which no receipt holds. An unfolded inbox entry a branch adds is lost the same way.
- #402 (hang): `root_code`'s spec walk follows symlinks, so a link out of the project counts outside code, and two folder links back to the project loop forever. Leif decided "fix in 0.6.2", with the finding as the scope.
- #377: a `.DS_Store` or `._session.md` in an archived folder is uncommitted, so the archive finish skips the folder forever.
- #371: after a move into done, nothing says to commit and run `chough doctor --fix`.
- #374: a front-matter conflict is reported only as `unreadable`, with shelve advice; and after `doctor --fix --redact` redacts a Timeline entry, `timeline_rewritten` advises restoring the leak.

File boundaries, from fable-lead: doctor.rs (the archive finish, the spec walk, the conflict and timeline advice), the archive finish in sessions.rs and next's line in main.rs. t4b keeps the stale-pointer check, session_md.rs, stage.rs, change.rs and the inbox, shelve and session-use code.
